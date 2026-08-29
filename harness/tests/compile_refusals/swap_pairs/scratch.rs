//! Deterministic disposable scratch custody and shell-free Cargo invocation.

use super::render::RenderedSource;
use macroonz_harness::report::{
    ForeignText, InfrastructureFailure, InfrastructureFault, SkipReason,
};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HostFailure {
    NotRun {
        reason: SkipReason,
        detail: ForeignText,
    },
    Infrastructure(InfrastructureFailure),
}

/// One atomically claimed qualification run.
pub(crate) struct Scratch {
    qualification: PathBuf,
    root: PathBuf,
    manifest: PathBuf,
    target: PathBuf,
}

impl Scratch {
    pub(crate) fn claimed() -> Result<Self, String> {
        let qualification = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("qualification")
            .join("depot-swap-refusals");
        fs::create_dir_all(&qualification).map_err(|error| {
            format!(
                "could not create qualification root {}: {error}",
                qualification.display()
            )
        })?;

        for ordinal in 0_u64.. {
            let root = qualification.join(format!("run-{ordinal}"));
            match fs::create_dir(&root) {
                Ok(()) => return Self::at(qualification, root),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => {
                    return Err(format!(
                        "could not claim qualification run {}: {error}",
                        root.display()
                    ));
                }
            }
        }

        Err("the qualification run ordinal space was exhausted".to_owned())
    }

    fn at(qualification: PathBuf, root: PathBuf) -> Result<Self, String> {
        let source = root.join("src").join("bin");
        fs::create_dir_all(&source)
            .map_err(|error| format!("could not create {}: {error}", source.display()))?;
        let manifest = root.join("Cargo.toml");
        fs::write(
            &manifest,
            concat!(
                "[package]\n",
                "name = \"macroonz-swap-pair-challenges\"\n",
                "version = \"0.0.0\"\n",
                "edition = \"2024\"\n",
                "rust-version = \"1.98\"\n",
                "publish = false\n\n",
                "[dependencies]\n",
                "macroonz-harness = { path = \"../../../../harness\", default-features = false }\n\n",
                "bakery = { package = \"macroonz\", path = \"../../../..\", default-features = false, features = [\"harness\"] }\n\n",
                "[workspace]\n",
            ),
        )
        .map_err(|error| format!("could not write {}: {error}", manifest.display()))?;
        Ok(Self {
            qualification,
            target: root.join("build"),
            root,
            manifest,
        })
    }

    pub(super) fn write(&self, rendered: &RenderedSource) -> Result<(), String> {
        self.write_source(&rendered.file_name, &rendered.source)
    }

    pub(crate) fn write_source(&self, file_name: &str, source: &str) -> Result<(), String> {
        let path = self.root.join("src").join("bin").join(file_name);
        fs::write(&path, source)
            .map_err(|error| format!("could not write {}: {error}", path.display()))
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn generate_lockfile(&self) -> Result<(), HostFailure> {
        self.require_toolchain()?;
        let output = Command::new("cargo")
            .arg("+1.98.0")
            .arg("generate-lockfile")
            .arg("--manifest-path")
            .arg(&self.manifest)
            .arg("--offline")
            .current_dir(&self.root)
            .output()
            .map_err(|error| {
                infrastructure(
                    InfrastructureFault::BackendInitializationFailed,
                    &format!("could not launch Cargo lock generation: {error}"),
                )
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(infrastructure(
                InfrastructureFault::BackendInitializationFailed,
                &failed_command("scratch lock generation", &output),
            ))
        }
    }

    pub(crate) fn check(&self, bin_name: &str) -> Result<Output, HostFailure> {
        Command::new("cargo")
            .arg("+1.98.0")
            .arg("check")
            .arg("--manifest-path")
            .arg(&self.manifest)
            .arg("--bin")
            .arg(bin_name)
            .arg("--target-dir")
            .arg(&self.target)
            .arg("--locked")
            .arg("--offline")
            .arg("--color")
            .arg("never")
            .arg("--message-format")
            .arg("json")
            .current_dir(&self.root)
            .output()
            .map_err(|error| {
                infrastructure(
                    InfrastructureFault::BackendInitializationFailed,
                    &format!("could not launch Cargo for {bin_name}: {error}"),
                )
            })
    }

    fn require_toolchain(&self) -> Result<(), HostFailure> {
        let output = Command::new("cargo")
            .arg("+1.98.0")
            .arg("--version")
            .current_dir(&self.root)
            .output()
            .map_err(|error| {
                infrastructure(
                    InfrastructureFault::BackendInitializationFailed,
                    &format!("could not launch Cargo toolchain preflight: {error}"),
                )
            })?;
        let availability = if output.status.success() {
            ToolchainAvailability::Available
        } else {
            ToolchainAvailability::Unavailable
        };
        classify_toolchain(
            availability,
            &failed_command("toolchain preflight", &output),
        )
    }

    pub(crate) fn finish(self, outcome: Result<(), String>) -> Result<(), String> {
        let cleanup = fs::remove_dir_all(&self.root)
            .map_err(|error| format!("could not remove {}: {error}", self.root.display()))
            .and_then(|()| match fs::remove_dir(&self.qualification) {
                Ok(()) => Ok(()),
                Err(error)
                    if matches!(
                        error.kind(),
                        ErrorKind::NotFound | ErrorKind::DirectoryNotEmpty
                    ) =>
                {
                    Ok(())
                }
                Err(error) => Err(format!(
                    "could not remove empty qualification root {}: {error}",
                    self.qualification.display()
                )),
            });
        match (outcome, cleanup) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(failure), Ok(())) | (Ok(()), Err(failure)) => Err(failure),
            (Err(failure), Err(cleanup_failure)) => Err(format!(
                "{failure}\nscratch cleanup also failed: {cleanup_failure}"
            )),
        }
    }
}

#[derive(Clone, Copy)]
enum ToolchainAvailability {
    Available,
    Unavailable,
}

fn classify_toolchain(
    availability: ToolchainAvailability,
    detail: &str,
) -> Result<(), HostFailure> {
    match availability {
        ToolchainAvailability::Available => Ok(()),
        ToolchainAvailability::Unavailable => Err(HostFailure::NotRun {
            reason: SkipReason::PrerequisiteAbsent,
            detail: ForeignText::admitted(detail.as_bytes()),
        }),
    }
}

fn infrastructure(fault: InfrastructureFault, detail: &str) -> HostFailure {
    HostFailure::Infrastructure(InfrastructureFailure::recorded(
        fault,
        Some(ForeignText::admitted(detail.as_bytes())),
    ))
}

#[test]
fn unavailable_toolchain_and_spawn_failure_keep_distinct_host_standing() {
    let unavailable = classify_toolchain(ToolchainAvailability::Unavailable, "toolchain missing");
    assert!(matches!(
        unavailable,
        Err(HostFailure::NotRun {
            reason: SkipReason::PrerequisiteAbsent,
            detail: _,
        })
    ));
    let spawn = infrastructure(
        InfrastructureFault::BackendInitializationFailed,
        "spawn refused",
    );
    assert!(matches!(
        spawn,
        HostFailure::Infrastructure(ref failure)
            if failure.fault() == InfrastructureFault::BackendInitializationFailed
    ));
}

pub(super) fn failed_command(context: &str, output: &Output) -> String {
    format!(
        "{context} exited with {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

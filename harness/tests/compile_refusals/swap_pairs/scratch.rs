//! Deterministic disposable scratch custody and shell-free Cargo invocation.

use super::render::RenderedSource;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// One atomically claimed qualification run.
pub(super) struct Scratch {
    root: PathBuf,
    manifest: PathBuf,
    target: PathBuf,
}

impl Scratch {
    pub(super) fn claimed() -> Result<Self, String> {
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
                Ok(()) => return Self::at(root),
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

    fn at(root: PathBuf) -> Result<Self, String> {
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
                "[workspace]\n",
            ),
        )
        .map_err(|error| format!("could not write {}: {error}", manifest.display()))?;
        Ok(Self {
            target: root.join("build"),
            root,
            manifest,
        })
    }

    pub(super) fn write(&self, rendered: &RenderedSource) -> Result<(), String> {
        let path = self.root.join("src").join("bin").join(&rendered.file_name);
        fs::write(&path, &rendered.source)
            .map_err(|error| format!("could not write {}: {error}", path.display()))
    }

    pub(super) fn generate_lockfile(&self) -> Result<(), String> {
        let output = Command::new("cargo")
            .arg("+1.98.0")
            .arg("generate-lockfile")
            .arg("--manifest-path")
            .arg(&self.manifest)
            .arg("--offline")
            .current_dir(&self.root)
            .output()
            .map_err(|error| format!("could not launch Cargo lock generation: {error}"))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(failed_command("scratch lock generation", &output))
        }
    }

    pub(super) fn check(&self, bin_name: &str) -> Result<Output, String> {
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
            .map_err(|error| format!("could not launch Cargo for {bin_name}: {error}"))
    }
}

pub(super) fn failed_command(context: &str, output: &Output) -> String {
    format!(
        "{context} exited with {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

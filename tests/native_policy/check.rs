//! The required package and feature selections over one unchanged checkout.

use super::probes;
use super::types::PolicyProfiles;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT: AtomicU32 = AtomicU32::new(0);

fn target() -> Result<&'static Path, String> {
    Path::new(env!("CARGO_TARGET_TMPDIR"))
        .parent()
        .ok_or_else(|| "Cargo supplied no target parent".to_owned())
}

pub(super) fn scratch() -> Result<PathBuf, String> {
    let parent = target()?.join("qualification/native-policy");
    std::fs::create_dir_all(&parent).map_err(|error| error.to_string())?;
    for _attempt in 0u16..1_024 {
        let next = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!("check-{}-{next}", std::process::id()));
        match std::fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("no unoccupied native-policy scratch directory remained".to_owned())
}

pub(super) fn cargo(
    root: &Path,
    profile: &Path,
    output: &Path,
    name: &str,
    arguments: &[&str],
) -> Result<Output, String> {
    let command = format!(
        "cargo +1.98.1 {}\nroot={}\nCLIPPY_CONF_DIR={}\n",
        arguments.join(" "),
        root.display(),
        profile.display()
    );
    std::fs::write(output.join(format!("{name}.command.txt")), command)
        .map_err(|error| error.to_string())?;
    let observed = Command::new("cargo")
        .arg("+1.98.1")
        .args(arguments)
        .current_dir(root)
        .env("CARGO_TARGET_DIR", target()?)
        .env("CLIPPY_CONF_DIR", profile)
        .env("CARGO_BUILD_JOBS", "1")
        .env("CARGO_INCREMENTAL", "0")
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("NEXTEST_PROFILE")
        .output()
        .map_err(|error| format!("{name}: {error}"))?;
    for (stream, bytes) in [("stdout", &observed.stdout), ("stderr", &observed.stderr)] {
        std::fs::write(output.join(format!("{name}.{stream}.log")), bytes)
            .map_err(|error| error.to_string())?;
    }
    Ok(observed)
}

fn accepted(name: &str, output: &Output) -> Result<(), String> {
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{name}: {}\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

fn lint(
    root: &Path,
    profile: &Path,
    scratch: &Path,
    name: &str,
    selection: &[&str],
) -> Result<(), String> {
    let arguments = ["clippy", "-j1", "--all-targets", "--locked", "--offline"]
        .into_iter()
        .chain(selection.iter().copied())
        .chain(["--", "-Dwarnings"])
        .collect::<Vec<_>>();
    accepted(name, &cargo(root, profile, scratch, name, &arguments)?)
}

pub(super) fn qualify() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let policy_path = root.join("clippy.toml");
    let profiles = PolicyProfiles::derive(
        std::fs::read_to_string(&policy_path).map_err(|error| error.to_string())?,
    )?;
    let scratch = scratch()?;
    let strict = scratch.join("strict");
    let native = scratch.join("native");
    for (path, text) in [(&strict, profiles.strict()), (&native, profiles.native())] {
        std::fs::create_dir(path).map_err(|error| error.to_string())?;
        std::fs::write(path.join("clippy.toml"), text).map_err(|error| error.to_string())?;
    }
    lint(
        root,
        &strict,
        &scratch,
        "core",
        &["--workspace", "--exclude", "macroonz", "--all-features"],
    )?;
    lint(root, &strict, &scratch, "default", &["-p", "macroonz"])?;
    lint(
        root,
        &strict,
        &scratch,
        "diet",
        &["-p", "macroonz", "--no-default-features"],
    )?;
    for feature in ["harness", "preemption", "full"] {
        lint(
            root,
            &strict,
            &scratch,
            feature,
            &[
                "-p",
                "macroonz",
                "--no-default-features",
                "--features",
                feature,
            ],
        )?;
    }
    lint(
        root,
        &native,
        &scratch,
        "native-only",
        &[
            "-p",
            "macroonz",
            "--no-default-features",
            "--features",
            "native-tooling",
        ],
    )?;
    lint(
        root,
        &native,
        &scratch,
        "native",
        &["-p", "macroonz", "--all-features"],
    )?;
    probes::observe(&scratch, &strict, &native)?;
    super::consumer::observe(root, &scratch, &strict)?;
    let after = std::fs::read_to_string(policy_path).map_err(|error| error.to_string())?;
    if after != profiles.strict() {
        return Err("the canonical policy changed during qualification".to_owned());
    }
    Ok(())
}

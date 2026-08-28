//! Example-only host choreography for compiling and supervising the coverage subject.

use macroonz::harness::fuzz::{FuzzExecution, RUSTC_COVERAGE_TOOLCHAIN};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

pub(super) fn declared_rustc() -> Result<PathBuf, String> {
    let output = Command::new("rustup")
        .args(["which", "--toolchain", RUSTC_COVERAGE_TOOLCHAIN, "rustc"])
        .output()
        .map_err(debug)?;
    if !output.status.success() {
        return Err(format!(
            "rustup could not resolve stable Rust {RUSTC_COVERAGE_TOOLCHAIN}: {}",
            output.status
        ));
    }
    let text = String::from_utf8(output.stdout).map_err(debug)?;
    let path = PathBuf::from(text.trim());
    if path.is_absolute() {
        Ok(path)
    } else {
        Err("rustup returned no absolute rustc path".to_owned())
    }
}

pub(super) fn run_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("qualification")
        .join(format!("rustc-coverage-example-{}", std::process::id()))
}

pub(super) fn compile_subject(
    rustc: &Path,
    manifest: &Path,
    run: &Path,
) -> Result<PathBuf, String> {
    let source = manifest
        .join("examples")
        .join("support")
        .join("rustc_coverage_subject.rs");
    let target = run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new(rustc)
        .args([
            "--edition=2024",
            "-C",
            "instrument-coverage",
            "-C",
            "opt-level=0",
        ])
        .arg(source)
        .arg("-o")
        .arg(&target)
        .output()
        .map_err(debug)?;
    if output.status.success() {
        Ok(target)
    } else {
        Err(format!(
            "instrumented subject compilation failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub(super) fn wait_for_exit(child: &mut Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(FuzzExecution::Success)
    } else {
        Ok(FuzzExecution::NonzeroExit(status.code()))
    }
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

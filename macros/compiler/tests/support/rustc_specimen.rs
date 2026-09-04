//! Stable-Rust specimen observation under Cargo-owned scratch custody.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static SPECIMEN_ORDINAL: AtomicU32 = AtomicU32::new(0);

pub(crate) fn observe_rustc(name: &str, source: &str, extra: &[&str]) -> Result<Output, String> {
    let scratch = specimen_path(name);
    let observed = observe_in(&scratch, source, extra);
    let cleaned = clean(&scratch);
    match (observed, cleaned) {
        (Ok(output), Ok(())) => Ok(output),
        (Err(refusal), Ok(())) => Err(refusal),
        (Ok(_), Err(cleanup)) => Err(format!(
            "the Rustc specimen passed but scratch cleanup refused at {}: {cleanup}",
            scratch.display()
        )),
        (Err(refusal), Err(cleanup)) => Err(format!(
            "{refusal}\nRustc specimen scratch cleanup also refused at {}: {cleanup}",
            scratch.display()
        )),
    }
}

fn specimen_path(name: &str) -> PathBuf {
    let ordinal = SPECIMEN_ORDINAL.fetch_add(1, Ordering::SeqCst);
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "macroonz_rustc_{name}_{}_{ordinal}",
        std::process::id()
    ))
}

fn observe_in(scratch: &Path, source: &str, extra: &[&str]) -> Result<Output, String> {
    std::fs::create_dir_all(scratch).map_err(|error| error.to_string())?;
    let source_path = scratch.join("specimen.rs");
    let executable = scratch.join(format!("specimen{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&source_path, source).map_err(|error| error.to_string())?;
    let mut command = Command::new("rustup");
    command
        .arg("run")
        .arg("1.98.1")
        .arg("rustc")
        .arg(&source_path)
        .arg("--edition=2024")
        .arg("-o")
        .arg(&executable)
        .args(extra);
    let compiled = command.output().map_err(|error| error.to_string())?;
    if compiled.status.success() {
        let executed = Command::new(&executable)
            .output()
            .map_err(|error| error.to_string())?;
        if !executed.status.success() {
            return Err(String::from_utf8_lossy(&executed.stderr).into_owned());
        }
    }
    Ok(compiled)
}

fn clean(scratch: &Path) -> Result<(), String> {
    match std::fs::remove_dir_all(scratch) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

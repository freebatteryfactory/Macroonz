//! A contaminated outer process checks explicit environment and working-directory custody.

use super::process::{completed, input, invocation, limits};
use macroonz::native_process::{self, CaptureEnd, ProcessRequest, ProcessStop};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

#[test]
fn native_invocation_drops_inherited_environment() -> Result<(), String> {
    let root = super::check::scratch()?;
    compile_subject(&root)?;
    let mut command =
        std::process::Command::new(std::env::current_exe().map_err(|error| error.to_string())?);
    command
        .args(super::process::arguments("process_child::subject"))
        .env("MACROONZ_PROCESS_INHERITED", "must-not-cross")
        .stdin(std::process::Stdio::from(input(&root, "environment")?));
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    let output = command.output().map_err(|error| error.to_string())?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    assert!(text.contains("MACROONZ_PROCESS_DECLARED=kept"));
    assert!(!text.contains("MACROONZ_PROCESS_INHERITED"));
    assert_eq!(
        std::fs::read(root.join("declared-directory")).map_err(|error| error.to_string())?,
        b"observed"
    );
    Ok(())
}

pub(super) fn inspect(root: &Path) -> Result<(), String> {
    let request = ProcessRequest::informed(
        root.join(format!(
            "environment-subject{}",
            std::env::consts::EXE_SUFFIX
        )),
        root.to_path_buf(),
        Vec::new(),
        vec![("MACROONZ_PROCESS_DECLARED".to_owned(), "kept".to_owned())],
        limits(Duration::from_secs(3), Duration::from_secs(3), 8192, 8192)?,
        &[],
    )
    .map_err(|error| error.to_string())?;
    let output =
        completed(native_process::run(&request, None).map_err(|error| error.to_string())?)?;
    assert_eq!(output.stop(), &ProcessStop::Exited);
    assert!(output.status().success(), "{output:?}");
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    std::io::stdout()
        .write_all(output.stdout().bytes())
        .map_err(|error| error.to_string())
}

fn compile_subject(root: &Path) -> Result<(), String> {
    let source = root.join("environment_subject.rs");
    std::fs::write(&source, include_str!("environment_subject.rs"))
        .map_err(|error| error.to_string())?;
    let output = std::process::Command::new("rustc")
        .arg("+1.98.1")
        .arg(&source)
        .args([
            "--edition=2024",
            "--forbid=unsafe_code",
            "--deny=warnings",
            "-o",
        ])
        .arg(root.join(format!(
            "environment-subject{}",
            std::env::consts::EXE_SUFFIX
        )))
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

#[test]
fn null_input_reaches_eof() -> Result<(), String> {
    let root = super::check::scratch()?;
    let bounds = limits(Duration::from_secs(3), Duration::from_secs(3), 8192, 8192)?;
    let request = invocation(&root, bounds)?;
    let output =
        completed(native_process::run(&request, None).map_err(|error| error.to_string())?)?;
    assert_eq!(output.stop(), &ProcessStop::Exited);
    assert!(!output.status().success());
    assert!(String::from_utf8_lossy(output.stderr().bytes()).contains("unknown controlled mode"));
    Ok(())
}

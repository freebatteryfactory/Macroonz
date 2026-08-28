//! Safe process execution for one stable rustc coverage observation.

use super::{
    CoverageObservation, FuzzExecution, RustcProfileRefusal, RustcProfileRequest,
    RustcProfileResult, read_lcov,
};
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::process::{Child, Command, Stdio};

/// Run one candidate through an already-instrumented target and read its coverage profile.
///
/// The candidate is written to target standard input.
///
/// The caller supplies exact target, tool, scratch, and process-supervision facts.
///
/// The supervisor owns waiting, any deadline or resource policy, termination, and the resulting [`FuzzExecution`] classification.
///
/// # Errors
///
/// Refuses an empty candidate, an existing deterministic case directory, process or filesystem failures, a missing successful profile, tool failures, or malformed coverage output.
pub fn observe_rustc_profile(
    request: &RustcProfileRequest,
    candidate: &[u8],
    case: u64,
    supervise: impl FnOnce(&mut Child) -> Result<FuzzExecution, String>,
) -> Result<RustcProfileResult, RustcProfileRefusal> {
    if candidate.is_empty() {
        return Err(RustcProfileRefusal::EmptyCandidate);
    }
    fs::create_dir_all(request.scratch())
        .map_err(|error| RustcProfileRefusal::CreateCase(error.to_string()))?;
    let case_directory = request.scratch().join(format!("case-{case:020}"));
    match fs::create_dir(&case_directory) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(RustcProfileRefusal::CaseAlreadyExists(case_directory));
        }
        Err(error) => return Err(RustcProfileRefusal::CreateCase(error.to_string())),
    }
    let raw = case_directory.join("coverage.profraw");
    let merged = case_directory.join("coverage.profdata");
    let execution = run_target(request, candidate, &raw, supervise)?;
    if !raw.is_file() {
        if matches!(execution, FuzzExecution::Success) {
            return Err(RustcProfileRefusal::MissingProfile);
        }
        return Ok(RustcProfileResult::established(
            execution,
            CoverageObservation::empty(),
        ));
    }
    merge_profile(request, &raw, &merged)?;
    let observation = export_coverage(request, &merged)?;
    Ok(RustcProfileResult::established(execution, observation))
}

fn run_target(
    request: &RustcProfileRequest,
    candidate: &[u8],
    raw: &std::path::Path,
    supervise: impl FnOnce(&mut Child) -> Result<FuzzExecution, String>,
) -> Result<FuzzExecution, RustcProfileRefusal> {
    let mut child = Command::new(request.target().executable())
        .args(request.target().arguments())
        .env("LLVM_PROFILE_FILE", raw)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| RustcProfileRefusal::StartTarget(error.to_string()))?;
    let Some(mut input) = child.stdin.take() else {
        return Err(RustcProfileRefusal::WriteTarget(
            "target standard input was not piped".to_owned(),
        ));
    };
    input
        .write_all(candidate)
        .map_err(|error| RustcProfileRefusal::WriteTarget(error.to_string()))?;
    drop(input);
    supervise(&mut child).map_err(RustcProfileRefusal::SuperviseTarget)
}

fn merge_profile(
    request: &RustcProfileRequest,
    raw: &std::path::Path,
    merged: &std::path::Path,
) -> Result<(), RustcProfileRefusal> {
    let status = Command::new(request.tools().profdata())
        .arg("merge")
        .arg("-sparse")
        .arg(raw)
        .arg("-o")
        .arg(merged)
        .status()
        .map_err(|error| RustcProfileRefusal::StartProfdata(error.to_string()))?;
    if status.success() {
        Ok(())
    } else {
        Err(RustcProfileRefusal::ProfdataFailed(status.code()))
    }
}

fn export_coverage(
    request: &RustcProfileRequest,
    merged: &std::path::Path,
) -> Result<CoverageObservation, RustcProfileRefusal> {
    let mut profile_argument = OsString::from("-instr-profile=");
    profile_argument.push(merged.as_os_str());
    let output = Command::new(request.tools().cov())
        .arg("export")
        .arg("-format=lcov")
        .arg(profile_argument)
        .arg(request.target().executable())
        .output()
        .map_err(|error| RustcProfileRefusal::StartCov(error.to_string()))?;
    if !output.status.success() {
        return Err(RustcProfileRefusal::CovFailed(output.status.code()));
    }
    read_lcov(&output.stdout).map_err(RustcProfileRefusal::Coverage)
}

//! Safe process execution for one stable rustc coverage observation.

use super::{
    CoverageObservation, FuzzExecution, ReadyPreflight, RustcProfileRefusal, RustcProfileResult,
    read_lcov,
};
use std::ffi::OsString;
use std::fs::{self, File};
use std::process::{Child, Command, Stdio};

/// Run one candidate through an already-instrumented target and read its coverage profile.
///
/// The candidate is written to target standard input.
///
/// The exact bytes are materialized before target start and the resulting file is opened as standard input, so supervision begins without pipe-writer backpressure.
///
/// Informed readiness supplies the exact target, matching tools, source root, and scratch storage.
///
/// The supervisor owns waiting, any deadline or resource policy, and the resulting [`FuzzExecution`] classification.
///
/// This operation accepts that classification only after the child has ended and terminates and reaps the child on every post-spawn refusal path.
///
/// # Errors
///
/// Refuses an empty candidate, an existing deterministic case directory, process or filesystem failures, a missing successful profile, tool failures, or malformed coverage output.
pub fn observe_rustc_profile(
    ready: &ReadyPreflight,
    candidate: &[u8],
    case: u64,
    supervise: impl FnOnce(&mut Child) -> Result<FuzzExecution, String>,
) -> Result<RustcProfileResult, RustcProfileRefusal> {
    if candidate.is_empty() {
        return Err(RustcProfileRefusal::EmptyCandidate);
    }
    fs::create_dir_all(ready.scratch())
        .map_err(|error| RustcProfileRefusal::CreateCase(error.to_string()))?;
    let case_directory = ready.scratch().join(format!("case-{case:020}"));
    match fs::create_dir(&case_directory) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(RustcProfileRefusal::CaseAlreadyExists(case_directory));
        }
        Err(error) => return Err(RustcProfileRefusal::CreateCase(error.to_string())),
    }
    let raw = case_directory.join("coverage.profraw");
    let merged = case_directory.join("coverage.profdata");
    let input_path = case_directory.join("candidate.bin");
    fs::write(&input_path, candidate)
        .map_err(|error| RustcProfileRefusal::WriteCandidate(error.to_string()))?;
    let execution = run_target(ready, &input_path, &raw, supervise)?;
    if !raw.is_file() {
        if matches!(execution, FuzzExecution::Success) {
            return Err(RustcProfileRefusal::MissingProfile);
        }
        return Ok(RustcProfileResult::established(
            execution,
            CoverageObservation::empty(),
        ));
    }
    merge_profile(ready, &raw, &merged)?;
    let observation = export_coverage(ready, &merged)?;
    Ok(RustcProfileResult::established(execution, observation))
}

fn run_target(
    ready: &ReadyPreflight,
    input_path: &std::path::Path,
    raw: &std::path::Path,
    supervise: impl FnOnce(&mut Child) -> Result<FuzzExecution, String>,
) -> Result<FuzzExecution, RustcProfileRefusal> {
    let input = File::open(input_path)
        .map_err(|error| RustcProfileRefusal::OpenCandidate(error.to_string()))?;
    let child = Command::new(ready.target().executable())
        .args(ready.target().arguments())
        .env("LLVM_PROFILE_FILE", raw)
        .stdin(Stdio::from(input))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| RustcProfileRefusal::StartTarget(error.to_string()))?;
    let mut process = TargetProcess::running(child);
    let execution = match supervise(process.child_mut()) {
        Ok(execution) => execution,
        Err(error) => {
            return process.refuse(RustcProfileRefusal::SuperviseTarget(error));
        }
    };
    process.finish(execution)
}

fn merge_profile(
    ready: &ReadyPreflight,
    raw: &std::path::Path,
    merged: &std::path::Path,
) -> Result<(), RustcProfileRefusal> {
    let status = Command::new(ready.tools().profdata())
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
    ready: &ReadyPreflight,
    merged: &std::path::Path,
) -> Result<CoverageObservation, RustcProfileRefusal> {
    let mut profile_argument = OsString::from("-instr-profile=");
    profile_argument.push(merged.as_os_str());
    let output = Command::new(ready.tools().cov())
        .arg("export")
        .arg("-format=lcov")
        .arg(profile_argument)
        .arg(ready.target().executable())
        .output()
        .map_err(|error| RustcProfileRefusal::StartCov(error.to_string()))?;
    if !output.status.success() {
        return Err(RustcProfileRefusal::CovFailed(output.status.code()));
    }
    read_lcov(ready.source_root(), &output.stdout).map_err(RustcProfileRefusal::Coverage)
}

/// One started target that remains responsible for termination and reaping until it proves otherwise.
struct TargetProcess {
    child: Child,
    custody: ProcessCustody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessCustody {
    Running,
    Reaped,
}

impl TargetProcess {
    fn running(child: Child) -> Self {
        Self {
            child,
            custody: ProcessCustody::Running,
        }
    }

    fn child_mut(&mut self) -> &mut Child {
        &mut self.child
    }

    fn finish(mut self, execution: FuzzExecution) -> Result<FuzzExecution, RustcProfileRefusal> {
        let status = match self.child_mut().try_wait() {
            Ok(status) => status,
            Err(error) => {
                return self.refuse(RustcProfileRefusal::InspectTarget(error.to_string()));
            }
        };
        if status.is_none() {
            return self.refuse(RustcProfileRefusal::SupervisorReturnedBeforeExit);
        }
        self.custody = ProcessCustody::Reaped;
        Ok(execution)
    }

    fn refuse(
        mut self,
        refusal: RustcProfileRefusal,
    ) -> Result<FuzzExecution, RustcProfileRefusal> {
        match self.terminate_and_reap() {
            Ok(()) => Err(refusal),
            Err(cleanup) => Err(RustcProfileRefusal::CleanupTarget {
                after: Box::new(refusal),
                cleanup,
            }),
        }
    }

    fn terminate_and_reap(&mut self) -> Result<(), String> {
        if let Ok(Some(_status)) = self.child_mut().try_wait() {
            self.custody = ProcessCustody::Reaped;
            return Ok(());
        }
        self.child_mut()
            .kill()
            .map_err(|error| format!("target termination failed: {error}"))?;
        self.child_mut()
            .wait()
            .map_err(|error| format!("target reap failed: {error}"))?;
        self.custody = ProcessCustody::Reaped;
        Ok(())
    }
}

impl Drop for TargetProcess {
    fn drop(&mut self) {
        if matches!(self.custody, ProcessCustody::Running) {
            let _cleanup = self.terminate_and_reap();
        }
    }
}

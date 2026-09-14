//! Coverage case execution through the caller's selected process executor.

use super::{
    CoverageCaseCleanup, CoverageCommand, CoverageCorpus, CoverageHostFailure, CoverageInvocation,
    CoverageObservation, CoverageReply, FuzzExecution, ReadyPreflight, RustcProfileRefusal,
    RustcProfileResult, read_lcov_mapped,
};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::Read;
use std::process::{Child, Command, Output, Stdio};

/// Runs one candidate with caller-supervised target execution and the default LLVM process road.
///
/// # Errors
/// Refuses invalid candidate standing, case storage, incomplete target supervision or coverage extraction.
pub fn observe_rustc_profile(
    ready: &ReadyPreflight,
    corpus: &mut CoverageCorpus,
    candidate: &[u8],
    supervise: impl FnOnce(&mut Child) -> Result<FuzzExecution, String>,
) -> Result<RustcProfileResult, RustcProfileRefusal> {
    let mut supervise = Some(supervise);
    observe_rustc_profile_with(ready, corpus, candidate, |invocation| {
        legacy_invocation(invocation, &mut supervise)
    })
    .map_err(legacy_failure)
}

/// Joins one candidate to target and LLVM observations from a declared executor.
///
/// # Errors
/// Retains executor failure separately, with case cleanup that must wait for any unfinished child.
pub fn observe_rustc_profile_with<E>(
    ready: &ReadyPreflight,
    corpus: &mut CoverageCorpus,
    candidate: &[u8],
    mut execute: impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<RustcProfileResult, CoverageHostFailure<E, RustcProfileRefusal>> {
    if candidate.is_empty() {
        return Err(RustcProfileRefusal::EmptyCandidate.into());
    }
    let case = corpus.reserve_execution(ready, candidate.len())?;
    fs::create_dir_all(ready.scratch())
        .map_err(|error| RustcProfileRefusal::CreateCase(error.to_string()))?;
    let directory = ready.scratch().join(format!("case-{case:020}"));
    match fs::create_dir(&directory) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(RustcProfileRefusal::CaseAlreadyExists(directory).into());
        }
        Err(error) => return Err(RustcProfileRefusal::CreateCase(error.to_string()).into()),
    }
    let result = observe_case(ready, candidate, case, &directory, &mut execute);
    let completed = match result {
        Ok(result) => Ok(result),
        Err(CoverageHostFailure::Refused(refusal)) => Err(refusal),
        Err(CoverageHostFailure::Executor {
            operation, error, ..
        }) => {
            return Err(CoverageHostFailure::Executor {
                operation,
                error,
                cleanup: Some(CoverageCaseCleanup::retained(directory)),
            });
        }
    };
    match fs::remove_dir_all(&directory) {
        Ok(()) => completed.map_err(CoverageHostFailure::Refused),
        Err(error) => Err(RustcProfileRefusal::CleanupCase {
            after: completed.err().map(Box::new),
            cleanup: error.to_string(),
        }
        .into()),
    }
}

fn observe_case<E>(
    ready: &ReadyPreflight,
    candidate: &[u8],
    case: u32,
    directory: &std::path::Path,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<RustcProfileResult, CoverageHostFailure<E, RustcProfileRefusal>> {
    let raw = directory.join("coverage.profraw");
    let merged = directory.join("coverage.profdata");
    let input_path = directory.join("candidate.bin");
    fs::write(&input_path, candidate)
        .map_err(|error| RustcProfileRefusal::WriteCandidate(error.to_string()))?;
    let input = File::open(input_path)
        .map_err(|error| RustcProfileRefusal::OpenCandidate(error.to_string()))?;
    let mut command = Command::new(ready.target().executable());
    command
        .args(ready.target().arguments())
        .env("LLVM_PROFILE_FILE", &raw);
    let reply = call(
        CoverageInvocation {
            operation: CoverageCommand::Target,
            command,
            input: Some(input),
            output_bound: None,
        },
        execute,
    )?;
    let CoverageReply::Target(execution) = reply else {
        return Err(RustcProfileRefusal::UnexpectedExecutorReply.into());
    };
    let observation = if raw.is_file() {
        merge_profile(ready, &raw, &merged, execute)?;
        export_coverage(ready, &merged, execute)?
    } else if execution == FuzzExecution::Success {
        return Err(RustcProfileRefusal::MissingProfile.into());
    } else {
        CoverageObservation::empty()
    };
    Ok(RustcProfileResult::established(
        case,
        candidate.to_vec(),
        execution,
        observation,
        ready.standing().clone(),
    ))
}

fn merge_profile<E>(
    ready: &ReadyPreflight,
    raw: &std::path::Path,
    merged: &std::path::Path,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<(), CoverageHostFailure<E, RustcProfileRefusal>> {
    let mut command = Command::new(ready.tools().profdata());
    command
        .args(["merge", "-sparse"])
        .arg(raw)
        .arg("-o")
        .arg(merged);
    let reply = call(
        CoverageInvocation {
            operation: CoverageCommand::Merge,
            command,
            input: None,
            output_bound: None,
        },
        execute,
    )?;
    let CoverageReply::Output(output) = reply else {
        return Err(RustcProfileRefusal::UnexpectedExecutorReply.into());
    };
    if output.status.success() {
        Ok(())
    } else {
        Err(RustcProfileRefusal::ProfdataFailed(output.status.code()).into())
    }
}

fn export_coverage<E>(
    ready: &ReadyPreflight,
    merged: &std::path::Path,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<CoverageObservation, CoverageHostFailure<E, RustcProfileRefusal>> {
    let mut profile_argument = OsString::from("-instr-profile=");
    profile_argument.push(merged.as_os_str());
    let mut command = Command::new(ready.tools().cov());
    command
        .args(["export", "-format=lcov"])
        .arg(profile_argument)
        .arg(ready.target().executable());
    let bound = ready.standing().campaign().budgets().export_bytes();
    let reply = call(
        CoverageInvocation {
            operation: CoverageCommand::Export,
            command,
            input: None,
            output_bound: Some(bound),
        },
        execute,
    )?;
    let CoverageReply::Output(output) = reply else {
        return Err(RustcProfileRefusal::UnexpectedExecutorReply.into());
    };
    let observed_at_least = u64::try_from(output.stdout.len()).unwrap_or(u64::MAX);
    if observed_at_least > bound {
        return Err(RustcProfileRefusal::CovOutputBudgetExhausted {
            bound,
            observed_at_least,
        }
        .into());
    }
    if !output.status.success() {
        return Err(RustcProfileRefusal::CovFailed(output.status.code()).into());
    }
    read_lcov_mapped(ready.source_roots(), &output.stdout)
        .map_err(|error| RustcProfileRefusal::Coverage(error).into())
}

fn call<E>(
    invocation: CoverageInvocation,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<CoverageReply, CoverageHostFailure<E, RustcProfileRefusal>> {
    let operation = invocation.operation;
    execute(invocation).map_err(|error| CoverageHostFailure::Executor {
        operation,
        error,
        cleanup: None,
    })
}

fn legacy_failure(
    failure: CoverageHostFailure<RustcProfileRefusal, RustcProfileRefusal>,
) -> RustcProfileRefusal {
    match failure {
        CoverageHostFailure::Refused(refusal) => refusal,
        CoverageHostFailure::Executor { error, cleanup, .. } => {
            if let Some(cleanup) = cleanup
                && let Err((_retained, cleanup_error)) = cleanup.remove()
            {
                return RustcProfileRefusal::CleanupCase {
                    after: Some(Box::new(error)),
                    cleanup: cleanup_error.to_string(),
                };
            }
            error
        }
    }
}

fn legacy_invocation<F: FnOnce(&mut Child) -> Result<FuzzExecution, String>>(
    mut invocation: CoverageInvocation,
    supervise: &mut Option<F>,
) -> Result<CoverageReply, RustcProfileRefusal> {
    match invocation.operation {
        CoverageCommand::Target => {
            let input = invocation.input.ok_or_else(|| {
                RustcProfileRefusal::OpenCandidate("target input missing".to_owned())
            })?;
            let child = invocation
                .command
                .stdin(Stdio::from(input))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|error| RustcProfileRefusal::StartTarget(error.to_string()))?;
            let mut process = TargetProcess::running(child);
            let Some(supervise) = supervise.take() else {
                return process
                    .refuse(RustcProfileRefusal::UnexpectedExecutorReply)
                    .map(CoverageReply::Target);
            };
            let execution = match supervise(process.child_mut()) {
                Ok(execution) => execution,
                Err(error) => {
                    return process
                        .refuse(RustcProfileRefusal::SuperviseTarget(error))
                        .map(CoverageReply::Target);
                }
            };
            process.finish(execution).map(CoverageReply::Target)
        }
        CoverageCommand::Merge => invocation
            .command
            .status()
            .map(|status| {
                CoverageReply::Output(Output {
                    status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            })
            .map_err(|error| RustcProfileRefusal::StartProfdata(error.to_string())),
        CoverageCommand::Export => {
            let child = invocation
                .command
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|error| RustcProfileRefusal::StartCov(error.to_string()))?;
            CovProcess::running(child)
                .bounded_output(invocation.output_bound.unwrap_or(0))
                .map(CoverageReply::Output)
        }
        CoverageCommand::Rustc(_) | CoverageCommand::Version(_) => {
            Err(RustcProfileRefusal::UnexpectedExecutorReply)
        }
    }
}

/// One started target that remains responsible for termination and reaping until it proves otherwise.
struct TargetProcess {
    process: OwnedChild,
}

/// Termination custody shared by the fallback's distinct completion policies.
struct OwnedChild {
    child: Child,
    custody: ProcessCustody,
    context: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessCustody {
    Running,
    Reaped,
}

impl TargetProcess {
    fn running(child: Child) -> Self {
        Self {
            process: OwnedChild::running(child, "target"),
        }
    }

    fn child_mut(&mut self) -> &mut Child {
        &mut self.process.child
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
        self.process.custody = ProcessCustody::Reaped;
        Ok(execution)
    }

    fn refuse(
        mut self,
        refusal: RustcProfileRefusal,
    ) -> Result<FuzzExecution, RustcProfileRefusal> {
        match self.process.terminate_and_reap() {
            Ok(()) => Err(refusal),
            Err(cleanup) => Err(RustcProfileRefusal::CleanupTarget {
                after: Box::new(refusal),
                cleanup,
            }),
        }
    }
}

/// One coverage-export process that remains responsible for termination and reaping until it proves otherwise.
struct CovProcess {
    process: OwnedChild,
}

impl CovProcess {
    fn running(child: Child) -> Self {
        Self {
            process: OwnedChild::running(child, "coverage export"),
        }
    }

    fn bounded_output(mut self, bound: u64) -> Result<Output, RustcProfileRefusal> {
        let Some(stdout) = self.process.child.stdout.take() else {
            return self.refuse(RustcProfileRefusal::ReadCov(
                "coverage export stdout was not piped".to_owned(),
            ));
        };
        let mut output = Vec::new();
        let mut bounded = stdout.take(bound.saturating_add(1));
        if let Err(error) = bounded.read_to_end(&mut output) {
            return self.refuse(RustcProfileRefusal::ReadCov(error.to_string()));
        }
        let observed_at_least = u64::try_from(output.len()).unwrap_or(u64::MAX);
        if observed_at_least > bound {
            return self.refuse(RustcProfileRefusal::CovOutputBudgetExhausted {
                bound,
                observed_at_least,
            });
        }
        let status = match self.process.child.wait() {
            Ok(status) => status,
            Err(error) => {
                return self.refuse(RustcProfileRefusal::WaitCov(error.to_string()));
            }
        };
        self.process.custody = ProcessCustody::Reaped;
        if status.success() {
            Ok(Output {
                status,
                stdout: output,
                stderr: Vec::new(),
            })
        } else {
            Err(RustcProfileRefusal::CovFailed(status.code()))
        }
    }

    fn refuse(mut self, refusal: RustcProfileRefusal) -> Result<Output, RustcProfileRefusal> {
        match self.process.terminate_and_reap() {
            Ok(()) => Err(refusal),
            Err(cleanup) => Err(RustcProfileRefusal::CleanupCov {
                after: Box::new(refusal),
                cleanup,
            }),
        }
    }
}

impl OwnedChild {
    fn running(child: Child, context: &'static str) -> Self {
        Self {
            child,
            custody: ProcessCustody::Running,
            context,
        }
    }

    fn terminate_and_reap(&mut self) -> Result<(), String> {
        if let Ok(Some(_status)) = self.child.try_wait() {
            self.custody = ProcessCustody::Reaped;
            return Ok(());
        }
        self.child
            .kill()
            .map_err(|error| format!("{} termination failed: {error}", self.context))?;
        self.child
            .wait()
            .map_err(|error| format!("{} reap failed: {error}", self.context))?;
        self.custody = ProcessCustody::Reaped;
        Ok(())
    }
}

impl Drop for OwnedChild {
    fn drop(&mut self) {
        if matches!(self.custody, ProcessCustody::Running) {
            let _cleanup = self.terminate_and_reap();
        }
    }
}

use super::{NativeCoverage, NativeCoverageFailure, NativeCoverageProcessError};
use crate::harness::fuzz::{
    CoverageCommand, CoverageCorpus, CoverageInvocation, CoverageReply, FuzzExecution,
    RustcProfileRefusal, RustcProfileResult,
};
use crate::native_process::{
    ProcessError, ProcessLimits, ProcessRequest, ProcessRun, ProcessStop, ProcessTool,
};

/// Executes one candidate and matching LLVM operations through the existing coverage owner.
///
/// # Errors
/// Retains native failures and pending cleanup without admitting a partial coverage observation.
pub fn observe(
    coverage: &NativeCoverage,
    corpus: &mut CoverageCorpus,
    candidate: &[u8],
) -> Result<RustcProfileResult, NativeCoverageFailure<RustcProfileRefusal>> {
    crate::harness::fuzz::observe_rustc_profile_with(
        &coverage.ready,
        corpus,
        candidate,
        |invocation| supervised(&coverage.tool, coverage.target_limits, invocation),
    )
    .map_err(super::cleanup::retain)
}

pub(super) fn supervised(
    tool: &ProcessTool,
    target_limits: ProcessLimits,
    invocation: CoverageInvocation,
) -> Result<CoverageReply, NativeCoverageProcessError> {
    let operation = invocation.operation();
    let limits = if operation == CoverageCommand::Target {
        target_limits
    } else {
        tool.limits()
    };
    let bound = invocation
        .output_bound()
        .map(|bound| usize::try_from(bound).unwrap_or(usize::MAX));
    let limits = ProcessLimits::informed(
        limits.execution(),
        limits.cleanup(),
        bound.map_or(limits.stdout(), |bound| bound.min(limits.stdout())),
        limits.stderr(),
    )
    .map_err(NativeCoverageProcessError::Invalid)?;
    let command = invocation.command();
    let arguments = command
        .get_args()
        .map(text)
        .collect::<Result<Vec<_>, _>>()?;
    let mut environment = tool.environment().to_vec();
    for (key, value) in command.get_envs() {
        let Some(value) = value else {
            return Err(NativeCoverageProcessError::Invalid(
                ProcessError::InvalidRequest(
                    "coverage commands cannot remove explicit environment entries".to_owned(),
                ),
            ));
        };
        environment.push((text(key)?, text(value)?));
    }
    let request = ProcessRequest::informed(
        command.get_program().into(),
        command
            .get_current_dir()
            .unwrap_or(tool.directory())
            .to_path_buf(),
        arguments,
        environment,
        limits,
        &[],
    )
    .map_err(NativeCoverageProcessError::Invalid)?;
    let run = crate::native_process::run(&request, invocation.into_input()).map_err(|error| {
        NativeCoverageProcessError::Start {
            request: Box::new(request.clone()),
            error,
        }
    })?;
    let ProcessRun::Finished(output) = run else {
        return Err(NativeCoverageProcessError::Execution {
            request: Box::new(request),
            run: Box::new(run),
        });
    };
    if operation == CoverageCommand::Target {
        let execution = match output.stop() {
            ProcessStop::Exited if output.status().success() => Some(FuzzExecution::Success),
            ProcessStop::Exited => Some(FuzzExecution::NonzeroExit(output.status().code())),
            ProcessStop::Deadline => Some(FuzzExecution::Timeout),
            ProcessStop::OutputLimit => Some(FuzzExecution::ResourceExhaustion),
            ProcessStop::ObservationFailed(_) => None,
        };
        if let Some(execution) = execution {
            return Ok(CoverageReply::Target(execution));
        }
    } else if output.stop() == &ProcessStop::Exited {
        return Ok(CoverageReply::Output(std::process::Output {
            status: output.status(),
            stdout: output.stdout().bytes().to_vec(),
            stderr: output.stderr().bytes().to_vec(),
        }));
    }
    Err(NativeCoverageProcessError::Execution {
        request: Box::new(request),
        run: Box::new(ProcessRun::Finished(output)),
    })
}

fn text(value: &std::ffi::OsStr) -> Result<String, NativeCoverageProcessError> {
    value.to_str().map(str::to_owned).ok_or_else(|| {
        NativeCoverageProcessError::Invalid(ProcessError::InvalidRequest(
            "coverage arguments and environment must be Unicode".to_owned(),
        ))
    })
}

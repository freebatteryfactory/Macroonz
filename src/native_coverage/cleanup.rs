use super::{NativeCoverageFailure, NativeCoverageProcessError};
use crate::harness::fuzz::CoverageHostFailure;
use crate::native_process::ProcessRun;
use std::time::Duration;

pub(super) fn retain<R>(
    cause: CoverageHostFailure<NativeCoverageProcessError, R>,
) -> NativeCoverageFailure<R> {
    settle(NativeCoverageFailure {
        cause,
        cleanup_error: None,
    })
}

pub(super) fn finish<R>(
    failure: NativeCoverageFailure<R>,
    budget: Duration,
) -> NativeCoverageFailure<R> {
    let cause = match failure.cause {
        CoverageHostFailure::Refused(refusal) => CoverageHostFailure::Refused(refusal),
        CoverageHostFailure::Executor {
            operation,
            error,
            cleanup,
        } => {
            let error = match error {
                NativeCoverageProcessError::Execution { request, run } => {
                    let run = match *run {
                        ProcessRun::Pending(pending) => pending.finish(budget),
                        finished @ ProcessRun::Finished(_) => finished,
                    };
                    NativeCoverageProcessError::Execution {
                        request,
                        run: Box::new(run),
                    }
                }
                invalid @ NativeCoverageProcessError::Invalid(_) => invalid,
                start @ NativeCoverageProcessError::Start { .. } => start,
            };
            CoverageHostFailure::Executor {
                operation,
                error,
                cleanup,
            }
        }
    };
    settle(NativeCoverageFailure {
        cause,
        cleanup_error: failure.cleanup_error,
    })
}

fn settle<R>(mut failure: NativeCoverageFailure<R>) -> NativeCoverageFailure<R> {
    if let CoverageHostFailure::Executor { error, cleanup, .. } = &mut failure.cause {
        if matches!(error, NativeCoverageProcessError::Execution { run, .. } if matches!(run.as_ref(), ProcessRun::Pending(_)))
        {
            return failure;
        }
        if let Some(case) = cleanup.take() {
            match case.remove() {
                Ok(()) => failure.cleanup_error = None,
                Err((retained, removal_error)) => {
                    *cleanup = Some(retained);
                    failure.cleanup_error = Some(removal_error.to_string());
                }
            }
        }
    }
    failure
}

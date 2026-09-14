use super::{NativeCoverageFailure, NativeCoverageProcessError};
use crate::harness::fuzz::CoverageHostFailure;
use crate::native_process::ProcessRun;

impl std::fmt::Display for NativeCoverageProcessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(error) => write!(formatter, "invalid process configuration: {error}"),
            Self::Start { error, .. } => write!(formatter, "process startup failed: {error}"),
            Self::Execution { run, .. } => match run.as_ref() {
                ProcessRun::Finished(output) => write!(
                    formatter,
                    "process stopped: {:?}; status {}",
                    output.stop(),
                    output.status()
                ),
                ProcessRun::Pending(pending) => write!(
                    formatter,
                    "process stopped: {:?}; cleanup pending",
                    pending.stop()
                ),
            },
        }
    }
}

impl std::error::Error for NativeCoverageProcessError {}

impl<R: std::fmt::Debug> std::fmt::Display for NativeCoverageFailure<R> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cause {
            CoverageHostFailure::Refused(refusal) => {
                write!(formatter, "coverage refused: {refusal:?}")
            }
            CoverageHostFailure::Executor {
                operation,
                error,
                cleanup,
            } => write!(
                formatter,
                "coverage {operation:?}: {error}; case cleanup pending: {}",
                cleanup.is_some()
            ),
        }?;
        if let Some(error) = &self.cleanup_error {
            write!(formatter, "; case cleanup failed: {error}")?;
        }
        Ok(())
    }
}

impl<R: std::fmt::Debug> std::error::Error for NativeCoverageFailure<R> {}

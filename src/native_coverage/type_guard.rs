use super::{NativeCoverage, NativeCoverageFailure, NativeCoverageProcessError};
use crate::harness::fuzz::{CoverageCorpus, CoverageHostFailure, ReadyPreflight};
use crate::native_process::{ProcessLimits, ProcessTool};
use std::time::Duration;

impl NativeCoverage {
    /// The explicit process policy used for compiler and LLVM operations.
    #[must_use]
    pub const fn tool(&self) -> &ProcessTool {
        &self.tool
    }

    /// The separately declared target execution and cleanup limits.
    #[must_use]
    pub const fn target_limits(&self) -> ProcessLimits {
        self.target_limits
    }

    /// The readiness and campaign standing established through bounded tool execution.
    #[must_use]
    pub const fn ready(&self) -> &ReadyPreflight {
        &self.ready
    }

    /// Opens the existing coverage frontier under this qualified standing.
    #[must_use]
    pub fn corpus(&self) -> CoverageCorpus {
        CoverageCorpus::opening(&self.ready)
    }
}

impl<R> NativeCoverageFailure<R> {
    /// The owning semantic refusal or exact native process failure.
    pub const fn cause(&self) -> &CoverageHostFailure<NativeCoverageProcessError, R> {
        &self.cause
    }

    /// The latest case-directory cleanup failure, if any.
    #[must_use]
    pub fn cleanup_error(&self) -> Option<&str> {
        self.cleanup_error.as_deref()
    }

    /// Retries unfinished child cleanup and then removes its retained case directory.
    ///
    /// This remains a failed observation and does not refund the attempted case or resume LLVM processing.
    pub fn finish_cleanup(self, budget: Duration) -> Self {
        super::super::cleanup::finish(self, budget)
    }
}

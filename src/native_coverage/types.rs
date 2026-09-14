use crate::harness::fuzz::{CoverageHostFailure, ReadyPreflight};
use crate::native_process::{ProcessError, ProcessLimits, ProcessRequest, ProcessRun, ProcessTool};

#[path = "type_guard.rs"]
mod guard;

/// Qualified coverage standing bound to explicit tool and target process policies.
#[derive(Debug)]
pub struct NativeCoverage {
    pub(super) ready: ReadyPreflight,
    pub(super) tool: ProcessTool,
    pub(super) target_limits: ProcessLimits,
}

/// A native execution failure retaining the exact request and any unfinished process.
#[derive(Debug)]
pub enum NativeCoverageProcessError {
    /// Explicit execution configuration could not be admitted.
    Invalid(ProcessError),
    /// The admitted process could not be started.
    Start {
        /// The exact selected request.
        request: Box<ProcessRequest>,
        /// The native startup failure.
        error: ProcessError,
    },
    /// The process stopped before a tool observation or retains unfinished cleanup.
    Execution {
        /// The exact request that ran.
        request: Box<ProcessRequest>,
        /// The actual process result and its resource custody.
        run: Box<ProcessRun>,
    },
}

/// A coverage refusal or native failure with retained process and case cleanup ownership.
#[derive(Debug)]
#[must_use]
pub struct NativeCoverageFailure<Refusal> {
    pub(super) cause: CoverageHostFailure<NativeCoverageProcessError, Refusal>,
    pub(super) cleanup_error: Option<String>,
}

use std::path::PathBuf;
use std::process::ExitStatus;
use std::time::Duration;

#[path = "type_guard.rs"]
mod guard;

/// Explicit execution and cleanup budgets and retained pipe bounds.
#[derive(Clone, Copy, Debug)]
pub struct ProcessLimits {
    execution: Duration,
    cleanup: Duration,
    stdout: usize,
    stderr: usize,
}

/// An explicitly selected tool, working directory, environment and resource policy.
#[derive(Clone, Debug)]
pub struct ProcessTool {
    executable: PathBuf,
    directory: PathBuf,
    environment: Vec<(String, String)>,
    limits: ProcessLimits,
}

/// A process invocation with an absolute executable and directory and no inherited environment.
#[derive(Clone, Debug)]
pub struct ProcessRequest {
    tool: ProcessTool,
    arguments: Vec<String>,
}

/// A requested resource mechanism whose absence must refuse before execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceControl {
    /// Enforce a resident-memory ceiling in bytes.
    MemoryBytes(u64),
    /// Enforce a process-group CPU-time ceiling.
    CpuTime(Duration),
    /// Prevent network access.
    DenyNetwork,
    /// Prevent an adversarial descendant from escaping process-group custody.
    PreventGroupEscape,
}

/// A refusal before a process has been returned to the supervision loop.
#[derive(Debug)]
pub enum ProcessError {
    /// The declared request cannot be represented faithfully.
    InvalidRequest(String),
    /// The selected native target supplies no process backend.
    Unavailable,
    /// A requested resource mechanism is not enforced by this backend.
    Unsupported(ResourceControl),
    /// Preparing memory or starting the operating-system process failed.
    Start(std::io::Error),
}

/// The observation that ended the execution phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessStop {
    /// The direct child exited before another stop condition was observed.
    Exited,
    /// The monotonic execution deadline was reached.
    Deadline,
    /// At least one output stream exceeded its declared retained bound.
    OutputLimit,
    /// Process or pipe observation failed.
    ObservationFailed(String),
}

/// The extent of one retained output prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaptureEnd {
    /// The pipe reached EOF without exceeding its bound.
    Eof,
    /// Additional bytes were observed beyond the retained prefix.
    LimitExceeded,
    /// Reading failed after the retained prefix.
    Failed(String),
}

/// One bounded output prefix and its independently reported completeness.
#[derive(Debug)]
pub struct CapturedOutput {
    pub(super) bytes: Vec<u8>,
    pub(super) end: CaptureEnd,
}

/// A terminated group request, reaped direct child and joined pipe readers.
#[derive(Debug)]
pub struct ProcessOutput {
    pub(super) status: ExitStatus,
    pub(super) stop: ProcessStop,
    pub(super) stdout: CapturedOutput,
    pub(super) stderr: CapturedOutput,
}

/// A finished observation or retained ownership of unfinished cleanup.
#[derive(Debug)]
#[must_use]
pub enum ProcessRun {
    /// Group termination was requested successfully and direct-child and reader cleanup finished.
    Finished(ProcessOutput),
    /// Cleanup exhausted its budget; the returned value still owns pending resources.
    Pending(Box<PendingProcess>),
}

/// Unfinished cleanup that can be retried with another explicit bounded budget.
#[derive(Debug)]
#[must_use]
pub struct PendingProcess {
    pub(super) running: Running,
}

#[derive(Debug)]
pub(super) struct Running {
    pub child: PlatformChild,
    pub stdout: CaptureTask,
    pub stderr: CaptureTask,
    pub stop: ProcessStop,
    pub cleanup_error: Option<String>,
}

#[derive(Debug)]
pub(super) enum CaptureTask {
    Reading(std::thread::JoinHandle<CapturedOutput>),
    Finished(CapturedOutput),
}

#[derive(Debug)]
pub(super) struct PlatformChild {
    #[cfg(windows)]
    pub inner: Box<dyn process_wrap::std::ChildWrapper>,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub inner: std::process::Child,
    pub termination: Termination,
    pub status: Option<ExitStatus>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Termination {
    #[cfg(any(windows, target_os = "linux", target_os = "macos"))]
    Required,
    Requested,
}

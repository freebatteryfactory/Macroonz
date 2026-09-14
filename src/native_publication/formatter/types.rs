use crate::compiler::identity::Identity;
use crate::native_process::{PendingProcess, ProcessOutput, ProcessRequest, ProcessRun};
use crate::native_publication::{CanonicalPublicationBytes, PublicationPath};
use std::path::PathBuf;

#[path = "type_guard.rs"]
mod guard;

/// An explicitly configured formatter with an observed supported version.
#[derive(Debug)]
pub struct Formatter {
    pub(super) process: ProcessRequest,
    pub(super) query: ProcessRequest,
    pub(super) version: ProcessOutput,
    pub(super) config: PathBuf,
    pub(super) configuration: Vec<u8>,
}

/// A formatter preparation failure retaining any unfinished version query.
#[derive(Debug)]
pub enum FormatError {
    /// The explicit configuration cannot be used faithfully.
    Configuration(String),
    /// Reading configuration or preparing the supplied scratch input failed.
    Filesystem(std::io::Error),
    /// Native process admission or startup failed.
    Process(crate::native_process::ProcessError),
    /// The selected executable did not establish the supported formatter profile.
    Query {
        /// The exact version command.
        request: Box<ProcessRequest>,
        /// The actual observation or still-owned cleanup resources.
        run: Box<ProcessRun>,
    },
}

/// Why formatter execution established no publishable text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatObservationError {
    /// The formatter did not exit successfully with complete bounded capture.
    Process,
    /// Stdout was not UTF-8 source with LF line endings and a terminal newline.
    Text,
    /// The explicit configuration changed or could not be compared after execution.
    Configuration(String),
}

/// Actual formatter output and its unchanged canonical source commitment.
#[derive(Debug)]
pub struct FormatOutput {
    pub(super) context: FormatContext,
    pub(super) process: ProcessOutput,
    pub(super) observation: Result<(), FormatObservationError>,
}

/// Finished formatting or retained ownership of unfinished native cleanup.
#[derive(Debug)]
#[must_use]
pub enum FormatRun {
    /// Native cleanup finished; text admission may still have refused.
    Finished(Box<FormatOutput>),
    /// Native cleanup remains owned and retryable.
    Pending(Box<PendingFormat>),
}

/// The original formatter context and its pending process resources.
#[derive(Debug)]
#[must_use]
pub struct PendingFormat {
    pub(super) context: FormatContext,
    pub(super) process: Box<PendingProcess>,
}

#[derive(Debug)]
pub(super) struct FormatContext {
    pub path: PublicationPath,
    pub canonical: Identity<CanonicalPublicationBytes>,
    pub source: String,
    pub request: ProcessRequest,
    pub version: String,
    pub config: PathBuf,
    pub configuration: Vec<u8>,
}

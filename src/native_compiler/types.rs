use crate::harness::oracle::{DiagnosticAnchor, ObservedCompilation, RelativeSourcePath};
use crate::native_process::{
    PendingProcess, ProcessError, ProcessOutput, ProcessRequest, ProcessStop,
};
use std::path::PathBuf;

#[path = "type_guard.rs"]
mod guard;

/// The single Cargo target selected for a compiler fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CargoTarget {
    /// The package's library target.
    Library,
    /// One explicitly named binary target.
    Binary(String),
}

/// A locked, offline Cargo build with an explicit package, target and output directory.
#[derive(Clone, Debug)]
pub struct CargoFixture {
    manifest: PathBuf,
    target_directory: PathBuf,
    package: String,
    target: CargoTarget,
    triple: String,
}

/// An admitted compiler invocation and its independently selected diagnostic locus.
#[derive(Clone, Debug)]
pub struct CompilerRequest {
    pub(super) process: ProcessRequest,
    pub(super) protocol: Protocol,
    pub(super) locus: RelativeSourcePath,
    pub(super) dependencies: Option<DependencyCapture>,
}

/// A refusal before compiler supervision begins.
#[derive(Debug)]
pub enum CompilerError {
    /// The fixture configuration cannot name the requested observation faithfully.
    Configuration(String),
    /// Native process admission or startup failed.
    Process(ProcessError),
}

/// Why completed process output did not establish a compiler observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompilerObservationError {
    /// Supervision ended execution before an ordinary compiler exit.
    Interrupted(ProcessStop),
    /// An abnormal exit, internal compiler error or absent selected diagnostic prevented an observation.
    ProcessFailure,
    /// One output line was not a complete, unambiguous JSON value.
    InvalidJson {
        /// The one-based output line.
        line: usize,
        /// The decoder's diagnostic.
        detail: String,
    },
    /// A required protocol field or consistent build outcome was absent.
    Protocol(String),
    /// The number of relevant error diagnostics was not exactly one.
    DiagnosticCount(usize),
    /// A relevant diagnostic did not carry exactly one primary span.
    PrimarySpanCount(usize),
    /// A relevant compiler error carried no stable Rust error code.
    UncodedDiagnostic,
    /// A source path or span could not be mapped to the declared logical root.
    Source(String),
}

/// The native process result and the compiler observation established from its complete output.
#[derive(Debug)]
pub struct CompilerOutput {
    pub(super) request: CompilerRequest,
    pub(super) process: ProcessOutput,
    pub(super) observation: Result<Observation, CompilerObservationError>,
    pub(super) dependencies: Option<Result<super::DependencyInfo, super::DependencyError>>,
}

/// A compiler observation or retained ownership of unfinished process cleanup.
#[derive(Debug)]
#[must_use]
pub enum CompilerRun {
    /// The process owner completed its direct-child and reader cleanup.
    Finished(Box<CompilerOutput>),
    /// The process owner has not finished cleanup and remains retryable.
    Pending(Box<PendingCompilation>),
}

/// The compiler request and its still-owned pending native process.
#[derive(Debug)]
#[must_use]
pub struct PendingCompilation {
    pub(super) request: CompilerRequest,
    pub(super) process: Box<PendingProcess>,
}

/// Why a native read-back did not supply values to the existing comparator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadBackError {
    /// Supervision stopped the reader before ordinary exit.
    Interrupted(ProcessStop),
    /// The reader did not exit successfully.
    ProcessFailure,
    /// The caller's value decoder refused the actual retained stdout.
    Decode(String),
}

#[derive(Clone, Debug)]
pub(super) enum Protocol {
    Rustc { artifact: PathBuf },
    Cargo(CargoFixture),
}

#[derive(Debug)]
pub(super) struct Observation {
    pub compilation: ObservedCompilation,
    pub artifact: Option<Artifact>,
}

#[derive(Debug)]
pub(super) struct Artifact {
    pub executable: Option<PathBuf>,
    pub cargo_fresh: Option<bool>,
    pub files: Vec<PathBuf>,
}

#[derive(Clone, Debug)]
pub(super) struct DependencyCapture {
    pub path: PathBuf,
    pub limits: super::DependencyLimits,
}

#[derive(Default)]
pub(super) struct Stream {
    pub errors: usize,
    pub anchors: Vec<DiagnosticAnchor>,
    pub artifact: Option<Artifact>,
    pub finished: Option<bool>,
}

pub(super) struct SelectedCargoMessage<'message> {
    value: &'message serde_json::Value,
}

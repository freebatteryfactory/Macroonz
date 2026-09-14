use crate::harness::muterprater::backend_archive::{BackendArchiveRefusal, BackendSourceRefusal};
use crate::harness::muterprater::{
    AdapterQualification, ArtifactManifestRefusal, CompiledSuiteArtifactManifest, FamilyLookup,
    MutationBackendInvocation, OwnerLookup, QualificationRefusal,
};
use crate::harness::oracle::RelativeSourcePath;
use crate::native_process::{
    PendingProcess, ProcessError, ProcessOutput, ProcessRequest, ProcessRun,
};
use std::path::PathBuf;

#[path = "type_guard.rs"]
mod guard;

/// Explicit Cargo and rustc executables used by one mutation backend.
#[derive(Clone, Debug)]
pub struct MutationToolchain {
    cargo: PathBuf,
    rustc: PathBuf,
}

/// Unique declared source paths and their aggregate read ceiling.
#[derive(Clone, Debug)]
pub struct MutationSources {
    pub(super) files: Vec<RelativeSourcePath>,
    pub(super) bytes: usize,
}

/// An admitted mutation command with explicit tool, source and output custody.
#[derive(Clone, Debug)]
pub struct MutationRequest {
    pub(super) process: ProcessRequest,
    pub(super) compiler: ProcessRequest,
    pub(super) sources: MutationSources,
    pub(super) output: PathBuf,
    pub(super) target: String,
}

/// The native operation being observed when preparation failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MutationPhase {
    /// Query the selected backend's version.
    BackendVersion,
    /// Query the compiler selected for backend builds.
    CompilerVersion,
}

/// A preparation or source-capture failure, retaining any unfinished query process.
#[derive(Debug)]
pub enum MutationError {
    /// The declared request cannot be represented by this profile.
    Configuration(String),
    /// A declared source or output operation failed.
    Filesystem(std::io::Error),
    /// The aggregate source input exceeded its declared byte bound.
    SourceBound {
        /// The declared aggregate maximum.
        bound: usize,
    },
    /// Native process admission or startup failed.
    Process(ProcessError),
    /// A version query did not establish the selected profile.
    Query {
        /// The query's semantic phase.
        phase: MutationPhase,
        /// The exact command that ran.
        request: Box<ProcessRequest>,
        /// The actual output or unfinished cleanup owner.
        run: Box<ProcessRun>,
        /// The reason no profile was established.
        cause: String,
    },
}

/// Why a finished backend process established no complete mutation observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MutationObservationError {
    /// The native stop, capture or backend exit was not a completed mutation observation.
    Process,
    /// The captured console cannot be read faithfully.
    Console(String),
    /// The announced mutant population does not equal the parsed report population.
    Roster,
    /// A declared source changed or could not be compared after execution.
    Sources(String),
    /// The existing artifact reader refused the output/source join.
    Artifact(ArtifactManifestRefusal),
    /// The existing adapter qualification refused the observed profile.
    Qualification(QualificationRefusal),
}

/// A failure retaining or independently comparing a mutation observation.
#[derive(Debug)]
pub enum MutationCustodyError {
    /// The execution established no complete mutation observation.
    Observation(MutationObservationError),
    /// Reading current source files failed.
    Source(Box<MutationError>),
    /// Canonical historical retention refused.
    Archive(BackendArchiveRefusal),
    /// Current source revisions differ from the execution manifest.
    Current(crate::harness::muterprater::ArtifactCustodyRefusal),
    /// Current source revisions differ from the historical manifest.
    Historical(BackendSourceRefusal),
}

/// A finished backend observation or retained ownership of pending cleanup.
#[derive(Debug)]
#[must_use]
pub enum MutationRun {
    /// Native cleanup finished; observation establishment may still have refused.
    Finished(Box<MutationOutput>),
    /// Native cleanup is unfinished and owns its original source context.
    Pending(Box<PendingMutation>),
}

/// An immutable execution result with original source and tool observations.
#[derive(Debug)]
pub struct MutationOutput {
    pub(super) context: ExecutionContext,
    pub(super) process: ProcessOutput,
    pub(super) observation: Result<Observation, MutationObservationError>,
}

/// A mutation execution retaining its source context until native cleanup finishes.
#[derive(Debug)]
#[must_use]
pub struct PendingMutation {
    pub(super) context: ExecutionContext,
    pub(super) process: Box<PendingProcess>,
}

#[derive(Debug)]
pub(super) struct SourceMaterial {
    pub file: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub(super) struct ExecutionContext {
    pub request: MutationRequest,
    pub invocation: MutationBackendInvocation,
    pub backend_version: ProcessOutput,
    pub compiler_version: ProcessOutput,
    pub sources: Vec<SourceMaterial>,
    pub owner: OwnerLookup,
    pub family: FamilyLookup,
}

#[derive(Debug)]
pub(super) struct Observation {
    pub manifest: CompiledSuiteArtifactManifest,
    pub qualification: AdapterQualification,
    pub console: String,
}

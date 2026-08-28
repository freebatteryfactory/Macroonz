//! Every public type of the fuzz home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::NamespacedName;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[path = "type_guard.rs"]
mod guard;

/// The stable product toolchain whose coverage format this home qualifies.
pub const RUSTC_COVERAGE_TOOLCHAIN: &str = "1.98.0";

/// Which coverage mechanism the campaign selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectedBackend {
    /// Stable rustc source instrumentation and its matching LLVM profile tools.
    RustcInstrumentCoverage,
}

/// One named ceiling retained with the rustc coverage mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedCeiling {
    /// Each candidate executes in a fresh instrumented process and therefore pays process startup cost.
    FreshProcessPerCandidate,
    /// The target must be compiled from available Rust source with coverage instrumentation.
    InstrumentedSourceTargetRequired,
    /// The matching `llvm-profdata` and `llvm-cov` tools must be installed with the toolchain.
    LlvmCoverageToolsRequired,
    /// The caller's host must supervise the child and classify its termination without an ambient harness clock.
    CallerSuppliesProcessSupervisor,
}

/// What one host class established for the selected backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostDisposition {
    /// Native stable-toolchain execution was observed on Windows.
    ObservedWindows,
    /// The same rustc mechanism remains unexecuted on a native Linux host.
    UnexecutedLinux,
    /// The same rustc mechanism remains unexecuted on a native macOS host.
    UnexecutedMacOs,
}

/// One capability a stable rustc coverage preflight may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreflightCapability {
    /// Rustc is the product MSRV.
    RustcMsrv,
    /// Rustc reported its host tuple.
    RustcHostTuple,
    /// Rustc reported its sysroot.
    RustcSysroot,
    /// Rustc reported the LLVM version it carries.
    LlvmReported,
    /// The matching `llvm-tools` component is installed.
    LlvmToolsPreview,
    /// The matching `llvm-profdata` executable is available at a declared path.
    LlvmProfdata,
    /// The matching `llvm-cov` executable is available at a declared path.
    LlvmCov,
    /// The target was compiled with stable `-C instrument-coverage`.
    InstrumentCoverage,
}

/// Whether one declared preflight capability was available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreflightStatus {
    /// The capability was present under the caller's declared fact.
    Available,
    /// The capability was absent under the caller's declared fact.
    Unavailable,
}

/// One caller-supplied preflight observation.
///
/// The harness never discovers these facts; the driver or adopter hands them in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreflightFact {
    capability: PreflightCapability,
    status: PreflightStatus,
}

/// Why a preflight roster was incomplete.
#[must_use = "a refusal is the reason fuzz preflight was not ready"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightIncomplete {
    /// At least one required capability was unavailable.
    Unavailable(PreflightCapability),
    /// A required capability was not present in the declared roster.
    Missing(PreflightCapability),
    /// The same capability was declared more than once.
    Duplicate(PreflightCapability),
    /// The same capability was declared with disagreeing availability.
    Contradictory(PreflightCapability),
}

/// A preflight roster in which every required capability was declared available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadyPreflight {
    backend: SelectedBackend,
}

/// The selected backend together with the ceilings and host dispositions it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendSelection {
    name: NamespacedName,
    backend: SelectedBackend,
    ceilings: Vec<NamedCeiling>,
    hosts: Vec<HostDisposition>,
}

/// Why a backend selection was refused.
#[must_use = "a refusal is the reason a fuzz backend selection was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendSelectionRefusal {
    /// No named ceiling was declared.
    NoCeiling,
    /// No host disposition was declared.
    NoHostDisposition,
    /// A required ceiling was absent from the declared roster.
    MissingRequiredCeiling(NamedCeiling),
    /// A required host disposition was absent from the declared roster.
    MissingRequiredHost(HostDisposition),
}

/// Exact bytes a coverage observation admitted as interesting for Macroonz reduction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterestingBytes {
    bytes: Vec<u8>,
}

/// Why interesting bytes were refused.
#[must_use = "a refusal is the reason interesting bytes were not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterestingBytesRefusal {
    /// The byte string was empty, so it cannot seed a reduction.
    Empty,
}

/// One canonical covered source point exported by `llvm-cov`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoveragePoint {
    /// One source line executed at least once.
    Line {
        /// The source spelling exported by the toolchain.
        source: String,
        /// The one-based source line.
        line: u64,
    },
    /// One reported source branch executed at least once.
    Branch {
        /// The source spelling exported by the toolchain.
        source: String,
        /// The one-based source line.
        line: u64,
        /// The toolchain's block ordinal.
        block: u64,
        /// The branch ordinal within the block.
        branch: u64,
    },
}

/// One canonical set of covered source points from a single candidate execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageObservation {
    pub(super) points: Vec<CoveragePoint>,
}

/// Why an LCOV coverage export was not admitted.
#[must_use = "a refusal is the reason a coverage export was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageReadRefusal {
    /// The export was not UTF-8.
    NonUtf8,
    /// A source record carried no path.
    EmptySource {
        /// The one-based LCOV record position.
        record: usize,
    },
    /// A coverage point appeared before its source record.
    MissingSource {
        /// The one-based LCOV record position.
        record: usize,
    },
    /// A line-coverage row was malformed.
    MalformedLine {
        /// The one-based LCOV record position.
        record: usize,
    },
    /// A branch-coverage row was malformed.
    MalformedBranch {
        /// The one-based LCOV record position.
        record: usize,
    },
}

/// The campaign's accumulated coverage frontier and retained interesting inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageCorpus {
    pub(super) observed: BTreeSet<CoveragePoint>,
    pub(super) interesting: Vec<InterestingBytes>,
}

/// What became of one candidate at the coverage frontier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageAdmission {
    /// The candidate added no covered point.
    Known,
    /// The candidate added at least one covered point and was retained exactly.
    Interesting(InterestingBytes),
}

/// Why one candidate could not be compared with the coverage frontier.
#[must_use = "a refusal is the reason a coverage candidate was not judged"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageAdmissionRefusal {
    /// The execution reported no covered point.
    EmptyObservation,
    /// The candidate bytes were empty.
    EmptyCandidate,
}

/// One bounded deterministic neighboring-input plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationPlan {
    budget: u32,
    byte_limit: usize,
    dictionary: Vec<Vec<u8>>,
}

/// Why a neighboring-input plan was not constructed.
#[must_use = "a refusal is the reason a mutation plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationPlanRefusal {
    /// The plan admitted no candidate.
    ZeroBudget,
    /// The plan admitted no candidate byte.
    ZeroByteLimit,
    /// One dictionary token was empty.
    EmptyDictionaryToken {
        /// The token's position in declared order.
        at: usize,
    },
}

/// Which deterministic operation produced one neighboring input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationKind {
    /// Flip one bit in one existing byte.
    BitFlip,
    /// Replace one byte with a boundary value.
    BoundarySubstitution,
    /// Increment one byte when it is below `0xff`.
    Increment,
    /// Decrement one byte when it is above zero.
    Decrement,
    /// Remove one byte while keeping the candidate nonempty.
    Delete,
    /// Insert one boundary byte.
    InsertBoundary,
    /// Duplicate one existing byte.
    Duplicate,
    /// Join one prefix of the seed to one suffix of a retained partner.
    Splice,
    /// Insert one caller-declared dictionary token.
    DictionaryInsert,
}

/// One nonempty deterministic neighbor and the operation that produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationCandidate {
    kind: MutationKind,
    bytes: Vec<u8>,
}

/// Why neighboring-input enumeration could not begin.
#[must_use = "a refusal is the reason neighboring inputs were not produced"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationRefusal {
    /// The seed was empty.
    EmptySeed,
    /// The seed already exceeded the plan's byte ceiling.
    SeedExceedsByteLimit,
    /// A declared splice partner was empty.
    EmptyPartner,
}

/// The exact tool paths used to read rustc coverage profiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustcCoverageTools {
    profdata: PathBuf,
    cov: PathBuf,
}

/// One already-instrumented Rust target and its declared arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentedTarget {
    executable: PathBuf,
    arguments: Vec<String>,
}

/// One declared rustc-profile observation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustcProfileRequest {
    target: InstrumentedTarget,
    tools: RustcCoverageTools,
    scratch: PathBuf,
}

/// Why a rustc-profile request was not constructed.
#[must_use = "a refusal is the reason a rustc coverage request was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustcProfileRequestRefusal {
    /// The target executable path was empty.
    Target,
    /// The `llvm-profdata` path was empty.
    Profdata,
    /// The `llvm-cov` path was empty.
    Cov,
    /// The scratch path was empty.
    Scratch,
}

/// How one instrumented target process ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FuzzExecution {
    /// The target exited successfully.
    Success,
    /// The target exited unsuccessfully with an optional platform exit code.
    NonzeroExit(Option<i32>),
    /// The caller's host classified the target as crashed.
    Crash(Option<i32>),
    /// The caller's host stopped the target at its declared time bound.
    Timeout,
    /// The caller's host stopped the target at its declared resource bound.
    ResourceExhaustion,
}

/// One target execution together with any coverage it flushed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustcProfileResult {
    execution: FuzzExecution,
    observation: CoverageObservation,
}

/// Why one rustc-profile execution could not produce a truthful result.
#[must_use = "a refusal is the reason a rustc coverage execution did not complete"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustcProfileRefusal {
    /// The caller supplied an empty candidate.
    EmptyCandidate,
    /// The deterministic case directory already existed.
    CaseAlreadyExists(PathBuf),
    /// The case directory could not be created.
    CreateCase(String),
    /// The target process could not be started.
    StartTarget(String),
    /// The candidate could not be written to target standard input.
    WriteTarget(String),
    /// The caller's declared supervisor could not complete or classify the target.
    SuperviseTarget(String),
    /// A successful target execution did not write its declared raw profile.
    MissingProfile,
    /// `llvm-profdata` could not be started.
    StartProfdata(String),
    /// `llvm-profdata` exited unsuccessfully.
    ProfdataFailed(Option<i32>),
    /// `llvm-cov` could not be started.
    StartCov(String),
    /// `llvm-cov` exited unsuccessfully.
    CovFailed(Option<i32>),
    /// The exported coverage was malformed.
    Coverage(CoverageReadRefusal),
}

/// Why compose-reduce-replay refused.
#[must_use = "a refusal is the reason fuzz compose did not mint a replay capsule"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeRefusal {
    /// The reduction road refused the seed under the bound probe.
    Reduction(crate::generate::ReductionRefusal),
    /// Replay did not re-establish the preserved fingerprint.
    ReplayFingerprintMoved,
    /// Replay established no failure.
    ReplayNoFailure,
}

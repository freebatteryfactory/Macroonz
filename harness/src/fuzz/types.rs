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

/// One logical source root and its physical checkout seat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageSourceRoot {
    logical: NamespacedName,
    checkout: PathBuf,
}

/// Why a coverage source root was not informed.
#[must_use = "a refusal is the reason a coverage source root was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageSourceRootRefusal {
    /// The checkout path was empty.
    EmptyCheckout,
    /// The checkout path was not absolute.
    RelativeCheckout,
    /// The checkout path contained a parent traversal.
    CheckoutTraversal,
    /// The checkout path could not be represented in the UTF-8 coverage document.
    NonUtf8Checkout,
}

/// One root-independent source identity carried by a coverage point.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoverageSource {
    root: NamespacedName,
    relative: String,
}

/// Which matching LLVM coverage tool preflight or execution names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoverageTool {
    /// The profile-merging tool.
    Profdata,
    /// The coverage-export tool.
    Cov,
}

/// Which rustc preflight command produced an observation or refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustcCommand {
    /// The verbose compiler identity query.
    VerboseVersion,
    /// The compiler sysroot query.
    Sysroot,
}

/// Which required rustc identity field was absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustcField {
    /// The compiler release.
    Release,
    /// The compiler host tuple.
    Host,
    /// The compiler's LLVM version.
    LlvmVersion,
    /// The compiler sysroot.
    Sysroot,
}

/// Why active stable-rustc coverage preflight did not establish readiness.
#[must_use = "a refusal is the reason rustc coverage preflight was not ready"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreflightIncomplete {
    /// The declared target could not be inspected.
    TargetUnavailable {
        /// The target path.
        path: PathBuf,
        /// The host error.
        error: String,
    },
    /// The declared target path did not name a file.
    TargetNotFile(PathBuf),
    /// The declared source root could not be inspected or canonicalized.
    SourceRootUnavailable {
        /// The source-root path.
        path: PathBuf,
        /// The host error.
        error: String,
    },
    /// The declared source root did not name a directory.
    SourceRootNotDirectory(PathBuf),
    /// The canonical source root violated coverage source-identity requirements.
    SourceRootIdentity(CoverageSourceRootRefusal),
    /// A rustc identity command could not be started.
    StartRustc {
        /// The command role.
        command: RustcCommand,
        /// The host error.
        error: String,
    },
    /// A rustc identity command exited unsuccessfully.
    RustcFailed {
        /// The command role.
        command: RustcCommand,
        /// The optional platform exit code.
        code: Option<i32>,
    },
    /// A rustc identity command did not return UTF-8.
    RustcOutputNotUtf8(RustcCommand),
    /// A required rustc identity field was absent.
    MissingRustcField(RustcField),
    /// The declared compiler was not the product toolchain.
    RustcRelease {
        /// The required stable release.
        required: &'static str,
        /// The observed release.
        observed: String,
    },
    /// Rustc reported a relative sysroot.
    RelativeRustcSysroot(PathBuf),
    /// A matching LLVM tool could not be started at its derived path.
    StartLlvmTool {
        /// The tool role.
        tool: CoverageTool,
        /// The derived tool path.
        path: PathBuf,
        /// The host error.
        error: String,
    },
    /// A matching LLVM tool exited unsuccessfully.
    LlvmToolFailed {
        /// The tool role.
        tool: CoverageTool,
        /// The optional platform exit code.
        code: Option<i32>,
    },
    /// A matching LLVM tool did not return UTF-8.
    LlvmToolOutputNotUtf8(CoverageTool),
    /// A matching LLVM tool did not report its version.
    MissingLlvmToolVersion(CoverageTool),
    /// The two tools derived from one sysroot reported different versions.
    LlvmToolVersionsDiffer {
        /// The `llvm-profdata` version.
        profdata: String,
        /// The `llvm-cov` version.
        cov: String,
    },
    /// The matching tools did not report rustc's LLVM version.
    RustcLlvmVersion {
        /// The LLVM version reported by rustc.
        rustc: String,
        /// The LLVM version reported by the tools.
        tools: String,
    },
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
        /// The root-independent source identity.
        source: CoverageSource,
        /// The one-based source line.
        line: u64,
    },
    /// One reported source branch executed at least once.
    Branch {
        /// The root-independent source identity.
        source: CoverageSource,
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
    /// A source record was relative rather than rooted in the declared checkout.
    RelativeSource {
        /// The one-based LCOV record position.
        record: usize,
    },
    /// A source record contained a parent traversal.
    SourceTraversal {
        /// The one-based LCOV record position.
        record: usize,
    },
    /// A source record was outside the declared checkout root.
    SourceOutsideRoot {
        /// The one-based LCOV record position.
        record: usize,
    },
    /// A source record named the checkout root without a source member.
    EmptyRelativeSource {
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

/// The exact tool paths derived from one qualified rustc sysroot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RustcCoverageTools {
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
    rustc: PathBuf,
    target: InstrumentedTarget,
    source_root: CoverageSourceRoot,
    scratch: PathBuf,
}

/// Why a rustc-profile request was not constructed.
#[must_use = "a refusal is the reason a rustc coverage request was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustcProfileRequestRefusal {
    /// The rustc executable path was empty.
    Rustc,
    /// The rustc executable path was relative.
    RelativeRustc,
    /// The target executable path was empty.
    Target,
    /// The target executable path was relative.
    RelativeTarget,
    /// The scratch path was empty.
    Scratch,
    /// The scratch path was relative.
    RelativeScratch,
}

/// One actively established rustc, target, source-root, and tool environment carrying a declared scratch path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadyPreflight {
    pub(super) request: RustcProfileRequest,
    pub(super) tools: RustcCoverageTools,
    pub(super) source_root: CoverageSourceRoot,
    pub(super) sysroot: PathBuf,
    pub(super) release: String,
    pub(super) host: String,
    pub(super) llvm_version: String,
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
    /// The candidate could not be materialized before target start.
    WriteCandidate(String),
    /// The materialized candidate could not be opened as target standard input.
    OpenCandidate(String),
    /// The caller's declared supervisor could not complete or classify the target.
    SuperviseTarget(String),
    /// The supervisor returned an outcome while the child was still running.
    SupervisorReturnedBeforeExit,
    /// The child process state could not be inspected after supervision.
    InspectTarget(String),
    /// Termination or reaping failed after another execution refusal.
    CleanupTarget {
        /// The refusal that required cleanup.
        after: Box<RustcProfileRefusal>,
        /// The cleanup failure.
        cleanup: String,
    },
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

//! The benchmark receiver's declarations: rows, bindings, scoped work readings, host input, and complete report stages.

#[path = "type_guard.rs"]
mod guard;

use crate::clock::{HarnessClock, MeasurementReading};
use crate::descriptor::{
    EncodeRefusal, NameRefusal, NamespacedName, Provenance, TrialTableRefusal,
};
use crate::identity::ContentAddress;
use crate::identity::{DomainTag, IdentityProfileVersion};
use crate::report::{FindingCause, TargetBinding, TargetTriple, ToolchainIdentity, TrialReport};
use crate::runner::{Invocation, TrialBinding};
use std::num::NonZeroU32;

/// The benchmark-row identity family's derivation domain and initial position.
pub const BENCH_ROW_KEY_TAG: DomainTag =
    DomainTag::declared("bench-row-key", IdentityProfileVersion::declared(1));

/// One benchmark workload's semantic reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkloadRef(NamespacedName);

/// One correctness-preflight seat's semantic reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreflightRef(NamespacedName);

/// One deliberately worse callable's semantic reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlantedWorseRef(NamespacedName);

/// One neutral complexity claim's semantic reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComplexityClaimRef(NamespacedName);

/// One scoped work observation's semantic reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkObservationRef(NamespacedName);

/// One authored benchmark table's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenchTableName(NamespacedName);

/// At least two distinct benchmark input sizes in authored order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSizeAxis(Vec<u64>);

/// Why an input-size axis was not admitted.
#[must_use = "a refusal is the reason an input-size axis was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSizeAxisRefusal {
    /// Fewer than two sizes were declared, so no growth relation can be observed.
    TooShort {
        /// The number of declared sizes.
        found: usize,
    },
    /// One size appeared twice in the authored axis.
    DuplicateSize {
        /// The repeated size.
        size: u64,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// One exact non-floating-point ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactRatio {
    numerator: u64,
    denominator: u64,
}

/// The sample, warmup, and exact ratio budgets one benchmark row declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredBudgets {
    samples: NonZeroU32,
    warmups: u32,
    ratio: ExactRatio,
}

/// Why declared benchmark budgets were not admitted.
#[must_use = "a refusal is the reason benchmark budgets were not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredBudgetsRefusal {
    /// No primary sample was declared, so no work evidence can be produced.
    NoSamples,
    /// The exact ratio numerator was zero, so no positive gap was declared.
    ZeroRatioNumerator,
    /// The exact ratio denominator was zero.
    ZeroRatioDenominator,
}

/// The contention posture under which a row is declared and invoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentionPosture {
    /// The caller declares no contended environment.
    NoDeclaredContention,
}

/// One nonempty owner-declared work-formula representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkFormula {
    bytes: Vec<u8>,
}

/// Why a work formula was not admitted.
#[must_use = "a refusal is the reason a work formula was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkFormulaRefusal {
    /// A present owner-declared formula carried no bytes; absence is represented by `None`.
    Empty,
}

/// The compact identity of one complete benchmark-row declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenchRowKey(ContentAddress);

/// The four semantic references one benchmark row joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchReferences {
    workload: WorkloadRef,
    preflight: PreflightRef,
    planted_worse: PlantedWorseRef,
    complexity: ComplexityClaimRef,
}

/// The four measurement declarations one benchmark row carries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchMeasurement {
    input_sizes: InputSizeAxis,
    budgets: DeclaredBudgets,
    contention: ContentionPosture,
    formula: Option<WorkFormula>,
}

/// One immutable benchmark-row declaration over the schema's eight facts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchRow {
    workload: WorkloadRef,
    input_sizes: InputSizeAxis,
    preflight: PreflightRef,
    planted_worse: PlantedWorseRef,
    budgets: DeclaredBudgets,
    contention: ContentionPosture,
    formula: Option<WorkFormula>,
    complexity: ComplexityClaimRef,
    key: BenchRowKey,
}

/// Why a benchmark row was not built.
#[must_use = "a refusal is the reason a benchmark row was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchRowRefusal {
    /// The row's canonical preimage outgrew the declared length width.
    Encoding(EncodeRefusal),
}

/// One benchmark workload function pointer with no captured closure environment.
///
/// The type does not establish purity, determinism, or unwind safety; those remain the caller's ceiling.
pub type BenchCall = fn(u64, &mut WorkRecorder) -> Result<(), WorkRecordingRefusal>;

/// One primary work-judge function pointer with no captured closure environment.
///
/// The type excludes wall readings by its input but does not establish purity, determinism, or unwind safety.
pub type WorkJudge = for<'reading> fn(&WorkJudgmentInput<'reading>) -> WorkJudgment;

/// The exact values one primary work judge may read.
#[derive(Debug, Clone, Copy)]
pub struct WorkJudgmentInput<'reading> {
    formula: Option<&'reading WorkFormula>,
    complexity: ComplexityClaimRef,
    budgets: DeclaredBudgets,
    measured: &'reading WorkCurve,
    planted_worse: &'reading WorkCurve,
}

/// One owner-bound work judge and the complexity reference it judges under.
#[derive(Debug, Clone, Copy)]
pub struct WorkJudgeBinding {
    complexity: ComplexityClaimRef,
    judge: WorkJudge,
}

/// One primary work judgment.
#[must_use = "a work conclusion is the primary benchmark judgment"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkConclusion {
    /// The curve satisfies the declared formula and complexity claim.
    Satisfied,
    /// The curve refuses under one typed cause.
    Refused(FindingCause),
}

/// Whether the declared exact gap distinguishes the measured and planted-worse curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkGapStanding {
    /// The declared gap was observed.
    Distinguished,
    /// The curves did not establish the declared gap.
    NotDistinguished(FindingCause),
}

/// One relational judgment over measured and planted-worse work curves.
#[must_use = "a work judgment carries all three primary benchmark readings"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkJudgment {
    measured: WorkConclusion,
    planted_worse: WorkConclusion,
    gap: WorkGapStanding,
}

/// Why one relational work judgment did not qualify a benchmark row.
#[must_use = "a refusal names the work reading that prevented qualification"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkQualificationRefusal {
    /// The deliberately worse curve was not both refused and distinguished from the measured curve.
    PlantedWorseNotDistinguished {
        /// The deliberately worse curve's conclusion.
        planted_worse: WorkConclusion,
        /// The declared exact-gap reading.
        gap: WorkGapStanding,
    },
    /// The measured curve did not satisfy the declared work claim after the control stood.
    MeasuredRefused(WorkConclusion),
}

/// One scoped work count at one input size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkCount {
    observation: WorkObservationRef,
    count: u64,
}

/// One input size and its ordered work counts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkCurvePoint {
    input_size: u64,
    counts: Vec<WorkCount>,
}

/// One ordered primary-work curve over the row's authored input axis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkCurve {
    points: Vec<WorkCurvePoint>,
}

/// One qualified timed pass: its retained work, accepted judgment, and caller-clock readings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryObservation {
    work: WorkCurve,
    judgment: WorkJudgment,
    measurements: Vec<MeasurementReading>,
}

/// The scoped recorder one benchmark callable may write through.
#[derive(Debug)]
pub struct WorkRecorder {
    counts: Vec<WorkCount>,
}

/// Why scoped work recording was refused.
#[must_use = "a refusal is the reason work recording did not complete"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkRecordingRefusal {
    /// A callable's declared observation name was not parsed.
    ObservationName(NameRefusal),
    /// The callable named an observation outside its binding's roster.
    UnknownObservation(WorkObservationRef),
    /// The callable could not represent the work amount it meant to record.
    AmountOverflow {
        /// The observation whose amount could not be formed.
        observation: WorkObservationRef,
        /// The input size at which arithmetic overflowed.
        input_size: u64,
    },
    /// Adding units would overflow the exact counter.
    CountOverflow {
        /// The observation being counted.
        observation: WorkObservationRef,
        /// The count already retained.
        current: u64,
        /// The additional units the callable requested.
        addition: u64,
    },
}

/// What makes one row executable as a benchmark without granting the callables authority over its meaning.
#[derive(Debug, Clone)]
pub struct BenchAttachment {
    workload: WorkloadRef,
    measured: BenchCall,
    planted_worse_ref: PlantedWorseRef,
    planted_worse: BenchCall,
    judge: WorkJudgeBinding,
    observations: Vec<WorkObservationRef>,
}

/// Why one benchmark attachment was not built.
#[must_use = "a refusal is the reason a benchmark attachment was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchAttachmentRefusal {
    /// No work observation was declared.
    NoObservation,
    /// One work observation appeared twice.
    DuplicateObservation {
        /// The repeated observation.
        observation: WorkObservationRef,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// One real correctness trial binding and invocation under a benchmark-owned reference.
#[derive(Clone)]
pub struct PreflightTrial {
    reference: PreflightRef,
    binding: TrialBinding,
    invocation: Invocation,
}

/// One benchmark row joined to every callable and preflight fact its host needs.
#[derive(Clone)]
pub struct BenchBinding {
    row: BenchRow,
    attachment: BenchAttachment,
    preflight: PreflightTrial,
}

/// Why one benchmark binding was not built.
#[must_use = "a refusal is the reason a benchmark binding was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchBindingRefusal {
    /// The row and attachment name different workloads.
    Workload {
        /// The row's workload.
        row: WorkloadRef,
        /// The attachment's workload.
        attachment: WorkloadRef,
    },
    /// The row and attachment name different planted-worse seats.
    PlantedWorse {
        /// The row's reference.
        row: PlantedWorseRef,
        /// The attachment's reference.
        attachment: PlantedWorseRef,
    },
    /// The row and preflight trial name different seats.
    Preflight {
        /// The row's reference.
        row: PreflightRef,
        /// The trial's reference.
        trial: PreflightRef,
    },
    /// The row and work judge name different complexity claims.
    Complexity {
        /// The row's claim.
        row: ComplexityClaimRef,
        /// The judge's claim.
        judge: ComplexityClaimRef,
    },
}

/// One nonempty benchmark table in authored order.
#[derive(Clone)]
pub struct BenchTable {
    name: BenchTableName,
    provenance: Provenance,
    bindings: Vec<BenchBinding>,
}

/// Why a benchmark table was not built.
#[must_use = "a refusal is the reason a benchmark table was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchTableRefusal {
    /// The table carried no benchmark row.
    Empty,
    /// Two bindings carried the same complete row identity.
    DuplicateRow {
        /// The duplicated row identity.
        row: BenchRowKey,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// The one refusal family a stamped benchmark-table function returns.
#[must_use = "a refusal is the reason a stamped benchmark table was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchStampRefusal {
    /// A namespaced reference was not parsed.
    Name(NameRefusal),
    /// An input-size axis was not admitted.
    InputSizeAxis(InputSizeAxisRefusal),
    /// Benchmark budgets were not admitted.
    Budgets(DeclaredBudgetsRefusal),
    /// A present work formula was not admitted.
    WorkFormula(WorkFormulaRefusal),
    /// A benchmark row was not built.
    Row(BenchRowRefusal),
    /// A benchmark attachment was not built.
    Attachment(BenchAttachmentRefusal),
    /// A benchmark binding was not built.
    Binding(BenchBindingRefusal),
    /// A correctness trial binding was not built.
    Preflight(TrialTableRefusal),
    /// The complete benchmark table was not built.
    Table(BenchTableRefusal),
}

/// The explicit host facts for one complete benchmark-table run.
#[derive(Debug, Clone)]
pub struct BenchInvocation {
    target: TargetBinding,
    clock: HarnessClock,
    contention: ContentionPosture,
}

/// Which target-binding fact disagreed before benchmark caller code could run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchTargetMismatch {
    /// The compilation target spellings differ.
    Target {
        /// The benchmark invocation's target.
        benchmark: TargetTriple,
        /// The preflight invocation's target.
        preflight: TargetTriple,
    },
    /// The toolchain identities differ.
    Toolchain {
        /// The benchmark invocation's toolchain.
        benchmark: ToolchainIdentity,
        /// The preflight invocation's toolchain.
        preflight: ToolchainIdentity,
    },
}

/// Why no complete benchmark report was produced.
#[must_use = "a refusal is the reason no benchmark report was produced"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchRunRefusal {
    /// A row's preflight invocation names another target or toolchain.
    PreflightTargetMismatch {
        /// The row that could not run.
        row: BenchRowKey,
        /// The exact target-binding member that disagreed.
        mismatch: BenchTargetMismatch,
    },
    /// A primary benchmark callable could not record its declared work.
    WorkNotRecorded {
        /// The row that could not complete.
        row: BenchRowKey,
        /// Which callable refused.
        phase: PrimaryWorkPhase,
        /// The exact scoped-recording refusal.
        refusal: WorkRecordingRefusal,
    },
    /// The post-qualification observation pass could not retain coherent work.
    SecondaryWorkRefused {
        /// The row that could not complete.
        row: BenchRowKey,
        /// Why no secondary observation was published.
        refusal: SecondaryObservationRefusal,
    },
}

/// Which primary callable failed to record a work curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaryWorkPhase {
    /// The measured callable.
    Measured,
    /// The deliberately worse callable.
    PlantedWorse,
}

/// Why the post-qualification wall-observation pass was not published.
#[must_use = "a refusal is the reason qualified caller-clock readings were not published"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondaryObservationRefusal {
    /// A discarded warmup call could not record its declared work.
    Warmup(WorkRecordingRefusal),
    /// A timed sample call could not record its declared work.
    Sample(WorkRecordingRefusal),
    /// The timed pass did not remain admitted under the same work judge.
    Judgment(WorkQualificationRefusal),
}

/// One benchmark row's stage-shaped outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchOutcome {
    /// Correctness preflight refused, so no benchmark callable ran.
    PreflightRefused,
    /// The deliberately worse control was not both refused and distinguished by the same work judge.
    PlantedWorseNotDistinguished {
        /// The measured curve.
        measured: WorkCurve,
        /// The planted-worse curve.
        planted_worse: WorkCurve,
        /// The relational judgment that did not activate the control.
        judgment: WorkJudgment,
    },
    /// The planted-worse curve refused, but the measured curve also refused.
    PrimaryWorkRefused {
        /// The measured curve.
        measured: WorkCurve,
        /// The planted-worse curve.
        planted_worse: WorkCurve,
        /// The relational judgment carrying the measured refusal.
        judgment: WorkJudgment,
    },
    /// Correctness, planted-worse activation, primary work, and secondary work coherence all held.
    #[non_exhaustive]
    Qualified {
        /// The qualified measured curve.
        measured: WorkCurve,
        /// The refused planted-worse curve.
        planted_worse: WorkCurve,
        /// The accepted measured, planted-worse, and exact-gap judgment.
        judgment: WorkJudgment,
        /// The qualified timed pass and its secondary caller-clock readings.
        secondary: SecondaryObservation,
    },
}

/// The stage occupied by one benchmark reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchStage {
    /// Correctness preflight refused.
    PreflightRefused,
    /// The deliberately worse control was not distinguished.
    PlantedWorseNotDistinguished,
    /// The measured primary work refused.
    PrimaryWorkRefused,
    /// The complete benchmark row qualified.
    Qualified,
}

/// One complete benchmark row's retained reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchReading {
    row: BenchRow,
    target: TargetBinding,
    preflight: TrialReport,
    outcome: BenchOutcome,
}

/// The complete authored benchmark table's report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchReport {
    table: BenchTableName,
    provenance: Provenance,
    readings: Vec<BenchReading>,
}

/// Why a benchmark report's verdict fold refused.
#[must_use = "a refusal names the first benchmark row that did not qualify"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchVerdictRefusal {
    row: BenchRowKey,
    stage: BenchStage,
}

//! Every public type of this home: what one benchmark row declares, and what running a table of them produces.

#[path = "type_guard.rs"]
mod guard;

use crate::clock::{HarnessClock, MeasurementReading};
use crate::descriptor::{
    EncodeRefusal, NameRefusal, NamespacedName, Provenance, TrialTableRefusal,
};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::{FindingCause, TargetBinding, TargetTriple, ToolchainIdentity, TrialReport};
use crate::runner::{Invocation, TrialBinding};
use std::num::NonZeroU32;

/// The derivation domain and starting position of the benchmark-row identity family.
pub const BENCH_ROW_KEY_TAG: DomainTag =
    DomainTag::declared("bench-row-key", IdentityProfileVersion::declared(1));

/// The workload one row measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkloadRef(NamespacedName);

/// The correctness preflight one row runs before it measures anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreflightRef(NamespacedName);

/// The deliberately worse control one row measures against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlantedWorseRef(NamespacedName);

/// The complexity claim a row is judged under, which this home never interprets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComplexityClaimRef(NamespacedName);

/// One kind of work a callable may count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkObservationRef(NamespacedName);

/// One authored benchmark table's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenchTableName(NamespacedName);

/// At least two distinct input sizes, in the order they were authored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSizeAxis(Vec<u64>);

/// Why an input-size axis was not admitted.
#[must_use = "a refusal is the reason an input-size axis was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSizeAxisRefusal {
    /// Fewer than two sizes were declared, so no growth relation can be observed.
    TooShort {
        /// How many sizes were declared.
        found: usize,
    },
    /// One size appeared twice on the axis.
    DuplicateSize {
        /// The repeated size.
        size: u64,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// One ratio held as two integers, so a comparison never depends on rounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactRatio {
    numerator: u64,
    denominator: u64,
}

/// The sample count, warmup count, and exact gap ratio one row declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredBudgets {
    samples: NonZeroU32,
    warmups: u32,
    ratio: ExactRatio,
}

/// Why declared budgets were not admitted.
#[must_use = "a refusal is the reason benchmark budgets were not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredBudgetsRefusal {
    /// No sample was declared, so no work evidence can be produced.
    NoSamples,
    /// The gap ratio's numerator was zero, so no positive gap was declared.
    ZeroRatioNumerator,
    /// The gap ratio's denominator was zero.
    ZeroRatioDenominator,
}

/// The contention posture a row is declared and invoked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentionPosture {
    /// The caller declares no contended environment.
    NoDeclaredContention,
}

/// Bytes the owner spells to say what work it expects, carried through unread.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkFormula {
    bytes: Vec<u8>,
}

/// Why a work formula was not admitted.
#[must_use = "a refusal is the reason a work formula was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkFormulaRefusal {
    /// A present formula carried no bytes, where absence is spelled `None`.
    Empty,
}

/// The identity derived from one complete row declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenchRowKey(ContentAddress);

/// The four names one row joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchReferences {
    workload: WorkloadRef,
    preflight: PreflightRef,
    planted_worse: PlantedWorseRef,
    complexity: ComplexityClaimRef,
}

/// The four measurement facts one row declares.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchMeasurement {
    input_sizes: InputSizeAxis,
    budgets: DeclaredBudgets,
    contention: ContentionPosture,
    formula: Option<WorkFormula>,
}

/// One immutable row: eight declared facts and the identity derived from all of them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchRow {
    references: BenchReferences,
    measurement: BenchMeasurement,
    key: BenchRowKey,
}

/// Why a benchmark row was not built.
#[must_use = "a refusal is the reason a benchmark row was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchRowRefusal {
    /// The row's canonical preimage outgrew the width the encoding declares.
    Encoding(EncodeRefusal),
}

/// One benchmark callable, taking an input size and the recorder it may count through.
///
/// A function pointer excludes captured state; it does not make the caller's function pure, deterministic, or unwind-safe.
pub type BenchCall = fn(u64, &mut WorkRecorder) -> Result<(), WorkRecordingRefusal>;

/// One owner-written judge over a pair of work curves.
///
/// Its input carries no clock reading, so no judgment can be a function of wall time.
pub type WorkJudge = for<'reading> fn(&WorkJudgmentInput<'reading>) -> WorkJudgment;

/// Everything a work judge is allowed to read.
#[derive(Debug, Clone, Copy)]
pub struct WorkJudgmentInput<'reading> {
    formula: Option<&'reading WorkFormula>,
    complexity: ComplexityClaimRef,
    budgets: DeclaredBudgets,
    measured: &'reading WorkCurve,
    planted_worse: &'reading WorkCurve,
}

/// One judge bound to the complexity claim it reads.
#[derive(Debug, Clone, Copy)]
pub struct WorkJudgeBinding {
    complexity: ComplexityClaimRef,
    judge: WorkJudge,
}

/// What a judge concluded about one curve.
#[must_use = "a work conclusion is the primary benchmark judgment"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkConclusion {
    /// The curve is the shape the row claimed.
    Satisfied,
    /// The curve refuses, under a cause the owner spelled.
    Refused(FindingCause),
}

/// Whether the declared exact gap separates the measured curve from the control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkGapStanding {
    /// The declared gap was observed.
    Distinguished,
    /// The two curves did not establish the declared gap.
    NotDistinguished(FindingCause),
}

/// One judge's three readings over a pair of curves.
#[must_use = "a work judgment carries all three primary benchmark readings"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkJudgment {
    measured: WorkConclusion,
    planted_worse: WorkConclusion,
    gap: WorkGapStanding,
}

/// Why a judgment did not qualify a row for timing.
#[must_use = "a refusal names the work reading that prevented qualification"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkQualificationRefusal {
    /// The control was not both refused and told apart from the measured curve.
    PlantedWorseNotDistinguished {
        /// What the judge concluded about the control.
        planted_worse: WorkConclusion,
        /// How the declared gap read.
        gap: WorkGapStanding,
    },
    /// The control stood, and the measured curve still did not satisfy the claim.
    MeasuredRefused(WorkConclusion),
}

/// One observation's exact count at one input size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkCount {
    observation: WorkObservationRef,
    count: u64,
}

/// One input size and the counts recorded at it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkCurvePoint {
    input_size: u64,
    counts: Vec<WorkCount>,
}

/// One callable's work across the row's whole input axis, in authored order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkCurve {
    points: Vec<WorkCurvePoint>,
}

/// What a qualified timed pass leaves behind: its own curve, the judgment that accepted it, and the clock readings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryObservation {
    work: WorkCurve,
    judgment: WorkJudgment,
    measurements: Vec<MeasurementReading>,
}

/// The counter a benchmark callable writes through, scoped to its binding's observations.
#[derive(Debug)]
pub struct WorkRecorder {
    counts: Vec<WorkCount>,
}

/// Why a callable's work was not recorded.
#[must_use = "a refusal is the reason work recording did not complete"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkRecordingRefusal {
    /// The callable's own observation name did not parse.
    ObservationName(NameRefusal),
    /// The callable named an observation its binding never declared.
    UnknownObservation(WorkObservationRef),
    /// The callable could not compute the amount it meant to record.
    AmountOverflow {
        /// The observation whose amount could not be formed.
        observation: WorkObservationRef,
        /// The input size at which the arithmetic overflowed.
        input_size: u64,
    },
    /// Adding the offered units would overflow the exact counter.
    CountOverflow {
        /// The observation being counted.
        observation: WorkObservationRef,
        /// The count already held.
        current: u64,
        /// The units the callable offered.
        addition: u64,
    },
}

/// What makes one row executable, without giving any callable a say in what the row means.
#[derive(Debug, Clone)]
pub struct BenchAttachment {
    workload: WorkloadRef,
    measured: BenchCall,
    planted_worse_ref: PlantedWorseRef,
    planted_worse: BenchCall,
    judge: WorkJudgeBinding,
    observations: Vec<WorkObservationRef>,
}

/// Why an attachment was not built.
#[must_use = "a refusal is the reason a benchmark attachment was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchAttachmentRefusal {
    /// No work observation was declared, so nothing could be counted.
    NoObservation,
    /// One observation appeared twice.
    DuplicateObservation {
        /// The repeated observation.
        observation: WorkObservationRef,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// A real trial, run under this home's preflight name before any measuring starts.
#[derive(Clone)]
pub struct PreflightTrial {
    reference: PreflightRef,
    binding: TrialBinding,
    invocation: Invocation,
}

/// One row joined to its callables and its preflight, with every name agreeing.
#[derive(Clone)]
pub struct BenchBinding {
    row: BenchRow,
    attachment: BenchAttachment,
    preflight: PreflightTrial,
}

/// Which name disagreed between a row and what was bound to it.
#[must_use = "a refusal is the reason a benchmark binding was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchBindingRefusal {
    /// The row and the attachment name different workloads.
    Workload {
        /// The row's workload.
        row: WorkloadRef,
        /// The attachment's workload.
        attachment: WorkloadRef,
    },
    /// The row and the attachment name different controls.
    PlantedWorse {
        /// The row's control.
        row: PlantedWorseRef,
        /// The attachment's control.
        attachment: PlantedWorseRef,
    },
    /// The row and the preflight trial name different preflights.
    Preflight {
        /// The row's preflight.
        row: PreflightRef,
        /// The trial's preflight.
        trial: PreflightRef,
    },
    /// The row and the judge name different complexity claims.
    Complexity {
        /// The row's claim.
        row: ComplexityClaimRef,
        /// The judge's claim.
        judge: ComplexityClaimRef,
    },
}

/// One nonempty table of bindings, in the order they were authored.
#[derive(Clone)]
pub struct BenchTable {
    name: BenchTableName,
    provenance: Provenance,
    bindings: Vec<BenchBinding>,
}

/// Why a table was not built.
#[must_use = "a refusal is the reason a benchmark table was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchTableRefusal {
    /// The table carried no row.
    Empty,
    /// Two bindings carried the same complete row identity.
    DuplicateRow {
        /// The duplicated identity.
        row: BenchRowKey,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// The one family a stamped table function returns, with every constructor's cause kept whole.
#[must_use = "a refusal is the reason a stamped benchmark table was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchStampRefusal {
    /// A namespaced name did not parse.
    Name(NameRefusal),
    /// An input-size axis was not admitted.
    InputSizeAxis(InputSizeAxisRefusal),
    /// Budgets were not admitted.
    Budgets(DeclaredBudgetsRefusal),
    /// A present work formula was not admitted.
    WorkFormula(WorkFormulaRefusal),
    /// A row was not built.
    Row(BenchRowRefusal),
    /// An attachment was not built.
    Attachment(BenchAttachmentRefusal),
    /// A binding was not built.
    Binding(BenchBindingRefusal),
    /// The preflight's trial binding was not built.
    Preflight(TrialTableRefusal),
    /// The table itself was not built.
    Table(BenchTableRefusal),
}

/// The host facts one whole table run is given, declared by whoever runs it.
#[derive(Debug, Clone)]
pub struct BenchInvocation {
    target: TargetBinding,
    clock: HarnessClock,
    contention: ContentionPosture,
}

/// Which target fact disagreed before any caller code ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchTargetMismatch {
    /// The compilation targets differ.
    Target {
        /// The benchmark invocation's target.
        benchmark: TargetTriple,
        /// The preflight invocation's target.
        preflight: TargetTriple,
    },
    /// The toolchains differ.
    Toolchain {
        /// The benchmark invocation's toolchain.
        benchmark: ToolchainIdentity,
        /// The preflight invocation's toolchain.
        preflight: ToolchainIdentity,
    },
}

/// Why no report was produced at all.
#[must_use = "a refusal is the reason no benchmark report was produced"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchRunRefusal {
    /// A row's preflight invocation names another target or toolchain.
    PreflightTargetMismatch {
        /// The row that could not run.
        row: BenchRowKey,
        /// The exact fact that disagreed.
        mismatch: BenchTargetMismatch,
    },
    /// A benchmark callable could not record the work it declared.
    WorkNotRecorded {
        /// The row that could not finish.
        row: BenchRowKey,
        /// Which callable refused.
        phase: PrimaryWorkPhase,
        /// Why the recorder refused.
        refusal: WorkRecordingRefusal,
    },
    /// The timed pass did not hold together.
    SecondaryWorkRefused {
        /// The row that could not finish.
        row: BenchRowKey,
        /// Why nothing was published for it.
        refusal: SecondaryObservationRefusal,
    },
}

/// Which of a row's two callables was running.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaryWorkPhase {
    /// The measured callable.
    Measured,
    /// The deliberately worse control.
    PlantedWorse,
}

/// Why the timed pass published nothing.
#[must_use = "a refusal is the reason qualified caller-clock readings were not published"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondaryObservationRefusal {
    /// A discarded warmup call could not record its work.
    Warmup(WorkRecordingRefusal),
    /// A timed sample call could not record its work.
    Sample(WorkRecordingRefusal),
    /// The timed pass no longer qualified under the same judge.
    Judgment(WorkQualificationRefusal),
}

/// How far one row got.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchOutcome {
    /// The correctness preflight refused, so no benchmark callable ran.
    PreflightRefused,
    /// The control was not both refused and told apart from the measured curve.
    PlantedWorseNotDistinguished {
        /// The measured curve.
        measured: WorkCurve,
        /// The control's curve.
        planted_worse: WorkCurve,
        /// The judgment that left the control inactive.
        judgment: WorkJudgment,
    },
    /// The control stood, and the measured curve refused.
    PrimaryWorkRefused {
        /// The measured curve.
        measured: WorkCurve,
        /// The control's curve.
        planted_worse: WorkCurve,
        /// The judgment carrying the measured refusal.
        judgment: WorkJudgment,
    },
    /// Correctness, the control, the primary work, and the timed pass all held.
    #[non_exhaustive]
    Qualified {
        /// The qualified measured curve.
        measured: WorkCurve,
        /// The refused control curve.
        planted_worse: WorkCurve,
        /// The accepted judgment over both.
        judgment: WorkJudgment,
        /// The timed pass and its clock readings.
        secondary: SecondaryObservation,
    },
}

/// The stage one outcome occupies, without the evidence that filled it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchStage {
    /// The correctness preflight refused.
    PreflightRefused,
    /// The control was not distinguished.
    PlantedWorseNotDistinguished,
    /// The measured work refused.
    PrimaryWorkRefused,
    /// The row qualified.
    Qualified,
}

/// What one row left behind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchReading {
    row: BenchRow,
    target: TargetBinding,
    preflight: TrialReport,
    outcome: BenchOutcome,
}

/// One reading per authored binding, in table order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchReport {
    table: BenchTableName,
    provenance: Provenance,
    readings: Vec<BenchReading>,
}

/// The first row that did not qualify, and where it stopped.
#[must_use = "a refusal names the first benchmark row that did not qualify"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchVerdictRefusal {
    row: BenchRowKey,
    stage: BenchStage,
}

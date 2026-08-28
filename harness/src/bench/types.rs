//! The stable public vocabulary road for declaration, work, and host execution.

pub use super::declaration::{
    BENCH_ROW_KEY_TAG, BenchMeasurement, BenchReferences, BenchRow, BenchRowKey, BenchRowRefusal,
    BenchTableName, ComplexityClaimRef, ContentionPosture, DeclaredBudgets, DeclaredBudgetsRefusal,
    ExactRatio, InputSizeAxis, InputSizeAxisRefusal, PlantedWorseRef, PreflightRef, WorkFormula,
    WorkFormulaRefusal, WorkObservationRef, WorkloadRef,
};
pub use super::work::{
    BenchAttachment, BenchAttachmentRefusal, BenchCall, SecondaryObservation,
    SecondaryObservationRefusal, WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint,
    WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment, WorkJudgmentInput,
    WorkQualificationRefusal, WorkRecorder, WorkRecordingRefusal,
};

#[path = "type_guard.rs"]
mod guard;

use crate::clock::HarnessClock;
use crate::descriptor::{NameRefusal, Provenance, TrialTableRefusal};
use crate::report::{TargetBinding, TargetTriple, ToolchainIdentity, TrialReport};
use crate::runner::{Invocation, TrialBinding};

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

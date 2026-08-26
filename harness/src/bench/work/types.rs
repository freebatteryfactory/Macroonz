//! The public vocabulary of recorded work, owner judgment, and executable work attachments.

#[path = "type_guard.rs"]
mod guard;

use super::super::declaration::{
    ComplexityClaimRef, DeclaredBudgets, PlantedWorseRef, WorkFormula, WorkObservationRef,
    WorkloadRef,
};
use crate::clock::MeasurementReading;
use crate::descriptor::NameRefusal;
use crate::report::FindingCause;

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

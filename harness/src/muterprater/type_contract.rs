//! The declarative trait participation: total conversions between this home's typed values and the axes and vocabularies they project into.
//!
//! Every realization here is a declaration rather than a computation — one constant answer per arm, stated once so every reader reaches the same word from the same value.
//! Writing them as conversions rather than as branches inside callers is what keeps two readers of one record from disagreeing.

use super::types::{
    ActivationAxis, ActivationDisposition, ClaimCeiling, ClaimPinnedGround, ExecutionAxis,
    MaterializationAxis, MutantKilledGround, MutationOutcome, MutationVerdict,
    ObligationDischargedGround, ObligationLane, ProofShape, ReadingSource, WrapOutcomeWord,
};
use crate::descriptor::AdmissionGround;

impl From<ActivationDisposition> for ActivationAxis {
    /// The axis one activation disposition reads as.
    ///
    /// The observed arm carries its evidence and the axis does not, so a caller that needs the evidence takes [`ActivationDisposition::evidence`](super::ActivationDisposition::evidence).
    fn from(disposition: ActivationDisposition) -> Self {
        match disposition {
            ActivationDisposition::Observed(_) => Self::Observed,
            ActivationDisposition::NotObserved => Self::NotObserved,
            ActivationDisposition::UnobservableUnderBackend => Self::UnobservableUnderBackend,
        }
    }
}

impl From<&MutationOutcome> for MutationVerdict {
    /// The verdict word one outcome reads as, which is what a census counts.
    fn from(outcome: &MutationOutcome) -> Self {
        match outcome {
            MutationOutcome::Killed(_) => Self::Killed,
            MutationOutcome::Survived => Self::Survived,
            MutationOutcome::Inconclusive(_) => Self::Inconclusive,
        }
    }
}

impl From<WrapOutcomeWord> for MaterializationAxis {
    /// Whether the damage the backend's word describes became executable.
    ///
    /// Caught, missed, and timed-out all describe a mutant that built, because the backend could not have run a command over one that did not.
    /// Unviable is the backend saying the damage never became a thing, and a tool failure is the backend saying it does not know.
    fn from(word: WrapOutcomeWord) -> Self {
        match word {
            WrapOutcomeWord::Caught | WrapOutcomeWord::Missed | WrapOutcomeWord::TimedOut => {
                Self::Built
            }
            WrapOutcomeWord::Unviable => Self::Unviable,
            WrapOutcomeWord::ToolFailed => Self::ToolFailed,
        }
    }
}

impl From<WrapOutcomeWord> for ExecutionAxis {
    /// What became of the witness execution the backend's word describes.
    ///
    /// A mutant that never materialized never ran, and a backend that failed around one learned nothing about it: both read as an infrastructure failure.
    fn from(word: WrapOutcomeWord) -> Self {
        match word {
            WrapOutcomeWord::Caught | WrapOutcomeWord::Missed => Self::Completed,
            WrapOutcomeWord::TimedOut => Self::TimedOut,
            WrapOutcomeWord::Unviable | WrapOutcomeWord::ToolFailed => Self::InfrastructureFailed,
        }
    }
}

impl From<ReadingSource> for ClaimCeiling {
    /// The most a reading taken from one output can establish.
    ///
    /// A console stream states which of the backend's own mutants its command rejected and carries no channel that could observe a firing, so the reading it affords tops out at witness rejection.
    fn from(source: ReadingSource) -> Self {
        match source {
            ReadingSource::ConsoleStream => Self::WitnessRejection,
        }
    }
}

impl From<&MutantKilledGround> for AdmissionGround {
    /// The word a kill's admission act states.
    ///
    /// The typed ground with its evidence stays on the proposal, and the admitted row cites the proposal by identity rather than copying it.
    fn from(_ground: &MutantKilledGround) -> Self {
        Self::MutantKilled
    }
}

impl From<&ClaimPinnedGround> for AdmissionGround {
    /// The word a pin's admission act states.
    fn from(_ground: &ClaimPinnedGround) -> Self {
        Self::ClaimPinned
    }
}

impl From<&ObligationDischargedGround> for AdmissionGround {
    /// The word a discharge's admission act states.
    fn from(_ground: &ObligationDischargedGround) -> Self {
        Self::ObligationDischarged
    }
}

impl From<ProofShape> for ObligationLane {
    /// The lane one opening's shape of proof routes to.
    ///
    /// A stated input with a stated answer is a descriptor row, a search over a generated population is a fuzz seed, and a fault placed at a sequence position is a chaos scenario.
    /// Three shapes, three lanes, and no shape reaches two.
    fn from(shape: ProofShape) -> Self {
        match shape {
            ProofShape::StatedCase => Self::TestRow,
            ProofShape::GeneratedSearch => Self::FuzzSeed,
            ProofShape::ScheduledFault => Self::ChaosScenario,
        }
    }
}

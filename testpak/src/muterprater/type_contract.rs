//! The proof-pressure engine's declarative trait participation: the total conversions between its typed axes and borrowed vocabularies.
//!
//! Every realization here is a DECLARATION rather than a computation — a
//! constant answer per arm, stated once so it is read in one place instead of
//! inferred from whichever road happened to need it.
//!
//! # The axis projections
//!
//! A record carries the disposition with its evidence; a reader that wants the
//! AXIS takes the projection. Writing them as conversions rather than as
//! branches inside a caller is what keeps every reader of activation and every
//! reader of a verdict reaching the same word from the same record.
//!
//! # The backend word conversions
//!
//! One outcome word from a compiled-mutation backend states two separate facts —
//! whether the damage materialized and what became of the witness execution —
//! and the two are declared apart because they are apart: an unviable mutant
//! never ran, and a timed-out one built and then did not finish.
//!
//! # The claim-ceiling conversion
//!
//! What a reading of a backend's output may at most establish follows from the OUTPUT it was taken from, so the source-to-ceiling conversion is declared here.
//! The ceiling's invariant readings live with its guard.
//!
//! # The routing table
//!
//! Which lane discharges an obligation follows from the SHAPE of proof its
//! opening asks for, and nothing else. Stated as a map so lane choice is a
//! planning decision a reader can check rather than a branch inside a planner.
//!
use super::types::{
    ActivationAxis, ActivationDisposition, ClaimCeiling, ClaimPinnedGround, ExecutionAxis,
    MaterializationAxis, MutantKilledGround, MutationOutcome, MutationVerdict,
    ObligationDischargedGround, ObligationLane, ProofShape, ReadingSource, WrapOutcomeWord,
};
use crate::descriptor::AdmissionGround;

impl From<ActivationDisposition> for ActivationAxis {
    /// The axis one activation disposition reads as.
    ///
    /// The observed arm carries its evidence and the axis does not; a caller
    /// that needs the evidence takes
    /// [`ActivationDisposition::evidence`](super::ActivationDisposition::evidence).
    fn from(disposition: ActivationDisposition) -> Self {
        match disposition {
            ActivationDisposition::Observed(_) => Self::Observed,
            ActivationDisposition::NotObserved => Self::NotObserved,
            ActivationDisposition::UnobservableUnderBackend => Self::UnobservableUnderBackend,
        }
    }
}

impl From<&MutationOutcome> for MutationVerdict {
    /// The verdict word one outcome reads as.
    ///
    /// The killed arm carries the rejection that killed it and the inconclusive
    /// arm carries the link of the chain that did not hold; the word is what a
    /// census counts.
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
    /// Caught, missed, and timed-out all describe a mutant that BUILT — the
    /// backend could not have run a command over one that did not. Unviable is
    /// the backend saying the damage never became a thing, and a tool failure is
    /// the backend saying it does not know.
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
    /// A mutant that never materialized never ran, and a backend that failed
    /// around one learned nothing about it: both read as an infrastructure
    /// failure, which is the arm that states that nothing was established about
    /// the subject.
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
    /// A console stream states which of the backend's own mutants its command
    /// rejected and carries no channel that could observe a damage firing, so
    /// the reading it affords tops out at witness rejection.
    fn from(source: ReadingSource) -> Self {
        match source {
            ReadingSource::ConsoleStream => Self::WitnessRejection,
        }
    }
}

impl From<&MutantKilledGround> for AdmissionGround {
    /// The ground at summary width — the word an admission act states.
    ///
    /// The typed ground with its evidence stays on the proposal, and the
    /// admitted row cites the proposal by identity rather than copying it.
    ///
    /// One row per ground, and the row is a constant rather than a match: which
    /// word a ground states is settled by which ground it IS, and there is no
    /// second ground here for a branch to choose between.
    fn from(_ground: &MutantKilledGround) -> Self {
        Self::MutantKilled
    }
}

impl From<&ClaimPinnedGround> for AdmissionGround {
    /// The word a pin's admission act states, on the terms
    /// [`MutantKilledGround`]'s row states.
    fn from(_ground: &ClaimPinnedGround) -> Self {
        Self::ClaimPinned
    }
}

impl From<&ObligationDischargedGround> for AdmissionGround {
    /// The word a discharge's admission act states, on the same terms.
    fn from(_ground: &ObligationDischargedGround) -> Self {
        Self::ObligationDischarged
    }
}

impl From<ProofShape> for ObligationLane {
    /// The lane one opening's shape of proof routes to.
    ///
    /// A stated input with a stated answer is a descriptor row; a search over a
    /// generated population is a fuzz seed; a fault placed at a sequence
    /// position is a chaos scenario. Three shapes, three lanes, and no shape
    /// reaches two.
    fn from(shape: ProofShape) -> Self {
        match shape {
            ProofShape::StatedCase => Self::TestRow,
            ProofShape::GeneratedSearch => Self::FuzzSeed,
            ProofShape::ScheduledFault => Self::ChaosScenario,
        }
    }
}

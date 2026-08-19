//! The proof-pressure engine's declarative surface: the total maps its arms are
//! read through.
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
//! # The backend word tables
//!
//! One outcome word from a compiled-mutation backend states two separate facts —
//! whether the damage materialized and what became of the witness execution —
//! and the two are declared apart because they are apart: an unviable mutant
//! never ran, and a timed-out one built and then did not finish.
//!
//! # The routing table
//!
//! Which lane discharges an obligation follows from the SHAPE of proof its
//! opening asks for, and nothing else. Stated as a map so lane choice is a
//! planning decision a reader can check rather than a branch inside a planner.

use super::types::{
    ActivationAxis, ActivationDisposition, AdmissionPatch, ExecutionAxis, MaterializationAxis,
    MutationOutcome, MutationVerdict, ObligationLane, ProofShape, ProposalGround, WrapOutcomeWord,
};
use crate::descriptor::{AdmissionGround, CapsulePosture};

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

impl From<&ProposalGround> for AdmissionGround {
    /// The ground at summary width — the word an admission act states.
    ///
    /// The typed ground with its evidence stays on the proposal, and the
    /// admitted row cites the proposal by identity rather than copying it.
    fn from(ground: &ProposalGround) -> Self {
        match ground {
            ProposalGround::MutantKilled { .. } => Self::MutantKilled,
            ProposalGround::ClaimPinned { .. } => Self::ClaimPinned,
            ProposalGround::ObligationDischarged { .. } => Self::ObligationDischarged,
        }
    }
}

impl From<CapsulePosture> for AdmissionPatch {
    /// Which two-part patch admitting on a ground of this posture would author.
    ///
    /// A replay-bearing ground authors the row and the depot capsule entry the
    /// row's replay reference points at; a discharge authors the row alone. The
    /// posture is the descriptor home's declaration and this map only names what
    /// it means for the human act at the road's exit — nothing here admits
    /// anything.
    fn from(posture: CapsulePosture) -> Self {
        match posture {
            CapsulePosture::ReplayBearing => Self::RowAndCapsule,
            CapsulePosture::NoCapsule => Self::RowAlone,
        }
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

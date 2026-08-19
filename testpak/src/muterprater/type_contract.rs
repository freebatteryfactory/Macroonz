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
//! # The claim ceiling
//!
//! What a reading of a backend's output may at most establish follows from the
//! OUTPUT it was taken from, so the ceiling is a map over the sources and the
//! verdicts it admits is a second one. Declared here rather than stated on a
//! profile, so no reading is handed a ceiling wider than its source affords.
//!
//! # The routing table
//!
//! Which lane discharges an obligation follows from the SHAPE of proof its
//! opening asks for, and nothing else. Stated as a map so lane choice is a
//! planning decision a reader can check rather than a branch inside a planner.
//!
//! # The seed roster's rendering
//!
//! The artifact-mutation roster's one projection is here for the same reason:
//! a person reading a plan needs the damage in words, and a constant answer per
//! arm is a declaration. Nothing decides anything by it.

use super::types::{
    ActivationAxis, ActivationDisposition, AdmissionPatch, ArtifactMutation, ClaimCeiling,
    ExecutionAxis, MaterializationAxis, MutationOutcome, MutationVerdict, ObligationLane,
    ProofShape, ProposalGround, ReadingSource, WrapOutcomeWord, WrappedBackend,
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

impl WrappedBackend {
    /// The backend's own name.
    ///
    /// A projection: a reader of a profile names the tool through it, and no
    /// decision anywhere consults it.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::CargoMutants => "cargo-mutants",
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

impl ClaimCeiling {
    /// The strongest verdict this ceiling grants.
    #[must_use]
    pub const fn strongest(self) -> MutationVerdict {
        match self {
            Self::WitnessRejection => MutationVerdict::Killed,
        }
    }

    /// Whether one verdict stands inside this ceiling.
    ///
    /// A kill and an inconclusive both stand inside witness rejection; survived
    /// stands outside it, because earning that word takes an activation the
    /// source offers no channel to observe.
    #[must_use]
    pub const fn admits(self, verdict: MutationVerdict) -> bool {
        match (self, verdict) {
            (Self::WitnessRejection, MutationVerdict::Killed | MutationVerdict::Inconclusive) => {
                true
            }
            (Self::WitnessRejection, MutationVerdict::Survived) => false,
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

impl ArtifactMutation {
    /// The damage rendered for a person.
    ///
    /// A projection: a plan and a survivor explanation name a row through it,
    /// and no decision anywhere consults it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::OrderPermuted => "the textual selection order is reversed",
            Self::IdentityRecycled => "every cause is emitted under one local key",
            Self::PlannedOutputOmitted => "a planned output is deleted",
            Self::UnplannedOutputAdded => "an unplanned output is appended",
            Self::ImplTargetAltered => "the implementation targets a different type",
            Self::ShapeAltered => "the declared body shape is changed",
            Self::OutputDuplicated => "a planned output is emitted twice",
            Self::TraitPathWrong => "the trait path names a different contract",
            Self::DecoyInComment => "the anchored bytes are planted in a comment",
            Self::ImplMemberDuplicated => "one member constant is emitted twice",
            Self::ImplMemberUnexpected => "a member nobody planned joins the implementation",
            Self::ConstructorPathAltered => "a row is built through another constructor",
            Self::ImplPostureAltered => "the implementation is written under another posture",
            Self::MeaningBearingAttributeAdded => "an attribute that decides something is added",
            Self::MalformedRust => "the artifact stops being well-formed Rust",
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

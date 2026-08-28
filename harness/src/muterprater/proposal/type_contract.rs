use super::types::{
    ClaimPinnedGround, MutantKilledGround, ObligationDischargedGround, ObligationLane, ProofShape,
};
use crate::descriptor::AdmissionGround;

impl From<&MutantKilledGround> for AdmissionGround {
    /// The word a kill's admission act states.
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
    fn from(shape: ProofShape) -> Self {
        match shape {
            ProofShape::StatedCase => Self::TestRow,
            ProofShape::GeneratedSearch => Self::FuzzSeed,
            ProofShape::ScheduledFault => Self::ChaosScenario,
        }
    }
}

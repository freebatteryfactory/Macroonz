//! The comparison nucleus that requires the actual witness and current report to agree.

use super::{
    HistoricalReplayStanding, ReplayJoinRefusal, ReplayMovement, ReplayOutcome, ReplayReading,
    WitnessLineage,
};
use crate::input::InputEnvelope;
use crate::report::{TrialReport, archive::ArchivedCapsule};

#[path = "guard_movement.rs"]
mod movement;

impl ReplayReading {
    /// One comparison whose current case and convention admit the retained witness.
    pub(in crate::report::replay) fn joined(
        historical: &ArchivedCapsule,
        current: &TrialReport,
        witness: &InputEnvelope,
    ) -> Result<Self, ReplayJoinRefusal> {
        let input = joined_input(historical.input(), current, witness)?;
        let movement = ReplayMovement::between(historical.key(), current.standing().key(), input);
        let standing = super::super::read::standing(historical, current);
        Ok(Self {
            movement,
            lineage: super::super::read::lineage(historical, input, movement),
            standing,
            outcome: super::super::read::outcome(historical, current, movement, standing),
        })
    }

    /// The independently compared execution coordinates.
    #[must_use]
    pub const fn movement(self) -> ReplayMovement {
        self.movement
    }

    /// The reached case relative to the historically recorded original case.
    #[must_use]
    pub const fn lineage(self) -> WitnessLineage {
        self.lineage
    }

    /// The historical comparison's accountability ceiling.
    #[must_use]
    pub const fn standing(self) -> HistoricalReplayStanding {
        self.standing
    }

    /// What the current attempt establishes on the joined witness.
    #[must_use]
    pub const fn outcome(self) -> ReplayOutcome {
        self.outcome
    }
}

fn joined_input(
    saved: &[u8],
    current: &TrialReport,
    witness: &InputEnvelope,
) -> Result<crate::report::ExecutionInput, ReplayJoinRefusal> {
    if saved != witness.payload() {
        return Err(ReplayJoinRefusal::WitnessBytesDiffer);
    }
    let input = current
        .standing()
        .key()
        .input()
        .ok_or(ReplayJoinRefusal::CurrentInputUnrecorded)?;
    if input.profile() != witness.profile() {
        return Err(ReplayJoinRefusal::CurrentProfileDiffers);
    }
    if input.case() != witness.case() {
        return Err(ReplayJoinRefusal::CurrentCaseDiffers);
    }
    Ok(input)
}

#[path = "guard_legacy.rs"]
mod legacy;

//! Reading what a plan decided into the statement a publication record is built from.

use super::{StampError, StampedPlan};
use crate::kind::{Destination, Kind, Role};
use crate::plan::Plan;

/// Read one plan into the statement of what the artifact standing at one seat will be.
///
/// The seat is the caller's: a kind that renders one artifact names its one role, and a kind that renders several names the role this stamp stands under.
///
/// # Errors
///
/// Returns [`StampError::SeatNotPlanned`] where the plan declares no member under that seat, and [`StampError::DestinationNotArtifact`] where the seat delivers anywhere other than a standalone artifact.
///
/// The seat is read before its delivery, so exactly one cause is true of any refused reading.
pub fn planned<K: Kind>(plan: &Plan<K>, role: K::Role) -> Result<StampedPlan, StampError> {
    let Some(member) = plan.membership().under(role) else {
        return Err(StampError::SeatNotPlanned {
            role_slot: role.slot(),
        });
    };
    if role.destination() != Destination::PublicationArtifact {
        return Err(StampError::DestinationNotArtifact {
            role_slot: role.slot(),
        });
    }
    Ok(StampedPlan {
        unit: member.output.semantic_key,
        staged: member.output.digest_contract,
    })
}

//! The preemption explore road on a target the pinned backend does not implement.

use super::types::{
    IncompleteExploration, PreemptionBounds, PreemptionModelResult, PreemptionOutcome,
    PreemptionReading,
};

/// Retain the request as typed backend unavailability without invoking the model.
#[must_use]
pub(super) const fn explored(
    bounds: PreemptionBounds,
    _model: fn() -> PreemptionModelResult,
) -> PreemptionReading {
    PreemptionReading::read(
        bounds,
        PreemptionOutcome::Incomplete(IncompleteExploration::Unavailable),
    )
}

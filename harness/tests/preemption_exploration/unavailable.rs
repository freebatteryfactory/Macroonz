//! Unsupported-target claims over the target-neutral result plane.

use macroonz_harness::preemption::{
    IncompleteExploration, PreemptionBound, PreemptionBounds, PreemptionBoundsRefusal,
    PreemptionModelFailure, PreemptionModelResult, PreemptionOutcome, attempted, explored,
};
use macroonz_harness::report::{InfrastructureFault, RunAttempt};

/// A model result which would establish a completed refusal if an unavailable backend invoked it.
fn refusing_model() -> PreemptionModelResult {
    Err(PreemptionModelFailure::reported(
        b"an unavailable backend invoked the model",
    ))
}

/// The public road stays present, retains the declaration, and reports typed unavailability without interpreting the model.
#[test]
fn an_unsupported_target_retains_typed_unavailability() -> Result<(), PreemptionBoundsRefusal> {
    let bounds = PreemptionBounds::declared(PreemptionBound::AtMost(2u32), 1_000u32)?;
    let reading = explored(bounds, refusing_model);
    assert_eq!(reading.bounds(), bounds);
    assert_eq!(
        reading.outcome(),
        &PreemptionOutcome::Incomplete(IncompleteExploration::Unavailable)
    );
    assert!(matches!(
        attempted(&reading),
        RunAttempt::InfrastructureFailed(ref failure)
            if failure.fault() == InfrastructureFault::BackendUnavailable
                && failure.foreign().is_none()
    ));
    Ok(())
}

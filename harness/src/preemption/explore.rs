//! The target-neutral preemption road: declared bounds and a typed model check in, one reading and one ordinary run-attempt projection out.

use super::backend;
use super::types::{
    IncompleteExploration, MODEL_BROKE, PreemptionBounds, PreemptionModelResult, PreemptionOutcome,
    PreemptionReading, PreemptionVerdict,
};
use crate::report::{
    FailureClass, FindingLocation, InfrastructureFailure, InfrastructureFault, RunAttempt,
    TrialConclusion, TrialFinding,
};

/// Explore one model under the declared bounds using the backend qualified for this target.
///
/// The model returns its check as a value, so only an explicit [`Err`] can establish [`PreemptionVerdict::ModelBroke`].
/// A target without the pinned backend returns typed unavailability through the same reading rather than removing the public door or compiling an unsupported dependency.
#[must_use]
pub fn explored(
    bounds: PreemptionBounds,
    model: fn() -> PreemptionModelResult,
) -> PreemptionReading {
    backend::explored(bounds, model)
}

/// Project one exploration onto the harness's existing attempt rail.
///
/// A completed verdict becomes an executed trial conclusion; an incomplete exploration becomes an infrastructure failure, so no backend failure can impersonate a subject verdict.
#[must_use]
pub fn attempted(reading: &PreemptionReading) -> RunAttempt {
    match reading.outcome() {
        PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld) => {
            RunAttempt::Executed(TrialConclusion::Passed)
        }
        PreemptionOutcome::Completed(PreemptionVerdict::ModelBroke { report }) => {
            RunAttempt::Executed(TrialConclusion::Refused(TrialFinding::established(
                FailureClass::RefusedByCheck,
                MODEL_BROKE,
                FindingLocation::at(file!(), line!()),
                report.clone(),
            )))
        }
        PreemptionOutcome::Incomplete(IncompleteExploration::Unavailable) => {
            RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
                InfrastructureFault::BackendUnavailable,
                None,
            ))
        }
        PreemptionOutcome::Incomplete(IncompleteExploration::InitializationFailed { report }) => {
            RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
                InfrastructureFault::BackendInitializationFailed,
                report.clone(),
            ))
        }
        PreemptionOutcome::Incomplete(IncompleteExploration::ExecutionUnresolved { report }) => {
            RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
                InfrastructureFault::BackendExecutionUnresolved,
                report.clone(),
            ))
        }
    }
}

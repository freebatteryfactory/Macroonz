//! Inverse consistency of a retained interpreted outcome with its retained trial.

use super::{
    ArchivedActivation, ArchivedMutation, ArchivedMutationOutcome, ArchivedRejection,
    MutationArchiveRefusal,
};
use crate::muterprater::{
    BaselineAxis, EquivalenceAxis, ExecutionAxis, InconclusiveCause, MaterializationAxis,
};
use crate::report::archive::{ArchivedAttempt, ArchivedConclusion, ArchivedTrial};

pub(crate) fn interpreted_trial_join(
    trial: &ArchivedTrial,
    mutation: &ArchivedMutation,
) -> Result<(), MutationArchiveRefusal> {
    if mutation.baseline() != BaselineAxis::Qualified
        || mutation.materialization() != MaterializationAxis::Built
        || !matches!(mutation.activation(), ArchivedActivation::Observed(_))
        || mutation.equivalence() != EquivalenceAxis::NotAssessed
    {
        return Err(MutationArchiveRefusal::InterpretedTrialMismatch);
    }
    match (mutation.execution(), mutation.outcome(), trial.attempt()) {
        (
            ExecutionAxis::Completed,
            ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(rejection)),
            ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)),
        ) if rejection == finding => Ok(()),
        (
            ExecutionAxis::Completed,
            ArchivedMutationOutcome::Survived,
            ArchivedAttempt::Executed(ArchivedConclusion::Passed),
        )
        | (
            ExecutionAxis::NotExecuted,
            ArchivedMutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ArchivedAttempt::SkippedWithReason(_),
        )
        | (
            ExecutionAxis::TimedOut,
            ArchivedMutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ArchivedAttempt::TimedOut,
        )
        | (
            ExecutionAxis::InfrastructureFailed,
            ArchivedMutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ArchivedAttempt::InfrastructureFailed {
                fault: _,
                foreign: _,
            },
        ) => Ok(()),
        _ => Err(MutationArchiveRefusal::InterpretedTrialMismatch),
    }
}

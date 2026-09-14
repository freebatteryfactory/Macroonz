//! Projection-specific joins over admitted historical parity and report owners.

use super::super::ProjectionArchiveRefusal;
use crate::muterprater::discovery_archive::ArchivedSelection;
use crate::muterprater::interpretation_archive::ArchivedParity;
use crate::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutation, ArchivedMutationIdentity, ArchivedMutationOutcome,
    ArchivedMutationSite, ArchivedRejection,
};
use crate::muterprater::{BaselineAxis, EquivalenceAxis, ExecutionAxis, MaterializationAxis};
use crate::report::archive::{ArchivedAttempt, ArchivedConclusion, ArchivedTrial};

pub(super) fn reports(
    parity: &ArchivedParity,
    baseline: &ArchivedTrial,
    selected: &ArchivedTrial,
) -> Result<(), ProjectionArchiveRefusal> {
    let production = parity.production_report();
    if baseline.key() != production.key()
        || selected.key() != production.key()
        || baseline.site() != selected.site()
        || baseline.claimed_posture() != production.claimed_posture()
        || selected.claimed_posture() != production.claimed_posture()
        || !matches!(
            baseline.attempt(),
            ArchivedAttempt::Executed(ArchivedConclusion::Passed)
        )
        || !matches!(
            selected.attempt(),
            ArchivedAttempt::Executed(ArchivedConclusion::Refused(_))
        )
    {
        return Err(ProjectionArchiveRefusal::ReportJoinMismatch);
    }
    Ok(())
}

pub(super) fn mutation(
    parity: &ArchivedParity,
    selection: &ArchivedSelection,
    selected: &ArchivedTrial,
    mutation: &ArchivedMutation,
) -> Result<(), ProjectionArchiveRefusal> {
    let target = mutation.target();
    let ArchivedMutationIdentity::CompiledProjection { point, alternative } = target.identity()
    else {
        return Err(ProjectionArchiveRefusal::MutationJoinMismatch);
    };
    if point != selection.point()
        || *alternative != selection.alternative()
        || target.family().is_none()
        || !matches!(target.site(), ArchivedMutationSite::Declared(_))
        || target.owner() != Some(parity.witness().row().claim())
        || mutation.baseline() != BaselineAxis::Qualified
        || mutation.materialization() != MaterializationAxis::Built
        || mutation.activation() != &ArchivedActivation::UnobservableUnderBackend
        || mutation.execution() != ExecutionAxis::Completed
        || mutation.equivalence() != EquivalenceAxis::NotAssessed
    {
        return Err(ProjectionArchiveRefusal::MutationJoinMismatch);
    }
    match (selected.attempt(), mutation.outcome()) {
        (
            ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)),
            ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(rejection)),
        ) if finding == rejection => Ok(()),
        _ => Err(ProjectionArchiveRefusal::MutationJoinMismatch),
    }
}

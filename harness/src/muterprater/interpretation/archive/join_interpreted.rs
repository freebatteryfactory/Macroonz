//! Exact historical joins without strengthening absent backend or policy preimages.

use super::super::InterpretedArchiveRefusal;
use crate::muterprater::discovery_archive::{
    ArchivedAlternative, ArchivedEvaluationSurface, ArchivedMutationPoint,
};
use crate::muterprater::specimen_archive::ArchivedProjectionPressure;
use crate::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutation, ArchivedMutationIdentity, ArchivedMutationSite,
    ArchivedMutationTarget,
};
use crate::report::archive::ArchivedTrial;

pub(super) fn trust(
    surface: &ArchivedEvaluationSurface,
    projection: &ArchivedProjectionPressure,
) -> Result<(), InterpretedArchiveRefusal> {
    let standing = projection.standing();
    if standing.pair().family() != surface.family()
        || standing.pair().surface().as_bytes() != surface.identity().as_bytes()
    {
        return Err(InterpretedArchiveRefusal::SurfaceJoinMismatch);
    }
    let (point, alternative) = surface
        .selected_alternative(standing.selection())
        .ok_or(InterpretedArchiveRefusal::SurfaceJoinMismatch)?;
    target(projection.mutation().target(), point, alternative)
}

pub(super) fn active(
    surface: &ArchivedEvaluationSurface,
    projection: &ArchivedProjectionPressure,
    report: &ArchivedTrial,
    mutation: &ArchivedMutation,
) -> Result<(), InterpretedArchiveRefusal> {
    let standing = projection.standing();
    if report.key() != standing.execution()
        || report.claimed_posture() != projection.parity().production_report().claimed_posture()
    {
        return Err(InterpretedArchiveRefusal::ReportJoinMismatch);
    }
    let (point, alternative) = surface
        .selected_alternative(standing.selection())
        .ok_or(InterpretedArchiveRefusal::SurfaceJoinMismatch)?;
    if !matches!(
        mutation.target().identity(),
        ArchivedMutationIdentity::Interpreted { .. }
    ) {
        return Err(InterpretedArchiveRefusal::TargetJoinMismatch);
    }
    target(mutation.target(), point, alternative)?;
    let ArchivedActivation::Observed(activation) = mutation.activation() else {
        return Err(InterpretedArchiveRefusal::ActivationJoinMismatch);
    };
    let selection = standing.selection();
    if activation.surface() != selection.surface()
        || activation.point() != selection.point()
        || activation.alternative() != selection.alternative()
        || activation.witness() != report.key().trial()
    {
        return Err(InterpretedArchiveRefusal::ActivationJoinMismatch);
    }
    Ok(())
}

fn target(
    target: &ArchivedMutationTarget,
    point: &ArchivedMutationPoint,
    alternative: &ArchivedAlternative,
) -> Result<(), InterpretedArchiveRefusal> {
    let (named_point, named_alternative) = match target.identity() {
        ArchivedMutationIdentity::CompiledProjection {
            point: claimed_point,
            alternative: claimed_alternative,
        }
        | ArchivedMutationIdentity::Interpreted {
            point: claimed_point,
            alternative: claimed_alternative,
        } => (claimed_point, claimed_alternative),
        ArchivedMutationIdentity::External(_) => {
            return Err(InterpretedArchiveRefusal::TargetJoinMismatch);
        }
    };
    if named_point != point.name()
        || named_alternative.as_bytes() != alternative.identity().as_bytes()
        || target.family() != Some(alternative.family())
        || target.owner() != Some(point.owner_claim())
        || !matches!(target.site(), ArchivedMutationSite::Declared(site) if site == point.activation_site())
    {
        return Err(InterpretedArchiveRefusal::TargetJoinMismatch);
    }
    Ok(())
}

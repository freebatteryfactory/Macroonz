//! Historical joins of the exact mutation, complete source and all witness judgments.

use super::super::{ArchivedAssessment, AssessmentArchiveRefusal};
use super::{interpreted_joins, read};
use crate::muterprater::EquivalenceAxis;
use crate::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutationIdentity, assessed_trial_join,
};
use crate::properties::Agreement;
use crate::report::archive::{ArchivedAttempt, ArchivedConclusion};

pub(super) fn joined(assessment: &ArchivedAssessment) -> Result<(), AssessmentArchiveRefusal> {
    let surface = assessment.surface();
    let pair = assessment.pair();
    let selection = assessment.selection();
    if pair.family() != surface.family()
        || pair.surface().as_bytes() != surface.identity().as_bytes()
        || selection.surface() != pair.surface()
        || assessment.baseline_content().identity() == assessment.selected_content().identity()
    {
        return Err(AssessmentArchiveRefusal::JoinMismatch);
    }
    let (point, alternative) = surface
        .selected_alternative(selection)
        .ok_or(AssessmentArchiveRefusal::JoinMismatch)?;
    let mutation = assessment.mutation();
    if !matches!(
        mutation.target().identity(),
        ArchivedMutationIdentity::Interpreted { .. }
    ) || assessment.witness().row().claim() != point.owner_claim()
    {
        return Err(AssessmentArchiveRefusal::JoinMismatch);
    }
    interpreted_joins::target(mutation.target(), point, alternative)
        .map_err(|_| AssessmentArchiveRefusal::JoinMismatch)?;
    let [
        production,
        baseline,
        compiled_baseline,
        compiled_selected,
        selected,
    ] = assessment.reports();
    for report in [baseline, compiled_baseline, compiled_selected, selected] {
        read::joined_reports(assessment.witness(), production, report)?;
    }
    if [production, baseline, compiled_baseline]
        .iter()
        .any(|report| {
            !matches!(
                report.attempt(),
                ArchivedAttempt::Executed(ArchivedConclusion::Passed)
            )
        })
        || selected.attempt() != compiled_selected.attempt()
    {
        return Err(AssessmentArchiveRefusal::JoinMismatch);
    }
    let ArchivedActivation::Observed(activation) = mutation.activation() else {
        return Err(AssessmentArchiveRefusal::JoinMismatch);
    };
    if activation.surface() != selection.surface()
        || activation.point() != selection.point()
        || activation.alternative() != selection.alternative()
        || activation.witness() != selected.key().trial()
    {
        return Err(AssessmentArchiveRefusal::JoinMismatch);
    }
    let expected = match assessment.difference() {
        Agreement::Agrees => EquivalenceAxis::NotAssessed,
        Agreement::Differs => EquivalenceAxis::Refuted,
    };
    if mutation.equivalence() != expected {
        return Err(AssessmentArchiveRefusal::JoinMismatch);
    }
    assessed_trial_join(selected, mutation)?;
    Ok(())
}

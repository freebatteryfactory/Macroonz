//! Bounded historical assessment reading without callback or live qualification reconstruction.

use super::super::{
    ASSESSMENT_ARCHIVE_TAG, ArchivedAssessment, ArchivedEvaluationPair, ArchivedValue,
    ArchivedValueConvention, AssessmentArchiveLimits, AssessmentArchiveRefusal,
    ParityArchiveLimits, ValueRole,
};
use super::members;
use crate::descriptor::archive::{read_binding, read_revision};
use crate::identity::BodyReader;
use crate::muterprater::discovery_archive::{read_selection, read_surface};
use crate::muterprater::specimen_archive::read_artifact;
use crate::muterprater::verdict_archive::read_mutation;
use crate::properties::Agreement;
use crate::report::archive::{ArchiveRefusal, claim, envelope, finish, frame, name, read_trial};

/// Read a complete historical assessment without invoking any current executable.
///
/// # Errors
///
/// Refuses malformed, oversized or contradictory retained claims without creating live qualification.
pub fn read_assessment(
    encoded: &[u8],
    limits: AssessmentArchiveLimits,
) -> Result<ArchivedAssessment, AssessmentArchiveRefusal> {
    let projection = limits.projection();
    let bytes = projection.bytes();
    let values = projection.parity();
    let (address, mut reader) = envelope(encoded, ASSESSMENT_ARCHIVE_TAG, 1, bytes)?;
    let surface = read_surface(frame(&mut reader, bytes)?, limits.surface())?;
    let witness = read_binding(frame(&mut reader, bytes)?, values.binding())?;
    let mutation = read_mutation(frame(&mut reader, bytes)?, projection.mutation())?;
    let pair = ArchivedEvaluationPair {
        family: name(&mut reader, bytes)?,
        production: read_revision(frame(&mut reader, bytes)?, bytes.field())?,
        evaluation: read_revision(frame(&mut reader, bytes)?, bytes.field())?,
        surface: claim(&mut reader, bytes)?,
    };
    let selection = read_selection(&mut reader, bytes)?;
    let baseline_content = read_artifact(&mut reader, projection)?;
    let selected_content = read_artifact(&mut reader, projection)?;
    let substrate = members::substrate(&mut reader, values)?;
    let difference = match reader.byte()? {
        0 => Agreement::Agrees,
        1 => Agreement::Differs,
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    };
    let reports = [
        read_trial(frame(&mut reader, bytes)?, projection.trial())?,
        read_trial(frame(&mut reader, bytes)?, projection.trial())?,
        read_trial(frame(&mut reader, bytes)?, projection.trial())?,
        read_trial(frame(&mut reader, bytes)?, projection.trial())?,
        read_trial(frame(&mut reader, bytes)?, projection.trial())?,
    ];
    let input = ArchivedValue {
        convention: members::convention(&mut reader, values)?,
        bytes: members::value(&mut reader, ValueRole::Input, values)?,
    };
    let convention = members::convention(&mut reader, values)?;
    let meanings = [
        value(&mut reader, &convention, ValueRole::Production, values)?,
        value(
            &mut reader,
            &convention,
            ValueRole::BaselineEvaluation,
            values,
        )?,
        value(
            &mut reader,
            &convention,
            ValueRole::CompiledBaseline,
            values,
        )?,
        value(
            &mut reader,
            &convention,
            ValueRole::CompiledSelected,
            values,
        )?,
        value(
            &mut reader,
            &convention,
            ValueRole::SelectedEvaluation,
            values,
        )?,
    ];
    finish(&reader)?;
    let assessment = ArchivedAssessment {
        encoded: encoded.to_vec(),
        address,
        surface,
        pair,
        selection,
        baseline_content,
        selected_content,
        witness,
        input,
        meanings,
        reports,
        substrate,
        difference,
        mutation,
    };
    super::assessment_joins::joined(&assessment)?;
    Ok(assessment)
}

fn value(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    convention: &ArchivedValueConvention,
    role: ValueRole,
    limits: ParityArchiveLimits,
) -> Result<ArchivedValue, AssessmentArchiveRefusal> {
    Ok(ArchivedValue {
        convention: convention.clone(),
        bytes: members::value(reader, role, limits)?,
    })
}

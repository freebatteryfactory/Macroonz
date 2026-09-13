//! Complete assessment retention with independent existing record and value bounds.

use super::encode::{encode_value, write_convention, write_substrate};
use super::size::{convention_size, substrate_size};
use super::{
    ASSESSMENT_ARCHIVE_TAG, ArchivedAssessment, AssessmentArchiveLimits, AssessmentArchiveRefusal,
    ValueEncoder, ValueRole, read_assessment,
};
use crate::descriptor::archive::{retain_binding, write_revision};
use crate::identity::{ContentAddress, encode_bytes};
use crate::muterprater::MutationAssessment;
use crate::muterprater::discovery_archive::{retain_surface, selection_size, write_selection};
use crate::muterprater::specimen_archive::{source_size, write_artifact};
use crate::muterprater::verdict_archive::retain_mutation;
use crate::properties::Agreement;
use crate::report::archive::{ArchiveRefusal, bounded, name_size, retain_trial, sum};

/// Retain a qualified assessment and every meaning actually compared or judged.
///
/// # Errors
///
/// Refuses known bounds before caller encoders, then encoding, complete-envelope or historical-join failures.
/// Encodings retain current borrowed values and do not authenticate host callbacks or snapshot prior interior state.
pub fn retain_assessment<Input, Meaning>(
    assessment: &MutationAssessment<'_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: AssessmentArchiveLimits,
) -> Result<ArchivedAssessment, AssessmentArchiveRefusal> {
    let mut body = metadata(assessment, limits)?;
    let projection = limits.projection();
    let values = projection.parity();
    let known = sum(&[
        32,
        body.len(),
        48,
        convention_size(input, values)?,
        convention_size(meaning, values)?,
    ])?;
    admit(known, limits)?;
    let qualified = assessment.reading().qualification();
    let observed = qualified.compiled().observation();
    write_convention(input, &mut body);
    let input_bytes = encode_value(input, observed.input(), ValueRole::Input, values)?;
    bounded(input_bytes.len(), projection.bytes())?;
    encode_bytes(&input_bytes, &mut body);
    admit(sum(&[known, input_bytes.len()])?, limits)?;
    write_convention(meaning, &mut body);
    let mut total = sum(&[known, input_bytes.len()])?;
    for (value, role) in [
        (observed.production(), ValueRole::Production),
        (qualified.baseline(), ValueRole::BaselineEvaluation),
        (qualified.compiled_baseline(), ValueRole::CompiledBaseline),
        (qualified.compiled_selected(), ValueRole::CompiledSelected),
        (qualified.selected(), ValueRole::SelectedEvaluation),
    ] {
        let encoded = encode_value(meaning, value, role, values)?;
        bounded(encoded.len(), projection.bytes())?;
        total = admit(sum(&[total, encoded.len()])?, limits)?;
        encode_bytes(&encoded, &mut body);
    }
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(ASSESSMENT_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_assessment(&encoded, limits)
}

fn metadata<Input, Meaning>(
    assessment: &MutationAssessment<'_, Input, Meaning>,
    limits: AssessmentArchiveLimits,
) -> Result<Vec<u8>, AssessmentArchiveRefusal> {
    let reading = assessment.reading();
    let qualified = reading.qualification();
    let compiled = qualified.compiled();
    let observed = compiled.observation();
    let projection = limits.projection();
    let bytes = projection.bytes();
    bounded(41, bytes)?;
    name_size(observed.pair().standing().family().name(), bytes)?;
    selection_size(observed.selection(), bytes)?;
    source_size(compiled.baseline_content().bytes().len(), projection)?;
    source_size(compiled.selected_content().bytes().len(), projection)?;
    substrate_size(qualified.substrate(), projection.parity())?;
    let surface = retain_surface(observed.surface(), limits.surface())?;
    let witness = retain_binding(reading.witness().binding(), projection.parity().binding())?;
    let mutation = retain_mutation(assessment.mutation(), projection.mutation())?;
    let mut body = Vec::new();
    for word in [1u32, 1, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    for nested in [surface.encoded(), witness.encoded(), mutation.encoded()] {
        bounded(nested.len(), bytes)?;
        encode_bytes(nested, &mut body);
    }
    let pair = observed.pair().standing();
    pair.family().name().encode_into(&mut body);
    write_revision(pair.production_revision(), &mut body);
    write_revision(pair.evaluation_revision(), &mut body);
    encode_bytes(pair.surface().address().as_bytes(), &mut body);
    write_selection(observed.selection(), &mut body);
    write_artifact(compiled.baseline_content(), &mut body);
    write_artifact(compiled.selected_content(), &mut body);
    write_substrate(qualified.substrate(), &mut body);
    body.push(u8::from(qualified.difference() == Agreement::Differs));
    for report in [
        reading.production_report(),
        reading.baseline_report(),
        reading.compiled_baseline_report(),
        reading.compiled_selected_report(),
        reading.selected_report(),
    ] {
        let retained = retain_trial(report, projection.trial())?;
        bounded(retained.encoded().len(), bytes)?;
        encode_bytes(retained.encoded(), &mut body);
    }
    Ok(body)
}

fn admit(total: usize, limits: AssessmentArchiveLimits) -> Result<usize, AssessmentArchiveRefusal> {
    if total > limits.projection().bytes().envelope() {
        Err(ArchiveRefusal::EnvelopeTooLarge.into())
    } else {
        Ok(total)
    }
}

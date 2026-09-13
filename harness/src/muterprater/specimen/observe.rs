//! Execution of one exact specimen request before witness judgment.

use super::demonstrate::materialize_specimens;
use super::types::{
    ArtifactContent, CompiledSpecimenContext, CompiledSpecimenHost, CompiledSpecimenRequest,
    CompiledSpecimenRole, SpecimenObservationRefusal,
};
use super::types::{
    CompiledMutationObservation, CompiledProjectionRefusal, SpecimenMaterializerBinding,
};
use crate::muterprater::MutationObservation;

/// Compile and execute both source roles for one existing mutation observation.
///
/// Each reached host result is retained without invoking a witness or requiring a rejection.
///
/// # Errors
///
/// Refuses a foreign materializer, unresolved selection, materialization failure or unchanged source bytes before host execution.
pub fn observe_mutation<'scope, Input, Meaning>(
    observation: &'scope MutationObservation<'scope, Input, Meaning>,
    materializer: &SpecimenMaterializerBinding,
    host: CompiledSpecimenHost<Input, Meaning>,
) -> Result<CompiledMutationObservation<'scope, Input, Meaning>, CompiledProjectionRefusal> {
    if let Some(cause) = observation.pair().standing().mismatch(materializer.pair()) {
        return Err(CompiledProjectionRefusal::MaterializerForAnotherPair(cause));
    }
    let selection = observation.selection();
    let (point, alternative) = observation
        .surface()
        .selected_alternative(selection)
        .map_err(CompiledProjectionRefusal::Selection)?;
    let (baseline_content, selected_content) =
        materialize_specimens(materializer, selection, point, alternative)?;
    let baseline = observe_specimen(
        &baseline_content,
        CompiledSpecimenRole::Baseline,
        point.original_operation(),
        observation.input(),
        observation.context(),
        host,
    );
    let selected = observe_specimen(
        &selected_content,
        CompiledSpecimenRole::Selected(selection),
        alternative.operation(),
        observation.input(),
        observation.context(),
        host,
    );
    Ok(CompiledMutationObservation::observed(
        observation,
        (baseline_content, selected_content),
        (baseline, selected),
    ))
}

pub(super) fn observe_specimen<Input, Meaning>(
    content: &ArtifactContent,
    role: CompiledSpecimenRole,
    operation: &[u8],
    input: &Input,
    standing: &CompiledSpecimenContext,
    host: CompiledSpecimenHost<Input, Meaning>,
) -> Result<Meaning, SpecimenObservationRefusal> {
    let request = CompiledSpecimenRequest::requested(content, role, operation, input, standing);
    let observation = host(request).map_err(SpecimenObservationRefusal::Host)?;
    if let Some(cause) = observation.mismatch(content.identity(), role, standing) {
        return Err(SpecimenObservationRefusal::Foreign(cause));
    }
    Ok(observation.into_meaning())
}

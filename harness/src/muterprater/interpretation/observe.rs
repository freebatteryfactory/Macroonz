//! Scoped evaluation observations before witness judgment or execution qualification.

use crate::muterprater::{
    ActiveSelection, CompiledSpecimenContext, EvaluationDirective, EvaluationPair,
    EvaluationSurface, MutationObservation, MutationObservationRefusal,
};
use crate::runner::Invocation;

/// Observe production, unchanged evaluation and one selected evaluation on the supplied input.
///
/// Reached call refusals remain in the reading and grant no qualification.
///
/// # Errors
///
/// Refuses a foreign pair or selection before calling production or evaluation.
pub fn observe_mutation<'scope, Input, Meaning>(
    surface: &'scope EvaluationSurface,
    pair: &'scope EvaluationPair<Input, Meaning>,
    input: &'scope Input,
    selection: ActiveSelection,
    invocation: &Invocation,
) -> Result<MutationObservation<'scope, Input, Meaning>, MutationObservationRefusal> {
    let (point, alternative) = selection_for(surface, pair, selection)?;
    let context = CompiledSpecimenContext::recorded(pair.standing(), invocation);
    let production = pair.production().evaluate(input);
    let baseline = pair
        .evaluation()
        .evaluate(input, EvaluationDirective::no_mutation());
    let selected = pair.evaluation().evaluate(
        input,
        EvaluationDirective::active(selection, point, alternative),
    );
    Ok(MutationObservation::observed(
        surface,
        pair,
        input,
        (selection, point, alternative),
        context,
        production,
        (baseline, selected),
    ))
}

pub(in crate::muterprater) fn selection_for<'surface, Input, Meaning>(
    surface: &'surface EvaluationSurface,
    pair: &EvaluationPair<Input, Meaning>,
    selection: ActiveSelection,
) -> Result<
    (
        &'surface crate::muterprater::MutationPoint,
        &'surface crate::muterprater::AdmittedAlternative,
    ),
    MutationObservationRefusal,
> {
    if pair.standing().surface() != surface.identity() {
        return Err(MutationObservationRefusal::Surface {
            expected: surface.identity(),
            found: pair.standing().surface(),
        });
    }
    surface
        .selected_alternative(selection)
        .map_err(MutationObservationRefusal::Selection)
}

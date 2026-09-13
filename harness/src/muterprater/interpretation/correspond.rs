//! Qualification of observed execution correspondence independently of any witness.

use crate::muterprater::{
    CompiledMutationObservation, MUTERPRATER_NAMESPACE, MutationQualificationRefusal,
    NO_MUTATION_PAIRING, QualifiedMutation,
};
use crate::properties::{Equivalence, SharedSubstrate, agreement};
use crate::report::{FindingCause, TrialConclusion, TrialFinding};
use core::num::NonZeroU32;

/// Qualify the three execution-road comparisons and positive selected activation.
///
/// The borrowed observation remains available when qualification refuses.
///
/// # Errors
///
/// Refuses missing meanings, invalid firing counts or disagreement under the pair's declared relation.
pub fn qualify_execution<'scope, Input, Meaning>(
    compiled: &'scope CompiledMutationObservation<'scope, Input, Meaning>,
    substrate: SharedSubstrate,
) -> Result<QualifiedMutation<'scope, Input, Meaning>, MutationQualificationRefusal> {
    let observed = compiled.observation();
    let baseline = observed
        .baseline()
        .as_ref()
        .map_err(|cause| MutationQualificationRefusal::BaselineEvaluation(*cause))?;
    let selected = observed
        .selected()
        .as_ref()
        .map_err(|cause| MutationQualificationRefusal::SelectedEvaluation(*cause))?;
    if baseline.firings() != 0 {
        return Err(MutationQualificationRefusal::BaselineActivated {
            firings: baseline.firings(),
        });
    }
    let firings =
        NonZeroU32::new(selected.firings()).ok_or(MutationQualificationRefusal::NotActivated)?;
    let compiled_baseline = compiled
        .baseline()
        .as_ref()
        .map_err(|cause| MutationQualificationRefusal::CompiledBaseline(cause.clone()))?;
    let compiled_selected = compiled
        .selected()
        .as_ref()
        .map_err(|cause| MutationQualificationRefusal::CompiledSelected(cause.clone()))?;
    let same = observed.pair().equivalence();
    agrees(
        same,
        observed.production(),
        baseline.meaning(),
        NO_MUTATION_PAIRING,
    )
    .map_err(MutationQualificationRefusal::BaselineDisagreement)?;
    agrees(
        same,
        observed.production(),
        compiled_baseline,
        "compiled-baseline-parity",
    )
    .map_err(MutationQualificationRefusal::CompiledBaselineDisagreement)?;
    agrees(
        same,
        selected.meaning(),
        compiled_selected,
        "compiled-selected-parity",
    )
    .map_err(MutationQualificationRefusal::SelectedDisagreement)?;
    let difference = same(observed.production(), selected.meaning());
    Ok(QualifiedMutation::qualified(
        compiled,
        [
            baseline.meaning(),
            selected.meaning(),
            compiled_baseline,
            compiled_selected,
        ],
        firings,
        substrate,
        difference,
    ))
}

fn agrees<Meaning>(
    same: Equivalence<Meaning>,
    left: &Meaning,
    right: &Meaning,
    pairing: &'static str,
) -> Result<(), TrialFinding> {
    match agreement(
        same,
        left,
        right,
        FindingCause::named(MUTERPRATER_NAMESPACE, pairing),
    ) {
        TrialConclusion::Passed => Ok(()),
        TrialConclusion::Refused(finding) => Err(finding),
    }
}

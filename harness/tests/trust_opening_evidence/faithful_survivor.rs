//! Faithful compiled mutation observations assessed by independent weak and strong witnesses.

use super::support::{COMPILED_SPECIMEN_HOST, SPECIMEN_HOST_CALLS, SPECIMEN_MATERIALIZER};
use super::support::{
    CompiledRosterMeaning, ORIGINAL_OPERATION, REVISION_TAG, SELECTED_OPERATION, active_selection,
    check, check_ref, compiled_suite_pressure, family, invocation, lock_specimen_tests,
    opened_trust, qualification_of, qualified_no_mutation, same, standard_projection, surface_with,
    trial_binding_with, unused_trial_call,
};
use macroonz_harness::descriptor::{Origin, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::interpret::{availability, execute_active};
use macroonz_harness::muterprater::interpret::{
    observe_mutation, observe_witness, qualify_execution, qualify_witness,
};
use macroonz_harness::muterprater::specimen;
use macroonz_harness::muterprater::{
    CompiledProjectionRefusal, EvaluationBinding, EvaluationCallRefusal, EvaluationDirective,
    EvaluationObservation, EvaluationPair, MeaningCheck, MutationVerdict, MutationWitness,
    ProductionBinding,
};
use macroonz_harness::muterprater::{
    EquivalenceAxis, MutationQualificationRefusal, MutationWitnessQualificationRefusal,
    SpecimenMaterializerBinding,
};
use macroonz_harness::properties::{Agreement, SharedSubstrate, SubstrateRef, SubstrateRoster};
use macroonz_harness::report::TrialConclusion;
use std::sync::atomic::{AtomicU32, Ordering};

static EVALUATION_CALLS: AtomicU32 = AtomicU32::new(0);
static WITNESS_CALLS: AtomicU32 = AtomicU32::new(0);

fn counted_evaluation(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    EVALUATION_CALLS.fetch_add(1, Ordering::SeqCst);
    evaluation(input, directive)
}

fn counted_well_formed(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    WITNESS_CALLS.fetch_add(1, Ordering::SeqCst);
    well_formed(meaning)
}

pub(super) fn substrate() -> Result<SharedSubstrate, String> {
    let declaration = SubstrateRef::named("faithful-gate", "declared-predicate")
        .map_err(|cause| format!("{cause:?}"))?;
    Ok(SharedSubstrate::Standing(
        SubstrateRoster::declared(&[declaration]).map_err(|cause| format!("{cause:?}"))?,
    ))
}

/// One real changed execution can survive an incomplete witness and be caught by an independent stronger witness without rerunning its subject.
#[test]
fn faithful_survivor_and_independent_catch_share_observations() -> Result<(), String> {
    let _guard = lock_specimen_tests().map_err(|cause| format!("{cause:?}"))?;
    let family = family("observed-faithful-gate").map_err(|cause| format!("{cause:?}"))?;
    let surface =
        surface_with(family, vec![SELECTED_OPERATION]).map_err(|cause| format!("{cause:?}"))?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        b"observed-faithful-gate",
    ));
    let pair = EvaluationPair::paired(
        ProductionBinding::declared(family, revision, production),
        EvaluationBinding::declared(&surface, revision, counted_evaluation),
        same,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let input = [1, 0, 0];
    let selection = active_selection(&surface).map_err(|cause| format!("{cause:?}"))?;
    let invocation = invocation().map_err(|cause| format!("{cause:?}"))?;
    EVALUATION_CALLS.store(0, Ordering::SeqCst);
    WITNESS_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    let observed = observe_mutation(&surface, &pair, &input, selection, &invocation)
        .map_err(|cause| format!("{cause:?}"))?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let compiled = specimen::observe_mutation(&observed, &materializer, COMPILED_SPECIMEN_HOST)
        .map_err(|cause| format!("{cause:?}"))?;
    let qualified =
        qualify_execution(&compiled, substrate()?).map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(qualified.difference(), Agreement::Differs);
    assert_eq!(WITNESS_CALLS.load(Ordering::SeqCst), 0);
    let weak = observe_witness(
        &qualified,
        witness(b"well-formed-only", counted_well_formed)?,
        &invocation,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let weak = qualify_witness(weak).map_err(|rejected| format!("{:?}", rejected.cause()))?;
    assert_eq!(weak.mutation().verdict(), MutationVerdict::Survived);
    assert_eq!(weak.mutation().equivalence(), EquivalenceAxis::Refuted);
    assert_eq!(WITNESS_CALLS.load(Ordering::SeqCst), 5);
    let strong = observe_witness(
        &qualified,
        witness(b"requires-the-result", check)?,
        &invocation,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let strong = qualify_witness(strong).map_err(|rejected| format!("{:?}", rejected.cause()))?;
    assert_eq!(strong.mutation().verdict(), MutationVerdict::Killed);
    super::assessment_archive::controls(&weak, &strong)?;
    super::faithful_proposal::controls(&weak, &strong)?;
    assert_eq!(weak.mutation().target(), strong.mutation().target());
    assert_ne!(
        weak.reading().selected_report().standing().key(),
        strong.reading().selected_report().standing().key()
    );
    assert_eq!(EVALUATION_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    Ok(())
}

fn unchanged_active(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    evaluation(input, directive)
        .map(|observed| EvaluationObservation::observed(production(input), observed.firings()))
}

fn coarse_relation(_left: &CompiledRosterMeaning, _right: &CompiledRosterMeaning) -> Agreement {
    Agreement::Agrees
}

/// A callback returning unchanged behavior cannot earn faithful selected execution; even a permissive relation cannot erase an actually disagreeing witness.
#[test]
fn unfaithful_activation_and_coarse_comparison_do_not_manufacture_agreement() -> Result<(), String>
{
    let _guard = lock_specimen_tests().map_err(|cause| format!("{cause:?}"))?;
    let family = family("unfaithful-observed-gate").map_err(|cause| format!("{cause:?}"))?;
    let surface =
        surface_with(family, vec![SELECTED_OPERATION]).map_err(|cause| format!("{cause:?}"))?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        b"unfaithful-observed-gate",
    ));
    let input = [1, 0, 0];
    let selection = active_selection(&surface).map_err(|cause| format!("{cause:?}"))?;
    let invocation = invocation().map_err(|cause| format!("{cause:?}"))?;
    let pair = EvaluationPair::paired(
        ProductionBinding::declared(family, revision, production),
        EvaluationBinding::declared(&surface, revision, unchanged_active),
        same,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let observed = observe_mutation(&surface, &pair, &input, selection, &invocation)
        .map_err(|cause| format!("{cause:?}"))?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let compiled = specimen::observe_mutation(&observed, &materializer, COMPILED_SPECIMEN_HOST)
        .map_err(|cause| format!("{cause:?}"))?;
    assert!(matches!(
        qualify_execution(&compiled, substrate()?),
        Err(MutationQualificationRefusal::SelectedDisagreement(_))
    ));
    assert_eq!(
        compiled
            .selected()
            .as_ref()
            .map_err(|cause| format!("{cause:?}"))?,
        &CompiledRosterMeaning::Unstated
    );
    let coarse_pair = EvaluationPair::paired(
        ProductionBinding::declared(family, revision, production),
        EvaluationBinding::declared(&surface, revision, unchanged_active),
        coarse_relation,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let coarse = observe_mutation(&surface, &coarse_pair, &input, selection, &invocation)
        .map_err(|cause| format!("{cause:?}"))?;
    let coarse_materializer =
        SpecimenMaterializerBinding::bound(&coarse_pair, SPECIMEN_MATERIALIZER);
    let coarse_compiled =
        specimen::observe_mutation(&coarse, &coarse_materializer, COMPILED_SPECIMEN_HOST)
            .map_err(|cause| format!("{cause:?}"))?;
    let qualified =
        qualify_execution(&coarse_compiled, substrate()?).map_err(|cause| format!("{cause:?}"))?;
    let reading = observe_witness(
        &qualified,
        witness(b"coarse-strong-check", check)?,
        &invocation,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let Err(rejected) = qualify_witness(reading) else {
        return Err("disagreeing witness was qualified".to_owned());
    };
    assert_eq!(
        rejected.cause(),
        MutationWitnessQualificationRefusal::SelectedReportsDisagreed
    );
    assert_eq!(
        rejected.reading().selected_report().attempt(),
        &macroonz_harness::report::RunAttempt::Executed(TrialConclusion::Passed)
    );
    Ok(())
}

pub(super) fn production(input: &[u32; 3]) -> CompiledRosterMeaning {
    let [value, _, _] = *input;
    if value == 0 {
        CompiledRosterMeaning::Unstated
    } else {
        CompiledRosterMeaning::Stated(1)
    }
}

pub(super) fn evaluation(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    let Some(resolved) = directive.resolved() else {
        return Ok(EvaluationObservation::observed(production(input), 0));
    };
    if resolved.point().original_operation() != ORIGINAL_OPERATION
        || resolved.alternative().operation() != SELECTED_OPERATION
    {
        return Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        ));
    }
    let [value, _, _] = *input;
    let observed = if value == 0 {
        CompiledRosterMeaning::Stated(1)
    } else {
        CompiledRosterMeaning::Unstated
    };
    Ok(EvaluationObservation::observed(observed, 1))
}

fn well_formed(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    match meaning {
        CompiledRosterMeaning::Stated(1) | CompiledRosterMeaning::Unstated => {
            TrialConclusion::Passed
        }
        CompiledRosterMeaning::Stated(_)
        | CompiledRosterMeaning::SetupRefused
        | CompiledRosterMeaning::ReadingRefused(_) => check(meaning),
    }
}

pub(super) fn witness(
    revision: &[u8],
    check: MeaningCheck<CompiledRosterMeaning>,
) -> Result<MutationWitness<CompiledRosterMeaning>, String> {
    let binding = trial_binding_with(
        "comparison-behaviour",
        Origin::HandWritten,
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, revision)),
        unused_trial_call,
    )
    .map_err(|refusal| format!("{refusal:?}"))?;
    MutationWitness::bound(
        binding,
        check_ref().map_err(|refusal| format!("{refusal:?}"))?,
        check,
    )
    .map_err(|refusal| format!("{refusal:?}"))
}

/// Real compiled pressure cannot open on an incomplete witness; the same faithful pair opens and kills under its stronger witness.
#[test]
fn faithful_execution_cannot_turn_the_exact_rejecting_witness_into_a_survivor() -> Result<(), String>
{
    let _guard = lock_specimen_tests().map_err(|refusal| format!("{refusal:?}"))?;
    let family = family("faithful-gate").map_err(|refusal| format!("{refusal:?}"))?;
    let surface =
        surface_with(family, vec![SELECTED_OPERATION]).map_err(|refusal| format!("{refusal:?}"))?;
    let revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"faithful-gate"));
    let pair = EvaluationPair::paired(
        ProductionBinding::declared(family, revision, production),
        EvaluationBinding::declared(&surface, revision, evaluation),
        same,
    )
    .map_err(|refusal| format!("{refusal:?}"))?;
    let input = [1, 0, 0];
    let selection = active_selection(&surface).map_err(|refusal| format!("{refusal:?}"))?;
    let weak = qualified_no_mutation(&pair, witness(b"well-formed-only", well_formed)?, &input)
        .map_err(|refusal| format!("{refusal:?}"))?;
    let weak = qualification_of(&weak).map_err(|refusal| format!("{refusal:?}"))?;
    let refused = standard_projection(&surface, weak, &pair, selection);
    assert!(matches!(
        refused,
        Err(super::support::MutationRoadFailure::Projection(
            CompiledProjectionRefusal::ProjectionDidNotReject
        )),
    ));

    let strong = qualified_no_mutation(&pair, witness(b"requires-the-result", check)?, &input)
        .map_err(|refusal| format!("{refusal:?}"))?;
    let strong = qualification_of(&strong).map_err(|refusal| format!("{refusal:?}"))?;
    let projection = standard_projection(&surface, strong, &pair, selection)
        .map_err(|refusal| format!("{refusal:?}"))?;
    let suite = compiled_suite_pressure().map_err(|refusal| format!("{refusal:?}"))?;
    let trust = opened_trust(availability(
        Some(&surface),
        Some(&suite),
        Some(&projection),
    ))
    .map_err(|refusal| format!("{refusal:?}"))?;
    let evidence = execute_active(
        &trust,
        &invocation().map_err(|refusal| format!("{refusal:?}"))?,
    )
    .map_err(|refusal| format!("{refusal:?}"))?;
    assert_eq!(evidence.meaning(), &CompiledRosterMeaning::Unstated);
    assert_eq!(evidence.mutation().verdict(), MutationVerdict::Killed);
    assert!(evidence.mutation().activation().evidence().is_some());
    Ok(())
}

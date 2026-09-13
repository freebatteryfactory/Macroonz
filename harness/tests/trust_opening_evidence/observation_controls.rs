//! Synthetic callback controls for observation refusals; real compilation is exercised by `faithful_survivor`.

use super::faithful_survivor::{evaluation, production, substrate, witness};
use super::support::{
    CompiledRosterMeaning, REVISION_TAG, SELECTED_OPERATION, SPECIMEN_MATERIALIZER,
    active_selection, check, check_ref, family, foreign_invocation, invocation, policy,
    surface_with, trial_binding_with, unused_trial_call,
};
use macroonz_harness::descriptor::{Origin, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::interpret::{
    observe_mutation, observe_witness, qualify_execution, qualify_witness,
};
use macroonz_harness::muterprater::{
    CompiledSpecimenHost, CompiledSpecimenHostRefusal, CompiledSpecimenObservation,
    CompiledSpecimenRole, EvaluationBinding, EvaluationCall, EvaluationCallRefusal,
    EvaluationObservation, EvaluationPair, EvaluationSurface, MutationObservationRefusal,
    MutationQualificationRefusal, MutationWitness, MutationWitnessObservationRefusal,
    MutationWitnessQualificationRefusal, ProductionBinding, RewriteAdmission,
    SpecimenMaterializerBinding, SpecimenObservationRefusal,
};
use macroonz_harness::muterprater::{rewrite, specimen};
use macroonz_harness::report::{ForeignText, TrialConclusion};

const HOST: CompiledSpecimenHost<[u32; 3], CompiledRosterMeaning> = |request| {
    let meaning = match request.role() {
        CompiledSpecimenRole::Baseline => CompiledRosterMeaning::Stated(1),
        CompiledSpecimenRole::Selected(_) => CompiledRosterMeaning::Unstated,
    };
    Ok(CompiledSpecimenObservation::executed(&request, meaning))
};
const BAD_BASELINE: CompiledSpecimenHost<[u32; 3], CompiledRosterMeaning> = |request| {
    Ok(CompiledSpecimenObservation::executed(
        &request,
        CompiledRosterMeaning::Unstated,
    ))
};
const UNVIABLE: CompiledSpecimenHost<[u32; 3], CompiledRosterMeaning> =
    |request| match request.role() {
        CompiledSpecimenRole::Baseline => HOST(request),
        CompiledSpecimenRole::Selected(_) => Err(CompiledSpecimenHostRefusal::Compilation(
            ForeignText::admitted(b"test compiler refusal"),
        )),
    };
const ZERO: EvaluationCall<[u32; 3], CompiledRosterMeaning> = |input, directive| {
    evaluation(input, directive).map(|value| EvaluationObservation::observed(*value.meaning(), 0))
};
const BASELINE_ACTIVATED: EvaluationCall<[u32; 3], CompiledRosterMeaning> = |input, directive| {
    evaluation(input, directive).map(|value| EvaluationObservation::observed(*value.meaning(), 4))
};
const NO_BASELINE: EvaluationCall<[u32; 3], CompiledRosterMeaning> = |input, directive| {
    if directive.resolved().is_none() {
        Err(EvaluationCallRefusal::NoMutationNotImplemented)
    } else {
        evaluation(input, directive)
    }
};
const NO_SELECTED: EvaluationCall<[u32; 3], CompiledRosterMeaning> = |input, directive| {
    if let Some(resolved) = directive.resolved() {
        Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        ))
    } else {
        evaluation(input, directive)
    }
};

fn fixture(
    call: EvaluationCall<[u32; 3], CompiledRosterMeaning>,
) -> Result<
    (
        EvaluationSurface,
        EvaluationPair<[u32; 3], CompiledRosterMeaning>,
    ),
    String,
> {
    let family = family("observation-refusals").map_err(|cause| format!("{cause:?}"))?;
    let surface =
        surface_with(family, vec![SELECTED_OPERATION]).map_err(|cause| format!("{cause:?}"))?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        b"observation-refusals",
    ));
    let pair = EvaluationPair::paired(
        ProductionBinding::declared(family, revision, production),
        EvaluationBinding::declared(&surface, revision, call),
        super::support::same,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    Ok((surface, pair))
}

#[test]
fn missing_execution_and_incorrect_activation_preserve_their_causes() -> Result<(), String> {
    let (scope, _) = fixture(evaluation)?;
    let declared = active_selection(&scope).map_err(|cause| format!("{cause:?}"))?;
    for (call, expected) in [
        (ZERO, MutationQualificationRefusal::NotActivated),
        (
            BASELINE_ACTIVATED,
            MutationQualificationRefusal::BaselineActivated { firings: 4 },
        ),
        (
            NO_BASELINE,
            MutationQualificationRefusal::BaselineEvaluation(
                EvaluationCallRefusal::NoMutationNotImplemented,
            ),
        ),
        (
            NO_SELECTED,
            MutationQualificationRefusal::SelectedEvaluation(
                EvaluationCallRefusal::ActiveSelectionNotImplemented(declared),
            ),
        ),
    ] {
        let (surface, pair) = fixture(call)?;
        let selection = active_selection(&surface).map_err(|cause| format!("{cause:?}"))?;
        let observed = observe_mutation(
            &surface,
            &pair,
            &[1, 0, 0],
            selection,
            &foreign_invocation(),
        )
        .map_err(|cause| format!("{cause:?}"))?;
        let compiled = specimen::observe_mutation(
            &observed,
            &SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER),
            HOST,
        )
        .map_err(|cause| format!("{cause:?}"))?;
        assert_eq!(
            qualify_execution(&compiled, substrate()?).err(),
            Some(expected)
        );
        assert!(compiled.baseline().is_ok());
        assert!(compiled.selected().is_ok());
    }
    Ok(())
}

#[test]
fn failed_or_disagreeing_compiled_roads_cannot_qualify() -> Result<(), String> {
    let (surface, pair) = fixture(evaluation)?;
    let selection = active_selection(&surface).map_err(|cause| format!("{cause:?}"))?;
    let observed = observe_mutation(
        &surface,
        &pair,
        &[1, 0, 0],
        selection,
        &foreign_invocation(),
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let baseline = specimen::observe_mutation(&observed, &materializer, BAD_BASELINE)
        .map_err(|cause| format!("{cause:?}"))?;
    assert!(matches!(
        qualify_execution(&baseline, substrate()?),
        Err(MutationQualificationRefusal::CompiledBaselineDisagreement(
            _
        ))
    ));
    let unviable = specimen::observe_mutation(&observed, &materializer, UNVIABLE)
        .map_err(|cause| format!("{cause:?}"))?;
    assert!(unviable.baseline().is_ok());
    assert!(matches!(
        qualify_execution(&unviable, substrate()?),
        Err(MutationQualificationRefusal::CompiledSelected(
            SpecimenObservationRefusal::Host(CompiledSpecimenHostRefusal::Compilation(_))
        ))
    ));
    Ok(())
}

#[test]
fn foreign_scope_and_witness_refuse_before_later_obligations() -> Result<(), String> {
    let (surface, pair) = fixture(evaluation)?;
    let selection = active_selection(&surface).map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(
        rewrite::admission_for(&surface, &pair, selection).map_err(|cause| format!("{cause:?}"))?,
        RewriteAdmission::Admitted
    );
    let empty = lower_discoveries(
        &policy(surface.family()).map_err(|cause| format!("{cause:?}"))?,
        Vec::new(),
    )
    .map_err(|cause| format!("{cause:?}"))?
    .into_parts()
    .1;
    assert!(matches!(
        observe_mutation(&empty, &pair, &[1, 0, 0], selection, &foreign_invocation()),
        Err(MutationObservationRefusal::Surface { .. })
    ));
    assert!(rewrite::admission_for(&empty, &pair, selection).is_err());
    let invocation = invocation().map_err(|cause| format!("{cause:?}"))?;
    let observed = observe_mutation(&surface, &pair, &[1, 0, 0], selection, &invocation)
        .map_err(|cause| format!("{cause:?}"))?;
    let compiled = specimen::observe_mutation(
        &observed,
        &SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER),
        HOST,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let qualified =
        qualify_execution(&compiled, substrate()?).map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(
        observe_witness(
            &qualified,
            witness(b"foreign-context", check)?,
            &foreign_invocation()
        )
        .err(),
        Some(MutationWitnessObservationRefusal::Context)
    );
    let binding = trial_binding_with(
        "another-claim",
        Origin::HandWritten,
        pair.standing().production_revision(),
        unused_trial_call,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let foreign = MutationWitness::bound(
        binding,
        check_ref().map_err(|cause| format!("{cause:?}"))?,
        check,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    assert!(matches!(
        observe_witness(&qualified, foreign, &invocation),
        Err(MutationWitnessObservationRefusal::Claim { .. })
    ));
    let rejects = witness(b"baseline-rejects", |_| {
        check(&CompiledRosterMeaning::Unstated)
    })?;
    let reading =
        observe_witness(&qualified, rejects, &invocation).map_err(|cause| format!("{cause:?}"))?;
    let Err(rejected) = qualify_witness(reading) else {
        return Err("failing baseline qualified".to_owned());
    };
    assert_eq!(
        rejected.cause(),
        MutationWitnessQualificationRefusal::ProductionDidNotPass
    );
    assert!(matches!(
        rejected.reading().production_report().attempt(),
        macroonz_harness::report::RunAttempt::Executed(TrialConclusion::Refused(_))
    ));
    Ok(())
}

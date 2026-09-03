//! Outside observations owned by the interpretation semantic home.

use super::support::{
    CompiledRosterMeaning, EVALUATION, EvaluationFn, MEANING_DISAGREEMENT, MutationRoadFailure,
    OWNER, REVISION_TAG, SELECTED_OPERATION, active_selection, check, check_ref, claim,
    compiled_suite_pressure, family, invocation, lock_specimen_tests, opened_trust, pair, policy,
    production, qualification_of, qualified_no_mutation, same, standard_projection, surface_with,
    trial_binding, witness,
};
use macroonz_harness::descriptor::{CheckRef, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::interpret::{
    availability, execute_active, observe_no_mutation, plan_pass, qualify_no_mutation,
};
use macroonz_harness::muterprater::{
    EvaluationBinding, EvaluationCallRefusal, EvaluationDirective, EvaluationObservation,
    EvaluationPair, EvaluationPairRefusal, InterpretedExecutionRefusal, InterpreterAvailability,
    MUTERPRATER_NAMESPACE, MissingTrustEvidence, MutationIdentity, MutationOutcome, MutationReport,
    MutationVerdict, MutationWitness, MutationWitnessRefusal, NO_MUTATION_PAIRING,
    NoMutationObservationRefusal, PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE,
    ParityQualificationRefusal, PlanRefusal, PlannedDamage, PressureBudget, PressureLane,
    ProductionBinding, ScopeShape, ScopedInvocation,
};
use macroonz_harness::properties::{SharedSubstrate, SubstrateRef, agreement};
use macroonz_harness::report::{RunAttempt, TrialConclusion};
use macroonz_harness::runner::Selection;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU32, Ordering};

static NO_MUTATION_CALL_ORDER: AtomicU32 = AtomicU32::new(0);

fn production_ordered(input: &[u32; 3]) -> CompiledRosterMeaning {
    if NO_MUTATION_CALL_ORDER
        .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        NO_MUTATION_CALL_ORDER.store(u32::MAX, Ordering::SeqCst);
    }
    production(input)
}

/// This capture-free hostile fixture returns a semantic disagreement rather than a call refusal.
const PARITY_BROKEN: EvaluationFn = |_input, directive| {
    Ok(EvaluationObservation::observed(
        CompiledRosterMeaning::Unstated,
        u32::from(directive.resolved().is_some()),
    ))
};

fn no_mutation_branch_omitted(
    _input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    match directive.resolved() {
        None => {
            if NO_MUTATION_CALL_ORDER
                .compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst)
                .is_err()
            {
                NO_MUTATION_CALL_ORDER.store(u32::MAX, Ordering::SeqCst);
            }
            Err(EvaluationCallRefusal::NoMutationNotImplemented)
        }
        Some(resolved) => Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        )),
    }
}

fn active_branch_omitted(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    match directive.resolved() {
        None => Ok(EvaluationObservation::observed(production(input), 0)),
        Some(resolved) => Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        )),
    }
}

/// This capture-free hostile fixture reports zero firing as a successful raw observation.
const ACTIVATION_MISSING: EvaluationFn = |input, directive| {
    Ok(if directive.resolved().is_some() {
        EvaluationObservation::observed(CompiledRosterMeaning::Unstated, 0)
    } else {
        EvaluationObservation::observed(production(input), 0)
    })
};

/// This capture-free hostile fixture reports an invalid positive no-mutation count.
const NO_MUTATION_ACTIVATES: EvaluationFn =
    |input, _directive| Ok(EvaluationObservation::observed(production(input), 1));

/// This capture-free fixture's active observation remains semantically lawful.
const ACTIVATION_SURVIVES: EvaluationFn = |input, directive| {
    Ok(EvaluationObservation::observed(
        production(input),
        u32::from(directive.resolved().is_some()),
    ))
};

fn check_passes(_meaning: &CompiledRosterMeaning) -> TrialConclusion {
    TrialConclusion::Passed
}

fn check_evaluation_meaning(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &CompiledRosterMeaning::Unstated,
        MEANING_DISAGREEMENT,
    )
}

fn check_refuses(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &CompiledRosterMeaning::SetupRefused,
        MEANING_DISAGREEMENT,
    )
}

fn scope(mutants: u32) -> Result<ScopedInvocation, MutationRoadFailure> {
    let profile = invocation()?.profile();
    let budget = PressureBudget::declared(mutants, profile)
        .map_err(|_| MutationRoadFailure::MissingAlternative)?;
    Ok(ScopedInvocation::scoped(ScopeShape::RepoWide, budget))
}

/// Claim: a production/evaluation pair cannot join bindings from different evaluation families.
///
/// Subject: the public pair constructor and its family coordinate.
/// Population: two admitted surfaces under distinct declared families.
/// Hostile control: production names the first family while evaluation is bound to the second surface.
/// Denominator: the constructor's one family mismatch coordinate.
/// Evidence ceiling: this establishes the typed family refusal for one outside fixture and does not inspect private pair state.
/// Retained regression: the claim remains in the interpretation claim module of the integration target.
#[test]
fn evaluation_pairs_refuse_foreign_families_before_execution() -> Result<(), MutationRoadFailure> {
    let first_family = family("constructor-family")?;
    let other_family = family("another-constructor-family")?;
    let other_surface = surface_with(other_family, vec![b"a <= b"])?;
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"evaluation"));
    assert!(matches!(
        EvaluationPair::paired(
            ProductionBinding::declared(first_family, production_revision, production),
            EvaluationBinding::declared(&other_surface, evaluation_revision, EVALUATION),
            same,
        ),
        Err(EvaluationPairRefusal::FamilyMismatch {
            production,
            evaluation,
        }) if production == first_family && evaluation == other_family
    ));
    Ok(())
}

/// Claim: an interpreted pass plans every admitted alternative once in canonical order and refuses empty or overspent plans.
///
/// Subject: the public interpretation planning operation.
/// Population: one two-alternative surface, its reversed authoring twin, one point-free surface, and one undersized budget.
/// Hostile control: reversed alternative input, no admitted points, and one budget slot for two runs.
/// Denominator: every admitted alternative and both declared refusal stages.
/// Evidence ceiling: this establishes inspectable plan algebra for these outside fixtures and does not claim execution cost.
/// Retained regression: canonical order, non-vacuity, and budget refusal remain together in the interpretation claim module.
#[test]
fn interpreted_pass_planning_is_canonical_non_vacuous_and_budgeted()
-> Result<(), MutationRoadFailure> {
    let family = family("planned-family")?;
    let surface = surface_with(family, vec![b"a <= b", b"a >= b"])?;
    let reversed = surface_with(family, vec![b"a >= b", b"a <= b"])?;
    assert_eq!(surface.identity(), reversed.identity());

    let planned = plan_pass(&surface, scope(2)?)?;
    let reversed_plan = plan_pass(&reversed, scope(2)?)?;
    assert_eq!(planned, reversed_plan);
    assert_eq!(planned.runs().len(), 2usize);
    let [point] = surface.points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    for (run, alternative) in planned.runs().iter().zip(point.admitted_alternatives()) {
        assert_eq!(run.lane(), PressureLane::InterpretedMutation);
        assert_eq!(
            run.target(),
            MutationIdentity::Interpreted {
                point: point.identity(),
                alternative: alternative.identity(),
            }
        );
        assert_eq!(
            run.damage(),
            PlannedDamage::Alternative(alternative.identity())
        );
        assert_eq!(
            run.selection(),
            &Selection::ByClaim(BTreeSet::from([claim()?]))
        );
        assert_eq!(run.budget().mutants(), 2u32);
    }

    let empty_policy = policy(family)?;
    let empty_surface = lower_discoveries(&empty_policy, Vec::new())?.into_parts().1;
    assert_eq!(
        plan_pass(&empty_surface, scope(1)?),
        Err(PlanRefusal::NoRunPlanned)
    );
    assert_eq!(
        plan_pass(&surface, scope(1)?),
        Err(PlanRefusal::BudgetOverspent {
            admitted: 1,
            planned: 2,
        })
    );
    Ok(())
}

/// Claim: no-mutation evidence states the exact pairing and two shared substrates declared by interpretation.
///
/// Subject: the public no-mutation observation and its retained parity reading.
/// Population: one hostile semantic disagreement under the exact pair and substrate constants.
/// Hostile control: a foreign substrate name is absent and the two meanings disagree.
/// Denominator: every interpretation-owned pairing and substrate spelling.
/// Evidence ceiling: this establishes declared names and retained typed causes, not a new encoded preimage owned by interpretation.
/// Retained regression: the exact constants, roster, and disagreement cause remain observed together.
#[test]
fn no_mutation_reading_retains_exact_pairing_and_substrate_identities()
-> Result<(), MutationRoadFailure> {
    assert_eq!(MUTERPRATER_NAMESPACE, "muterprater");
    assert_eq!(NO_MUTATION_PAIRING, "no-mutation-parity");
    assert_eq!(PARITY_DECLARATION_SUBSTRATE, "one-declaration");
    assert_eq!(PARITY_RENDERING_SUBSTRATE, "rendering-engine");

    let family = family("identity-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, PARITY_BROKEN)?;
    let input = [1u32, 0, 0];
    let reading = observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_passes)?,
        &input,
        &invocation()?,
    )?;
    let declaration = SubstrateRef::named(MUTERPRATER_NAMESPACE, PARITY_DECLARATION_SUBSTRATE)
        .map_err(|_| MutationRoadFailure::Name)?;
    let rendering = SubstrateRef::named(MUTERPRATER_NAMESPACE, PARITY_RENDERING_SUBSTRATE)
        .map_err(|_| MutationRoadFailure::Name)?;
    let foreign = SubstrateRef::named(MUTERPRATER_NAMESPACE, "foreign-substrate")
        .map_err(|_| MutationRoadFailure::Name)?;
    let SharedSubstrate::Standing(roster) = reading.substrate() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(roster.standing(), &BTreeSet::from([declaration, rendering]));
    assert!(!roster.standing().contains(&foreign));
    assert!(matches!(
        reading.conclusion(),
        TrialConclusion::Refused(finding)
            if finding.cause().family() == MUTERPRATER_NAMESPACE
                && finding.cause().local() == NO_MUTATION_PAIRING
    ));
    Ok(())
}

/// Claim: admitted active evidence derives survival from the retained report rather than from firing alone.
///
/// Subject: the public interpretation execution road and admitted mutation report.
/// Population: one selected alternative whose active meaning passes its witness check.
/// Hostile control: the active callback reports a positive firing while preserving the passing meaning.
/// Denominator: the selected active execution and its one retained report.
/// Evidence ceiling: this establishes report-derived survival for one outside fixture after the existing specimen gate.
/// Retained regression: the interpretation claim remains separate from specimen materialization claims.
pub(super) fn interpreted_survivor() -> Result<MutationReport, MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("surviving-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, ACTIVATION_SURVIVES)?;
    let input = [1u32, 0, 0];
    let standing = qualified_no_mutation(&pair, witness()?, &input)?;
    let qualification = qualification_of(&standing)?;
    let suite = compiled_suite_pressure()?;
    let selection = active_selection(&surface)?;
    let projection = standard_projection(&surface, qualification, &pair, selection)?;
    let trust = opened_trust(availability(
        Some(&surface),
        Some(&suite),
        Some(&projection),
    ))?;
    let evidence = execute_active(&trust, &invocation()?)?;
    assert!(matches!(
        (evidence.report().attempt(), evidence.mutation().outcome()),
        (
            RunAttempt::Executed(TrialConclusion::Passed),
            MutationOutcome::Survived,
        )
    ));
    Ok(evidence.mutation().clone())
}

#[test]
fn active_classification_is_derived_from_the_admitted_report() -> Result<(), MutationRoadFailure> {
    assert_eq!(interpreted_survivor()?.verdict(), MutationVerdict::Survived);
    Ok(())
}

/// Claim: a passing witness conclusion cannot launder semantic disagreement into parity qualification.
///
/// Subject: the public no-mutation observation and qualification roads.
/// Population: one pair whose production and evaluation meanings disagree while both checks pass.
/// Hostile control: the witness always passes both meanings.
/// Denominator: the complete no-mutation reading and its qualification decision.
/// Evidence ceiling: this establishes semantic agreement as an independent gate for one outside fixture.
/// Retained regression: the hostile passing-check reversal remains in the interpretation claim module.
#[test]
fn no_mutation_agreement_must_be_earned() -> Result<(), MutationRoadFailure> {
    let family = family("parity-hostile-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, PARITY_BROKEN)?;
    let input = [1u32, 0, 0];
    let standing = qualified_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_passes)?,
        &input,
    )?;
    let rejection = standing
        .rejection()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::MeaningsDisagreed,
        ))?;
    assert_eq!(
        rejection.cause(),
        ParityQualificationRefusal::MeaningsDisagreed
    );
    assert!(matches!(
        rejection.reading().conclusion(),
        TrialConclusion::Refused(_)
    ));
    Ok(())
}

/// Claim: no-mutation reports retain semantic roles and production refusal precedes evaluation refusal.
///
/// Subject: the public no-mutation qualification decision order.
/// Population: evaluation-only refusal, production-only refusal, and refusal of both roles.
/// Hostile control: three witness callables independently reverse which role qualifies.
/// Denominator: every report-qualification ordering branch before activation and meaning agreement.
/// Evidence ceiling: this establishes the declared priority for these outside meanings and reports.
/// Retained regression: all role reversals remain one interpretation-owned observation.
#[test]
fn no_mutation_report_roles_and_refusal_priority_are_observed() -> Result<(), MutationRoadFailure> {
    let family = family("report-role-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, PARITY_BROKEN)?;
    let input = [1u32, 0, 0];

    let evaluation_rejected = qualified_no_mutation(&pair, witness()?, &input)?;
    assert!(matches!(
        evaluation_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::EvaluationDidNotQualify
    ));

    let production_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_evaluation_meaning)?,
        &input,
        &invocation()?,
    )?);
    assert!(matches!(
        production_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::ProductionDidNotQualify
    ));

    let both_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_refuses)?,
        &input,
        &invocation()?,
    )?);
    assert!(matches!(
        both_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::ProductionDidNotQualify
    ));
    Ok(())
}

/// Claim: no-mutation qualification requires the evaluation callback to report zero firings.
///
/// Subject: the public no-mutation qualification road.
/// Population: one semantically agreeing pair whose no-mutation callback reports one firing.
/// Hostile control: meanings and reports qualify while the firing count alone is invalid.
/// Denominator: the activation gate between report qualification and meaning agreement.
/// Evidence ceiling: this establishes the exact one-firing refusal and retained count for one outside fixture.
/// Retained regression: the positive-firing reversal remains in the interpretation claim module.
#[test]
fn no_mutation_requires_zero_firings() -> Result<(), MutationRoadFailure> {
    let family = family("no-mutation-firing-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, NO_MUTATION_ACTIVATES)?;
    let input = [1u32, 0, 0];
    let standing = qualified_no_mutation(&pair, witness()?, &input)?;
    let rejection = standing
        .rejection()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::NoMutationActivated { firings: 1 },
        ))?;
    assert_eq!(
        rejection.cause(),
        ParityQualificationRefusal::NoMutationActivated { firings: 1 }
    );
    assert_eq!(rejection.reading().evaluation_firings(), 1);
    Ok(())
}

/// Claim: evaluation call refusals preserve no-mutation versus exact active directive posture.
///
/// Subject: the public observation and active execution callback boundaries.
/// Population: one callback omitting no-mutation and one callback omitting active selection.
/// Hostile control: each callback implements the opposite branch lawfully.
/// Denominator: both evaluation-call refusal variants and the production-before-evaluation order.
/// Evidence ceiling: this establishes callback posture and ordering for two outside fixtures after existing trust gates.
/// Retained regression: both branch reversals remain one interpretation-owned observation.
#[test]
fn evaluation_call_refusals_preserve_directive_posture() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let no_mutation_family = family("no-mutation-branch-omitted")?;
    let no_mutation_surface = surface_with(no_mutation_family, vec![SELECTED_OPERATION])?;
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"evaluation"));
    let no_mutation_pair = EvaluationPair::paired(
        ProductionBinding::declared(no_mutation_family, production_revision, production_ordered),
        EvaluationBinding::declared(
            &no_mutation_surface,
            evaluation_revision,
            no_mutation_branch_omitted,
        ),
        same,
    )?;
    let input = [1u32, 0, 0];
    NO_MUTATION_CALL_ORDER.store(0, Ordering::SeqCst);
    assert!(matches!(
        observe_no_mutation(&no_mutation_pair, witness()?, &input, &invocation()?),
        Err(NoMutationObservationRefusal::EvaluationCall(
            EvaluationCallRefusal::NoMutationNotImplemented,
        ))
    ));
    assert_eq!(NO_MUTATION_CALL_ORDER.load(Ordering::SeqCst), 2);

    let active_family = family("active-branch-omitted")?;
    let active_surface = surface_with(active_family, vec![SELECTED_OPERATION])?;
    let active_pair = pair(active_family, &active_surface, active_branch_omitted)?;
    let standing = qualified_no_mutation(&active_pair, witness()?, &input)?;
    let qualification = qualification_of(&standing)?;
    let selection = active_selection(&active_surface)?;
    let projection = standard_projection(&active_surface, qualification, &active_pair, selection)?;
    let suite = compiled_suite_pressure()?;
    let trust = opened_trust(availability(
        Some(&active_surface),
        Some(&suite),
        Some(&projection),
    ))?;
    assert!(matches!(
        execute_active(&trust, &invocation()?),
        Err(InterpretedExecutionRefusal::EvaluationCall(
            EvaluationCallRefusal::ActiveSelectionNotImplemented(found),
        )) if found == selection
    ));
    Ok(())
}

/// Claim: a selected alternative reporting zero firings cannot become interpreted mutation evidence.
///
/// Subject: the public active interpretation execution road.
/// Population: one trusted selected alternative whose callback reports zero active firings.
/// Hostile control: the callback returns a successful raw observation with no activation.
/// Denominator: the one selected execution and its activation admission boundary.
/// Evidence ceiling: this establishes the exact dud refusal for one outside fixture after the existing specimen gate.
/// Retained regression: zero-firing non-vacuity remains in the interpretation claim module.
#[test]
fn an_unfired_selection_is_not_mutation_evidence() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("dud-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, ACTIVATION_MISSING)?;
    let input = [1u32, 0, 0];
    let standing = qualified_no_mutation(&pair, witness()?, &input)?;
    let qualification = qualification_of(&standing)?;
    let suite = compiled_suite_pressure()?;
    let selection = active_selection(&surface)?;
    let projection = standard_projection(&surface, qualification, &pair, selection)?;
    let trust = opened_trust(availability(
        Some(&surface),
        Some(&suite),
        Some(&projection),
    ))?;
    assert!(matches!(
        execute_active(&trust, &invocation()?),
        Err(InterpretedExecutionRefusal::DudPlant(dud)) if dud.selection() == selection
    ));
    Ok(())
}

/// Claim: a mutation witness cannot pair a callable with a different check identity than its trial row declares.
///
/// Subject: the public mutation-witness constructor.
/// Population: one trial binding and one foreign check identity.
/// Hostile control: the original check callable is offered under the foreign identity.
/// Denominator: the constructor's check-coordinate join.
/// Evidence ceiling: this establishes outside unwritability through the smart constructor and does not expose witness fields.
/// Retained regression: the exact expected and found coordinates remain observed.
#[test]
fn a_mutation_witness_keeps_its_check_identity_and_callable_together()
-> Result<(), MutationRoadFailure> {
    let expected = check_ref()?;
    let found = CheckRef::named(OWNER, "another-check").map_err(|_| MutationRoadFailure::Name)?;
    assert!(matches!(
        MutationWitness::bound(trial_binding()?, found, check),
        Err(MutationWitnessRefusal::CheckMismatch {
            expected: refusal_expected,
            found: refusal_found,
        }) if refusal_expected == expected && refusal_found == found
    ));
    Ok(())
}

/// Claim: interpretation availability requires a surface, generic suite pressure, and exact compiled projection pressure in order.
///
/// Subject: the public interpretation availability operation.
/// Population: the three prefixes of one otherwise lawful evidence tuple.
/// Hostile control: each prefix omits the next required authority while retaining all earlier seats.
/// Denominator: every missing-evidence branch before cross-surface validation.
/// Evidence ceiling: this establishes absence ordering and no authority beyond the supplied typed values.
/// Retained regression: every missing seat remains observed together in the interpretation claim module.
#[test]
fn interpretation_availability_requires_every_evidence_book_in_order()
-> Result<(), MutationRoadFailure> {
    let surface = surface_with(family("local-family")?, vec![b"a <= b"])?;
    let suite = compiled_suite_pressure()?;
    assert!(matches!(
        availability::<[u32; 3], CompiledRosterMeaning>(None, None, None),
        InterpreterAvailability::NoConformingSurface
    ));
    assert!(matches!(
        availability::<[u32; 3], CompiledRosterMeaning>(Some(&surface), None, None),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledSuitePressure,
        }
    ));
    assert!(matches!(
        availability::<[u32; 3], CompiledRosterMeaning>(Some(&surface), Some(&suite), None),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledProjectionPressure,
        }
    ));
    Ok(())
}

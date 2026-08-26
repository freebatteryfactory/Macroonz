//! Outside claims over exact compiled specimen materialization, request custody, joins, and witness outcomes.

use super::support::{
    CACHED_SIBLING_OBSERVATION_HOST, COMPILED_SPECIMEN_HOST, SPECIMEN_HOST_CALLS,
    SPECIMEN_MATERIALIZER, SPECIMEN_MATERIALIZER_CALLS, UNCHANGED_SPECIMEN_MATERIALIZER,
    WRONG_SELECTED_SPECIMEN, clear_cached_sibling_observation, lock_specimen_tests,
    omitted_baseline_branch, omitted_specimen_branch, specimen_source,
};
use super::{
    CLAIM_MISMATCH_EVALUATION_CALLS, EVALUATION, MutationRoadFailure, ORIGINAL_OPERATION,
    SELECTED_OPERATION, active_selection, check, check_ref, claim, evaluation_counted,
    evaluation_reads_resolved_payload, family, foreign_invocation, invocation, pair,
    pair_with_evaluation_revision, surface_with, trial_binding, trial_binding_for,
};
use macroonz_harness::descriptor::ClaimRef;
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::interpret::{observe_no_mutation, qualify_no_mutation};
use macroonz_harness::muterprater::specimen::demonstrate_compiled_projection;
use macroonz_harness::muterprater::{
    ARTIFACT_CONTENT_TAG, CompiledProjectionRefusal, CompiledSpecimenHostRefusal,
    CompiledSpecimenObservationMismatch, EvaluationPairStandingMismatch, MutationWitness,
    ParityQualificationRefusal, SelectionRefusal, SpecimenMaterializerBinding,
    SpecimenMaterializerRefusal,
};
use std::sync::atomic::Ordering;

/// Claim: exact projection pressure requires both materializer branches, changed bytes, and the selected operation in the selected source.
///
/// Subject: `demonstrate_compiled_projection` before and across its two host calls.
/// Population: a missing baseline branch, a missing selected branch, identical renderings, and wrong selected bytes.
/// Hostile control: call counters prove pre-host refusals, while the real host refuses selected bytes that omit the requested operation.
/// Denominator: both directive postures and both artifact roles for one admitted selection.
/// Evidence ceiling: the final control runs the local pinned compiler host for one generated program and does not authenticate arbitrary caller hosts.
/// Retained regression: all four reversals remain in the `trust_opening_evidence` target.
#[test]
fn exact_projection_requires_one_real_selected_artifact() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("compiled-artifact-boundary")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, evaluation_reads_resolved_payload)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;

    let baseline_omitted = SpecimenMaterializerBinding::bound(&pair, omitted_baseline_branch);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &baseline_omitted,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::BaselineMaterialization(
            SpecimenMaterializerRefusal::NoMutationNotImplemented,
        ))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let omitted = SpecimenMaterializerBinding::bound(&pair, omitted_specimen_branch);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &omitted,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::SelectedMaterialization(
            SpecimenMaterializerRefusal::ActiveSelectionNotImplemented(found),
        )) if found == selection
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let unchanged = SpecimenMaterializerBinding::bound(&pair, UNCHANGED_SPECIMEN_MATERIALIZER);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &unchanged,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::ArtifactDidNotChange(_))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let wrong_selected = SpecimenMaterializerBinding::bound(&pair, WRONG_SELECTED_SPECIMEN);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &wrong_selected,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::SelectedHost(
            CompiledSpecimenHostRefusal::Meaning(_),
        ))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    Ok(())
}

/// Claim: a host observation must reproduce the exact request currently being judged.
///
/// Subject: the request-to-observation join inside `demonstrate_compiled_projection`.
/// Population: one baseline request followed by one selected request with different source identities.
/// Hostile control: a host caches the lawful baseline observation and returns it for the selected request.
/// Denominator: both roles and their independently derived content identities.
/// Evidence ceiling: this establishes structural observation custody and does not claim the hostile host ran a compiler.
/// Retained regression: the cached-sibling reversal remains in the `trust_opening_evidence` target.
#[test]
fn host_observations_must_join_the_current_specimen_request() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("compiled-observation-boundary")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    clear_cached_sibling_observation()?;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            CACHED_SIBLING_OBSERVATION_HOST,
        ),
        Err(CompiledProjectionRefusal::SelectedObservation(
            CompiledSpecimenObservationMismatch::Content { expected, found },
        )) if expected.address()
                == ContentAddress::derived(ARTIFACT_CONTENT_TAG, &specimen_source(b"a <= b"))
            && found.address()
                == ContentAddress::derived(ARTIFACT_CONTENT_TAG, &specimen_source(ORIGINAL_OPERATION))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    Ok(())
}

/// Claim: a separately compiled baseline must pass its retained witness before selected execution begins.
///
/// Subject: the baseline report gate in `demonstrate_compiled_projection`.
/// Population: one compiled baseline whose exact input makes the retained check refuse.
/// Hostile control: call counts show both sources render but only the baseline reaches the host.
/// Denominator: the baseline role and the selected role withheld behind it.
/// Evidence ceiling: the local pinned compiler and one input establish ordering and qualification, not general compiler correctness.
/// Retained regression: the refusing baseline remains an outside integration observation.
#[test]
fn compiled_baseline_must_pass_before_selected_execution() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("compiled-baseline-outcome")?;
    let surface = surface_with(family, vec![SELECTED_OPERATION])?;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [0u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::BaselineDidNotQualify)
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 1);
    Ok(())
}

/// Claim: selected pressure requires the qualified execution and an actual witness rejection.
///
/// Subject: the execution-key and selected-report gates in `demonstrate_compiled_projection`.
/// Population: one foreign invocation and one selected artifact whose behavior survives the check.
/// Hostile control: the foreign invocation reaches no callback, while the survivor reaches both materializer and host roles.
/// Denominator: both post-baseline gates for one selected artifact.
/// Evidence ceiling: the local pinned compiler and one selection establish these reversals only for the declared fixture.
/// Retained regression: both refusals remain outside observations in the same integration target.
#[test]
fn selected_compiled_behavior_must_reject_under_qualified_execution()
-> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("compiled-selected-survives")?;
    let surface = surface_with(family, vec![b"input > 0"])?;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &foreign_invocation(),
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::InvocationForAnotherExecution)
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::ProjectionDidNotReject)
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    Ok(())
}

/// Claim: exact pressure keeps its witness claim and surface-issued selection together before effects.
///
/// Subject: the claim and selection joins in `demonstrate_compiled_projection`.
/// Population: one witness from another claim and one selection from another surface.
/// Hostile control: evaluation, materializer, and host counters remain zero for the crossed joins.
/// Denominator: both structural joins that precede source rendering.
/// Evidence ceiling: this establishes pre-effect typed rejection and says nothing about callback behavior after lawful joins.
/// Retained regression: both crossed joins remain one claim-family test in the integration target.
#[test]
fn projection_requires_its_witness_claim_and_surface_selection() -> Result<(), MutationRoadFailure>
{
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("claim-bound-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, evaluation_counted)?;
    let foreign_witness =
        MutationWitness::bound(trial_binding_for("another-behaviour")?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        foreign_witness,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let expected_claim = claim()?;
    let foreign_claim = ClaimRef::named(super::OWNER, "another-behaviour")
        .map_err(|_| MutationRoadFailure::Name)?;
    CLAIM_MISMATCH_EVALUATION_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::WitnessForAnotherClaim { expected, found })
            if expected == expected_claim && found == foreign_claim
    ));
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let local_standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let local_qualification =
        local_standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let foreign_surface = surface_with(family, vec![b"a >= b"])?;
    let foreign_selection = active_selection(&foreign_surface)?;
    let expected_surface = surface.identity();
    let found_surface = foreign_surface.identity();
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            local_qualification,
            &materializer,
            foreign_selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::Selection(
            SelectionRefusal::SelectionFromAnotherSurface { expected, found },
        )) if expected == expected_surface && found == found_surface
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);
    Ok(())
}

/// Claim: a materializer must retain the exact qualified pair revision before rendering.
///
/// Subject: the pair-standing join in `demonstrate_compiled_projection`.
/// Population: one materializer bound to another evaluation revision under the same family and surface.
/// Hostile control: both materializer and host call counters remain zero.
/// Denominator: the evaluation-revision member of one otherwise matching pair standing.
/// Evidence ceiling: this establishes the named mismatch member and pre-effect refusal for one crossed revision.
/// Retained regression: the pair-revision reversal remains in the specimen claim module.
#[test]
fn materializer_must_match_the_qualified_pair_revision() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("same-family-pair-scope")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let revision_pair = pair_with_evaluation_revision(
        family,
        &surface,
        EVALUATION,
        b"another-evaluation-revision",
    )?;
    assert_ne!(revision_pair.standing(), pair.standing());
    let materializer = SpecimenMaterializerBinding::bound(&revision_pair, SPECIMEN_MATERIALIZER);
    let selection = active_selection(&surface)?;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::MaterializerForAnotherPair(
            EvaluationPairStandingMismatch::EvaluationRevision { expected, found },
        )) if expected == pair.standing().evaluation_revision()
            && found == revision_pair.standing().evaluation_revision()
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);
    Ok(())
}

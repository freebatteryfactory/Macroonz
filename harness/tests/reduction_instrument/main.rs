//! Bound semantic and generic reducers retain their exact path, preservation evidence, and replay ceiling.

use macroonz_harness::descriptor::{DerivedRevision, GeneratedSupportSchemaId, RevisionBinding};
use macroonz_harness::generate::reduce::{capture_replay, reduce, shrink_verdict};
use macroonz_harness::generate::types::{
    ByteReducerExecution, ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget,
    ReductionHalt, ReductionPlan, ReductionPlanRefusal, ReductionProbeBinding,
    ReductionProbeRefusal, ReductionRefusal, SemanticCandidateRefusal, SemanticCandidates,
    SemanticReducerBinding, SemanticReducerId, ShrinkVerdict,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion, encode_bytes};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, Fingerprint, GenerationProfile,
    MinimizationProfile, REPLAY_CAPSULE_TAG, ReplayCapsule, ReplayPosture, TrialConclusion,
    TrialFinding, TrialReport, TrialSite,
};
use macroonz_harness::runner::Invocation;
use std::fmt;

#[path = "../support/trial_fixture.rs"]
mod trial_fixture;

use trial_fixture::TrialFixture;

const PRESERVED_CAUSE: FindingCause = FindingCause::named("harness", "preserved-failure");
const MOVED_CAUSE: FindingCause = FindingCause::named("harness", "moved-failure");
const REVISION_TAG: DomainTag =
    DomainTag::declared("reduction-revision", IdentityProfileVersion::declared(1));
const SCHEMA_TAG: DomainTag =
    DomainTag::declared("reduction-schema", IdentityProfileVersion::declared(1));

const fn independent_replay_posture_slot(posture: ReplayPosture) -> u8 {
    match posture {
        ReplayPosture::ExactDerived => 0,
        ReplayPosture::DeclaredByAuthor => 1,
        ReplayPosture::UnavailableBecauseUntracked => 2,
    }
}

fn independently_derived_capsule_identity(capsule: &ReplayCapsule) -> ContentAddress {
    let mut preimage = Vec::new();
    encode_bytes(capsule.key().address().as_bytes(), &mut preimage);
    encode_bytes(capsule.input(), &mut preimage);
    encode_bytes(capsule.fingerprint().address().as_bytes(), &mut preimage);
    encode_bytes(capsule.generation().name().as_bytes(), &mut preimage);
    preimage.extend_from_slice(&capsule.generation().version().to_be_bytes());
    encode_bytes(capsule.minimization().name().as_bytes(), &mut preimage);
    preimage.extend_from_slice(&capsule.minimization().version().to_be_bytes());
    encode_bytes(capsule.schema().address().as_bytes(), &mut preimage);
    preimage.push(independent_replay_posture_slot(capsule.posture()));
    ContentAddress::derived(REPLAY_CAPSULE_TAG, &preimage)
}

enum ReductionRoadFailure {
    Plan(ReductionPlanRefusal),
    Run(ReductionRefusal),
    Fixture,
}

impl fmt::Debug for ReductionRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
            Self::Run(refusal) => formatter.debug_tuple("Run").field(refusal).finish(),
            Self::Fixture => formatter.write_str("Fixture"),
        }
    }
}

impl From<ReductionPlanRefusal> for ReductionRoadFailure {
    fn from(refusal: ReductionPlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

impl From<ReductionRefusal> for ReductionRoadFailure {
    fn from(refusal: ReductionRefusal) -> Self {
        Self::Run(refusal)
    }
}

fn trial_fingerprint(cause: FindingCause) -> Option<Fingerprint> {
    Some(trial_fixture()?.fingerprint(cause))
}

fn probe(input: &[u8]) -> ProbeOutcome {
    let (Some(preserved), Some(moved)) = (
        trial_fingerprint(PRESERVED_CAUSE),
        trial_fingerprint(MOVED_CAUSE),
    ) else {
        return ProbeOutcome::NoFailure;
    };
    match input {
        [1u8, 2u8, 3u8] | [1u8, 2u8] | [1u8] => ProbeOutcome::Reproduced(preserved),
        [0u8, 0u8] | [2u8] => ProbeOutcome::Reproduced(moved),
        _ => ProbeOutcome::NoFailure,
    }
}

fn refused_trial(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        PRESERVED_CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

fn passed_trial(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn revision_derived_from(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

fn trial_fixture() -> Option<TrialFixture> {
    TrialFixture::named(
        "generic-byte-reduction",
        "reduction",
        "reduction",
        "outside-consumer",
        "reduction-candidates",
        TrialSite::located(module_path!(), file!(), line!(), "reduction"),
    )
}

fn trial_report_with(call: fn(&Invocation) -> TrialConclusion) -> Option<TrialReport> {
    trial_fixture()?.report(call, revision_derived_from(b"trial"))
}

fn probe_binding(revision: RevisionBinding) -> Option<ReductionProbeBinding> {
    trial_fixture()?.probe_binding(
        refused_trial,
        revision_derived_from(b"trial"),
        GenerationProfile::declared("reduction-input", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        revision,
        probe,
    )
}

fn semantic_candidates(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    match input {
        [1u8, 2u8, 3u8] => SemanticCandidates::proposed(input, vec![vec![1u8, 2u8], vec![1u8]]),
        _ => SemanticCandidates::proposed(input, Vec::new()),
    }
}

fn first_semantic_step(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    match input {
        [1u8, 2u8, 3u8] => SemanticCandidates::proposed(input, vec![vec![1u8, 2u8]]),
        _ => SemanticCandidates::proposed(input, Vec::new()),
    }
}

fn second_semantic_step(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    match input {
        [1u8, 2u8] => SemanticCandidates::proposed(input, vec![vec![1u8]]),
        _ => SemanticCandidates::proposed(input, Vec::new()),
    }
}

fn non_descending_candidates(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    SemanticCandidates::proposed(input, vec![input.to_vec()])
}

#[test]
fn generic_reduction_preserves_one_fingerprint_and_reports_every_candidate_class()
-> Result<(), ReductionRoadFailure> {
    let Some(preserved) = trial_fingerprint(PRESERVED_CAUSE) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let Some(moved) = trial_fingerprint(MOVED_CAUSE) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("generic-byte-reduction", 1u32),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(16u32),
    )?;
    let Some(binding) = probe_binding(revision_derived_from(b"probe")) else {
        return Err(ReductionRoadFailure::Fixture);
    };

    assert!(plan.semantic_reducers().is_empty());
    assert_eq!(
        reduce(&plan, &[9u8], &binding),
        Err(ReductionRefusal::BaselineDidNotFail)
    );
    assert_eq!(
        reduce(&plan, &[2u8], &binding),
        Err(ReductionRefusal::BaselineFingerprintDiffers { found: moved })
    );
    assert_eq!(
        shrink_verdict(preserved, &[1u8], probe),
        ShrinkVerdict::Accepted
    );
    assert_eq!(
        shrink_verdict(preserved, &[2u8], probe),
        ShrinkVerdict::RejectedFingerprintMoved { found: moved }
    );
    assert_eq!(
        shrink_verdict(preserved, &[9u8], probe),
        ShrinkVerdict::RejectedNoFailure
    );

    let evidence = reduce(&plan, &[1u8, 2u8], &binding)?;
    let reduced = evidence.outcome();
    assert_eq!(reduced.input(), &[1u8]);
    assert_eq!(reduced.fingerprint(), preserved);
    assert_eq!(reduced.census().accepted(), 1u32);
    assert_eq!(reduced.census().fingerprint_moved(), 2u32);
    assert_eq!(reduced.census().no_failure(), 4u32);
    assert_eq!(reduced.census().probes(), 7u32);
    assert_eq!(reduced.halt(), ReductionHalt::FixedPointReached);
    assert_eq!(
        evidence.byte_reducer(),
        ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing)
    );
    assert!(evidence.semantic_reducers().is_empty());
    Ok(())
}

#[test]
fn semantic_reducer_custody_and_replay_posture_are_run_derived() -> Result<(), ReductionRoadFailure>
{
    assert_eq!(ReplayPosture::ExactDerived.slot(), 0);
    assert_eq!(ReplayPosture::DeclaredByAuthor.slot(), 1);
    assert_eq!(ReplayPosture::UnavailableBecauseUntracked.slot(), 2);

    let reducer = SemanticReducerId::named("harness", "sequence-aware")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let derived_revision = revision_derived_from(b"semantic-reducer");
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("semantic-reduction", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        vec![SemanticReducerBinding::bound(
            reducer,
            derived_revision,
            semantic_candidates,
        )],
        FingerprintPreservation::Required,
        ReductionBudget::declared(2),
    )?;
    let Some(binding) = probe_binding(revision_derived_from(b"probe")) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let evidence = reduce(&plan, &[1u8, 2u8, 3u8], &binding)?;
    let [execution] = evidence.semantic_reducers() else {
        return Err(ReductionRoadFailure::Fixture);
    };
    assert_eq!(execution.reducer(), reducer);
    assert_eq!(execution.revision(), derived_revision);
    assert_eq!(execution.candidates(), 2);
    assert_eq!(execution.probes(), 2);
    assert_eq!(
        evidence.byte_reducer(),
        ByteReducerExecution::NotReachedBecauseBudgetSpent
    );
    assert_eq!(evidence.outcome().input(), &[1u8]);
    assert_eq!(evidence.outcome().halt(), ReductionHalt::BudgetExhausted);
    assert_eq!(evidence.replay_posture(), ReplayPosture::ExactDerived);

    let capsule = capture_replay(&evidence);
    assert_eq!(capsule.key(), evidence.standing().key());
    assert_eq!(capsule.input(), evidence.outcome().input());
    assert_eq!(capsule.fingerprint(), evidence.outcome().fingerprint());
    assert_eq!(capsule.generation(), evidence.generation());
    assert_eq!(capsule.minimization(), evidence.minimization());
    assert_eq!(capsule.schema(), evidence.schema());
    assert_eq!(capsule.posture(), ReplayPosture::ExactDerived);
    assert_eq!(
        capsule.identity(),
        independently_derived_capsule_identity(&capsule)
    );

    let declared_plan = ReductionPlan::declared(
        MinimizationProfile::declared("semantic-reduction", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        vec![SemanticReducerBinding::bound(
            reducer,
            RevisionBinding::declared(ContentAddress::derived(
                REVISION_TAG,
                b"declared-semantic-reducer",
            )),
            semantic_candidates,
        )],
        FingerprintPreservation::Required,
        ReductionBudget::declared(2),
    )?;
    let declared = reduce(&declared_plan, &[1u8, 2u8, 3u8], &binding)?;
    assert_eq!(declared.replay_posture(), ReplayPosture::DeclaredByAuthor);
    let declared_capsule = capture_replay(&declared);
    assert_eq!(declared_capsule.posture(), ReplayPosture::DeclaredByAuthor);

    // The non-vacuity control on the posture's seat in the identity preimage: these two capsules
    // agree on every other member — one execution key, one reduced input, one fingerprint, one
    // generation and minimization profile, one schema — and differ in posture alone, so equal
    // identities here would mean cache and replay authority no longer moves when the posture does.
    assert_eq!(declared_capsule.key(), capsule.key());
    assert_eq!(declared_capsule.input(), capsule.input());
    assert_eq!(declared_capsule.fingerprint(), capsule.fingerprint());
    assert_eq!(declared_capsule.generation(), capsule.generation());
    assert_eq!(declared_capsule.minimization(), capsule.minimization());
    assert_eq!(declared_capsule.schema(), capsule.schema());
    assert_ne!(declared_capsule.posture(), capsule.posture());
    assert_ne!(declared_capsule.identity(), capsule.identity());

    let Some(untracked_binding) = probe_binding(RevisionBinding::untracked(
        ContentAddress::derived(REVISION_TAG, b"untracked-probe"),
    )) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let untracked = reduce(&plan, &[1u8, 2u8, 3u8], &untracked_binding)?;
    assert_eq!(
        untracked.replay_posture(),
        ReplayPosture::UnavailableBecauseUntracked
    );
    assert_eq!(
        capture_replay(&untracked).posture(),
        ReplayPosture::UnavailableBecauseUntracked
    );
    Ok(())
}

#[test]
fn semantic_reducers_run_in_declared_order_over_the_current_best()
-> Result<(), ReductionRoadFailure> {
    let first = SemanticReducerId::named("harness", "first-semantic-step")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let second = SemanticReducerId::named("harness", "second-semantic-step")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("ordered-semantic-reduction", 1u32),
        ByteReducerId::ChunkRemovalAndZeroing,
        vec![
            SemanticReducerBinding::bound(
                first,
                revision_derived_from(b"first-semantic-step"),
                first_semantic_step,
            ),
            SemanticReducerBinding::bound(
                second,
                revision_derived_from(b"second-semantic-step"),
                second_semantic_step,
            ),
        ],
        FingerprintPreservation::Required,
        ReductionBudget::declared(2u32),
    )?;
    let Some(binding) = probe_binding(revision_derived_from(b"ordered-probe")) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let evidence = reduce(&plan, &[1u8, 2u8, 3u8], &binding)?;
    let [first_execution, second_execution] = evidence.semantic_reducers() else {
        return Err(ReductionRoadFailure::Fixture);
    };
    assert_eq!(first_execution.reducer(), first);
    assert_eq!(first_execution.candidates(), 1usize);
    assert_eq!(first_execution.probes(), 1usize);
    assert_eq!(second_execution.reducer(), second);
    assert_eq!(second_execution.candidates(), 1usize);
    assert_eq!(second_execution.probes(), 1usize);
    assert_eq!(evidence.outcome().input(), &[1u8]);
    assert_eq!(evidence.outcome().halt(), ReductionHalt::BudgetExhausted);
    assert_eq!(
        evidence.byte_reducer(),
        ByteReducerExecution::NotReachedBecauseBudgetSpent
    );
    Ok(())
}

#[test]
fn semantic_candidate_and_plan_boundaries_refuse_non_descent_and_duplicate_identity()
-> Result<(), ReductionRoadFailure> {
    assert_eq!(
        SemanticCandidates::proposed(&[1u8, 2u8], vec![vec![9u8, 9u8]]),
        Err(SemanticCandidateRefusal::NotStrictlySmaller {
            position: 0,
            predecessor_bytes: 2,
            candidate_bytes: 2,
        })
    );
    let reducer = SemanticReducerId::named("harness", "duplicate")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let revision = revision_derived_from(b"duplicate");
    assert!(matches!(
        ReductionPlan::declared(
            MinimizationProfile::declared("semantic-reduction", 1),
            ByteReducerId::ChunkRemovalAndZeroing,
            vec![
                SemanticReducerBinding::bound(reducer, revision, semantic_candidates),
                SemanticReducerBinding::bound(reducer, revision, semantic_candidates),
            ],
            FingerprintPreservation::Required,
            ReductionBudget::declared(2),
        ),
        Err(ReductionPlanRefusal::DuplicateSemanticReducer(found)) if found == reducer
    ));
    Ok(())
}

#[test]
fn plan_and_probe_bindings_refuse_the_first_unwarranted_claim() -> Result<(), ReductionRoadFailure>
{
    let reducer = SemanticReducerId::named("harness", "ordered-refusal")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let revision = revision_derived_from(b"ordered-refusal");
    assert!(matches!(
        ReductionPlan::declared(
            MinimizationProfile::declared("ordered-refusal", 1u32),
            ByteReducerId::ChunkRemovalAndZeroing,
            vec![
                SemanticReducerBinding::bound(reducer, revision, semantic_candidates),
                SemanticReducerBinding::bound(reducer, revision, semantic_candidates),
            ],
            FingerprintPreservation::Required,
            ReductionBudget::declared(0u32),
        ),
        Err(ReductionPlanRefusal::ZeroReductionBudget)
    ));

    let Some(report) = trial_report_with(passed_trial) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    assert!(matches!(
        ReductionProbeBinding::bound(
            &report,
            GenerationProfile::declared("passed-probe", 1u32),
            GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"passed")),
            revision,
            probe,
        ),
        Err(ReductionProbeRefusal::TrialPassed)
    ));
    Ok(())
}

#[test]
fn trial_and_probe_revisions_move_independently() -> Result<(), ReductionRoadFailure> {
    let Some(fixture) = trial_fixture() else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let first_trial = revision_derived_from(b"first-trial");
    let second_trial = revision_derived_from(b"second-trial");
    let probe_revision = revision_derived_from(b"one-probe");
    let generation = GenerationProfile::declared("revision-separation", 1);
    let schema = GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"separation"));
    let Some(first) = fixture.probe_binding(
        refused_trial,
        first_trial,
        generation,
        schema,
        probe_revision,
        probe,
    ) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    let Some(second) = fixture.probe_binding(
        refused_trial,
        second_trial,
        generation,
        schema,
        probe_revision,
        probe,
    ) else {
        return Err(ReductionRoadFailure::Fixture);
    };

    assert_eq!(first.revision(), probe_revision);
    assert_eq!(second.revision(), probe_revision);
    assert_ne!(
        first.standing().key().revisions(),
        second.standing().key().revisions()
    );
    assert_eq!(first.preserved(), second.preserved());
    Ok(())
}

#[test]
fn invoked_semantic_reducer_refusal_keeps_reducer_and_cause() -> Result<(), ReductionRoadFailure> {
    let reducer = SemanticReducerId::named("harness", "hostile-candidates")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("hostile-candidates", 1u32),
        ByteReducerId::ChunkRemovalAndZeroing,
        vec![SemanticReducerBinding::bound(
            reducer,
            revision_derived_from(b"hostile-candidates"),
            non_descending_candidates,
        )],
        FingerprintPreservation::Required,
        ReductionBudget::declared(4u32),
    )?;
    let Some(binding) = probe_binding(revision_derived_from(b"probe")) else {
        return Err(ReductionRoadFailure::Fixture);
    };
    assert_eq!(
        reduce(&plan, &[1u8, 2u8, 3u8], &binding),
        Err(ReductionRefusal::SemanticReducerRefused {
            reducer,
            cause: SemanticCandidateRefusal::NotStrictlySmaller {
                position: 0usize,
                predecessor_bytes: 3usize,
                candidate_bytes: 3usize,
            },
        })
    );
    Ok(())
}

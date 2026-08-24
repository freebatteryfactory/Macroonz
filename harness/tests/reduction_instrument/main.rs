//! Bound semantic and generic reducers retain their exact path, preservation evidence, and replay ceiling.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite,
    GeneratedSupportSchemaId, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row,
    SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz_harness::generate::{
    ByteReducerExecution, ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget,
    ReductionHalt, ReductionPlan, ReductionPlanRefusal, ReductionProbeBinding, ReductionRefusal,
    SemanticCandidateRefusal, SemanticCandidates, SemanticReducerBinding, SemanticReducerId,
    ShrinkVerdict, capture_replay, reduce, shrink_verdict,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, ReplayPosture, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialId,
    TrialProfile, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};
use std::fmt;

const PRESERVED_CAUSE: FindingCause = FindingCause::named("harness", "preserved-failure");
const MOVED_CAUSE: FindingCause = FindingCause::named("harness", "moved-failure");
const REVISION_TAG: DomainTag =
    DomainTag::declared("reduction-revision", IdentityProfileVersion::declared(1));
const SCHEMA_TAG: DomainTag =
    DomainTag::declared("reduction-schema", IdentityProfileVersion::declared(1));

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
    let coordinates = TrialCoordinates::over(
        ClaimRef::named("harness", "generic-byte-reduction").ok()?,
        SubjectRoute::named("harness", "byte-input").ok()?,
        CheckRef::named("harness", "fingerprint-preserved").ok()?,
        PopulationRef::named("harness", "reduction-candidates").ok()?,
    );
    let key = TrialKey::over(coordinates);
    let trial = TrialId::of_key(key, TrialProfile::Unprofiled);
    Some(Fingerprint::over(
        trial,
        cause,
        FailureClass::PropertyDisagreement,
    ))
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

fn trial_binding() -> Option<TrialBinding> {
    let subject = SubjectRoute::named("harness", "byte-input").ok()?;
    let check = CheckRef::named("harness", "fingerprint-preserved").ok()?;
    let row = Row::declared(
        ClaimRef::named("harness", "generic-byte-reduction").ok()?,
        ExecutionSuite::named("harness", "reduction").ok()?,
        Classification::authored(
            vec![Role::named("harness", "reduction").ok()?],
            vec![Tag::named("harness", "outside-consumer").ok()?],
        )
        .ok()?,
        subject,
        check,
        PopulationRef::named("harness", "reduction-candidates").ok()?,
        Origin::HandWritten,
    )
    .ok()?;
    let revision = RevisionBinding::derived(ContentAddress::derived(REVISION_TAG, b"trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, refused_trial),
        Provenance::Unproduced,
    )
    .ok()
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("x86_64-pc-windows-msvc"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "reduction"),
        HarnessClock::unavailable(),
    )
}

fn probe_binding(revision: RevisionBinding) -> Option<ReductionProbeBinding> {
    let trial = trial_binding()?;
    let report = run_one(&trial, &invocation());
    ReductionProbeBinding::bound(
        &report,
        GenerationProfile::declared("reduction-input", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        revision,
        probe,
    )
    .ok()
}

fn semantic_candidates(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    match input {
        [1u8, 2u8, 3u8] => SemanticCandidates::proposed(input, vec![vec![1u8, 2u8], vec![1u8]]),
        _ => SemanticCandidates::proposed(input, Vec::new()),
    }
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
    let Some(binding) = probe_binding(RevisionBinding::derived(ContentAddress::derived(
        REVISION_TAG,
        b"probe",
    ))) else {
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
    let reducer = SemanticReducerId::named("harness", "sequence-aware")
        .map_err(|_| ReductionRoadFailure::Fixture)?;
    let derived_revision =
        RevisionBinding::derived(ContentAddress::derived(REVISION_TAG, b"semantic-reducer"));
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
    let Some(binding) = probe_binding(RevisionBinding::derived(ContentAddress::derived(
        REVISION_TAG,
        b"probe",
    ))) else {
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
    let revision = RevisionBinding::derived(ContentAddress::derived(REVISION_TAG, b"duplicate"));
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

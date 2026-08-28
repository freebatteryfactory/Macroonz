//! The example-specific semantic failure used to demonstrate Macroonz reduction and replay.

use macroonz::harness::clock::HarnessClock;
use macroonz::harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite,
    GeneratedSupportSchemaId, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row,
    SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz::harness::fuzz::{InterestingBytes, compose_reduce_replay};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding,
};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, ReplayPosture, TargetBinding,
    TimeBudget, TrialConclusion, TrialFinding, TrialId, TrialProfile, TrialSite,
};
use macroonz::harness::runner::{Invocation, TrialBinding, run_one};

const PRESERVED_CAUSE: FindingCause = FindingCause::named("macroonz.example", "coverage-road");
const SCHEMA_TAG: DomainTag = DomainTag::declared(
    "rustc-coverage-example",
    IdentityProfileVersion::declared(1),
);

pub(super) fn reduce_and_replay(
    interesting: &InterestingBytes,
    target: TargetBinding,
    revision: RevisionBinding,
) -> Result<(), String> {
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("rustc-coverage-example", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(16),
    )
    .map_err(debug)?;
    let binding = probe_binding(target, revision)?;
    let capsule = compose_reduce_replay(interesting, &plan, &binding).map_err(debug)?;
    if capsule.input() == [1] && capsule.posture() == ReplayPosture::ExactDerived {
        Ok(())
    } else {
        Err("reduction did not produce the expected exact replay".to_owned())
    }
}

fn trial_fingerprint() -> Option<Fingerprint> {
    let coordinates = TrialCoordinates::over(
        ClaimRef::named("macroonz.example", "coverage-road").ok()?,
        SubjectRoute::named("macroonz.example", "byte-input").ok()?,
        CheckRef::named("macroonz.example", "fingerprint-preserved").ok()?,
        PopulationRef::named("macroonz.example", "coverage-seeds").ok()?,
    );
    let trial = TrialId::of_key(TrialKey::over(coordinates), TrialProfile::Unprofiled);
    Some(Fingerprint::over(
        trial,
        PRESERVED_CAUSE,
        FailureClass::PropertyDisagreement,
    ))
}

fn probe(input: &[u8]) -> ProbeOutcome {
    let Some(fingerprint) = trial_fingerprint() else {
        return ProbeOutcome::NoFailure;
    };
    match input {
        [1, 2, 3] | [1, 2] | [1] => ProbeOutcome::Reproduced(fingerprint),
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

fn probe_binding(
    target: TargetBinding,
    revision: RevisionBinding,
) -> Result<ReductionProbeBinding, String> {
    let trial = trial_binding(revision)?;
    let report = run_one(&trial, &invocation(target));
    ReductionProbeBinding::bound(
        &report,
        GenerationProfile::declared("rustc-coverage-example", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        revision,
        probe,
    )
    .map_err(debug)
}

fn trial_binding(revision: RevisionBinding) -> Result<TrialBinding, String> {
    let subject = SubjectRoute::named("macroonz.example", "byte-input").map_err(debug)?;
    let check = CheckRef::named("macroonz.example", "fingerprint-preserved").map_err(debug)?;
    let row = Row::declared(
        ClaimRef::named("macroonz.example", "coverage-road").map_err(debug)?,
        ExecutionSuite::named("macroonz.example", "fuzz").map_err(debug)?,
        Classification::authored(
            vec![Role::named("macroonz.example", "fuzz").map_err(debug)?],
            vec![Tag::named("macroonz.example", "coverage").map_err(debug)?],
        )
        .map_err(debug)?,
        subject,
        check,
        PopulationRef::named("macroonz.example", "coverage-seeds").map_err(debug)?,
        Origin::HandWritten,
    )
    .map_err(debug)?;
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, refused_trial),
        Provenance::Unproduced,
    )
    .map_err(debug)
}

fn invocation(target: TargetBinding) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000),
        ),
        target,
        TrialSite::located(module_path!(), file!(), line!(), "rustc-coverage-example"),
        HarnessClock::unavailable(),
    )
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

//! A deliberately wrong independent expectation supplies the reduction control, never a claimed compiler defect.

use super::recipe_observation::{Outcome, observe};
use super::support::{FuzzRoadFailure, external};
use super::trial_fixture::{TrialFixture, synthetic_target};
use macroonz_harness::descriptor::{DerivedRevision, GeneratedSupportSchemaId, RevisionBinding};
use macroonz_harness::fuzz::{InterestingBytes, compose_reduce_replay};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion, encode_bytes};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, GenerationProfile, MinimizationProfile,
    ReplayPosture, TargetBinding, TrialConclusion, TrialFinding, TrialSite,
};
use macroonz_harness::runner::Invocation;
use std::io::Write;

const CAUSE: FindingCause = FindingCause::named("harness", "planted-all-recipes-refuse");

fn fixture(target: TargetBinding) -> Option<TrialFixture> {
    TrialFixture::named(
        "recipe-reduction-control",
        "fuzz",
        "control",
        "planted",
        "recipe-grammar",
        TrialSite::located(module_path!(), file!(), line!(), "recipe-reduction-control"),
        target,
    )
}

fn probe(input: &[u8]) -> ProbeOutcome {
    let observed = observe(input);
    assert!(
        observed.is_ok(),
        "the planted probe found an incomplete compiler output"
    );
    match (observed, fixture(synthetic_target())) {
        (Ok(Outcome::Baked(_)), Some(fixture)) => {
            ProbeOutcome::Reproduced(fixture.fingerprint(CAUSE))
        }
        _ => ProbeOutcome::NoFailure,
    }
}

fn planted_disagreement(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

pub(super) fn reduce_control(
    earned: &InterestingBytes,
    target: TargetBinding,
    revision: RevisionBinding,
) -> Result<(), FuzzRoadFailure> {
    let fixture = fixture(target).ok_or(FuzzRoadFailure::Fixture)?;
    let mut material = Vec::new();
    encode_bytes(revision.revision().as_bytes(), &mut material);
    encode_bytes(include_bytes!("recipe_control.rs"), &mut material);
    let probe_revision = RevisionBinding::derived(DerivedRevision::from_material(&material));
    let schema = GeneratedSupportSchemaId::over(ContentAddress::derived(
        DomainTag::declared(
            "recipe-reduction-control",
            IdentityProfileVersion::declared(1),
        ),
        include_bytes!("recipe_control.rs"),
    ));
    let binding = fixture
        .probe_binding(
            planted_disagreement,
            revision,
            GenerationProfile::declared("recipe-grammar", 1),
            schema,
            probe_revision,
            probe,
        )
        .ok_or(FuzzRoadFailure::Fixture)?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("recipe-planted-control", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(512),
    )
    .map_err(external)?;
    let capsule = compose_reduce_replay(earned, &plan, &binding)?;
    assert!(capsule.input().len() < earned.as_bytes().len());
    assert_eq!(capsule.posture(), ReplayPosture::ExactDerived);
    assert_eq!(
        probe(capsule.input()),
        ProbeOutcome::Reproduced(capsule.fingerprint())
    );
    assert_eq!(probe(b"not a recipe"), ProbeOutcome::NoFailure);
    writeln!(
        std::io::stdout().lock(),
        "Planted reduction only: {} -> {} bytes; replay {:?}",
        earned.as_bytes().len(),
        capsule.input().len(),
        capsule.posture()
    )
    .map_err(external)?;
    Ok(())
}

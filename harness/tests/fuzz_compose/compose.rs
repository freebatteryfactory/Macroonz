//! The composition claims: a malformed road refuses, interesting bytes compose into an exact derived replay, and composition refuses when the seed does not fail.

use super::support::{FuzzRoadFailure, interesting_bytes, probe, probe_binding};
use macroonz_harness::fuzz::{ComposeRefusal, compose_reduce_replay};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionRefusal,
};
use macroonz_harness::report::{MinimizationProfile, ReplayPosture};

#[test]
fn hostile_surface_refuses_malformed_fuzz_road() -> Result<(), FuzzRoadFailure> {
    let interesting = interesting_bytes("hostile-compose", &[9])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose-hostile", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(4),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    match compose_reduce_replay(&interesting, &plan, &binding) {
        Err(ComposeRefusal::Reduction(ReductionRefusal::BaselineDidNotFail)) => Ok(()),
        Err(refusal) => Err(FuzzRoadFailure::Compose(refusal)),
        Ok(_) => Err(FuzzRoadFailure::Fixture),
    }
}

#[test]
fn interesting_bytes_compose_into_exact_derived_replay() -> Result<(), FuzzRoadFailure> {
    let interesting = interesting_bytes("compose-replay", &[1, 2, 3])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(16),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let capsule = compose_reduce_replay(&interesting, &plan, &binding)?;
    assert_eq!(capsule.input(), &[1u8]);
    assert_eq!(capsule.posture(), ReplayPosture::ExactDerived);
    assert_eq!(
        probe(capsule.input()),
        ProbeOutcome::Reproduced(capsule.fingerprint())
    );
    Ok(())
}

#[test]
fn compose_refuses_when_seed_does_not_fail() -> Result<(), FuzzRoadFailure> {
    let interesting = interesting_bytes("compose-refusal", &[9])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(4),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    match compose_reduce_replay(&interesting, &plan, &binding) {
        Err(ComposeRefusal::Reduction(ReductionRefusal::BaselineDidNotFail)) => Ok(()),
        Err(refusal) => Err(FuzzRoadFailure::Compose(refusal)),
        Ok(_) => Err(FuzzRoadFailure::Fixture),
    }
}

//! The generic byte reducer is consumed under an explicitly empty semantic-reducer roster and observed at every preservation outcome.

use std::fmt;
use threadpak_testpak::descriptor::{
    CheckRef, ClaimRef, PopulationRef, SubjectRoute, TrialCoordinates, TrialKey,
};
use threadpak_testpak::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionHalt,
    ReductionPlan, ReductionPlanRefusal, ReductionRefusal, ShrinkVerdict, reduce, shrink_verdict,
};
use threadpak_testpak::report::{
    FailureClass, FindingCause, Fingerprint, MinimizationProfile, TrialId, TrialProfile,
};

const PRESERVED_CAUSE: FindingCause = FindingCause::named("testpak", "preserved-failure");
const MOVED_CAUSE: FindingCause = FindingCause::named("testpak", "moved-failure");

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
        ClaimRef::named("testpak", "generic-byte-reduction").ok()?,
        SubjectRoute::named("testpak", "byte-input").ok()?,
        CheckRef::named("testpak", "fingerprint-preserved").ok()?,
        PopulationRef::named("testpak", "reduction-candidates").ok()?,
    );
    let key = TrialKey::over(coordinates).ok()?;
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
        [1u8, 2u8] | [1u8] => ProbeOutcome::Reproduced(preserved),
        [0u8, 0u8] | [2u8] => ProbeOutcome::Reproduced(moved),
        _ => ProbeOutcome::NoFailure,
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
        &[],
        FingerprintPreservation::Required,
        ReductionBudget::declared(16u32),
    )?;

    assert!(plan.semantic_reducers().is_empty());
    assert_eq!(
        reduce(&plan, &[9u8], preserved, probe),
        Err(ReductionRefusal::BaselineDidNotFail)
    );
    assert_eq!(
        reduce(&plan, &[2u8], preserved, probe),
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

    let reduced = reduce(&plan, &[1u8, 2u8], preserved, probe)?;
    assert_eq!(reduced.input(), &[1u8]);
    assert_eq!(reduced.fingerprint(), preserved);
    assert_eq!(reduced.census().accepted(), 1u32);
    assert_eq!(reduced.census().fingerprint_moved(), 2u32);
    assert_eq!(reduced.census().no_failure(), 4u32);
    assert_eq!(reduced.census().probes(), 7u32);
    assert_eq!(reduced.halt(), ReductionHalt::FixedPointReached);
    Ok(())
}

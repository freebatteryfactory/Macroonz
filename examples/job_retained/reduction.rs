//! Bounded reduction preserves the independently observed Job work-count failure.

use macroonz::harness::descriptor::GeneratedSupportSchemaId;
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, ReductionProbeRefusal, ReductionRefusal, capture_replay, reduce,
};
use macroonz::harness::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, ReplayCapsule, RunAttempt,
    TrialConclusion, TrialReport,
};
use macroonz::workflow::InputRun;

pub(super) fn capsule(original: &InputRun, lawful: &InputRun) -> Result<ReplayCapsule, String> {
    let schema = GeneratedSupportSchemaId::over(
        crate::checks::decoder()
            .map_err(crate::debug)?
            .profile()
            .schema(),
    );
    let before = crate::checks::observations();
    assert!(matches!(
        binding(crate::selected(lawful)?, schema),
        Err(ReductionProbeRefusal::TrialPassed)
    ));
    let binding = binding(crate::selected(original)?, schema).map_err(crate::debug)?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("neutral-work-count", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(32),
    )
    .map_err(crate::debug)?;
    assert!(matches!(
        reduce(&plan, &[1, 2], &binding),
        Err(ReductionRefusal::BaselineCaseDiffers {
            expected: _,
            found: _
        })
    ));
    assert_eq!(crate::checks::observations(), before);
    let evidence = reduce(&plan, original.input().payload(), &binding).map_err(crate::debug)?;
    let capsule = capture_replay(&evidence);
    assert_eq!(capsule.input(), &[4]);
    assert_eq!(
        probe(capsule.input()),
        ProbeOutcome::Reproduced(binding.preserved())
    );
    Ok(capsule)
}

fn binding(
    report: &TrialReport,
    schema: GeneratedSupportSchemaId,
) -> Result<ReductionProbeBinding, ReductionProbeRefusal> {
    ReductionProbeBinding::bound(
        report,
        GenerationProfile::declared("neutral-work-count", 1),
        schema,
        crate::checks::revision(include_bytes!("reduction.rs")),
        probe,
    )
}

fn probe(bytes: &[u8]) -> ProbeOutcome {
    crate::checks::probe();
    let Ok(run) = crate::run(bytes) else {
        return ProbeOutcome::NoFailure;
    };
    let Ok(report) = crate::selected(&run) else {
        return ProbeOutcome::NoFailure;
    };
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            ProbeOutcome::Reproduced(Fingerprint::of(report.trial(), finding))
        }
        RunAttempt::Executed(TrialConclusion::Passed)
        | RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => ProbeOutcome::NoFailure,
    }
}

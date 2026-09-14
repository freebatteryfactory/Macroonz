//! Existing reduction over the actual generated trial and its unchanged independent oracle.

use macroonz::harness::descriptor::GeneratedSupportSchemaId;
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz::harness::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, ReplayCapsule, RunAttempt, TrialConclusion,
};
use macroonz::workflow::InputRun;

pub(super) fn capsule(original: &InputRun) -> Result<ReplayCapsule, String> {
    let binding = ReductionProbeBinding::bound(
        crate::selected(original)?,
        GenerationProfile::declared("counting-input", 1),
        GeneratedSupportSchemaId::over(
            crate::checks::decoder()
                .map_err(crate::debug)?
                .profile()
                .schema(),
        ),
        crate::checks::revision(b"generated-count-trial-probe-v1"),
        probe,
    )
    .map_err(crate::debug)?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("counting-shrink", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(32),
    )
    .map_err(crate::debug)?;
    let evidence = reduce(&plan, original.input().payload(), &binding).map_err(crate::debug)?;
    Ok(capture_replay(&evidence))
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

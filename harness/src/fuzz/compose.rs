//! Pure Macroonz composition over coverage-admitted bytes.

use super::{ComposeRefusal, InterestingBytes};
use crate::generate::{ProbeOutcome, ReductionPlan, ReductionProbeBinding, capture_replay, reduce};
use crate::report::ReplayCapsule;

/// Reduce interesting bytes under a Macroonz probe binding and mint a replay capsule.
///
/// # Errors
///
/// Returns [`ComposeRefusal`] when reduction refuses the seed or replay does not preserve the fingerprint.
pub fn compose_reduce_replay(
    interesting: &InterestingBytes,
    plan: &ReductionPlan,
    binding: &ReductionProbeBinding,
) -> Result<ReplayCapsule, ComposeRefusal> {
    let evidence =
        reduce(plan, interesting.as_bytes(), binding).map_err(ComposeRefusal::Reduction)?;
    let capsule = capture_replay(&evidence);
    match (binding.probe())(capsule.input()) {
        ProbeOutcome::Reproduced(fp) if fp == capsule.fingerprint() => Ok(capsule),
        ProbeOutcome::Reproduced(_) => Err(ComposeRefusal::ReplayFingerprintMoved),
        ProbeOutcome::NoFailure => Err(ComposeRefusal::ReplayNoFailure),
    }
}

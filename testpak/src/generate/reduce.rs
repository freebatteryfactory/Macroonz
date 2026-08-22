//! Fingerprint-preserving minimization: the generic byte reducer, and the one
//! law every reducer admits a candidate under.
//!
//! Minimization preserves the fingerprint. A shrunk input must carry the same
//! failure fingerprint or the shrink is rejected — no minimizing into a
//! different bug — and this file is where that law is realized rather than
//! restated: [`shrink_verdict`] is the whole of it, the byte passes below call
//! it, and a semantic reducer an owner writes calls the same function.
//!
//! # The reducer
//!
//! Two byte-level passes at halving window widths — remove a window, then zero
//! a window — repeated until a whole round admits nothing or the probe budget
//! is spent. That pair is the realization of
//! [`ByteReducerId::ChunkRemovalAndZeroing`](super::types::ByteReducerId::ChunkRemovalAndZeroing),
//! the sole arm of a closed roster, which is why there is no dispatch here: no
//! second generic reducer exists to dispatch to.

use super::types::{
    FingerprintProbe, ProbeOutcome, ReductionCensus, ReductionHalt, ReductionOutcome,
    ReductionPlan, ReductionRefusal, ShrinkVerdict,
};
use crate::report::Fingerprint;

/// What one offered candidate, or one whole pass, moved.
///
/// Private to this algorithm: the three answers exist so a pass and the round
/// above it read the same way.
enum Step {
    /// A candidate was admitted, so the best input moved.
    Admitted,
    /// Nothing was admitted, so the best input is where it was.
    Refused,
    /// The probe budget is spent and nothing further may be offered.
    Spent,
}

/// One reduction in flight: the best input so far, its accounting, what remains
/// of the budget, and the law every candidate is judged by.
struct Reduction {
    best: Vec<u8>,
    census: ReductionCensus,
    probes_left: u32,
    preserved: Fingerprint,
    probe: FingerprintProbe,
}

impl Reduction {
    /// Offer one candidate, admitting it only if it carries the fingerprint
    /// through.
    ///
    /// The verdict is counted before it is acted on, so a rejected shrink is on
    /// the record whether or not the reduction went anywhere afterwards.
    fn offer(&mut self, candidate: Vec<u8>) -> Step {
        if self.probes_left == 0 {
            return Step::Spent;
        }
        self.probes_left = self.probes_left.saturating_sub(1);
        let verdict = shrink_verdict(self.preserved, &candidate, self.probe);
        self.census.count(verdict);
        match verdict {
            ShrinkVerdict::Accepted => {
                self.best = candidate;
                Step::Admitted
            }
            ShrinkVerdict::RejectedFingerprintMoved { found: _ }
            | ShrinkVerdict::RejectedNoFailure => Step::Refused,
        }
    }
}

/// Whether one candidate input may stand in for the input it shrinks.
///
/// # Authority
///
/// This is the whole of the preservation law, and it is deliberately one
/// function: the generic byte passes in this file and any semantic reducer an
/// owner writes admit a candidate on exactly the same ground, so "the shrink
/// preserved the fingerprint" means one thing everywhere in the harness.
///
/// A candidate that reaches a DIFFERENT fingerprint is refused and the
/// fingerprint it reached is carried out, so a reader sees where the reduction
/// declined to wander to.
///
/// # Nonclaims
///
/// It says the candidate carries the same fingerprint. It does not say the
/// candidate is smaller — deciding what "smaller" means is the reducer's, and
/// this law would admit a larger candidate just as readily.
#[must_use]
pub fn shrink_verdict(
    preserved: Fingerprint,
    candidate: &[u8],
    probe: FingerprintProbe,
) -> ShrinkVerdict {
    match probe(candidate) {
        ProbeOutcome::NoFailure => ShrinkVerdict::RejectedNoFailure,
        ProbeOutcome::Reproduced(found) => {
            if found == preserved {
                ShrinkVerdict::Accepted
            } else {
                ShrinkVerdict::RejectedFingerprintMoved { found }
            }
        }
    }
}

/// Minimize one failing input while preserving its failure fingerprint.
///
/// # Authority
///
/// The first act is the BASELINE probe: the input handed in must still fail,
/// and must fail under the fingerprint it was told to preserve. A reduction
/// that skipped that step could shrink a passing input forever, or minimize a
/// find into a bug it was never asked about — so both are refusals rather than
/// outcomes.
///
/// Every candidate afterwards is admitted by [`shrink_verdict`] and by nothing
/// else, and every candidate is counted: the census reports how many shrinks
/// were refused because the fingerprint moved, which is the evidence that
/// minimization stayed on the bug it started from.
///
/// The reducer's candidate order is deterministic from the plan and input. The caller-supplied probe is a capture-free function pointer, not a purity proof; for the same sequence of probe outcomes, this operation produces the same reduction outcome.
///
/// # Bounds
///
/// The reduction budget bounds candidate probes, not rounds, and the baseline
/// probe does not spend it. Termination stands on two independent facts: every
/// round that admits anything spends at least one probe from a finite budget,
/// and each pass either strictly shortens the best input or strictly advances
/// its own offset.
///
/// # Errors
///
/// Refuses an input that does not fail, then an input that fails under a
/// different fingerprint than the one it was told to preserve.
///
/// # Nonclaims
///
/// It runs the generic byte reducer and nothing else. The semantic-reducer identifiers a plan carries are not callable bindings, so this operation cannot claim those passes ran. A future run-bound replay account must stand on actual reducer custody rather than infer it from this roster.
pub fn reduce(
    plan: &ReductionPlan,
    input: &[u8],
    preserved: Fingerprint,
    probe: FingerprintProbe,
) -> Result<ReductionOutcome, ReductionRefusal> {
    let ProbeOutcome::Reproduced(baseline) = probe(input) else {
        return Err(ReductionRefusal::BaselineDidNotFail);
    };
    if baseline != preserved {
        return Err(ReductionRefusal::BaselineFingerprintDiffers { found: baseline });
    }

    let mut state = Reduction {
        best: input.to_vec(),
        census: ReductionCensus::opening(),
        probes_left: plan.budget().probes(),
        preserved,
        probe,
    };
    let halt = loop {
        match round(&mut state) {
            Step::Spent => break ReductionHalt::BudgetExhausted,
            Step::Refused => break ReductionHalt::FixedPointReached,
            Step::Admitted => {}
        }
    };
    Ok(ReductionOutcome::reduced(
        state.best,
        preserved,
        state.census,
        halt,
    ))
}

/// One round: both passes at every window width, from the whole input down to a
/// single byte.
fn round(state: &mut Reduction) -> Step {
    let mut progress = Step::Refused;
    let mut window = state.best.len();
    while window > 0 {
        match removal_pass(state, window) {
            Step::Spent => return Step::Spent,
            Step::Admitted => progress = Step::Admitted,
            Step::Refused => {}
        }
        match zeroing_pass(state, window) {
            Step::Spent => return Step::Spent,
            Step::Admitted => progress = Step::Admitted,
            Step::Refused => {}
        }
        window = halved(window);
    }
    progress
}

/// Remove one window at a time, keeping every removal the fingerprint survives.
///
/// An admitted removal leaves the offset where it is, because the best input
/// just got shorter and the next window starts at the same place.
fn removal_pass(state: &mut Reduction, window: usize) -> Step {
    let mut progress = Step::Refused;
    let mut offset = 0usize;
    while offset < state.best.len() {
        let candidate = without(&state.best, offset, window);
        match state.offer(candidate) {
            Step::Spent => return Step::Spent,
            Step::Admitted => progress = Step::Admitted,
            Step::Refused => offset = offset.saturating_add(window),
        }
    }
    progress
}

/// Zero one window at a time, keeping every zeroing the fingerprint survives.
///
/// A window already all zeros is skipped rather than probed: offering it would
/// spend a probe on the input the reduction already holds.
fn zeroing_pass(state: &mut Reduction, window: usize) -> Step {
    let mut progress = Step::Refused;
    let mut offset = 0usize;
    while offset < state.best.len() {
        if all_zero(&state.best, offset, window) {
            offset = offset.saturating_add(window);
            continue;
        }
        let candidate = zeroed(&state.best, offset, window);
        match state.offer(candidate) {
            Step::Spent => return Step::Spent,
            Step::Admitted => progress = Step::Admitted,
            Step::Refused => {}
        }
        offset = offset.saturating_add(window);
    }
    progress
}

/// The next window width down, ending at zero.
fn halved(window: usize) -> usize {
    window.checked_div(2usize).unwrap_or(0usize)
}

/// The input with one window removed.
fn without(bytes: &[u8], offset: usize, window: usize) -> Vec<u8> {
    let end = offset.saturating_add(window);
    bytes
        .iter()
        .copied()
        .take(offset)
        .chain(bytes.iter().copied().skip(end))
        .collect()
}

/// The input with one window zeroed.
fn zeroed(bytes: &[u8], offset: usize, window: usize) -> Vec<u8> {
    let mut candidate = bytes.to_vec();
    for byte in candidate.iter_mut().skip(offset).take(window) {
        *byte = 0u8;
    }
    candidate
}

/// Whether one window is already all zeros.
fn all_zero(bytes: &[u8], offset: usize, window: usize) -> bool {
    bytes
        .iter()
        .skip(offset)
        .take(window)
        .all(|byte| *byte == 0u8)
}

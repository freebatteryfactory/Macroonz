//! Fingerprint-preserving minimization: the law a shrink is admitted under, the reducers that propose one, and the replay account a finished reduction becomes.
//!
//! A shrunk input must carry the same failure fingerprint or the shrink is rejected — no minimizing into a different bug.
//! [`shrink_verdict`] is the whole of that law, and semantic candidates and byte passes alike reach it through one reduction state.
//!
//! The generic reducer is two byte-level passes at halving window widths — remove a window, then zero a window — repeated until a whole round admits nothing or the probe budget is spent.
//! That pair is the realization of [`ByteReducerId::ChunkRemovalAndZeroing`](super::types::ByteReducerId::ChunkRemovalAndZeroing), the sole arm of a closed roster, which is why there is no dispatch here.

use super::types::{
    ByteReducerExecution, FingerprintProbe, ProbeOutcome, ReductionCensus, ReductionEvidence,
    ReductionHalt, ReductionOutcome, ReductionPlan, ReductionProbeBinding, ReductionRefusal,
    SemanticReducerExecution, ShrinkVerdict,
};
use crate::report::{Fingerprint, ReplayCapsule, ReplayPosture};

/// What one offered candidate, or one whole pass, moved.
enum Step {
    /// A candidate was admitted, so the best input moved.
    Admitted,
    /// Nothing was admitted, so the best input is where it was.
    Refused,
    /// The probe budget is spent and nothing further may be offered.
    Spent,
}

/// One reduction in flight: the best input so far, its accounting, what remains of the budget, and the law every candidate is judged by.
struct Reduction {
    best: Vec<u8>,
    census: ReductionCensus,
    probes_left: u32,
    preserved: Fingerprint,
    probe: FingerprintProbe,
}

impl Reduction {
    /// Offer one candidate, admitting it only if it carries the fingerprint through.
    ///
    /// The verdict is counted before it is acted on, so a rejected shrink is on the record whether or not the reduction went anywhere afterwards.
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

/// What the probe budget has left after a phase.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Budget {
    /// Probes remain, so the next phase may still offer candidates.
    Standing,
    /// Nothing further may be offered.
    Spent,
}

/// What the semantic phase left behind: the invocations to retain, the replay ceiling they lowered it to, and what the budget stands at.
struct SemanticPhase {
    path: Vec<SemanticReducerExecution>,
    replay: ReplayPosture,
    budget: Budget,
}

/// What one reducer's candidate sequence cost.
struct Offered {
    probes: usize,
    budget: Budget,
}

/// Whether one candidate input may stand in for the input it shrinks.
///
/// This is the whole of the preservation law, and it is deliberately one function: semantic and generic candidates are admitted on exactly the same ground, so a preserved fingerprint means one thing everywhere.
/// A candidate that reaches a different fingerprint is refused and the fingerprint it reached is carried out, so a reader sees where the reduction declined to wander to.
///
/// It says the candidate carries the same fingerprint.
/// It does not say the candidate is smaller — what smaller means is the reducer's, and this law would admit a larger candidate just as readily.
#[must_use]
pub fn shrink_verdict(
    preserved: Fingerprint,
    candidate: &[u8],
    probe: FingerprintProbe,
) -> ShrinkVerdict {
    match probe(candidate) {
        ProbeOutcome::NoFailure => ShrinkVerdict::RejectedNoFailure,
        ProbeOutcome::Reproduced(found) if found == preserved => ShrinkVerdict::Accepted,
        ProbeOutcome::Reproduced(found) => ShrinkVerdict::RejectedFingerprintMoved { found },
    }
}

/// Minimize one failing input under one report-bound probe while preserving its fingerprint.
///
/// The first act is the baseline probe: the input handed in must still fail, and must fail under the fingerprint it was told to preserve.
/// Every candidate afterwards is admitted by [`shrink_verdict`] and by nothing else, and every candidate is counted.
///
/// Semantic reducers run first, in the plan's authored order, and the generic byte reducer follows when they leave probe budget.
/// The caller-supplied functions are capture-free pointers, not purity proofs; for the same candidate sequences and probe outcomes, this operation produces the same evidence.
///
/// # Bounds
///
/// The reduction budget bounds candidate probes rather than rounds, and the baseline probe does not spend it.
/// Termination stands on two independent facts: every round that admits anything spends at least one probe from a finite budget, and each pass either strictly shortens the best input or strictly advances its own offset.
///
/// # Errors
///
/// Refuses an input that does not fail, then an input that fails under a different fingerprint than the report-derived one, then an invoked semantic reducer whose typed candidate sequence refuses.
pub fn reduce(
    plan: &ReductionPlan,
    input: &[u8],
    binding: &ReductionProbeBinding,
) -> Result<ReductionEvidence, ReductionRefusal> {
    let probe = binding.probe();
    let preserved = binding.preserved();
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

    let semantic = semantic_phase(plan, &mut state, binding.replay_posture())?;
    let (halt, byte_reducer) = match semantic.budget {
        Budget::Spent => (
            ReductionHalt::BudgetExhausted,
            ByteReducerExecution::NotReachedBecauseBudgetSpent,
        ),
        Budget::Standing => (
            byte_passes(&mut state),
            ByteReducerExecution::Executed(plan.byte_reducer()),
        ),
    };

    let outcome = ReductionOutcome::reduced(state.best, preserved, state.census, halt);
    Ok(ReductionEvidence::recorded(
        binding,
        plan.profile(),
        semantic.path,
        byte_reducer,
        outcome,
        semantic.replay,
    ))
}

/// Capture one run-bound replay account from completed reduction evidence.
///
/// No input, execution key, profile, schema, or posture is accepted beside the evidence.
/// The capsule's posture is the meet already computed across the report attachment, the reduction probe, and every semantic reducer actually invoked.
/// Holding the returned capsule is still not human admission.
#[must_use]
pub fn capture_replay(evidence: &ReductionEvidence) -> ReplayCapsule {
    ReplayCapsule::captured(
        evidence.standing(),
        evidence.outcome().input(),
        evidence.outcome().fingerprint(),
        evidence.generation(),
        evidence.minimization(),
        evidence.schema(),
        evidence.replay_posture(),
    )
}

/// Offer every bound semantic reducer's candidates, in the plan's authored order.
///
/// A binding is invoked over the current best input, and only an invoked binding lowers the replay ceiling or reaches the retained path.
fn semantic_phase(
    plan: &ReductionPlan,
    state: &mut Reduction,
    opening: ReplayPosture,
) -> Result<SemanticPhase, ReductionRefusal> {
    let mut phase = SemanticPhase {
        path: Vec::new(),
        replay: opening,
        budget: Budget::Standing,
    };
    for binding in plan.semantic_reducers() {
        if state.probes_left == 0 {
            phase.budget = Budget::Spent;
            break;
        }
        let candidates = binding.call(&state.best).map_err(|cause| {
            ReductionRefusal::SemanticReducerRefused {
                reducer: binding.reducer(),
                cause,
            }
        })?;
        phase.replay = phase.replay.meet_revision(binding.revision().posture());
        let authored = candidates.candidates().len();
        let offered = offer_each(state, candidates.into_candidates());
        phase.path.push(SemanticReducerExecution::recorded(
            *binding,
            authored,
            offered.probes,
        ));
        if state.probes_left == 0 || offered.budget == Budget::Spent {
            phase.budget = Budget::Spent;
            break;
        }
    }
    Ok(phase)
}

/// Offer one reducer's candidates in order, stopping where the budget does.
fn offer_each(state: &mut Reduction, candidates: Vec<Vec<u8>>) -> Offered {
    let mut offered = Offered {
        probes: 0usize,
        budget: Budget::Standing,
    };
    for candidate in candidates {
        match state.offer(candidate) {
            Step::Admitted | Step::Refused => offered.probes = offered.probes.saturating_add(1),
            Step::Spent => {
                offered.budget = Budget::Spent;
                break;
            }
        }
    }
    offered
}

/// Run the generic reducer until a round admits nothing or the probe budget is spent.
fn byte_passes(state: &mut Reduction) -> ReductionHalt {
    loop {
        match round(state) {
            Step::Spent => return ReductionHalt::BudgetExhausted,
            Step::Refused => return ReductionHalt::FixedPointReached,
            Step::Admitted => {}
        }
    }
}

/// One round: both passes at every window width, from the whole input down to a single byte.
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
/// An admitted removal leaves the offset where it is, because the best input just got shorter and the next window starts at the same place.
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
/// A window already all zeros is skipped rather than probed: offering it would spend a probe on the input the reduction already holds.
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

//! Candidate proving cannot mix the parent, candidate and invocation's decoded types.

use macroonz_harness::input::BoundInput;
use macroonz_harness::muterprater::MutationTarget;
use macroonz_harness::muterprater::propose::prove_candidate;
use macroonz_harness::runner::{Invocation, TrialBinding, TrialTable};

fn wrong_candidate(
    parent: &TrialTable<BoundInput<u8>>,
    candidate: TrialBinding<BoundInput<u16>>,
    target: &MutationTarget,
    invocation: &Invocation<BoundInput<u8>>,
) {
    let _refused = prove_candidate(parent, candidate, target, invocation);
}

fn wrong_invocation(
    parent: &TrialTable<BoundInput<u8>>,
    candidate: TrialBinding<BoundInput<u8>>,
    target: &MutationTarget,
    invocation: &Invocation<BoundInput<u16>>,
) {
    let _refused = prove_candidate(parent, candidate, target, invocation);
}

fn main() {}

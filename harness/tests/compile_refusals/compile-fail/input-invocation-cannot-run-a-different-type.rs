//! A bound trial cannot receive another decoded input type.

use macroonz_harness::input::BoundInput;
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};

fn bypass(binding: TrialBinding<BoundInput<u16>>, invocation: Invocation<BoundInput<u8>>) {
    let _wrong_type = run_one(&binding, &invocation);
}

fn main() {}

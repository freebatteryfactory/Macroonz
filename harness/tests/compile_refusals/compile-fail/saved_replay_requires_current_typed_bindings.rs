//! Saved bytes cannot execute against a differently typed current binding.

use macroonz_harness::input::{BoundInput, InputBinding, InputLimits};
use macroonz_harness::report::archive::ArchivedCapsule;
use macroonz_harness::runner::{Invocation, TrialBinding, replay};

fn bypass(historical: &ArchivedCapsule, binding: &TrialBinding<BoundInput<u16>>, decoder: &InputBinding<u8>, invocation: Invocation) {
    let _wrong_type = replay(historical, binding, decoder, invocation, InputLimits::declared(4096, 64));
}

fn main() {}

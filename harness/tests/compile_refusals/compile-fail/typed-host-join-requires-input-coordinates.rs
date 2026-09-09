//! A unit host observation cannot enter the typed host join.

use macroonz_harness::input::BoundInput;
use macroonz_harness::report::HostTrialRecord;
use macroonz_harness::runner::{Invocation, TrialBinding, record_input_one};

fn bypass(binding: TrialBinding<BoundInput<u8>>, invocation: Invocation<BoundInput<u8>>, record: HostTrialRecord) {
    let _unjoined = record_input_one(&binding, &invocation, record);
}

fn main() {}

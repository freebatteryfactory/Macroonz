//! A host record naming only a trial cannot establish input-bearing execution.

use macroonz_harness::input::BoundInput;
use macroonz_harness::report::HostTrialRecord;
use macroonz_harness::runner::{Invocation, TrialBinding, record_one};

fn bypass(binding: TrialBinding<BoundInput<u8>>, invocation: Invocation<BoundInput<u8>>, record: HostTrialRecord) {
    let _unjoined = record_one(&binding, &invocation, record);
}

fn main() {}

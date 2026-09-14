//! Cloning a typed value cannot retain an envelope beside a different decoder result.

use macroonz_harness::input::BoundInput;
use macroonz_harness::runner::Invocation;

fn bypass(invocation: Invocation<BoundInput<u8>>) {
    let _other = invocation.clone();
}

fn main() {}

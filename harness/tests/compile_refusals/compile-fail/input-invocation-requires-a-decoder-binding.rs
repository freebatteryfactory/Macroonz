//! A raw value cannot enter typed execution without admitted bytes and a decoder binding.

use macroonz_harness::runner::Invocation;

fn bypass(invocation: Invocation) {
    let _unbound = invocation.with_input(7u8);
}

fn main() {}

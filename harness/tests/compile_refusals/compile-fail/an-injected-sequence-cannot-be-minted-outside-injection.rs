//! Claim: An injected sequence can be minted only after one selected schedule has been placed completely.
//!
//! Subject: The private injected-sequence seats at the public crate boundary.
//! Population: One lawful schedule name and an empty command roster offered through a direct struct literal.
//! Hostile control: The fixture supplies both fields with their correct types while bypassing `fault::inject`.
//! Denominator: Both private fields whose construction records one complete placement result.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: Either field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::descriptor::NamespacedName;
use macroonz_harness::fault::InjectedSequence;

fn bypass_injection(schedule: NamespacedName) -> InjectedSequence<u8, (), ()> {
    InjectedSequence {
        schedule,
        commands: Vec::new(),
    }
}

fn main() {}

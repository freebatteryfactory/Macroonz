//! Claim: An injected command can be minted only by the operation that seats scheduled adapters beside it.
//!
//! Subject: The private injected-command seats at the public crate boundary.
//! Population: One command and an empty adapter roster offered through a direct struct literal.
//! Hostile control: The fixture supplies both fields with their correct types while bypassing `fault::inject`.
//! Denominator: Both private fields whose construction records one admitted placement result.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: Either field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::fault::InjectedCommand;

fn bypass_injection() -> InjectedCommand<u8, (), ()> {
    InjectedCommand {
        command: 1u8,
        faults: Vec::new(),
    }
}

fn main() {}

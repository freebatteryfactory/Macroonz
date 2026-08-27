//! Claim: A compiled read-back roster can be minted only by the constructor that refuses duplicate declarations.
//!
//! Subject: The private compiled declaration roster at the public crate boundary.
//! Population: One borrowed read-back slice offered through a direct struct literal.
//! Hostile control: The fixture supplies the field with its exact type while bypassing `DeclaredReadBackRoster::declared`.
//! Denominator: The only private field whose construction establishes duplicate-free compiled membership.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: The roster field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::oracle::{DeclaredReadBack, DeclaredReadBackRoster};

fn bypass_declaration<'spec>(
    members: &'spec [DeclaredReadBack<'spec>],
) -> DeclaredReadBackRoster<'spec> {
    DeclaredReadBackRoster { members }
}

fn main() {}

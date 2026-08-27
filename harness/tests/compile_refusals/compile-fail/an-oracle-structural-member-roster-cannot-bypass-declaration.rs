//! Claim: A structural member roster can be minted only by the constructor that refuses duplicate declarations.
//!
//! Subject: The private structural declaration roster at the public crate boundary.
//! Population: One borrowed declaration slice offered through a direct struct literal.
//! Hostile control: The fixture supplies the field with its exact type while bypassing `DeclaredMemberRoster::declared`.
//! Denominator: The only private field whose construction establishes duplicate-free structural membership.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: The roster field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::oracle::{DeclaredMember, DeclaredMemberRoster};

fn bypass_declaration<'spec>(
    members: &'spec [DeclaredMember<'spec>],
) -> DeclaredMemberRoster<'spec> {
    DeclaredMemberRoster { members }
}

fn main() {}

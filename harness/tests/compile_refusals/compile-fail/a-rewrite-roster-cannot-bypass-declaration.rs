//! Claim: A rewrite roster can be minted only by the constructor that refuses empty and duplicate declarations.
//!
//! Subject: The private descriptor roster at the public crate boundary.
//! Population: One structurally invalid empty roster offered through a direct struct literal.
//! Hostile control: The fixture supplies the field with its correct type while bypassing `RewriteRoster::declared`.
//! Denominator: The only private field whose construction establishes roster admission.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: The roster field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::muterprater::RewriteRoster;

fn bypass_declaration() -> RewriteRoster {
    RewriteRoster {
        descriptors: Vec::new(),
    }
}

fn main() {}

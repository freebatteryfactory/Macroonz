//! Claim: A structural path segment can be minted only by the path constructor that refuses embedded separators.
//!
//! Subject: The private structural path segment mint at the public crate boundary.
//! Population: One segment spelling offered directly through the tuple constructor.
//! Hostile control: The fixture supplies the private field's exact owned-string type while bypassing `StructuralPath::relative` and `StructuralPath::absolute`.
//! Denominator: The only private field whose construction establishes that a segment is indivisible.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: The segment field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::oracle::StructuralPathSegment;

fn bypass_informing(spelling: String) -> StructuralPathSegment {
    StructuralPathSegment(spelling)
}

fn main() {}

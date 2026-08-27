//! Claim: A structural path can be minted only after its root and every indivisible segment have been informed together.
//!
//! Subject: Both private structural path seats at the public crate boundary.
//! Population: One lawful root and an already informed segment roster offered through a direct struct literal.
//! Hostile control: The fixture receives both fields at their exact public types while bypassing `StructuralPath::relative` and `StructuralPath::absolute`.
//! Denominator: Both private fields whose construction establishes one complete path.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: Either path field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::oracle::{StructuralPath, StructuralPathRoot, StructuralPathSegment};

fn bypass_informing(
    root: StructuralPathRoot,
    segments: Vec<StructuralPathSegment>,
) -> StructuralPath {
    StructuralPath { root, segments }
}

fn main() {}

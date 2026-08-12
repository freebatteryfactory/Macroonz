//! The reversal for the root calculus's limit admission: a declared magnitude
//! that bounds nothing does not get admitted.
//!
//! `Limit` and `ConstLimit` are extension points, so the number a family
//! declares is whatever its author wrote. The admission witness is what a
//! declaration must pass through before a road treats it as a fact, and this
//! file is the proof that the check is real rather than decorative: a family
//! declaring a magnitude past the workspace ceiling stops the compiler during
//! const evaluation, so no artifact carrying it is ever produced.

use threadpak::types::{AdmittedLimit, ConstLimit, Limit, WORKSPACE_LIMIT_CEILING};

struct PastTheCeiling;

impl Limit for PastTheCeiling {}

impl ConstLimit for PastTheCeiling {
    const MAX: usize = WORKSPACE_LIMIT_CEILING + 1;
}

const REFUSED: AdmittedLimit<PastTheCeiling> = AdmittedLimit::under_ceiling();

fn main() {
    let _ = REFUSED.max();
}

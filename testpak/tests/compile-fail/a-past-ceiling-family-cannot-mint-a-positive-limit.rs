//! The reversal for witness COMPOSITION: the stronger witness refuses a
//! past-ceiling family with the BASE witness's own diagnostic.
//!
//! `PositiveLimit` claims two facts — the family stands under the admitting
//! plane's ceiling, and the family admits an item — and only one of them is its
//! own. The ceiling fact belongs to `AdmittedLimit`, and the stronger witness
//! carries that witness rather than restating its comparison. This file is what
//! makes that composition falsifiable: a family past the ceiling stops the
//! compiler at the STRONGER mint, and the message it stops with is the base
//! mint's, because the base mint is what ran.
//!
//! Restating the ceiling assertion inside the positive road would keep this file
//! failing to compile — and would say nothing about whether the two roads still
//! agree, which is the defect the composition closes.

use threadpak::types::{ConstLimit, Limit, LimitAdmissionProfile, PositiveLimit};

/// The qualification plane's own admitting ceiling, declared here because this
/// is the plane doing the admitting.
struct QualificationProfile;

impl LimitAdmissionProfile for QualificationProfile {
    const MAX_DECLARED_LIMIT: usize = 64;
}

/// A family that admits an item — so positivity is satisfied and the ceiling is
/// the only fact left to fail — and declares a magnitude past the ceiling.
struct PastTheCeiling;

impl Limit for PastTheCeiling {}

impl ConstLimit for PastTheCeiling {
    const MAX: usize = 65;
}

/// Taken in a constant so the family's declared maximum is read while the
/// declaration is still being checked rather than only when an artifact is
/// emitted.
const REFUSED: PositiveLimit<PastTheCeiling, QualificationProfile> =
    PositiveLimit::inhabited_under_profile();

fn main() {
    let _ = REFUSED.max();
}

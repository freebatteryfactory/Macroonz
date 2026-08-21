//! The reversal for profile-scoped limit admission: a declared magnitude past
//! the admitting plane's ceiling does not get admitted.
//!
//! `Limit` and `ConstLimit` are extension points, so the number a family
//! declares is whatever its author wrote. The machine owns the admission-witness
//! algebra and declares no ceiling of its own; the ceiling belongs to the plane
//! doing the admitting, and the authoring plane wrote its own down. This file is
//! the proof that the comparison is real rather than decorative: a family
//! declaring a magnitude past the AUTHORING profile's ceiling stops the compiler
//! during const evaluation, so no artifact carrying it is ever produced.

use threadpak::types::{
    AdmittedLimit, ConstLimit, DeclaredMagnitude, Limit, LimitAdmissionProfile,
};
use threadpak_macroc::AuthoringLimitProfile;

struct PastTheCeiling;

impl Limit for PastTheCeiling {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for PastTheCeiling {
    const MAX: usize = AuthoringLimitProfile::MAX_DECLARED_LIMIT + 1;
}

const REFUSED: AdmittedLimit<PastTheCeiling, AuthoringLimitProfile> =
    AdmittedLimit::under_profile();

fn main() {
    let _ = REFUSED.max();
}

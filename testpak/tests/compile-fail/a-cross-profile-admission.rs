//! The reversal for the profile tag: one plane's admission is not another's.
//!
//! The machine owns the admission-witness algebra and no ceiling; each plane
//! writes its own ceiling down and admits its own families against it. That
//! division is only real if the witness says WHICH plane admitted the family —
//! otherwise a magnitude admitted under a wide ceiling would satisfy a seat that
//! declared a narrow one, and the narrow declaration would be a comment.
//!
//! The profile rides as a type parameter, so the two admissions are different
//! types whatever their magnitudes and no coercion joins them. The family below
//! is small enough for both planes; what refuses is not the number but the
//! claim, and the claim is the whole content of the witness.

use threadpak::types::{
    AdmittedLimit, ConstLimit, DeclaredMagnitude, Limit, LimitAdmissionProfile,
};
use threadpak_macroc::AuthoringLimitProfile;

/// A second plane's ceiling, declared here because this file is the plane
/// declaring it.
struct ForeignProfile;

impl LimitAdmissionProfile for ForeignProfile {
    const MAX_DECLARED_LIMIT: usize = 64;
}

/// A family well inside both ceilings.
struct SmallFamily;

impl Limit for SmallFamily {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SmallFamily {
    const MAX: usize = 8;
}

/// The family, admitted under the foreign plane. Lawful on its own terms.
const ELSEWHERE: AdmittedLimit<SmallFamily, ForeignProfile> = AdmittedLimit::under_profile();

/// A seat that has decided which plane's admission it will act on.
fn seats_an_authoring_admission(_: &AdmittedLimit<SmallFamily, AuthoringLimitProfile>) {}

fn main() {
    seats_an_authoring_admission(&ELSEWHERE);
}

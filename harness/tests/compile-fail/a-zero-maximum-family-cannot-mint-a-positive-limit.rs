//! The reversal for the witness split: the STRONGER witness refuses the family
//! the weaker one admits.
//!
//! A family declaring `MAX = 0` is a lawful declaration for a seat that holds
//! nothing, and `AdmittedLimit` admits it on purpose — the empty-only bound is a
//! real bound, and `Bounded::empty` under it is an honest empty collection. What
//! the same family can never satisfy is a road that PROMISES AN INHABITANT, and
//! `PositiveLimit` is the evidence those roads demand.
//!
//! So the split is only honest if the strong mint actually refuses here. The
//! same family is admitted one line above and refused one line below; if
//! positivity had been folded into the base witness, the first line would have
//! failed too and the empty-only seat would have been refused with it.

use macroonz::{
    AdmittedLimit, Bounded, ConstLimit, DeclaredMagnitude, Limit, LimitAdmissionProfile,
    PositiveLimit,
};

/// The qualification plane's own admitting ceiling, declared here because this
/// is the plane doing the admitting. It is deliberately narrow: nothing under it
/// is close to it, so the number under judgement below is the family's zero and
/// never the ceiling.
struct QualificationProfile;

impl LimitAdmissionProfile for QualificationProfile {
    const MAX_DECLARED_LIMIT: usize = 64;
}

/// A limit family admitting no item at all.
struct NoItemAtAll;

impl Limit for NoItemAtAll {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for NoItemAtAll {
    const MAX: usize = 0;
}

/// The lawful half, and it must stay lawful: the weak witness admits the family,
/// and the empty-only seat under it exists.
const ADMITTED: AdmittedLimit<NoItemAtAll, QualificationProfile> = AdmittedLimit::under_profile();

/// The unlawful half, taken in a constant so the family's declared maximum is
/// read while the declaration is still being checked rather than only when an
/// artifact is emitted.
const REFUSED: PositiveLimit<NoItemAtAll, QualificationProfile> =
    PositiveLimit::inhabited_under_profile();

fn main() {
    let nothing: Bounded<u8, NoItemAtAll> = Bounded::empty();
    let _ = nothing.len();
    let _ = ADMITTED.max();
    let _ = REFUSED.max();
}

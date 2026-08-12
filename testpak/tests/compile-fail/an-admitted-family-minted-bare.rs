//! The reversal for band 00's family admission: the witness cannot be written
//! down, only earned.
//!
//! A witness whose seats an outside caller could fill would be a proof token
//! anybody could forge, and every road demanding it would be demanding nothing.
//! Both seats are private, so the only value of this type that exists is one an
//! admission run returned.

use core::marker::PhantomData;
use threadpak::refusal::{
    AdmittedRefusalFamily, FamilyAdmissionCoverage, FamilyShape, RefusalFamily,
};

struct DemoFamily;

impl RefusalFamily for DemoFamily {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotCanonical"];
}

fn main() {
    let _forged: AdmittedRefusalFamily<DemoFamily> = AdmittedRefusalFamily {
        coverage: FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection,
        _family: PhantomData,
    };
}

//! The reversal for band 00's family admission: the witness cannot be written
//! down, only earned.
//!
//! A witness whose seats an outside caller could fill would be a proof token
//! anybody could forge, and every road demanding it would be demanding nothing.
//! The coverage moving into the TYPE sharpens the point rather than softening
//! it: naming the strongest coverage in the annotation is free, and it buys
//! nothing, because both seats are private and the only value of this type that
//! exists is one an admission road returned.

use core::marker::PhantomData;
use threadpak::refusal::{AdmittedRefusalFamily, FamilyShape, OrderProjected, RefusalFamily};

struct DemoFamily;

impl RefusalFamily for DemoFamily {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotCanonical"];
}

fn main() {
    let _forged: AdmittedRefusalFamily<DemoFamily, OrderProjected> = AdmittedRefusalFamily {
        _family: PhantomData,
        _coverage: PhantomData,
    };
}

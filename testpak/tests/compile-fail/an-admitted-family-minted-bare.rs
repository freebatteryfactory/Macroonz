//! The reversal for band 00's family admission: the witness's seats cannot be
//! written down.
//!
//! A witness whose seats an outside caller could fill would be a proof token
//! anybody could forge, and every road demanding it would be demanding nothing.
//! The coverage moving into the TYPE sharpens the point rather than softening
//! it: naming the strongest coverage in the annotation is free, and it buys
//! nothing, because both seats are private and the struct literal below is not a
//! value a caller can write.
//!
//! # What this file establishes, exactly
//!
//! REPRESENTATION PRIVACY, and that is a narrower claim than *only earned*. A
//! witness has no sealed VALUE to keep — both its seats are `PhantomData` — so
//! what privacy buys here is only that the literal below does not compile. A
//! public road returning one, whether an associated function or a free function
//! beside the two admission roads, would leave this error exactly where it is
//! while handing out the token unearned.
//!
//! That absence is not derived. `admit_shape` and `admit_order` are themselves
//! public roads returning this witness, so no read separates a third one from
//! them without a declaration of which mints are the mints — and the tree
//! carries no such declaration.

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

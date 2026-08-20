//! The reversal for the SHAPE half of failing closed on an unwatchable cause set:
//! reading a context's watch set is a road that can refuse,
//! and no caller can take it as a fact.
//!
//! A cause set names up to the declared source magnitude,
//! and one trigger roster seat carries one identity.
//! The derivation used to answer with the first declaration,
//! which produced a value byte-for-byte the shape of a complete watch set —
//! so a plan committed to three declarations, while watching one,
//! read as CURRENT after the other two changed,
//! and nothing downstream could tell the two apart.
//! It now refuses instead, and the refusal is in the SIGNATURE:
//! a caller cannot receive an `InvalidationSet` from this road
//! without disposing of the case where the set cannot be watched.
//!
//! Restoring an unconditional answer makes `watch_set` total again,
//! this file compiles,
//! and trybuild reports that a case expected to fail succeeded.
//! That is what makes this fixture the control for the shape,
//! rather than a note beside it.
//!
//! # What this fixture does NOT establish, and why it cannot from here
//!
//! It does not establish the BEHAVIOR —
//! that a two-declaration context refuses and a one-declaration context does not.
//! That claim needs a multi-source `CauseAnchoring`, and the judge cannot build one:
//! `OwnerIdentityRef::decoded` is crate-internal,
//! so no road outside the services hands a caller a fragment identity at all.
//! The behavioral half is owed rather than held anywhere,
//! and it opens to a committed lane here
//! the day a public decode road exists for an owner identity reference.

use threadpak_macroc::{
    DeriveImplProjection, InvalidationSet, OwnerContentAccount, ProjectionContext,
};

fn watched(
    context: &ProjectionContext,
    content: &OwnerContentAccount<DeriveImplProjection>,
) -> InvalidationSet {
    context.watch_set(content)
}

fn main() {
    let _road = watched;
}

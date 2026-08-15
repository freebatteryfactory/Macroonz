//! The reversal for the runtime ladder's gate: a family whose owner did not
//! declare its magnitude evidence-selected cannot even name the base runtime
//! witness, so it has no value to mint or pass to a runtime-capacity road.
//!
//! The selected family is the positive control. Beside it stand both other
//! authority states: one family supplies a source-declared magnitude and one
//! leaves its magnitude unstated. Neither is evidence-selected, and both stop
//! at `LimitWitness`'s own bound before a consumer can receive a value.
//!
//! Nothing is minted here. That is the stronger test: a fixture outside this
//! crate has no public mint, while the type bound proves that even a future
//! owner mint cannot produce the wrong authority and that
//! `Bounded::admitted` cannot name such a witness in its signature.
//!
//! # The recorded diagnostic carries a population nobody wrote
//!
//! `rustc` answers an unsatisfied bound by listing the types that DO satisfy it,
//! and that list is derived from the impls rather than from anything authored
//! here. So the committed `.stderr` beside this file carries the roster of every
//! family on the runtime ladder, and a family joining or leaving it moves this
//! snapshot and fails this test. It is a drift detector rather than a count: the
//! compiler shortens the list past a threshold of its own, so the exact roster is
//! readable only while it is short, and a repository join deriving the population
//! from the sources is still owed.

use threadpak::types::{
    Bounded, BoundedConstruction, ConstLimit, DeclaredMagnitude, EvidenceSelectedLimit,
    EvidenceSelectedMagnitude, Limit, LimitWitness, UnstatedMagnitude,
};

/// A family whose owner declared the magnitude evidence-selected.
struct SelectedFamily;

impl Limit for SelectedFamily {
    type Authority = EvidenceSelectedMagnitude;
}

impl EvidenceSelectedLimit for SelectedFamily {}

/// A family whose owner supplies a source-declared magnitude instead.
struct SourceDeclaredFamily;

impl Limit for SourceDeclaredFamily {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SourceDeclaredFamily {
    const MAX: usize = 8;
}

/// A family whose owner did not. It is a lawful limit family and bounds seats
/// like any other; what it has not done is admit the runtime ladder.
struct UnstatedFamily;

impl Limit for UnstatedFamily {
    type Authority = UnstatedMagnitude;
}

/// The lawful half: a selected-family witness can be named at the consumer.
fn selected(
    witness: &LimitWitness<SelectedFamily>,
) -> Result<Bounded<u8, SelectedFamily>, BoundedConstruction> {
    Bounded::admitted(Vec::new(), witness)
}

/// The source-declared ladder cannot name or pass the runtime witness.
fn source_declared(
    witness: &LimitWitness<SourceDeclaredFamily>,
) -> Result<Bounded<u8, SourceDeclaredFamily>, BoundedConstruction> {
    Bounded::admitted(Vec::new(), witness)
}

/// The unstated family cannot name or pass the runtime witness either.
fn unstated(
    witness: &LimitWitness<UnstatedFamily>,
) -> Result<Bounded<u8, UnstatedFamily>, BoundedConstruction> {
    Bounded::admitted(Vec::new(), witness)
}

fn main() {
    let _ = selected;
    let _ = source_declared;
    let _ = unstated;
}

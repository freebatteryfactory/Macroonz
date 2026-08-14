//! The reversal for the runtime ladder's gate: a family whose owner never
//! declared its magnitude evidence-selected has no road to a runtime capacity.
//!
//! The two families below differ in exactly one line. Both are limit families;
//! one declares `EvidenceSelectedLimit` and the other does not, and nothing else
//! about them differs — same shape, same absence of a compile-time magnitude,
//! same seat. So the refusal below can only be the missing declaration, and the
//! lawful half above it is what says the bound is satisfiable at all.
//!
//! Nothing is minted here either. The bound sits on the mint, so naming the mint
//! as a value is enough to make the compiler settle it, and a fixture outside
//! this crate has no `LimitWitness` to build in any case.
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
    CapacityAdmission, EvidenceSelectedLimit, EvidenceSelectedMagnitude, Limit, LimitWitness,
    PositiveLimitWitness, UnstatedMagnitude,
};

/// A family whose owner declared the magnitude evidence-selected.
struct DeclaredFamily;

impl Limit for DeclaredFamily {
    type Authority = EvidenceSelectedMagnitude;
}

impl EvidenceSelectedLimit for DeclaredFamily {}

/// A family whose owner did not. It is a lawful limit family and bounds seats
/// like any other; what it has not done is admit the runtime ladder.
struct UndeclaredFamily;

impl Limit for UndeclaredFamily {
    type Authority = UnstatedMagnitude;
}

/// The lawful half, and it must stay lawful: the declared family reaches the
/// mint.
const DECLARED: fn(
    LimitWitness<DeclaredFamily>,
) -> Result<PositiveLimitWitness<DeclaredFamily>, CapacityAdmission> =
    PositiveLimitWitness::inhabited;

/// The unlawful half: the same mint, named for a family that never declared the
/// ladder it belongs to.
const UNDECLARED: fn(
    LimitWitness<UndeclaredFamily>,
) -> Result<PositiveLimitWitness<UndeclaredFamily>, CapacityAdmission> =
    PositiveLimitWitness::inhabited;

fn main() {
    let _ = DECLARED;
    let _ = UNDECLARED;
}

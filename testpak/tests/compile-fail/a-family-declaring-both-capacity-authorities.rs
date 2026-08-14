//! The reversal for the capacity-authority split: one family cannot declare two
//! roads to one capacity.
//!
//! A declared magnitude and an evidence-selected one are two authorities over
//! the same fact — how many items this family admits — and a family claiming
//! both leaves the machine with two answers and no rule for choosing between
//! them. That defect used to be a sentence in `crate::types`: the doc comment
//! said such a family would state two authorities and that no bound could see
//! it. A documented impossibility the type system permits is the two-halves
//! shape this repository exists to refuse, so the sentence is now the arity of
//! an associated type.
//!
//! The two families below differ in exactly one line. Both declare
//! `DeclaredMagnitude` as their authority and both supply a compile-time
//! maximum; one stops there and the other adds the runtime ladder. So the
//! refusal below can only be the second authority, and the lawful half above it
//! is what says the declaration it repeats is satisfiable at all.
//!
//! Nothing is minted here. The exclusion lives at the DECLARATION rather than at
//! any road, so writing the declaration is enough to settle it and no witness,
//! profile, or collection has to be built to reach the refusal.

use threadpak::types::{ConstLimit, DeclaredMagnitude, EvidenceSelectedLimit, Limit};

/// A family declaring exactly one capacity authority, and the ladder that
/// authority admits.
struct OneAuthority;

impl Limit for OneAuthority {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for OneAuthority {
    const MAX: usize = 8;
}

/// The same declaration again, so that what differs below is the second ladder
/// and nothing else.
struct TwoAuthorities;

impl Limit for TwoAuthorities {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for TwoAuthorities {
    const MAX: usize = 8;
}

/// The unlawful half: the runtime ladder, declared for a family whose authority
/// is already the compile-time one.
impl EvidenceSelectedLimit for TwoAuthorities {}

fn main() {
    let _ = OneAuthority::MAX;
    let _ = TwoAuthorities::MAX;
}

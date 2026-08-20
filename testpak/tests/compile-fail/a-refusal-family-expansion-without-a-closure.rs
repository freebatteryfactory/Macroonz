//! The receipt-rich road's first unwritable road: a receipt nobody proved.
//!
//! `RefusalFamilyExpansion` is this family's own view over the closed expansion
//! the generic terminal binds, and it has exactly one constructor. That
//! constructor is crate-internal, so the whole road — capture, plan, origin
//! graph, trace, rendering, closure, explanation, cause order — is not a
//! sequence a caller is trusted to follow. It is the only way to hold the value
//! at all.
//!
//! The TERMINAL's own `ClosedExpansion::bound` is public and is meant to be: a
//! caller holding a plan, a closure proved against that plan, and an explanation
//! answered over the two has already walked the road, and the terminal refuses
//! the pairings that disagree rather than hiding the door. What stays unwritable
//! is this family's view, whose one road in is `compile_refusal`.
//!
//! **One fixture covers every seat that dies here.** Deleting the surface, the
//! plan, the closure, the explanation, or the cause order is the same unwritable
//! move: each of them is a parameter of this one constructor, and a caller who
//! cannot reach the constructor cannot omit one of its arguments. A second
//! fixture per seat would prove the same privacy four more times.
//!
//! No value is constructed below. Naming the constructor is the proof.
//!
//! The committed `.stderr` beside this file carries the private constructor's
//! own signature, so a seat joining or leaving that signature moves this
//! snapshot and fails this test. The rendering was hand-adjusted for the
//! constructor this fixture now names, under no toolchain, and is verified at
//! first toolchain contact.

use threadpak_macroc::RefusalFamilyExpansion;

fn main() {
    let _ = RefusalFamilyExpansion::bound;
}

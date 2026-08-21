//! The root calculus's owed red twin, discharged.
//!
//! `NonEmptyBounded::singleton` is a TOTAL structural constructor: it cannot
//! form the failing case. The only way one item could exceed a family's maximum
//! is a family declaring `MAX = 0`, and the constructor's `const` block rejects
//! that instantiation. Post-monomorphization refusal IS compile-time refusal —
//! no artifact carrying this road under a zero-maximum family is ever produced.

use threadpak::types::{ConstLimit, DeclaredMagnitude, Limit, NonEmptyBounded};

/// A limit family admitting no item at all.
struct NoItemAtAll;

impl Limit for NoItemAtAll {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for NoItemAtAll {
    const MAX: usize = 0;
}

/// The road taken in a constant, so the family's declared maximum is read while
/// the declaration is still being checked rather than only when an artifact is
/// emitted.
const REFUSED: NonEmptyBounded<u8, NoItemAtAll> = NonEmptyBounded::singleton(1);

fn main() {
    let _ = REFUSED.len();
}

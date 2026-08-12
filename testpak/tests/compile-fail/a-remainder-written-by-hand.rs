//! The reversal for the remainder witness: the count belongs to the truncation.
//!
//! `NonEmptyBounded::admitted_prefix` is the one construction road in the machine
//! that drops anything, and what it hands back is a witness rather than a number.
//! The distinction only holds while the witness has exactly one mint: a
//! `PrefixRemainder` a caller could assemble would be a count nobody truncated
//! by, and every downstream seat reading it would be reporting an assertion
//! instead of an act.
//!
//! So the omission seat is private and the road is the only mint. The struct
//! literal below is the shape that would reopen it, and it does not compile.

use threadpak::types::PrefixRemainder;

/// A seat that acts on how much a truncation dropped.
fn reads_a_remainder(remainder: PrefixRemainder) -> usize {
    remainder.omitted()
}

fn main() {
    // No truncation happened here. The count is chosen, and the seat that
    // carries it is not the caller's to write.
    let fabricated = PrefixRemainder { omitted: 7 };
    let _ = reads_a_remainder(fabricated);
}

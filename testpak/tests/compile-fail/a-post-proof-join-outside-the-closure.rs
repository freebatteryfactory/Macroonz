//! The emitted tree is inside the proof, and the join that would put it outside
//! again is not a public road.
//!
//! Joining an emission's tokens used to be a public act performed AFTER the
//! closure returned. The exact token stream a compiler was handed was therefore
//! assembled past the proof boundary, and the closure identity said nothing
//! about it: a second caller joining the same units in another order, or joining
//! a subset, would have produced a different emission that the same closure
//! still vouched for.
//!
//! The join is crate-internal now, with one caller — the proof itself — which is
//! also what keeps a caller from supplying a digest for bytes an emission does
//! not carry. This is the reversal of that law: a mutant re-opening a public
//! post-proof join has to make this road reachable, and while it is not, this
//! fixture does not compile.
//!
//! No value is constructed below and nothing is called. Naming the road is the
//! proof: a private associated function is unreachable from here whatever it
//! would be handed.

use threadpak_macroc::CarriedTokens;

fn main() {
    let _join = CarriedTokens::joined;
}

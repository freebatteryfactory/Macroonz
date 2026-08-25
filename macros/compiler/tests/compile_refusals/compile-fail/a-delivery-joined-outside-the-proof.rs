//! The join that puts one delivery's tokens together is inside the proof, and there is no public road to it.
//!
//! Joining outside the proof would put the exact token stream a build receives past the boundary the closure identity commits to: a second caller joining the same units in another order, or joining a subset, would produce a different delivery the same closure still vouched for.
//! The join is crate-internal, with one caller — the partitioning inside `Closure::proved` — which is also what keeps a caller from supplying a digest for bytes a delivery does not carry.
//!
//! Nothing is called below; naming the road is the proof, because a private associated function is unreachable from here whatever it is handed.

use macroonz_compiler::CarriedTokens;

fn main() {
    let _join = CarriedTokens::joined;
}

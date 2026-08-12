//! The emitted tree is inside the proof, and the join that would put it outside
//! again does not exist as a public road.
//!
//! Joining the rendered units used to be a public act performed AFTER the
//! closure returned. The exact token stream a compiler was handed was therefore
//! assembled past the proof boundary, and the closure identity said nothing
//! about it: a second caller joining the same units in another order, or joining
//! a subset, would have produced a different emission that the same closure
//! still vouched for.
//!
//! The join is crate-internal now, with one caller — the proof itself — and the
//! closure keeps the joined tree and commits to its digest. This is the reversal
//! of that law: a mutant re-opening a public post-proof join has to make this
//! method reachable, and while it is not, this fixture does not compile.
//!
//! No value is constructed below. The signature and the call alone are the
//! proof.

use threadpak_macroc::RenderedProjection;
use threadpak_macroc::planning::RenderedImplementation;

fn main() {
    let join: fn(&RenderedProjection<RenderedImplementation>) = |rendered| {
        let _ = rendered.joined_tree();
    };
    let _ = join;
}

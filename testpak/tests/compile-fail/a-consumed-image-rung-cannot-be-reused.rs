//! The image ladder's affinity reversal: a rung handed over is gone.
//!
//! Band 16 declares the ladder affine — "each transition is a sealed constructor
//! consuming `self` and returning the stronger type or a typed refusal". A
//! `Copy` rung contradicts that sentence at the declaration: the caller passes
//! the weaker rung into the transition and still holds it afterwards, so the two
//! rungs stand side by side and the ladder is a suggestion.
//!
//! The four transitions are owed, so this fixture does not use one. It does not
//! need to: affinity is a property of the TYPE, and any road that takes a rung by
//! value asks the question. `consumes` below is a stand-in for the sealed
//! transition, and the second call is the reuse the ladder forbids.
//!
//! This is the reversal that activates on exactly the defect. While the rungs
//! derived `Copy`, both calls compiled and this file was green — which is what a
//! reversal proving nothing looks like.
//!
//! No rung is constructed and none could be: the ladder has no public
//! constructor, so the parameter binding alone is the proof.

fn consumes(rung: threadpak::image::ExecutableImage) -> threadpak::image::ExecutableImage {
    rung
}

fn main() {
    let reuse: fn(threadpak::image::ExecutableImage) = |rung| {
        let _first = consumes(rung);
        let _second = consumes(rung);
    };
    let _ = reuse;
}

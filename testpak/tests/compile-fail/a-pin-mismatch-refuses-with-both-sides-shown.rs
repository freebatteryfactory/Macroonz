//! A delivery whose expected schema identity is not the published one is refused
//! with ONE owned diagnostic naming both sides.
//!
//! Not a cascade of field errors somewhere inside a generated table: the
//! comparison is a `macro_rules!` pattern, so it refuses before the tokens it
//! guards are parsed as Rust at all. The delivery below fills BOTH cargo seats,
//! so what is recorded beside this file is the whole of what a consumer with an
//! incoherent published pair is shown.
//!
//! # What this fixture does NOT establish
//!
//! That the two seats were withheld TOGETHER. `compile_error!` fires during
//! expansion and stops the build before name resolution runs, so an item a
//! released deferred seat would have declared cannot be reached for and found
//! missing — the attempt produces no second error either way, which is a fact
//! about when rustc gives up rather than about this gate.
//!
//! What establishes the together is the gate's SHAPE: two arms, one clause
//! grammar, and a refusing arm whose body is this diagnostic and nothing else.
//! Neither `$trials` nor `$deferred` is written on that road, so there is no
//! arrangement of tokens in which half the delivery gets through.

fn main() {}

threadpak_testpak::generated_support! {
    expected: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    harness: threadpak_testpak,
    trials: { },
    deferred: { struct WithheldCargo; },
}

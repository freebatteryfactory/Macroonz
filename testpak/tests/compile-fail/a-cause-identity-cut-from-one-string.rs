//! The reversal for band 00's structural cause identity: the string road is
//! gone.
//!
//! A cause identity used to be one `&'static str` that a caller cut into a
//! family and a local key by convention — `CauseId::declared("family.local")`.
//! Nothing checked where the cut fell, so `demo.left` + `not-canonical` and
//! `demo` + `left.not-canonical` were the same value, and a family's ownership
//! of its own cause survived only as a spelling habit.
//!
//! The identity is now the pair, and this file is the proof that the retired
//! road cannot be taken: a one-argument mint does not typecheck, and a mint that
//! hands the joined text to the family seat does not either, because a family
//! identity is its own type rather than a string.

use threadpak::refusal::CauseId;

fn main() {
    let _one_string = CauseId::declared("demo.left.not-canonical");
    let _bare_strings = CauseId::declared("demo.left", "not-canonical");
}

//! A non-empty list under a ceiling that admits no item is refused at const evaluation.
//!
//! The guard is a const block inside the constructor, and the constructor is `const`, so a `const` item forces the refusal where the crate is read — no run needed to meet it.

use macroonz_compiler::NonEmpty;

const HELD: NonEmpty<u8, 0> = NonEmpty::one(7u8);

fn main() {
    let _held = HELD;
}

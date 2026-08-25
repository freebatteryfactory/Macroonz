//! A captured input's issued denominator is the builder's reading rather than a caller's number.

use macroonz_compiler::{Bounded, CapturedInput};

fn main() {
    let _forged = CapturedInput {
        trees: Bounded::empty(),
        issued: 99,
    };
}

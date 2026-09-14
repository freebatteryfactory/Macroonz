//! Offered tokens cannot become a completed tree through public fields.

use macroonz_compiler::{Bounded, GeneratedToken, GeneratedTree};

fn main() {
    let _forged = GeneratedTree {
        tokens: Bounded::from_array([GeneratedToken::word("#")]),
        source_spans: vec![None],
    };
}

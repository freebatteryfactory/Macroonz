//! A bounded collection can be minted only by the constructors that establish its ceiling.

use macroonz_compiler::Bounded;

fn main() {
    let _bounded = Bounded::<u8, 1>(vec![1]);
}

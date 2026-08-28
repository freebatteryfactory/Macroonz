//! A fixed offering wider than its declared ceiling refuses while the call is compiled.

use macroonz_compiler::Bounded;

fn main() {
    let _held = Bounded::<u8, 1>::from_array([1, 2]);
}

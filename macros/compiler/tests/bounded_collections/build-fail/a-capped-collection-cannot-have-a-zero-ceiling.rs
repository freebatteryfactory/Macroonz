//! A capped collection cannot carry its required first item under a ceiling of zero.

use macroonz_compiler::Capped;

fn main() {
    let _held = Capped::<u8, 0>::first_n(1, core::iter::empty());
}

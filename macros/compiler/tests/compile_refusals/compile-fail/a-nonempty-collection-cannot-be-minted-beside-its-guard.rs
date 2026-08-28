//! A non-empty collection can be minted only by the constructors that establish its required first item.

use macroonz_compiler::NonEmpty;

fn main() {
    let _nonempty = NonEmpty::<u8, 1> {
        head: 1,
        tail: Vec::new(),
    };
}

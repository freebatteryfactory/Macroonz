//! A keyed roster can be minted only by the constructors that establish nonemptiness, magnitude, and caller-key uniqueness.

use macroonz_compiler::{KeyedRoster, NonEmpty};

fn main() {
    let _roster = KeyedRoster::<u8, &'static str, 1> {
        members: NonEmpty::one(1),
        keys: NonEmpty::one("one"),
    };
}

//! A capped collection can be minted only with its constructor-derived capping posture.

use macroonz_compiler::{Capped, Capping, NonEmpty};

fn main() {
    let _capped = Capped::<u8, 1> {
        items: NonEmpty::one(1),
        capping: Capping::Complete,
    };
}

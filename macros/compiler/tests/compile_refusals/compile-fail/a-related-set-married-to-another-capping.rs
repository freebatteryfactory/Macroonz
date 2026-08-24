//! A capping belongs to the set it was built beside, and to no other.
//!
//! A capping is a claim ABOUT a set.
//! Written as a second value beside the identities, it is a claim a holder can carry away and tell about a different set — so the coarse commitment a capped diagnostic carries could be shown under the capping of a diagnostic that dropped nothing, both halves individually honest and the pair a lie.
//! No runtime check catches that shape, because there is nothing wrong at either end.
//!
//! The two leave the one road married inside the set: the readers hand back a borrowed slice and a posture, neither of which is the seat behind it, and the seats themselves are not the caller's to write.

use macroonz::{Bounded, Family, RelatedIdentity, RelatedSet};

const FAMILY: Family = Family::declared("lane/capping");

fn main() {
    let complete = RelatedSet::derived_over(FAMILY, &[vec![7_u8], vec![8_u8]]);
    let capped = RelatedSet::derived_over(FAMILY, &vec![vec![9_u8]; 128]);

    // What a reader is handed: a borrowed projection of the identities, and the posture beside them.
    let _identities: &[RelatedIdentity] = complete.carried();

    // The marriage the seats refuse.
    let _crossed = RelatedSet {
        carried: Bounded::empty(),
        capping: capped.capping(),
    };
}

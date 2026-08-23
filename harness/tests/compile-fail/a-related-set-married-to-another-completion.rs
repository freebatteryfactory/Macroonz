//! The reversal for the related set's marriage: a completion belongs to the set
//! it was built beside, and to no other.
//!
//! A completion is a claim ABOUT a set. Written as a second value beside the
//! carry, it is a claim a holder can carry away and tell about a different set —
//! so the coarse identities one truncated diagnostic carries could be shown under
//! the completion of a diagnostic that dropped nothing, both halves individually
//! honest and the pair a lie. No runtime check catches that shape: there is
//! nothing wrong at either end.
//!
//! So the two leave the one road married inside `RelatedSet`, the seats are
//! private, and there is no road back out to a loose pair — no public two-value
//! constructor, no `into_parts`, and no owned carry. What is left to a caller
//! wanting the cross-wire is the struct literal below, and writing the seats is
//! not the caller's to do.
//!
//! The fixture stays on that one shape on purpose. Privacy is checked after type
//! checking, so a second attempt failing earlier would swallow this error and
//! leave the record attesting something else.

use threadpak_macroc::RelatedSet;

fn main() {
    // Two roads, two postures. Each set is honest about itself: the first
    // carries every issue it established, the second overran the declared
    // magnitude and says how many identities it left behind.
    let complete = RelatedSet::derived_over(1_u8, &[vec![7_u8], vec![8_u8]]);
    let truncated = RelatedSet::derived_over(1_u8, &vec![vec![9_u8]; 4096]);

    // The marriage: the complete set's identities wearing the truncated set's
    // posture. The seats are not the caller's to write.
    let _crossed = RelatedSet {
        carried: complete.carried().clone(),
        completion: truncated.completion(),
    };
}

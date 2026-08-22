//! The family-isolation lane: what a domain separates, and what a position
//! separates, observed from outside the crate that decides both.
//!
//! A type cannot carry either of these. That two preimages under different
//! families reach unrelated addresses, and that one family at two positions
//! reaches unrelated addresses, are facts about the derivation context this
//! crate assembles — so they are observed here, over the public roads, with no
//! private seat touched.
//!
//! # Reversals
//!
//! The separations are required to HOLD APART rather than merely to exist. A
//! derivation that ignored the family segment would still hand back thirty-two
//! bytes and would still look like an answer; what it would not do is give two
//! families two names for one preimage. Each observation below is a pair that
//! must disagree, and the spelling observation is what says why.

use threadpak_testpak::identity::{
    ContentAddress, DomainTag, HARNESS_IDENTITY_PROFILE, IdentityProfileVersion,
};

/// The position used by the synthetic lane families.
const FIRST: IdentityProfileVersion = IdentityProfileVersion::declared(1);

/// One preimage, used for every derivation below, so the only thing that varies
/// between two addresses is the context they were derived under.
const PREIMAGE: &[u8] = b"family-isolation-lane-preimage";

/// Two families over one preimage reach two unrelated addresses.
///
/// This is what the family segment buys, and it is proven rather than asserted:
/// without it, a schema identity and a trial key derived over identical
/// material would be one value, and a caller holding either could read it as
/// the other.
#[test]
fn two_families_over_one_preimage_do_not_share_an_address() {
    let one = ContentAddress::derived(DomainTag::declared("lane-family-one", FIRST), PREIMAGE);
    let other = ContentAddress::derived(DomainTag::declared("lane-family-two", FIRST), PREIMAGE);
    assert_ne!(one.as_bytes(), other.as_bytes());
}

/// One family at two positions over one preimage reaches two unrelated
/// addresses.
///
/// This is what a position buys, and it is the whole of what a move performs: a
/// family that advances renames every address it derives, so a reader holding
/// the earlier name can tell it apart from the later one. A position that did
/// not reach the context would be a number in a doc comment.
#[test]
fn one_family_at_two_positions_does_not_share_an_address() {
    let family = "lane-positioned-family";
    let first = ContentAddress::derived(DomainTag::declared(family, FIRST), PREIMAGE);
    let second = ContentAddress::derived(
        DomainTag::declared(family, IdentityProfileVersion::declared(2)),
        PREIMAGE,
    );
    assert_ne!(first.as_bytes(), second.as_bytes());
}

/// One family's position does not reach another family's context.
///
/// The two separations compose rather than substitute: moving one family leaves
/// every address under every other family with its name, which is the whole
/// reason a position rides its own tag instead of standing over the crate.
#[test]
fn a_family_that_moves_leaves_its_neighbour_where_it_was() {
    let neighbour = DomainTag::declared("lane-neighbour-family", FIRST);
    let before = ContentAddress::derived(neighbour, PREIMAGE);

    let moved = DomainTag::declared("lane-moving-family", IdentityProfileVersion::declared(7));
    let elsewhere = ContentAddress::derived(moved, PREIMAGE);

    let after = ContentAddress::derived(neighbour, PREIMAGE);
    assert_eq!(before.as_bytes(), after.as_bytes());
    assert_ne!(before.as_bytes(), elsewhere.as_bytes());
}

/// The derivation context is spelled stem, then family, then position.
///
/// The order is the reason a position belongs to the family it is written
/// beside. A version segment ahead of the family would read as the stem's, and
/// this is where that reading is ruled out by the string itself rather than by
/// a paragraph about it.
#[test]
fn the_context_spells_the_family_ahead_of_its_position() {
    let context = HARNESS_IDENTITY_PROFILE.context_for(DomainTag::declared("lane-spelling", FIRST));
    assert_eq!(
        context,
        format!("{}/lane-spelling/v1", HARNESS_IDENTITY_PROFILE.stem())
    );
}

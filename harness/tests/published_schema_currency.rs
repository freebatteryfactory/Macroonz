//! The publication-currency lane: the two published sides and the identity the
//! current declaration actually derives are one value.
//!
//! The gate in `descriptor::gate` compares the producer's literal against this
//! harness's literal, and says so honestly: agreement there is COHERENCE. Two
//! literals that agree because publication never ran agree just as loudly. What
//! the gate cannot see is exactly what this lane observes — whether either side
//! is CURRENT — by deriving the identity from the declaration itself and
//! requiring both published spellings to equal it.
//!
//! # Nonclaims
//!
//! Currency, and nothing else.
//! A pin that is current over a declaration is evidence about that declaration, not about the structural authorship of neighboring facts; those joins live at the descriptor owner.

use threadpak_macroc::planning::EXPECTED_GENERATED_SUPPORT_SCHEMA_ID;
use threadpak_testpak::descriptor::{
    GeneratedSupportSchema, PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID,
};

/// The identity the current declaration derives, through the public owner road
/// and nothing else.
fn derived() -> Option<[u8; 32]> {
    let schema = GeneratedSupportSchema::published().ok()?;
    let identity = schema.identity().ok()?;
    Some(*identity.address().as_bytes())
}

/// The harness's published literal is the identity the current declaration
/// derives.
///
/// The gate's own arms carry these digits as pattern tokens — an arm matches
/// tokens and cannot read a constant — so a stale value here is a gate that opens
/// on a declaration that has moved. The constant and the arms are written in one
/// base, in one order, in one layout, so the three spellings are compared by eye
/// where no lane can reach them; what this seat holds is the constant against the
/// derivation.
#[test]
fn the_harness_published_literal_is_current() {
    let derived = derived();
    assert!(
        derived.is_some_and(|bytes| &bytes == PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID),
        "the current declaration derives {derived:02x?}, and the harness publishes {PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID:02x?}"
    );
}

/// The services' expected literal is the same identity.
///
/// Two crates, one published fact. The producer cannot read the harness's
/// constant — a `macro_rules!` arm matches tokens — so the two spellings are
/// held together here rather than by anybody remembering.
#[test]
fn the_services_expected_literal_is_current() {
    let derived = derived();
    let expected = EXPECTED_GENERATED_SUPPORT_SCHEMA_ID.as_bytes();
    assert!(
        derived.is_some_and(|bytes| bytes == *expected),
        "the current declaration derives {derived:02x?}, and the services expect {expected:02x?}"
    );
}

/// The two published sides are one value.
///
/// The gate establishes this at every consumer invocation; stating it here as
/// well is what makes the three-value equality a closed triangle rather than
/// two independent comparisons that could each be satisfied by a different
/// pair.
#[test]
fn the_two_published_sides_agree() {
    assert_eq!(
        PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID,
        EXPECTED_GENERATED_SUPPORT_SCHEMA_ID.as_bytes()
    );
}

//! Revision authority: derivation sees material, foreign addresses remain weaker, and only two derived halves authorize cache reuse.

use macroonz_harness::descriptor::{
    DERIVED_REVISION_DOMAIN, DerivedRevision, RevisionBinding, RevisionPosture,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    CacheEligibility, ReplayPosture, attachment_cache_eligibility, attachment_replay_posture,
};

const FOREIGN_DOMAIN: DomainTag =
    DomainTag::declared("foreign-revision", IdentityProfileVersion::declared(1));

/// A derivation repeats over identical material and moves when the material moves.
#[test]
fn derived_revision_is_minted_from_material() {
    let first = DerivedRevision::from_material(b"subject-v1");
    let repeated = DerivedRevision::from_material(b"subject-v1");
    let moved = DerivedRevision::from_material(b"subject-v2");

    assert_eq!(first, repeated);
    assert_ne!(first, moved);
    assert_eq!(
        first.revision(),
        ContentAddress::derived(DERIVED_REVISION_DOMAIN, b"subject-v1")
    );
    assert_ne!(
        first.revision(),
        ContentAddress::derived(FOREIGN_DOMAIN, b"subject-v1")
    );
    assert_eq!(
        RevisionBinding::derived(first).posture(),
        RevisionPosture::Derived
    );
}

/// A caller-held address stays weaker and cannot authorize a cache hit or exact replay.
#[test]
fn declared_address_cannot_inherit_derived_authority() {
    let declared = RevisionBinding::declared(ContentAddress::derived(
        FOREIGN_DOMAIN,
        b"imported-revision",
    ));
    let derived = RevisionBinding::derived(DerivedRevision::from_material(b"local-revision"));

    assert_eq!(
        attachment_cache_eligibility(derived.posture(), declared.posture()),
        CacheEligibility::NeverEligible
    );
    assert_eq!(
        attachment_replay_posture(derived.posture(), declared.posture()),
        ReplayPosture::DeclaredByAuthor
    );
}

/// Two material-derived halves authorize the strong cache and replay postures.
#[test]
fn two_derived_halves_open_only_their_exact_postures() {
    let subject = RevisionBinding::derived(DerivedRevision::from_material(b"subject"));
    let check = RevisionBinding::derived(DerivedRevision::from_material(b"check"));

    assert_eq!(
        attachment_cache_eligibility(subject.posture(), check.posture()),
        CacheEligibility::Eligible
    );
    assert_eq!(
        attachment_replay_posture(subject.posture(), check.posture()),
        ReplayPosture::ExactDerived
    );
}

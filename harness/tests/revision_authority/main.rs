//! Revision authority: derivation sees material, foreign addresses remain weaker, and only two derived halves authorize cache reuse.

use macroonz_harness::descriptor::{
    CheckRef, ClaimRef, Classification, DERIVED_REVISION_DOMAIN, DerivedRevision, ExecutionSuite,
    Origin, PopulationRef, RevisionBinding, RevisionPosture, Role, Row, SubjectRoute, Tag,
    TrialTableRefusal,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    CacheEligibility, ReplayPosture, attachment_cache_eligibility, attachment_replay_posture,
};

const FOREIGN_DOMAIN: DomainTag =
    DomainTag::declared("foreign-revision", IdentityProfileVersion::declared(1));

const OWNER: &str = "revision-authority";

fn row(
    claim: &'static str,
    suite: &'static str,
    role: &'static str,
    tag: &'static str,
) -> Result<Row, TrialTableRefusal> {
    Ok(Row::declared(
        ClaimRef::named(OWNER, claim)?,
        ExecutionSuite::named(OWNER, suite)?,
        Classification::authored(
            vec![Role::named(OWNER, role)?],
            vec![Tag::named(OWNER, tag)?],
        )?,
        SubjectRoute::named(OWNER, "subject")?,
        CheckRef::named(OWNER, "check")?,
        PopulationRef::named(OWNER, "population")?,
        Origin::HandWritten,
    )?)
}

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

/// The meet is the weaker posture for every ordered pair, including reversal controls.
#[test]
fn revision_posture_meet_is_exhaustive_and_commutative() {
    let cases = [
        (RevisionPosture::Derived, RevisionPosture::Derived),
        (RevisionPosture::Derived, RevisionPosture::Declared),
        (RevisionPosture::Derived, RevisionPosture::Untracked),
        (RevisionPosture::Declared, RevisionPosture::Declared),
        (RevisionPosture::Declared, RevisionPosture::Untracked),
        (RevisionPosture::Untracked, RevisionPosture::Untracked),
    ];
    let expected_postures = [
        RevisionPosture::Derived,
        RevisionPosture::Declared,
        RevisionPosture::Untracked,
        RevisionPosture::Declared,
        RevisionPosture::Untracked,
        RevisionPosture::Untracked,
    ];

    for ((left, right), expected_posture) in cases.into_iter().zip(expected_postures) {
        assert_eq!(left.meet(right), expected_posture);
        assert_eq!(right.meet(left), expected_posture);
    }
}

/// Row bytes retain every declared field while the trial key moves only with its four coordinates.
#[test]
fn row_bytes_and_trial_key_keep_separate_identity_boundaries() -> Result<(), TrialTableRefusal> {
    let baseline = row("claim", "suite-a", "role-a", "tag-a")?;
    let another_suite = row("claim", "suite-b", "role-a", "tag-a")?;
    let another_classification = row("claim", "suite-a", "role-b", "tag-b")?;
    let another_claim = row("another-claim", "suite-a", "role-a", "tag-a")?;

    assert_ne!(baseline.canonical_bytes(), another_suite.canonical_bytes());
    assert_ne!(
        baseline.canonical_bytes(),
        another_classification.canonical_bytes()
    );
    assert_eq!(baseline.trial_key(), another_suite.trial_key());
    assert_eq!(baseline.trial_key(), another_classification.trial_key());
    assert_ne!(baseline.trial_key(), another_claim.trial_key());

    assert_eq!(
        baseline.canonical_bytes().as_bytes(),
        &[
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105, 111, 110, 45, 97,
            117, 116, 104, 111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 5, 99, 108, 97, 105, 109,
            0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105, 111, 110, 45, 97, 117, 116, 104,
            111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 7, 115, 117, 105, 116, 101, 45, 97, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105, 111, 110, 45,
            97, 117, 116, 104, 111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 6, 114, 111, 108, 101,
            45, 97, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105,
            111, 110, 45, 97, 117, 116, 104, 111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 5, 116,
            97, 103, 45, 97, 0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105, 111, 110, 45,
            97, 117, 116, 104, 111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 7, 115, 117, 98, 106,
            101, 99, 116, 0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105, 111, 110, 45, 97,
            117, 116, 104, 111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 5, 99, 104, 101, 99, 107,
            0, 0, 0, 0, 0, 0, 0, 18, 114, 101, 118, 105, 115, 105, 111, 110, 45, 97, 117, 116, 104,
            111, 114, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 10, 112, 111, 112, 117, 108, 97, 116,
            105, 111, 110, 1,
        ]
    );
    assert_eq!(
        baseline.trial_key().address().as_bytes(),
        &[
            15, 190, 145, 46, 46, 55, 32, 153, 189, 162, 158, 123, 63, 119, 46, 255, 37, 165, 6,
            44, 234, 167, 113, 83, 91, 221, 25, 63, 180, 67, 110, 112,
        ]
    );
    Ok(())
}

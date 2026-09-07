//! Historical capsule integrity, authority separation and independent byte controls.

mod fixture;
mod process;
mod vector;

use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, read_capsule, retain_capsule,
};
use macroonz_harness::report::{FailureClass, ReplayPosture};
use vector::{InputKind, Vector};

const LIMITS: ArchiveLimits = ArchiveLimits::declared(4096, 2048);

#[test]
fn unit_capsules_retain_the_unit_key_domain_without_inventing_case_identity() -> Result<(), ()> {
    let capsule = fixture::unit_capsule()?;
    let record = retain_capsule(&capsule, LIMITS).map_err(|_| ())?;
    assert!(record.key().input().is_none());
    assert_eq!(record.identity(), capsule.identity());
    assert_eq!(record.key().address(), capsule.key().address());
    let exact = ArchiveLimits::declared(record.encoded().len(), LIMITS.field());
    assert_eq!(retain_capsule(&capsule, exact).map_err(|_| ())?, record);
    Ok(())
}

#[test]
fn metadata_is_an_integrity_bound_claim_without_writer_authentication() -> Result<(), ()> {
    let mut vector = Vector::declared(InputKind::Bound);
    let original = read_capsule(&vector.encoded(), LIMITS).map_err(|_| ())?;
    *vector.metadata.get_mut(9).ok_or(())? = b'O';
    let modified = read_capsule(&vector.encoded(), LIMITS).map_err(|_| ())?;
    assert_ne!(original.address(), modified.address());
    assert_eq!(original.identity(), modified.identity());
    assert_eq!(original.key().address(), modified.key().address());
    assert_eq!(modified.key().input().ok_or(())?.namespace(), "Outside");
    assert_eq!(modified.claimed_posture(), ReplayPosture::DeclaredByAuthor);
    Ok(())
}

#[test]
fn nested_preimages_require_complete_framing_and_their_exact_joined_addresses() -> Result<(), ()> {
    let mut vector = Vector::declared(InputKind::Bound);
    vector.key.push(0);
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    vector = Vector::declared(InputKind::Bound);
    vector.fingerprint.push(0);
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    vector = Vector::declared(InputKind::Bound);
    vector.capsule.push(0);
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.capsule.get_mut(57).ok_or(())? ^= 1;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::IdentityJoinMismatch)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.capsule.last_mut().ok_or(())? = 9;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidSlot)
    );
    Ok(())
}

#[test]
fn independent_vectors_retain_all_capsule_coordinates_as_historical_claims() -> Result<(), ()> {
    for kind in [InputKind::Unit, InputKind::Bound] {
        let typed = matches!(kind, InputKind::Bound);
        let vector = Vector::declared(kind);
        let encoded = vector.encoded();
        let record = read_capsule(&encoded, LIMITS).map_err(|_| ())?;
        assert_eq!(record.encoded(), encoded);
        assert_eq!(
            record.address().as_bytes(),
            &vector::hash("historical-replay-capsule/v1", &vector.body())
        );
        assert_eq!(
            record.identity().as_bytes(),
            &vector::hash("replay-capsule/v2", &vector.capsule)
        );
        let key_family = if typed {
            "input-execution-key/v1"
        } else {
            "execution-key/v1"
        };
        assert_eq!(
            record.key().address().as_bytes(),
            &vector::hash(key_family, &vector.key)
        );
        assert_eq!(record.key().trial().as_bytes(), &[1; 32]);
        assert_eq!(record.key().subject().as_bytes(), &[2; 32]);
        assert_eq!(record.key().check().as_bytes(), &[3; 32]);
        assert_eq!(record.key().invocation().cases().cases(), 3);
        assert_eq!(record.key().invocation().bytes().bytes(), 55);
        assert_eq!(record.key().invocation().time().nanoseconds(), 89);
        assert_eq!(record.key().target().target().spelling(), "x86-test");
        assert_eq!(record.key().target().toolchain().spelling(), "rust-test");
        assert_eq!(record.key().input().is_some(), typed);
        if let Some(input) = record.key().input() {
            assert_eq!(input.case().as_bytes(), &[4; 32]);
            assert_eq!(input.decoder().as_bytes(), &[5; 32]);
            assert_eq!(input.posture(), ReplayPosture::ExactDerived);
            assert_eq!(input.namespace(), "outside");
            assert_eq!(input.profile().name(), "bytes");
            assert_eq!(input.profile().version(), 7);
            assert_eq!(input.schema().as_bytes(), &[6; 32]);
        }
        assert_eq!(record.input(), &[1]);
        assert_eq!(record.fingerprint().trial(), record.key().trial());
        assert_eq!(record.fingerprint().family(), "fixture");
        assert_eq!(record.fingerprint().local(), "disagrees");
        assert_eq!(
            record.fingerprint().class(),
            FailureClass::PropertyDisagreement
        );
        assert_eq!(
            record.fingerprint().address().as_bytes(),
            &vector::hash("failure-fingerprint/v1", &vector.fingerprint)
        );
        assert_eq!(record.generation().name(), "profile");
        assert_eq!(record.generation().version(), 2);
        assert_eq!(record.minimization().name(), "reduce");
        assert_eq!(record.minimization().version(), 3);
        assert_eq!(record.schema().as_bytes(), &[7; 32]);
        assert_eq!(record.claimed_posture(), ReplayPosture::DeclaredByAuthor);
    }
    Ok(())
}

#[test]
fn earned_capsule_retains_original_case_and_reached_witness() -> Result<(), ()> {
    let capsule = fixture::capsule()?;
    let record = retain_capsule(&capsule, LIMITS).map_err(|_| ())?;
    assert_eq!(record.identity(), capsule.identity());
    assert_eq!(record.key().address(), capsule.key().address());
    assert_eq!(
        record.fingerprint().address(),
        capsule.fingerprint().address()
    );
    assert_eq!(record.input(), &[1]);
    let original = fixture::report(&[1, 2, 3])?;
    let witness = fixture::report(record.input())?;
    assert_eq!(record.key().address(), original.standing().key().address());
    assert_ne!(record.key().address(), witness.standing().key().address());
    let detached = read_capsule(record.encoded(), LIMITS).map_err(|_| ())?;
    drop(record);
    drop(capsule);
    assert_eq!(detached.input(), &[1]);
    assert_eq!(detached.key().input().ok_or(())?.namespace(), "archive");
    Ok(())
}

#[test]
fn limits_apply_to_both_retention_and_loading() -> Result<(), ()> {
    let capsule = fixture::capsule()?;
    let record = retain_capsule(&capsule, LIMITS).map_err(|_| ())?;
    let exact = ArchiveLimits::declared(record.encoded().len(), LIMITS.field());
    assert_eq!(retain_capsule(&capsule, exact).map_err(|_| ())?, record);
    assert_eq!(
        read_capsule(record.encoded(), exact).map_err(|_| ())?,
        record
    );
    let short = ArchiveLimits::declared(record.encoded().len().saturating_sub(1), LIMITS.field());
    assert_eq!(
        retain_capsule(&capsule, short),
        Err(ArchiveRefusal::EnvelopeTooLarge)
    );
    assert_eq!(
        read_capsule(record.encoded(), short),
        Err(ArchiveRefusal::EnvelopeTooLarge)
    );
    let tiny_field = ArchiveLimits::declared(4096, 31);
    assert_eq!(
        retain_capsule(&capsule, tiny_field),
        Err(ArchiveRefusal::FieldTooLarge)
    );
    assert_eq!(
        read_capsule(record.encoded(), tiny_field),
        Err(ArchiveRefusal::FieldTooLarge)
    );
    Ok(())
}

#[test]
fn malformed_envelopes_refuse_even_when_the_outer_digest_is_recomputed() -> Result<(), ()> {
    let mut vector = Vector::declared(InputKind::Bound);
    vector.format = 2;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::UnsupportedFormat { found: 2 })
    );
    vector.format = 1;
    vector.kind = 2;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::WrongKind { found: 2 })
    );
    vector.kind = 1;
    vector.custody = 1;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::UnsupportedCustody { found: 1 })
    );
    vector.custody = 0;
    let mut trailing = vector.body();
    trailing.push(0);
    assert_eq!(
        read_capsule(&vector::envelope(&trailing), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    let mut corrupt = vector.encoded();
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_capsule(&corrupt, LIMITS),
        Err(ArchiveRefusal::AddressMismatch)
    );
    for length in 0..vector.body().len() {
        let body = vector.body();
        let truncated = body.get(..length).ok_or(())?;
        assert!(read_capsule(&vector::envelope(truncated), LIMITS).is_err());
    }
    let mut oversized = vec![0u8; 12];
    oversized
        .get_mut(..4)
        .ok_or(())?
        .copy_from_slice(&1u32.to_be_bytes());
    oversized
        .get_mut(4..8)
        .ok_or(())?
        .copy_from_slice(&1u32.to_be_bytes());
    oversized.extend_from_slice(&u64::MAX.to_be_bytes());
    assert!(read_capsule(&vector::envelope(&oversized), LIMITS).is_err());
    Ok(())
}

#[test]
fn nested_identities_text_discriminants_and_posture_cannot_hide_behind_integrity() -> Result<(), ()>
{
    let mut vector = Vector::declared(InputKind::Bound);
    *vector.fingerprint.get_mut(8).ok_or(())? = 9;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::IdentityJoinMismatch)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.capsule.get_mut(8).ok_or(())? ^= 1;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::IdentityJoinMismatch)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.fingerprint.last_mut().ok_or(())? = 9;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidSlot)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.metadata.first_mut().ok_or(())? = 9;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidSlot)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.metadata.get_mut(9).ok_or(())? = 0xff;
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidText)
    );
    vector = Vector::declared(InputKind::Bound);
    *vector.key.last_mut().ok_or(())? = 2;
    let key_hash = vector::hash("input-execution-key/v1", &vector.key);
    vector
        .capsule
        .get_mut(8..40)
        .ok_or(())?
        .copy_from_slice(&key_hash);
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::PostureMismatch)
    );
    vector = Vector::declared(InputKind::Bound);
    vector
        .key
        .get_mut(..8)
        .ok_or(())?
        .copy_from_slice(&31u64.to_be_bytes());
    assert_eq!(
        read_capsule(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidAddressWidth)
    );
    Ok(())
}

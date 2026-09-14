//! Independent specimen framing and typed admission before runner execution.

use arbitrary::Unstructured;
use macroonz_harness::descriptor::{DerivedRevision, NamespacedName, RevisionBinding};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::input::{InputBinding, InputLimits, InputProfile, InputRefusal, pack, read};

const LIMITS: InputLimits = InputLimits::declared(256, 16);

fn profile(version: u32, schema: &[u8]) -> Result<InputProfile, ()> {
    Ok(InputProfile::declared(
        NamespacedName::named("input-lane", "byte").map_err(|_| ())?,
        version,
        ContentAddress::derived(
            DomainTag::declared("input-lane-schema", IdentityProfileVersion::declared(1)),
            schema,
        ),
    ))
}

fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(include_bytes!("input.rs")))
}

fn digit(source: &mut Unstructured<'_>) -> arbitrary::Result<u8> {
    let [byte] = source.bytes(1)? else {
        return Err(arbitrary::Error::NotEnoughData);
    };
    if *byte > 9 {
        return Err(arbitrary::Error::IncorrectFormat);
    }
    Ok(*byte)
}

fn refuses(_source: &mut Unstructured<'_>) -> arbitrary::Result<u8> {
    Err(arbitrary::Error::IncorrectFormat)
}

fn addressed(body: &[u8]) -> Vec<u8> {
    let mut envelope =
        blake3::derive_key("macroonz/harness-identity/trial-input/v1", body).to_vec();
    envelope.extend_from_slice(body);
    envelope
}

fn body(profile: InputProfile) -> Vec<u8> {
    let mut bytes = b"\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x0ainput-lane\x00\x00\x00\x00\x00\x00\x00\x04byte\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x20".to_vec();
    bytes.extend_from_slice(profile.schema().as_bytes());
    bytes.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x01\x07");
    bytes
}

#[test]
fn input_envelope_matches_an_independent_wire_vector() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let expected = addressed(&body(profile));
    assert_eq!(expected.len(), 119);
    let written = pack(profile, &[7], LIMITS).map_err(|_| ())?;
    assert_eq!(written.encoded(), expected);
    assert_eq!(read(profile, &expected, LIMITS), Ok(written));
    Ok(())
}

#[test]
fn different_specimens_reach_the_decoder_as_different_values() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let binding = InputBinding::declared(profile, revision(), digit);
    let first = binding
        .decode(pack(profile, &[2], LIMITS).map_err(|_| ())?)
        .map_err(|_| ())?;
    let second = binding
        .decode(pack(profile, &[7], LIMITS).map_err(|_| ())?)
        .map_err(|_| ())?;
    assert_eq!(*first.value(), 2);
    assert_eq!(*second.value(), 7);
    assert_ne!(first.envelope().case(), second.envelope().case());
    assert_eq!(first.revision(), revision());
    assert_eq!(second.envelope().payload(), &[7]);
    assert_eq!(binding.profile(), profile);
    assert_eq!(binding.revision(), first.revision());
    Ok(())
}

#[test]
fn decoder_refusal_and_partial_consumption_admit_no_value() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let binding = InputBinding::declared(profile, revision(), digit);
    for (input, expected) in [
        (
            &[][..],
            InputRefusal::DecoderRefused(arbitrary::Error::NotEnoughData),
        ),
        (
            &[10][..],
            InputRefusal::DecoderRefused(arbitrary::Error::IncorrectFormat),
        ),
        (&[7, 8][..], InputRefusal::TrailingInputBytes { count: 1 }),
    ] {
        let result = binding.decode(pack(profile, input, LIMITS).map_err(|_| ())?);
        assert_eq!(result.err(), Some(expected));
    }
    Ok(())
}

#[test]
fn reader_and_decoder_require_the_independently_supplied_profile() -> Result<(), ()> {
    let expected = profile(3, b"one digit")?;
    let valid = pack(expected, &[7], LIMITS).map_err(|_| ())?;
    let renamed = InputProfile::declared(
        NamespacedName::named("input-lane", "other").map_err(|_| ())?,
        3,
        expected.schema(),
    );
    for (other, refusal) in [
        (profile(4, b"one digit")?, InputRefusal::ProfileMismatch),
        (renamed, InputRefusal::ProfileMismatch),
        (profile(3, b"two digits")?, InputRefusal::SchemaMismatch),
    ] {
        assert_eq!(read(other, valid.encoded(), LIMITS), Err(refusal));
        let binding = InputBinding::declared(other, revision(), refuses);
        assert_eq!(binding.decode(valid.clone()).err(), Some(refusal));
        assert_ne!(
            pack(other, &[7], LIMITS).map_err(|_| ())?.case(),
            valid.case()
        );
    }
    Ok(())
}

#[test]
fn the_decoder_revision_is_retained_without_renaming_the_specimen() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let envelope = pack(profile, &[7], LIMITS).map_err(|_| ())?;
    let original = InputBinding::declared(profile, revision(), digit)
        .decode(envelope.clone())
        .map_err(|_| ())?;
    let changed =
        RevisionBinding::declared(DerivedRevision::from_material(b"other decoder").revision());
    let successor = InputBinding::declared(profile, changed, digit)
        .decode(envelope)
        .map_err(|_| ())?;
    assert_eq!(original.value(), successor.value());
    assert_eq!(original.envelope().case(), successor.envelope().case());
    assert_ne!(original.revision(), successor.revision());
    assert_eq!(successor.revision(), changed);
    Ok(())
}

#[test]
fn corrupt_or_incomplete_envelopes_refuse() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let expected = addressed(&body(profile));
    let mut corrupt = expected.clone();
    *corrupt.last_mut().ok_or(())? = 8;
    assert_eq!(
        read(profile, &corrupt, LIMITS),
        Err(InputRefusal::AddressMismatch)
    );
    for prefix in 0..expected.len() {
        assert!(read(profile, expected.get(..prefix).ok_or(())?, LIMITS).is_err());
    }
    let raw_body = body(profile);
    for prefix in 0..raw_body.len() {
        let short = addressed(raw_body.get(..prefix).ok_or(())?);
        assert!(read(profile, &short, LIMITS).is_err());
    }
    Ok(())
}

#[test]
fn self_consistent_digest_does_not_admit_false_framing() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let mut unsupported = body(profile);
    *unsupported.get_mut(3).ok_or(())? = 2;
    assert_eq!(
        read(profile, &addressed(&unsupported), LIMITS),
        Err(InputRefusal::UnsupportedFormat { found: 2 })
    );
    let mut trailing = body(profile);
    trailing.push(9);
    assert_eq!(
        read(profile, &addressed(&trailing), LIMITS),
        Err(InputRefusal::TrailingEnvelopeBytes { count: 1 })
    );
    let mut enormous = body(profile);
    enormous.get_mut(4..12).ok_or(())?.fill(255);
    assert!(matches!(
        read(profile, &addressed(&enormous), LIMITS),
        Err(InputRefusal::Truncated | InputRefusal::LengthOutsidePlatform { .. })
    ));
    Ok(())
}

#[test]
fn reader_and_writer_enforce_both_independent_byte_limits() -> Result<(), ()> {
    let profile = profile(3, b"one digit")?;
    let valid = pack(profile, &[7], LIMITS).map_err(|_| ())?;
    assert_eq!(
        read(profile, valid.encoded(), InputLimits::declared(118, 16)),
        Err(InputRefusal::EnvelopeTooLarge)
    );
    assert_eq!(
        pack(profile, &[7], InputLimits::declared(118, 16)),
        Err(InputRefusal::EnvelopeTooLarge)
    );
    assert_eq!(
        read(profile, valid.encoded(), InputLimits::declared(119, 0)),
        Err(InputRefusal::PayloadTooLarge)
    );
    assert_eq!(
        pack(profile, &[7], InputLimits::declared(119, 0)),
        Err(InputRefusal::PayloadTooLarge)
    );
    assert_eq!(
        read(profile, valid.encoded(), InputLimits::declared(119, 1)),
        Ok(valid)
    );
    Ok(())
}

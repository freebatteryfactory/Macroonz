//! Bounded admission of historical capsule bytes without live evidence mints.

use super::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedCapsule, ArchivedExecution,
    ArchivedFingerprint, ArchivedInput, ArchivedProfile, CAPSULE_ARCHIVE_TAG,
};
use crate::identity::{BodyReader, ContentAddress, addressed_body};
use crate::report::{
    ByteBudget, CaseBudget, EXECUTION_KEY_TAG, FINGERPRINT_TAG, FailureClass,
    INPUT_EXECUTION_KEY_TAG, InvocationProfile, REPLAY_CAPSULE_TAG, ReplayPosture, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity,
};

/// Read an owned historical capsule under independent resource ceilings.
///
/// # Errors
///
/// Refuses oversized, malformed, unsupported or internally contradictory material.
/// Integrity admission does not authenticate the writer or establish a current execution.
pub fn read_capsule(
    encoded: &[u8],
    limits: ArchiveLimits,
) -> Result<ArchivedCapsule, ArchiveRefusal> {
    if encoded.len() > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    let (address, body) = addressed_body(
        encoded,
        CAPSULE_ARCHIVE_TAG,
        |address| address,
        ArchiveRefusal::Truncated,
        |_derived| ArchiveRefusal::AddressMismatch,
    )?;
    let mut reader = cursor(body);
    let format = reader.u32()?;
    if format != 1 {
        return Err(ArchiveRefusal::UnsupportedFormat { found: format });
    }
    let kind = reader.u32()?;
    if kind != 1 {
        return Err(ArchiveRefusal::WrongKind { found: kind });
    }
    let custody = reader.u32()?;
    if custody != 0 {
        return Err(ArchiveRefusal::UnsupportedCustody { found: custody });
    }
    let key_bytes = frame(&mut reader, limits)?;
    let key = execution(key_bytes, &mut reader, limits)?;
    let fingerprint = fingerprint(frame(&mut reader, limits)?, limits)?;
    if key.trial != fingerprint.trial {
        return Err(ArchiveRefusal::IdentityJoinMismatch);
    }
    let capsule_bytes = frame(&mut reader, limits)?;
    finish(&reader)?;
    let mut capsule_reader = cursor(capsule_bytes);
    if claim(&mut capsule_reader, limits)?.as_bytes() != key.address.as_bytes() {
        return Err(ArchiveRefusal::IdentityJoinMismatch);
    }
    let input = frame(&mut capsule_reader, limits)?;
    if claim(&mut capsule_reader, limits)?.as_bytes() != fingerprint.address.as_bytes() {
        return Err(ArchiveRefusal::IdentityJoinMismatch);
    }
    let generation = profile(&mut capsule_reader, limits)?;
    let minimization = profile(&mut capsule_reader, limits)?;
    let schema = claim(&mut capsule_reader, limits)?;
    let posture = posture(capsule_reader.byte()?)?;
    finish(&capsule_reader)?;
    if key
        .input
        .as_ref()
        .is_some_and(|standing| standing.posture.slot() > posture.slot())
    {
        return Err(ArchiveRefusal::PostureMismatch);
    }
    Ok(ArchivedCapsule {
        encoded: encoded.to_vec(),
        address,
        identity: ContentAddress::derived(REPLAY_CAPSULE_TAG, capsule_bytes),
        key,
        fingerprint,
        input: input.to_vec(),
        generation,
        minimization,
        schema,
        posture,
    })
}

fn cursor(bytes: &[u8]) -> BodyReader<'_, ArchiveRefusal> {
    BodyReader::over(bytes, ArchiveRefusal::Truncated, |declared| {
        ArchiveRefusal::LengthOutsidePlatform { declared }
    })
}

fn frame<'body>(
    reader: &mut BodyReader<'body, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<&'body [u8], ArchiveRefusal> {
    let bytes = reader.bytes()?;
    if bytes.len() > limits.field() {
        return Err(ArchiveRefusal::FieldTooLarge);
    }
    Ok(bytes)
}

fn text<'body>(
    reader: &mut BodyReader<'body, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<&'body str, ArchiveRefusal> {
    core::str::from_utf8(frame(reader, limits)?).map_err(|_invalid| ArchiveRefusal::InvalidText)
}

fn claim(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<AddressClaim, ArchiveRefusal> {
    <[u8; 32]>::try_from(frame(reader, limits)?)
        .map(AddressClaim)
        .map_err(|_wrong_width| ArchiveRefusal::InvalidAddressWidth)
}

fn finish(reader: &BodyReader<'_, ArchiveRefusal>) -> Result<(), ArchiveRefusal> {
    if reader.remaining() != 0 {
        return Err(ArchiveRefusal::TrailingBytes);
    }
    Ok(())
}

fn posture(slot: u8) -> Result<ReplayPosture, ArchiveRefusal> {
    match slot {
        0 => Ok(ReplayPosture::ExactDerived),
        1 => Ok(ReplayPosture::DeclaredByAuthor),
        2 => Ok(ReplayPosture::UnavailableBecauseUntracked),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn profile(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedProfile, ArchiveRefusal> {
    Ok(ArchivedProfile {
        name: text(reader, limits)?.to_owned(),
        version: reader.u32()?,
    })
}

fn execution(
    bytes: &[u8],
    envelope: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedExecution, ArchiveRefusal> {
    let mut reader = cursor(bytes);
    let trial = claim(&mut reader, limits)?;
    let subject = claim(&mut reader, limits)?;
    let check = claim(&mut reader, limits)?;
    let invocation = InvocationProfile::declared(
        CaseBudget::declared(reader.u32()?),
        ByteBudget::declared(reader.u64()?),
        TimeBudget::declared(reader.u64()?),
    );
    let target = TargetBinding::bound(
        TargetTriple::declared(text(&mut reader, limits)?),
        ToolchainIdentity::declared(text(&mut reader, limits)?),
    );
    let (tag, input) = match envelope.byte()? {
        0 => (EXECUTION_KEY_TAG, None),
        1 => {
            let namespace = text(envelope, limits)?.to_owned();
            let profile = profile(envelope, limits)?;
            if namespace.is_empty() || profile.name.is_empty() {
                return Err(ArchiveRefusal::InvalidText);
            }
            let schema = claim(envelope, limits)?;
            let case = claim(&mut reader, limits)?;
            let decoder = claim(&mut reader, limits)?;
            let posture = posture(reader.byte()?)?;
            (
                INPUT_EXECUTION_KEY_TAG,
                Some(ArchivedInput {
                    namespace,
                    profile,
                    schema,
                    case,
                    decoder,
                    posture,
                }),
            )
        }
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    finish(&reader)?;
    Ok(ArchivedExecution {
        address: ContentAddress::derived(tag, bytes),
        trial,
        subject,
        check,
        invocation,
        target,
        input,
    })
}

fn fingerprint(bytes: &[u8], limits: ArchiveLimits) -> Result<ArchivedFingerprint, ArchiveRefusal> {
    let mut reader = cursor(bytes);
    let trial = claim(&mut reader, limits)?;
    let family = text(&mut reader, limits)?.to_owned();
    let local = text(&mut reader, limits)?.to_owned();
    let class = match reader.byte()? {
        0 => FailureClass::RefusedByCheck,
        1 => FailureClass::PropertyDisagreement,
        2 => FailureClass::OracleDisagreement,
        3 => FailureClass::SubjectPanic,
        4 => FailureClass::BudgetExhausted,
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    finish(&reader)?;
    Ok(ArchivedFingerprint {
        address: ContentAddress::derived(FINGERPRINT_TAG, bytes),
        trial,
        family,
        local,
        class,
    })
}

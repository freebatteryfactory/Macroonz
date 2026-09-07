//! Shared bounded readings of canonical report identity material.

use super::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedExecution, ArchivedFingerprint,
    ArchivedInput, ArchivedProfile,
};
use crate::identity::{BodyReader, ContentAddress, DomainTag, addressed_body};
use crate::report::{
    ByteBudget, CaseBudget, EXECUTION_KEY_TAG, FINGERPRINT_TAG, FailureClass,
    INPUT_EXECUTION_KEY_TAG, InvocationProfile, ReplayPosture, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity,
};

pub(super) fn cursor(bytes: &[u8]) -> BodyReader<'_, ArchiveRefusal> {
    BodyReader::over(bytes, ArchiveRefusal::Truncated, |declared| {
        ArchiveRefusal::LengthOutsidePlatform { declared }
    })
}

pub(super) fn frame<'body>(
    reader: &mut BodyReader<'body, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<&'body [u8], ArchiveRefusal> {
    let bytes = reader.bytes()?;
    if bytes.len() > limits.field() {
        return Err(ArchiveRefusal::FieldTooLarge);
    }
    Ok(bytes)
}

pub(super) fn text<'body>(
    reader: &mut BodyReader<'body, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<&'body str, ArchiveRefusal> {
    core::str::from_utf8(frame(reader, limits)?).map_err(|_invalid| ArchiveRefusal::InvalidText)
}

pub(super) fn claim(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<AddressClaim, ArchiveRefusal> {
    <[u8; 32]>::try_from(frame(reader, limits)?)
        .map(AddressClaim)
        .map_err(|_wrong_width| ArchiveRefusal::InvalidAddressWidth)
}

pub(super) fn finish(reader: &BodyReader<'_, ArchiveRefusal>) -> Result<(), ArchiveRefusal> {
    if reader.remaining() != 0 {
        return Err(ArchiveRefusal::TrailingBytes);
    }
    Ok(())
}

pub(super) fn posture(slot: u8) -> Result<ReplayPosture, ArchiveRefusal> {
    match slot {
        0 => Ok(ReplayPosture::ExactDerived),
        1 => Ok(ReplayPosture::DeclaredByAuthor),
        2 => Ok(ReplayPosture::UnavailableBecauseUntracked),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

pub(super) fn profile(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedProfile, ArchiveRefusal> {
    Ok(ArchivedProfile {
        name: text(reader, limits)?.to_owned(),
        version: reader.u32()?,
    })
}

pub(super) fn execution(
    bytes: &[u8],
    envelope: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedExecution, ArchiveRefusal> {
    let mut reader = cursor(bytes);
    let trial = claim(&mut reader, limits)?;
    let subject = claim(&mut reader, limits)?;
    let check = claim(&mut reader, limits)?;
    let (invocation, target) = context(&mut reader, limits)?;
    let (tag, input) = match envelope.byte()? {
        0 => (EXECUTION_KEY_TAG, None),
        1 => (
            INPUT_EXECUTION_KEY_TAG,
            Some(input(&mut reader, envelope, limits)?),
        ),
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

pub(super) fn context(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<(InvocationProfile, TargetBinding), ArchiveRefusal> {
    let invocation = InvocationProfile::declared(
        CaseBudget::declared(reader.u32()?),
        ByteBudget::declared(reader.u64()?),
        TimeBudget::declared(reader.u64()?),
    );
    let target = TargetBinding::bound(
        TargetTriple::declared(text(reader, limits)?),
        ToolchainIdentity::declared(text(reader, limits)?),
    );
    Ok((invocation, target))
}

pub(super) fn input(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    metadata: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedInput, ArchiveRefusal> {
    let namespace = text(metadata, limits)?.to_owned();
    let profile = profile(metadata, limits)?;
    if namespace.is_empty() || profile.name.is_empty() {
        return Err(ArchiveRefusal::InvalidText);
    }
    Ok(ArchivedInput {
        namespace,
        profile,
        schema: claim(metadata, limits)?,
        case: claim(reader, limits)?,
        decoder: claim(reader, limits)?,
        posture: posture(reader.byte()?)?,
    })
}

pub(super) fn fingerprint(
    bytes: &[u8],
    limits: ArchiveLimits,
) -> Result<ArchivedFingerprint, ArchiveRefusal> {
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

pub(super) fn envelope(
    encoded: &[u8],
    tag: DomainTag,
    expected_kind: u32,
    limits: ArchiveLimits,
) -> Result<(ContentAddress, BodyReader<'_, ArchiveRefusal>), ArchiveRefusal> {
    if encoded.len() > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    let (address, body) = addressed_body(
        encoded,
        tag,
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
    if kind != expected_kind {
        return Err(ArchiveRefusal::WrongKind { found: kind });
    }
    let custody = reader.u32()?;
    if custody != 0 {
        return Err(ArchiveRefusal::UnsupportedCustody { found: custody });
    }
    Ok((address, reader))
}

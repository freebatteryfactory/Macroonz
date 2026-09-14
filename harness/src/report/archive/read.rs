//! Bounded admission of historical capsule bytes without live evidence mints.

use super::identity::{
    claim, cursor, envelope, execution, fingerprint, finish, frame, posture, profile,
};
use super::{ArchiveLimits, ArchiveRefusal, ArchivedCapsule, CAPSULE_ARCHIVE_TAG};
use crate::identity::ContentAddress;
use crate::report::REPLAY_CAPSULE_TAG;

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
    let (address, mut reader) = envelope(encoded, CAPSULE_ARCHIVE_TAG, 1, limits)?;
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

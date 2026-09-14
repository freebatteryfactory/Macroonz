//! Historical capsule encoding through the report owner's existing canonical preimages.

use super::{ArchiveLimits, ArchiveRefusal, ArchivedCapsule, CAPSULE_ARCHIVE_TAG, read_capsule};
use crate::identity::{ContentAddress, encode_bytes};
use crate::report::{
    ExecutionInput, ExecutionKey, FindingCause, Fingerprint, GenerationProfile,
    MinimizationProfile, ReplayCapsule, execution_key_preimage, fingerprint_preimage,
    input_execution_key_preimage, replay_capsule_preimage,
};

/// Retain an earned capsule as bounded historical data for caller-owned storage.
///
/// # Errors
///
/// Refuses independent field or envelope ceilings before allocating canonical preimages.
pub fn retain_capsule(
    capsule: &ReplayCapsule,
    limits: ArchiveLimits,
) -> Result<ArchivedCapsule, ArchiveRefusal> {
    let total = capsule_size(
        capsule.key(),
        capsule.fingerprint(),
        capsule.input(),
        capsule.generation(),
        capsule.minimization(),
        limits,
    )?;
    let fingerprint = capsule.fingerprint();
    let fingerprint_bytes = fingerprint_preimage(
        fingerprint.trial(),
        fingerprint.cause(),
        fingerprint.class(),
    );
    let capsule_bytes = replay_capsule_preimage(capsule);
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    write_execution(capsule.key(), &mut body);
    encode_bytes(&fingerprint_bytes, &mut body);
    encode_bytes(&capsule_bytes, &mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(CAPSULE_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_capsule(&encoded, limits)
}

pub(crate) fn sum(parts: &[usize]) -> Result<usize, ArchiveRefusal> {
    parts.iter().try_fold(0usize, |total, part| {
        total
            .checked_add(*part)
            .ok_or(ArchiveRefusal::SizeOutsidePlatform)
    })
}

pub(crate) fn bounded(length: usize, limits: ArchiveLimits) -> Result<usize, ArchiveRefusal> {
    if length > limits.field() {
        return Err(ArchiveRefusal::FieldTooLarge);
    }
    Ok(length)
}

pub(crate) fn capsule_size(
    key: &ExecutionKey,
    fingerprint: Fingerprint,
    input: &[u8],
    generation: GenerationProfile,
    minimization: MinimizationProfile,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let execution_size = execution_size(key, limits)?;
    let fingerprint_size = fingerprint_size(fingerprint.cause(), limits)?;
    let input = bounded(input.len(), limits)?;
    let generation = bounded(generation.name().len(), limits)?;
    let minimization = bounded(minimization.name().len(), limits)?;
    let capsule_size = bounded(sum(&[153, input, generation, minimization])?, limits)?;
    let total = sum(&[60, execution_size, fingerprint_size, capsule_size])?;
    if total > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    Ok(total)
}

pub(crate) fn fingerprint_size(
    cause: FindingCause,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let family = bounded(cause.family().len(), limits)?;
    let local = bounded(cause.local().len(), limits)?;
    bounded(sum(&[57, family, local])?, limits)
}

pub(crate) fn execution_size(
    key: &ExecutionKey,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    bounded(32, limits)?;
    let target = bounded(key.target().target().spelling().len(), limits)?;
    let toolchain = bounded(key.target().toolchain().spelling().len(), limits)?;
    let input_tail = if key.input().is_some() {
        81usize
    } else {
        0usize
    };
    let key_size = bounded(sum(&[156, target, toolchain, input_tail])?, limits)?;
    let metadata = if let Some(standing) = key.input() {
        let profile = standing.profile();
        let namespace = bounded(profile.name().namespace().written().len(), limits)?;
        let name = bounded(profile.name().stem().written().len(), limits)?;
        sum(&[60, namespace, name])?
    } else {
        0
    };
    sum(&[9, key_size, metadata])
}

pub(crate) fn write_execution(key: &ExecutionKey, body: &mut Vec<u8>) {
    let key_bytes = key.input().map_or_else(
        || {
            execution_key_preimage(
                key.trial(),
                key.subject(),
                key.check(),
                key.invocation(),
                key.target(),
            )
        },
        |input| input_execution_key_preimage(key, input),
    );
    encode_bytes(&key_bytes, body);
    if let Some(input) = key.input() {
        body.push(1);
        write_input_metadata(input, body);
    } else {
        body.push(0);
    }
}

pub(super) fn write_input_metadata(input: ExecutionInput, body: &mut Vec<u8>) {
    let profile = input.profile();
    profile.name().encode_into(body);
    body.extend_from_slice(&profile.version().to_be_bytes());
    encode_bytes(profile.schema().as_bytes(), body);
}

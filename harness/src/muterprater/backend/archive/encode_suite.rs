//! Historical suite-pressure encoding composes the existing complete backend archive.

use super::types::OriginalMaterial;
use super::{
    ArchivedSuitePressure, SUITE_PRESSURE_ARCHIVE_TAG, SuitePressureArchiveLimits,
    SuitePressureArchiveRefusal, read_suite_pressure,
};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::muterprater::{
    BackendVersionPosture, CompiledSuitePressure, GrammarStanding, MutationVerdict,
};
use crate::report::archive::{ArchiveRefusal, bounded, sum};

/// Retain complete suite pressure with the manifest's absent-originals posture.
///
/// # Errors
///
/// Refuses outer or nested bounds and contradictory profile or first-kill joins.
pub fn retain_suite_pressure(
    pressure: &CompiledSuitePressure,
    limits: SuitePressureArchiveLimits,
) -> Result<ArchivedSuitePressure, SuitePressureArchiveRefusal> {
    retained(pressure, None, limits)
}

/// Retain complete suite pressure with caller-held console and complete original sources.
///
/// # Errors
///
/// Refuses independent bounds, contradictory joins and original material outside the backend owner's custody contract.
pub fn retain_suite_pressure_with_material(
    pressure: &CompiledSuitePressure,
    console: &str,
    sources: &[(&str, &[u8])],
    limits: SuitePressureArchiveLimits,
) -> Result<ArchivedSuitePressure, SuitePressureArchiveRefusal> {
    retained(
        pressure,
        Some(OriginalMaterial { console, sources }),
        limits,
    )
}

pub(crate) fn retained(
    pressure: &CompiledSuitePressure,
    original: Option<OriginalMaterial<'_>>,
    limits: SuitePressureArchiveLimits,
) -> Result<ArchivedSuitePressure, SuitePressureArchiveRefusal> {
    let checked = checked_version(pressure)?;
    let manifest = pressure.custody().manifest();
    let ordinal = first_kill(pressure)?;
    let total = suite_pressure_size(pressure, original, limits)?;
    let backend = super::encode::retained(manifest, original, limits.backend())?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 1, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    body.push(1);
    encode_bytes(checked.as_bytes(), &mut body);
    encode_bytes(backend.encoded(), &mut body);
    encode_length(ordinal, &mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded
        .extend_from_slice(ContentAddress::derived(SUITE_PRESSURE_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_suite_pressure(&encoded, limits)
}

fn first_kill(pressure: &CompiledSuitePressure) -> Result<usize, SuitePressureArchiveRefusal> {
    let (ordinal, kill) = pressure
        .custody()
        .manifest()
        .reading()
        .run()
        .reports()
        .iter()
        .enumerate()
        .find(|(_, report)| report.verdict() == MutationVerdict::Killed)
        .ok_or(SuitePressureArchiveRefusal::FirstKillMismatch)?;
    if kill != pressure.kill() {
        return Err(SuitePressureArchiveRefusal::FirstKillMismatch);
    }
    u64::try_from(ordinal).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    Ok(ordinal)
}

pub(crate) fn suite_pressure_size(
    pressure: &CompiledSuitePressure,
    original: Option<OriginalMaterial<'_>>,
    limits: SuitePressureArchiveLimits,
) -> Result<usize, SuitePressureArchiveRefusal> {
    let checked = checked_version(pressure)?;
    first_kill(pressure)?;
    let manifest = pressure.custody().manifest();
    let total = sum(&[
        69,
        bounded(checked.len(), limits.bytes())?,
        bounded(
            super::size::encoded_size(manifest, original, limits.backend())?,
            limits.bytes(),
        )?,
    ])?;
    if total > limits.bytes().envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}

fn checked_version(pressure: &CompiledSuitePressure) -> Result<&str, SuitePressureArchiveRefusal> {
    let qualification = pressure.qualification();
    let GrammarStanding::Checked(checked) = qualification.standing() else {
        return Err(SuitePressureArchiveRefusal::QualificationMismatch);
    };
    let BackendVersionPosture::Stated(stated) = qualification.profile().version() else {
        return Err(SuitePressureArchiveRefusal::QualificationMismatch);
    };
    if qualification.profile() != pressure.custody().manifest().reading().profile()
        || stated != checked
    {
        return Err(SuitePressureArchiveRefusal::QualificationMismatch);
    }
    Ok(checked.spelling())
}

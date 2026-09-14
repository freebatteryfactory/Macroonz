//! Historical suite-pressure joins without current custody or adapter qualification.

use super::super::{
    ArchivedSuitePressure, SUITE_PRESSURE_ARCHIVE_TAG, SuitePressureArchiveLimits,
    SuitePressureArchiveRefusal, read_backend,
};
use crate::muterprater::MutationVerdict;
use crate::report::archive::{ArchiveRefusal, envelope, finish, frame, text};

/// Read a complete historical suite-pressure claim under independent bounds.
///
/// # Errors
///
/// Refuses malformed nested data, unchecked or mismatched grammar claims, and any ordinal other than the run's first kill.
pub fn read_suite_pressure(
    encoded: &[u8],
    limits: SuitePressureArchiveLimits,
) -> Result<ArchivedSuitePressure, SuitePressureArchiveRefusal> {
    let (address, mut reader) = envelope(encoded, SUITE_PRESSURE_ARCHIVE_TAG, 1, limits.bytes())?;
    match reader.byte()? {
        1 => {}
        0 => return Err(SuitePressureArchiveRefusal::QualificationMismatch),
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    }
    let checked_version = text(&mut reader, limits.bytes())?;
    let manifest = read_backend(frame(&mut reader, limits.bytes())?, limits.backend())?;
    let kill_ordinal = reader.u64()?;
    finish(&reader)?;
    if checked_version != manifest.profile().version() {
        return Err(SuitePressureArchiveRefusal::QualificationMismatch);
    }
    let (first, kill) = manifest
        .run()
        .reports()
        .iter()
        .enumerate()
        .find(|(_, report)| MutationVerdict::from(report.outcome()) == MutationVerdict::Killed)
        .ok_or(SuitePressureArchiveRefusal::FirstKillMismatch)?;
    if u64::try_from(first).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)? != kill_ordinal {
        return Err(SuitePressureArchiveRefusal::FirstKillMismatch);
    }
    let kill = kill.clone();
    Ok(ArchivedSuitePressure {
        encoded: encoded.to_vec(),
        address,
        checked_version: checked_version.to_owned(),
        manifest,
        kill_ordinal,
        kill,
    })
}

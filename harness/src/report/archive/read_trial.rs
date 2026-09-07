//! Historical trial admission without report, foreign-text or clock mints.

use super::super::{
    ArchiveLimits, ArchiveRefusal, ArchivedAttempt, ArchivedClockFailure, ArchivedConclusion,
    ArchivedFinding, ArchivedForeignText, ArchivedMeasurement, ArchivedSite, ArchivedTrial,
    ArchivedTruncation, TRIAL_ARCHIVE_TAG,
};
use super::identity::{envelope, execution, fingerprint, finish, frame, posture, text};
use crate::identity::BodyReader;
use crate::report::{FOREIGN_TEXT_MAX_BYTES, InfrastructureFault, SkipReason, TextFidelity};

/// Read an owned historical trial under independent byte ceilings.
///
/// # Errors
///
/// Refuses malformed, oversized, unsupported or internally contradictory records.
/// Admission establishes integrity and grants no current execution or writer authority.
pub fn read_trial(encoded: &[u8], limits: ArchiveLimits) -> Result<ArchivedTrial, ArchiveRefusal> {
    let (address, mut reader) = envelope(encoded, TRIAL_ARCHIVE_TAG, 2, limits)?;
    let key_bytes = frame(&mut reader, limits)?;
    let key = execution(key_bytes, &mut reader, limits)?;
    let posture = posture(reader.byte()?)?;
    if key
        .input()
        .is_some_and(|input| input.posture().slot() > posture.slot())
    {
        return Err(ArchiveRefusal::PostureMismatch);
    }
    let site = ArchivedSite {
        module_path: text(&mut reader, limits)?.to_owned(),
        file: text(&mut reader, limits)?.to_owned(),
        line: reader.u32()?,
        name: text(&mut reader, limits)?.to_owned(),
    };
    let attempt = attempt(&mut reader, limits)?;
    if let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) = &attempt
        && finding.fingerprint().trial() != key.trial()
    {
        return Err(ArchiveRefusal::IdentityJoinMismatch);
    }
    let measurement = measurement(&mut reader)?;
    finish(&reader)?;
    Ok(ArchivedTrial {
        encoded: encoded.to_vec(),
        address,
        key,
        posture,
        site,
        attempt,
        measurement,
    })
}

fn attempt(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedAttempt, ArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedAttempt::Executed(ArchivedConclusion::Passed)),
        1 => Ok(ArchivedAttempt::Executed(ArchivedConclusion::Refused(
            Box::new(finding(reader, limits)?),
        ))),
        2 => Ok(ArchivedAttempt::SkippedWithReason(skip(reader.byte()?)?)),
        3 => Ok(ArchivedAttempt::TimedOut),
        4 => Ok(ArchivedAttempt::InfrastructureFailed {
            fault: fault(reader.byte()?)?,
            foreign: foreign(reader, limits)?,
        }),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn skip(slot: u8) -> Result<SkipReason, ArchiveRefusal> {
    match slot {
        0 => Ok(SkipReason::BudgetExhausted),
        1 => Ok(SkipReason::TargetUnsupported),
        2 => Ok(SkipReason::PrerequisiteAbsent),
        3 => Ok(SkipReason::SatisfiedByCachedExecution),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn fault(slot: u8) -> Result<InfrastructureFault, ArchiveRefusal> {
    match slot {
        0 => Ok(InfrastructureFault::GenerationUnavailable),
        1 => Ok(InfrastructureFault::SupportAbsent),
        2 => Ok(InfrastructureFault::CaptureFailed),
        3 => Ok(InfrastructureFault::BackendUnavailable),
        4 => Ok(InfrastructureFault::BackendInitializationFailed),
        5 => Ok(InfrastructureFault::BackendExecutionUnresolved),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

pub(crate) fn finding(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedFinding, ArchiveRefusal> {
    Ok(ArchivedFinding {
        fingerprint: fingerprint(frame(reader, limits)?, limits)?,
        file: text(reader, limits)?.to_owned(),
        line: reader.u32()?,
        foreign: foreign(reader, limits)?,
    })
}

pub(crate) fn foreign(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<Option<ArchivedForeignText>, ArchiveRefusal> {
    match reader.byte()? {
        0 => return Ok(None),
        1 => {}
        _ => return Err(ArchiveRefusal::InvalidSlot),
    }
    let bytes = frame(reader, limits)?;
    if bytes.len() > FOREIGN_TEXT_MAX_BYTES {
        return Err(ArchiveRefusal::InvalidForeignText);
    }
    let truncation = match reader.byte()? {
        0 => ArchivedTruncation::Complete,
        1 => {
            let admitted = reader.u64()?;
            let offered = reader.u64()?;
            if usize::try_from(admitted) != Ok(bytes.len())
                || bytes.len() != FOREIGN_TEXT_MAX_BYTES
                || offered <= admitted
            {
                return Err(ArchiveRefusal::InvalidForeignText);
            }
            ArchivedTruncation::TruncatedAt { admitted, offered }
        }
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    let fidelity = match reader.byte()? {
        0 => TextFidelity::Exact,
        1 => TextFidelity::LossyReplacement,
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    if core::str::from_utf8(bytes).is_ok() != (fidelity == TextFidelity::Exact) {
        return Err(ArchiveRefusal::InvalidForeignText);
    }
    Ok(Some(ArchivedForeignText {
        bytes: bytes.to_vec(),
        truncation,
        fidelity,
    }))
}

fn measurement(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
) -> Result<ArchivedMeasurement, ArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedMeasurement::Observed(reader.u64()?)),
        1 => Ok(ArchivedMeasurement::Unavailable),
        2 => Ok(ArchivedMeasurement::Failed(clock_failure(reader)?)),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn clock_failure(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
) -> Result<ArchivedClockFailure, ArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedClockFailure::OpeningRefused),
        1 => Ok(ArchivedClockFailure::ClosingRefused),
        2 => Ok(ArchivedClockFailure::OpeningUnwound),
        3 => Ok(ArchivedClockFailure::ClosingUnwound),
        4 => {
            let opened = reader.u64()?;
            let closed = reader.u64()?;
            if closed >= opened {
                return Err(ArchiveRefusal::InvalidMeasurement);
            }
            Ok(ArchivedClockFailure::Regressed { opened, closed })
        }
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

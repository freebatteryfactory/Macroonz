//! Complete size admission before trial encoding allocates any preimage.

use super::encode::{bounded, execution_size, sum};
use super::{ArchiveLimits, ArchiveRefusal};
use crate::clock::{ClockFailure, MeasurementReading};
use crate::report::{ForeignText, RunAttempt, TrialConclusion, TrialReport, Truncation};

pub(super) fn encoded_size(
    report: &TrialReport,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let key = execution_size(report.standing().key(), limits)?;
    let site = report.site();
    let module = bounded(site.module_path().len(), limits)?;
    let file = bounded(site.file().len(), limits)?;
    let name = bounded(site.name().len(), limits)?;
    let attempt = attempt_size(report.attempt(), limits)?;
    let measurement = match report.measurement() {
        MeasurementReading::Observed(_) => 9,
        MeasurementReading::Unavailable => 1,
        MeasurementReading::Failed(ClockFailure::Regressed {
            opened: _,
            closed: _,
        }) => 18,
        MeasurementReading::Failed(
            ClockFailure::OpeningRefused
            | ClockFailure::ClosingRefused
            | ClockFailure::OpeningUnwound
            | ClockFailure::ClosingUnwound,
        ) => 2,
    };
    let total = sum(&[73, key, module, file, name, attempt, measurement])?;
    if total > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    Ok(total)
}

fn attempt_size(attempt: &RunAttempt, limits: ArchiveLimits) -> Result<usize, ArchiveRefusal> {
    match attempt {
        RunAttempt::Executed(TrialConclusion::Passed) | RunAttempt::TimedOut => Ok(1),
        RunAttempt::SkippedWithReason(_) => Ok(2),
        RunAttempt::InfrastructureFailed(failure) => {
            sum(&[2, foreign_size(failure.foreign(), limits)?])
        }
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            let family = bounded(finding.cause().family().len(), limits)?;
            let local = bounded(finding.cause().local().len(), limits)?;
            let fingerprint = bounded(sum(&[57, family, local])?, limits)?;
            let file = bounded(finding.located().file().len(), limits)?;
            sum(&[
                21,
                fingerprint,
                file,
                foreign_size(finding.foreign(), limits)?,
            ])
        }
    }
}

fn foreign_size(
    foreign: Option<&ForeignText>,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let Some(foreign) = foreign else {
        return Ok(1);
    };
    let bytes = bounded(foreign.bytes().len(), limits)?;
    let counts = match foreign.truncation() {
        Truncation::Complete => 0,
        Truncation::TruncatedAt { admitted, offered } => {
            u64::try_from(admitted).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
            u64::try_from(offered).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
            16
        }
    };
    sum(&[11, bytes, counts])
}

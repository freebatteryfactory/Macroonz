//! Complete size admission before trial encoding allocates any preimage.

use super::encode::{bounded, execution_size, fingerprint_size, sum};
use super::{ArchiveLimits, ArchiveRefusal};
use crate::clock::{ClockAttribution, ClockFailure, MeasurementReading};
use crate::report::{
    ForeignText, RunAttempt, TrialConclusion, TrialFinding, TrialReport, Truncation,
};

pub(crate) fn encoded_size(
    report: &TrialReport,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let key = execution_size(report.standing().key(), limits)?;
    let site = report.site();
    let module = bounded(site.module_path().len(), limits)?;
    let file = bounded(site.file().len(), limits)?;
    let name = bounded(site.name().len(), limits)?;
    let attempt = attempt_size(report.attempt(), limits)?;
    let measurement = measurement_size(report.measurement());
    let attribution = match report.clock_attribution() {
        ClockAttribution::Unspecified => 0,
        ClockAttribution::Synthetic | ClockAttribution::Monotonic => 1,
    };
    let total = sum(&[
        73,
        key,
        module,
        file,
        name,
        attempt,
        measurement,
        attribution,
    ])?;
    if total > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    Ok(total)
}

pub(crate) const fn measurement_size(reading: MeasurementReading) -> usize {
    match reading {
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
    }
}

fn attempt_size(attempt: &RunAttempt, limits: ArchiveLimits) -> Result<usize, ArchiveRefusal> {
    match attempt {
        RunAttempt::Executed(TrialConclusion::Passed) | RunAttempt::TimedOut => Ok(1),
        RunAttempt::SkippedWithReason(_) => Ok(2),
        RunAttempt::InfrastructureFailed(failure) => {
            sum(&[2, foreign_size(failure.foreign(), limits)?])
        }
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            sum(&[1, finding_size(finding, limits)?])
        }
    }
}

pub(crate) fn finding_size(
    finding: &TrialFinding,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let fingerprint = fingerprint_size(finding.cause(), limits)?;
    let file = bounded(finding.located().file().len(), limits)?;
    sum(&[
        20,
        fingerprint,
        file,
        foreign_size(finding.foreign(), limits)?,
    ])
}

pub(crate) fn foreign_size(
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

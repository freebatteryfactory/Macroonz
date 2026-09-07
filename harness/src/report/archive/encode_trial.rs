//! Historical trial encoding from the complete report, without executing its subject.

use super::encode::write_execution;
use super::size_trial::encoded_size;
use super::{ArchiveLimits, ArchiveRefusal, ArchivedTrial, TRIAL_ARCHIVE_TAG, read_trial};
use crate::clock::{ClockFailure, MeasurementReading};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::report::{
    ForeignText, InfrastructureFault, RunAttempt, SkipReason, TextFidelity, TrialConclusion,
    TrialId, TrialReport, Truncation, fingerprint_preimage,
};

/// Retain a complete trial as bounded historical data for caller-owned storage.
///
/// # Errors
///
/// Refuses independent field or envelope ceilings before allocating canonical preimages.
pub fn retain_trial(
    report: &TrialReport,
    limits: ArchiveLimits,
) -> Result<ArchivedTrial, ArchiveRefusal> {
    let total = encoded_size(report, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&2u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    write_execution(report.standing().key(), &mut body);
    body.push(report.standing().replay().slot());
    let site = report.site();
    encode_bytes(site.module_path().as_bytes(), &mut body);
    encode_bytes(site.file().as_bytes(), &mut body);
    body.extend_from_slice(&site.line().to_be_bytes());
    encode_bytes(site.name().as_bytes(), &mut body);
    attempt(report.trial(), report.attempt(), &mut body);
    measurement(report.measurement(), &mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(TRIAL_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_trial(&encoded, limits)
}

fn attempt(trial: TrialId, attempt: &RunAttempt, body: &mut Vec<u8>) {
    match attempt {
        RunAttempt::Executed(TrialConclusion::Passed) => body.push(0),
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            body.push(1);
            encode_bytes(
                &fingerprint_preimage(trial, finding.cause(), finding.class()),
                body,
            );
            encode_bytes(finding.located().file().as_bytes(), body);
            body.extend_from_slice(&finding.located().line().to_be_bytes());
            foreign(finding.foreign(), body);
        }
        RunAttempt::SkippedWithReason(reason) => {
            body.push(2);
            body.push(match reason {
                SkipReason::BudgetExhausted => 0,
                SkipReason::TargetUnsupported => 1,
                SkipReason::PrerequisiteAbsent => 2,
                SkipReason::SatisfiedByCachedExecution => 3,
            });
        }
        RunAttempt::TimedOut => body.push(3),
        RunAttempt::InfrastructureFailed(failure) => {
            body.push(4);
            body.push(match failure.fault() {
                InfrastructureFault::GenerationUnavailable => 0,
                InfrastructureFault::SupportAbsent => 1,
                InfrastructureFault::CaptureFailed => 2,
                InfrastructureFault::BackendUnavailable => 3,
                InfrastructureFault::BackendInitializationFailed => 4,
                InfrastructureFault::BackendExecutionUnresolved => 5,
            });
            foreign(failure.foreign(), body);
        }
    }
}

fn foreign(foreign: Option<&ForeignText>, body: &mut Vec<u8>) {
    let Some(foreign) = foreign else {
        body.push(0);
        return;
    };
    body.push(1);
    encode_bytes(foreign.bytes(), body);
    match foreign.truncation() {
        Truncation::Complete => body.push(0),
        Truncation::TruncatedAt { admitted, offered } => {
            body.push(1);
            encode_length(admitted, body);
            encode_length(offered, body);
        }
    }
    body.push(match foreign.fidelity() {
        TextFidelity::Exact => 0,
        TextFidelity::LossyReplacement => 1,
    });
}

fn measurement(reading: MeasurementReading, body: &mut Vec<u8>) {
    match reading {
        MeasurementReading::Observed(duration) => {
            body.push(0);
            body.extend_from_slice(&duration.nanoseconds().to_be_bytes());
        }
        MeasurementReading::Unavailable => body.push(1),
        MeasurementReading::Failed(failure) => {
            body.push(2);
            clock_failure(failure, body);
        }
    }
}

fn clock_failure(failure: ClockFailure, body: &mut Vec<u8>) {
    match failure {
        ClockFailure::OpeningRefused => body.push(0),
        ClockFailure::ClosingRefused => body.push(1),
        ClockFailure::OpeningUnwound => body.push(2),
        ClockFailure::ClosingUnwound => body.push(3),
        ClockFailure::Regressed { opened, closed } => {
            body.push(4);
            body.extend_from_slice(&opened.nanoseconds().to_be_bytes());
            body.extend_from_slice(&closed.nanoseconds().to_be_bytes());
        }
    }
}

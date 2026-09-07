//! Complete trial retention observed through independent vectors and both report roads.

use super::{
    fixture,
    trial_vector::{body, envelope, foreign, refusal},
};
use macroonz_harness::clock::{ClockFailure, HarnessClock, MeasurementReading, RecordedDuration};
use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, ArchivedAttempt, ArchivedClockFailure, ArchivedConclusion,
    ArchivedMeasurement, ArchivedTruncation, read_trial, retain_trial,
};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, ForeignText, InfrastructureFailure,
    InfrastructureFault, ReplayPosture, RunAttempt, SkipReason, TextFidelity, TrialConclusion,
    TrialFinding,
};

pub(super) const LIMITS: ArchiveLimits = ArchiveLimits::declared(16384, 8192);

#[test]
fn writers_preserve_all_skip_infrastructure_and_clock_failures() -> Result<(), ()> {
    for reason in [
        SkipReason::BudgetExhausted,
        SkipReason::TargetUnsupported,
        SkipReason::PrerequisiteAbsent,
        SkipReason::SatisfiedByCachedExecution,
    ] {
        let report = fixture::host_report(
            RunAttempt::SkippedWithReason(reason),
            MeasurementReading::Unavailable,
        )?;
        let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
        assert_eq!(
            record.attempt(),
            &ArchivedAttempt::SkippedWithReason(reason)
        );
    }
    for fault in [
        InfrastructureFault::GenerationUnavailable,
        InfrastructureFault::SupportAbsent,
        InfrastructureFault::CaptureFailed,
        InfrastructureFault::BackendUnavailable,
        InfrastructureFault::BackendInitializationFailed,
        InfrastructureFault::BackendExecutionUnresolved,
    ] {
        let report = fixture::host_report(
            RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(fault, None)),
            MeasurementReading::Unavailable,
        )?;
        let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
        assert_eq!(
            record.attempt(),
            &ArchivedAttempt::InfrastructureFailed {
                fault,
                foreign: None
            }
        );
    }
    for (live, historical) in [
        (
            ClockFailure::OpeningRefused,
            ArchivedClockFailure::OpeningRefused,
        ),
        (
            ClockFailure::ClosingRefused,
            ArchivedClockFailure::ClosingRefused,
        ),
        (
            ClockFailure::OpeningUnwound,
            ArchivedClockFailure::OpeningUnwound,
        ),
        (
            ClockFailure::ClosingUnwound,
            ArchivedClockFailure::ClosingUnwound,
        ),
    ] {
        let report = fixture::host_report(RunAttempt::TimedOut, MeasurementReading::Failed(live))?;
        let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
        assert_eq!(
            record.measurement(),
            ArchivedMeasurement::Failed(historical)
        );
        assert_eq!(record.attempt(), &ArchivedAttempt::TimedOut);
    }
    Ok(())
}

#[test]
fn an_actual_clock_regression_retains_both_ticks_without_changing_execution_identity()
-> Result<(), ()> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TICK: AtomicU64 = AtomicU64::new(23);
    let reading = HarnessClock::reading(|| TICK.fetch_sub(1, Ordering::SeqCst))
        .begin()
        .finish();
    let report = fixture::host_report(RunAttempt::Executed(TrialConclusion::Passed), reading)?;
    let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
    assert_eq!(
        record.measurement(),
        ArchivedMeasurement::Failed(ArchivedClockFailure::Regressed {
            opened: 23,
            closed: 22
        })
    );
    assert_eq!(
        record.attempt(),
        &ArchivedAttempt::Executed(ArchivedConclusion::Passed)
    );
    let exact = ArchiveLimits::declared(record.encoded().len(), LIMITS.field());
    assert_eq!(retain_trial(&report, exact).map_err(|_| ())?, record);
    let unavailable = fixture::host_report(
        RunAttempt::Executed(TrialConclusion::Passed),
        MeasurementReading::Unavailable,
    )?;
    let other = retain_trial(&unavailable, LIMITS).map_err(|_| ())?;
    assert_eq!(record.key(), other.key());
    assert_ne!(record.address(), other.address());
    Ok(())
}

#[test]
fn independent_trial_preserves_identity_site_finding_and_raw_clock_regression() -> Result<(), ()> {
    let mut measurement = vec![2, 4];
    measurement.extend_from_slice(&23u64.to_be_bytes());
    measurement.extend_from_slice(&7u64.to_be_bytes());
    let attempt = refusal(&foreign(&[0xff, b'a'], &[0, 1]));
    let record = read_trial(&envelope(&body(&attempt, &measurement)), LIMITS).map_err(|_| ())?;
    assert_eq!(record.key().trial().as_bytes(), &[1; 32]);
    assert_eq!(record.key().input().ok_or(())?.case().as_bytes(), &[4; 32]);
    assert_eq!(record.claimed_posture(), ReplayPosture::DeclaredByAuthor);
    assert_eq!(record.site().module_path(), "outside::subject");
    assert_eq!(record.site().file(), "subject.rs");
    assert_eq!(record.site().line(), 11);
    assert_eq!(record.site().name(), "original trial");
    assert_eq!(
        record.measurement(),
        ArchivedMeasurement::Failed(ArchivedClockFailure::Regressed {
            opened: 23,
            closed: 7
        })
    );
    let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) = record.attempt() else {
        return Err(());
    };
    assert_eq!(finding.fingerprint().family(), "fixture");
    assert_eq!(finding.fingerprint().local(), "disagrees");
    assert_eq!(
        finding.fingerprint().class(),
        FailureClass::PropertyDisagreement
    );
    assert_eq!(finding.fingerprint().trial(), record.key().trial());
    assert_eq!(finding.file(), "check.rs");
    assert_eq!(finding.line(), 29);
    let text = finding.foreign().ok_or(())?;
    assert_eq!(text.bytes(), &[0xff, b'a']);
    assert_eq!(text.truncation(), ArchivedTruncation::Complete);
    assert_eq!(text.fidelity(), TextFidelity::LossyReplacement);
    Ok(())
}

#[test]
fn every_attempt_and_measurement_slot_remains_distinct() -> Result<(), ()> {
    let mut attempts = vec![
        (
            vec![0],
            ArchivedAttempt::Executed(ArchivedConclusion::Passed),
        ),
        (vec![3], ArchivedAttempt::TimedOut),
    ];
    for (slot, reason) in [
        (0, SkipReason::BudgetExhausted),
        (1, SkipReason::TargetUnsupported),
        (2, SkipReason::PrerequisiteAbsent),
        (3, SkipReason::SatisfiedByCachedExecution),
    ] {
        attempts.push((vec![2, slot], ArchivedAttempt::SkippedWithReason(reason)));
    }
    for (slot, fault) in [
        (0, InfrastructureFault::GenerationUnavailable),
        (1, InfrastructureFault::SupportAbsent),
        (2, InfrastructureFault::CaptureFailed),
        (3, InfrastructureFault::BackendUnavailable),
        (4, InfrastructureFault::BackendInitializationFailed),
        (5, InfrastructureFault::BackendExecutionUnresolved),
    ] {
        attempts.push((
            vec![4, slot, 0],
            ArchivedAttempt::InfrastructureFailed {
                fault,
                foreign: None,
            },
        ));
    }
    let mut observed = vec![0];
    observed.extend_from_slice(&0u64.to_be_bytes());
    let measurements = [
        (observed, ArchivedMeasurement::Observed(0)),
        (vec![1], ArchivedMeasurement::Unavailable),
        (
            vec![2, 0],
            ArchivedMeasurement::Failed(ArchivedClockFailure::OpeningRefused),
        ),
        (
            vec![2, 1],
            ArchivedMeasurement::Failed(ArchivedClockFailure::ClosingRefused),
        ),
        (
            vec![2, 2],
            ArchivedMeasurement::Failed(ArchivedClockFailure::OpeningUnwound),
        ),
        (
            vec![2, 3],
            ArchivedMeasurement::Failed(ArchivedClockFailure::ClosingUnwound),
        ),
    ];
    for (attempt_bytes, attempt) in attempts {
        for (measurement_bytes, measurement) in &measurements {
            let record = read_trial(&envelope(&body(&attempt_bytes, measurement_bytes)), LIMITS)
                .map_err(|_| ())?;
            assert_eq!(record.attempt(), &attempt);
            assert_eq!(record.measurement(), *measurement);
        }
    }
    Ok(())
}

#[test]
fn actual_typed_reports_and_host_records_retain_complete_data() -> Result<(), ()> {
    for payload in [vec![2, 3], vec![1, 2, 3]] {
        let report = fixture::report(&payload)?;
        let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
        assert_eq!(record.key().address(), report.standing().key().address());
        assert_eq!(record.claimed_posture(), report.standing().replay());
        assert_eq!(record.site().file(), report.site().file());
        assert_eq!(record.site().line(), report.site().line());
        assert_eq!(
            read_trial(record.encoded(), LIMITS).map_err(|_| ())?,
            record
        );
    }
    let original = vec![0xff; 5001];
    let report = fixture::host_report(
        RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
            InfrastructureFault::CaptureFailed,
            Some(ForeignText::admitted(&original)),
        )),
        MeasurementReading::Observed(RecordedDuration::recorded(19)),
    )?;
    let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
    assert!(record.key().input().is_none());
    assert_eq!(record.measurement(), ArchivedMeasurement::Observed(19));
    let ArchivedAttempt::InfrastructureFailed { fault, foreign } = record.attempt() else {
        return Err(());
    };
    assert_eq!(*fault, InfrastructureFault::CaptureFailed);
    let foreign = foreign.as_ref().ok_or(())?;
    assert_eq!(foreign.bytes(), original.get(..4096).ok_or(())?);
    assert_eq!(
        foreign.truncation(),
        ArchivedTruncation::TruncatedAt {
            admitted: 4096,
            offered: 5001
        }
    );
    assert_eq!(foreign.fidelity(), TextFidelity::LossyReplacement);
    Ok(())
}

#[test]
fn retention_observes_exact_size_bounds_for_every_attempt_road() -> Result<(), ()> {
    let finding = TrialFinding::established(
        FailureClass::OracleDisagreement,
        FindingCause::named("caller", "cause"),
        FindingLocation::at("refusal.rs", 17),
        Some(ForeignText::admitted(b"exact")),
    );
    let attempts = [
        RunAttempt::Executed(TrialConclusion::Passed),
        RunAttempt::Executed(TrialConclusion::Refused(finding)),
        RunAttempt::SkippedWithReason(SkipReason::PrerequisiteAbsent),
        RunAttempt::TimedOut,
        RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
            InfrastructureFault::SupportAbsent,
            None,
        )),
    ];
    for attempt in attempts {
        let report = fixture::host_report(
            attempt,
            MeasurementReading::Failed(ClockFailure::ClosingUnwound),
        )?;
        let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
        let exact = ArchiveLimits::declared(record.encoded().len(), LIMITS.field());
        assert_eq!(retain_trial(&report, exact).map_err(|_| ())?, record);
        assert_eq!(read_trial(record.encoded(), exact).map_err(|_| ())?, record);
        let short =
            ArchiveLimits::declared(record.encoded().len().saturating_sub(1), LIMITS.field());
        assert_eq!(
            retain_trial(&report, short),
            Err(ArchiveRefusal::EnvelopeTooLarge)
        );
        assert_eq!(
            read_trial(record.encoded(), short),
            Err(ArchiveRefusal::EnvelopeTooLarge)
        );
        let narrow = ArchiveLimits::declared(LIMITS.envelope(), 31);
        assert_eq!(
            retain_trial(&report, narrow),
            Err(ArchiveRefusal::FieldTooLarge)
        );
        assert_eq!(
            read_trial(record.encoded(), narrow),
            Err(ArchiveRefusal::FieldTooLarge)
        );
    }
    Ok(())
}

#[test]
fn recomputed_integrity_cannot_hide_malformed_trials() -> Result<(), ()> {
    let bytes = body(&refusal(&foreign(b"exact", &[0, 0])), &[1]);
    for length in 0..bytes.len() {
        assert!(read_trial(&envelope(bytes.get(..length).ok_or(())?), LIMITS).is_err());
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert_eq!(
        read_trial(&envelope(&trailing), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    for (offset, value, refusal) in [
        (0usize, 2u32, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (4, 1, ArchiveRefusal::WrongKind { found: 1 }),
        (8, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut malformed = bytes.clone();
        malformed
            .get_mut(offset..offset.saturating_add(4))
            .ok_or(())?
            .copy_from_slice(&value.to_be_bytes());
        assert_eq!(read_trial(&envelope(&malformed), LIMITS), Err(refusal));
    }
    for attempt in [vec![9], vec![2, 9], vec![4, 9], refusal(&[9])] {
        assert_eq!(
            read_trial(&envelope(&body(&attempt, &[1])), LIMITS),
            Err(ArchiveRefusal::InvalidSlot)
        );
    }
    let mut mismatched = refusal(&[0]);
    *mismatched.get_mut(17).ok_or(())? ^= 1;
    assert_eq!(
        read_trial(&envelope(&body(&mismatched, &[1])), LIMITS),
        Err(ArchiveRefusal::IdentityJoinMismatch)
    );
    let mut enormous = bytes.clone();
    enormous
        .get_mut(12..20)
        .ok_or(())?
        .copy_from_slice(&u64::MAX.to_be_bytes());
    assert!(read_trial(&envelope(&enormous), LIMITS).is_err());
    let mut corrupt = envelope(&bytes);
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_trial(&corrupt, LIMITS),
        Err(ArchiveRefusal::AddressMismatch)
    );
    Ok(())
}

#[test]
fn foreign_loss_and_measurement_claims_must_match_their_material() -> Result<(), ()> {
    for (bytes, fidelity) in [(b"exact".as_slice(), 1), ([0xff].as_slice(), 0)] {
        let attempt = refusal(&foreign(bytes, &[0, fidelity]));
        assert_eq!(
            read_trial(&envelope(&body(&attempt, &[1])), LIMITS),
            Err(ArchiveRefusal::InvalidForeignText)
        );
    }
    let mut tail = vec![1];
    tail.extend_from_slice(&4096u64.to_be_bytes());
    tail.extend_from_slice(&u64::MAX.to_be_bytes());
    tail.push(0);
    let offered = foreign(&vec![b'a'; 4096], &tail);
    let record = read_trial(&envelope(&body(&refusal(&offered), &[1])), LIMITS).map_err(|_| ())?;
    let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) = record.attempt() else {
        return Err(());
    };
    assert_eq!(
        finding.foreign().ok_or(())?.truncation(),
        ArchivedTruncation::TruncatedAt {
            admitted: 4096,
            offered: u64::MAX
        }
    );
    for count in [4095u64, 4096] {
        tail.get_mut(9..17)
            .ok_or(())?
            .copy_from_slice(&count.to_be_bytes());
        let attempt = refusal(&foreign(&vec![b'a'; 4096], &tail));
        assert_eq!(
            read_trial(&envelope(&body(&attempt, &[1])), LIMITS),
            Err(ArchiveRefusal::InvalidForeignText)
        );
    }
    for closed in [23u64, 24] {
        let mut measurement = vec![2, 4];
        measurement.extend_from_slice(&23u64.to_be_bytes());
        measurement.extend_from_slice(&closed.to_be_bytes());
        assert_eq!(
            read_trial(&envelope(&body(&[0], &measurement)), LIMITS),
            Err(ArchiveRefusal::InvalidMeasurement)
        );
    }
    for measurement in [vec![9], vec![2, 9]] {
        assert_eq!(
            read_trial(&envelope(&body(&[0], &measurement)), LIMITS),
            Err(ArchiveRefusal::InvalidSlot)
        );
    }
    Ok(())
}

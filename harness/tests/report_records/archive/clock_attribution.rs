//! Source classification survives actual execution and historical custody without gaining verdict authority.

use super::{fixture, process, run_fixture, trial_vector, trials::LIMITS};
use macroonz_harness::clock::{
    ClockAttribution, ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading,
    RecordedDuration,
};
use macroonz_harness::descriptor::Origin;
use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, ArchivedDisposition, RunArchiveLimits, read_run, read_trial,
    retain_run, retain_trial,
};
use macroonz_harness::report::{HostTrialRecord, RunAttempt, TrialConclusion};
use macroonz_harness::runner::{
    Selection, SelectionPlan, record_one, run_all, run_one, trial_identity,
};

const SYNTHETIC: HarnessClock = HarnessClock::reading_as(zero, ClockAttribution::Synthetic);
const DECLARED_MONOTONIC: HarnessClock =
    HarnessClock::fallible_as(refuses, ClockAttribution::Monotonic);

fn zero() -> u64 {
    0
}

fn refuses() -> Result<u64, ClockReadRefusal> {
    Err(ClockReadRefusal::Refused)
}

fn attributed_body(attempt: &[u8], measurement: &[u8], slot: u8) -> Result<Vec<u8>, ()> {
    let mut body = trial_vector::body(attempt, measurement);
    body.get_mut(..4)
        .ok_or(())?
        .copy_from_slice(&2u32.to_be_bytes());
    body.push(slot);
    Ok(body)
}

#[test]
fn constant_sources_and_actual_trials_keep_declared_attribution_beside_their_readings()
-> Result<(), ()> {
    let binding = fixture::binding(|_| TrialConclusion::Passed)?;
    let baseline = run_one(&binding, &fixture::invocation());
    for (clock, attribution, reading) in [
        (
            HarnessClock::unavailable(),
            ClockAttribution::Unspecified,
            MeasurementReading::Unavailable,
        ),
        (
            HarnessClock::reading(zero),
            ClockAttribution::Unspecified,
            MeasurementReading::Observed(RecordedDuration::recorded(0)),
        ),
        (
            SYNTHETIC,
            ClockAttribution::Synthetic,
            MeasurementReading::Observed(RecordedDuration::recorded(0)),
        ),
        (
            DECLARED_MONOTONIC,
            ClockAttribution::Monotonic,
            MeasurementReading::Failed(ClockFailure::OpeningRefused),
        ),
    ] {
        assert_eq!(clock.attribution(), attribution);
        let report = run_one(&binding, &fixture::invocation_with(clock));
        assert_eq!(report.clock_attribution(), attribution);
        assert_eq!(report.measurement(), reading);
        assert_eq!(report.attempt(), baseline.attempt());
        assert_eq!(report.standing(), baseline.standing());
        let archived = retain_trial(&report, LIMITS).map_err(|_| ())?;
        assert_eq!(archived.clock_attribution(), attribution);
        assert_eq!(
            read_trial(archived.encoded(), LIMITS).map_err(|_| ())?,
            archived
        );
    }
    Ok(())
}

#[test]
fn host_join_preserves_its_own_attribution_and_never_borrows_the_invocation_label() -> Result<(), ()>
{
    let binding = fixture::binding(|_| TrialConclusion::Passed)?;
    let trial = trial_identity(binding.row());
    let invocation = fixture::invocation_with(DECLARED_MONOTONIC);
    let reading = MeasurementReading::Observed(RecordedDuration::recorded(0));
    let legacy = HostTrialRecord::recorded(trial, RunAttempt::TimedOut, reading);
    assert_eq!(legacy.clock_attribution(), ClockAttribution::Unspecified);
    let legacy = record_one(&binding, &invocation, legacy).map_err(|_| ())?;
    assert_eq!(legacy.clock_attribution(), ClockAttribution::Unspecified);
    for attribution in [ClockAttribution::Synthetic, ClockAttribution::Monotonic] {
        let host = HostTrialRecord::recorded_with_attribution(
            trial,
            RunAttempt::TimedOut,
            reading,
            attribution,
        );
        assert_eq!(host.clock_attribution(), attribution);
        let report = record_one(&binding, &invocation, host).map_err(|_| ())?;
        assert_eq!(report.clock_attribution(), attribution);
        assert_eq!(report.measurement(), legacy.measurement());
        assert_eq!(report.attempt(), legacy.attempt());
        assert_eq!(report.standing(), legacy.standing());
    }
    Ok(())
}

#[test]
fn independent_versions_retain_declared_source_slots_and_old_unspecified_standing() -> Result<(), ()>
{
    let old_body = trial_vector::body(&[0], &[1]);
    let old = read_trial(&trial_vector::envelope(&old_body), LIMITS).map_err(|_| ())?;
    assert_eq!(old.clock_attribution(), ClockAttribution::Unspecified);
    for (slot, attribution) in [
        (1, ClockAttribution::Synthetic),
        (2, ClockAttribution::Monotonic),
    ] {
        let body = attributed_body(&[0], &[1], slot)?;
        let record = read_trial(&trial_vector::envelope(&body), LIMITS).map_err(|_| ())?;
        assert_eq!(record.clock_attribution(), attribution);
        assert_eq!(record.key(), old.key());
        assert_eq!(record.attempt(), old.attempt());
        assert_eq!(record.measurement(), old.measurement());
        assert_ne!(record.address(), old.address());
    }
    let mut trailing_old = old_body;
    trailing_old.push(1);
    assert_eq!(
        read_trial(&trial_vector::envelope(&trailing_old), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    Ok(())
}

#[test]
fn writers_preserve_old_bytes_and_independently_place_the_new_source_slot() -> Result<(), ()> {
    let readings = [
        MeasurementReading::Observed(RecordedDuration::recorded(0)),
        MeasurementReading::Unavailable,
        MeasurementReading::Failed(ClockFailure::OpeningRefused),
        MeasurementReading::Failed(ClockFailure::ClosingRefused),
        MeasurementReading::Failed(ClockFailure::OpeningUnwound),
        MeasurementReading::Failed(ClockFailure::ClosingUnwound),
    ];
    for reading in readings {
        let old_report = fixture::host_report(RunAttempt::TimedOut, reading)?;
        let old = retain_trial(&old_report, LIMITS).map_err(|_| ())?;
        assert_eq!(old.encoded().get(32..36).ok_or(())?, &1u32.to_be_bytes());
        for (attribution, slot) in [
            (ClockAttribution::Synthetic, 1),
            (ClockAttribution::Monotonic, 2),
        ] {
            let report = fixture::host_report_as(RunAttempt::TimedOut, reading, attribution)?;
            let record = retain_trial(&report, LIMITS).map_err(|_| ())?;
            let mut expected_body = old.encoded().get(32..).ok_or(())?.to_vec();
            expected_body
                .get_mut(..4)
                .ok_or(())?
                .copy_from_slice(&2u32.to_be_bytes());
            expected_body.push(slot);
            assert_eq!(record.encoded(), trial_vector::envelope(&expected_body));
            assert_eq!(record.clock_attribution(), attribution);
            assert_eq!(record.key(), old.key());
            assert_eq!(record.measurement(), old.measurement());
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
        }
    }
    Ok(())
}

#[test]
fn attributed_regressions_and_hostile_headers_cannot_hide_behind_recomputed_integrity()
-> Result<(), ()> {
    let mut measurement = vec![2, 4];
    measurement.extend_from_slice(&23u64.to_be_bytes());
    measurement.extend_from_slice(&7u64.to_be_bytes());
    for slot in [1, 2] {
        let body = attributed_body(&[0], &measurement, slot)?;
        let record = read_trial(&trial_vector::envelope(&body), LIMITS).map_err(|_| ())?;
        assert_eq!(
            record.measurement(),
            macroonz_harness::report::archive::ArchivedMeasurement::Failed(
                macroonz_harness::report::archive::ArchivedClockFailure::Regressed {
                    opened: 23,
                    closed: 7
                }
            )
        );
        for length in 0..body.len() {
            assert!(
                read_trial(
                    &trial_vector::envelope(body.get(..length).ok_or(())?),
                    LIMITS
                )
                .is_err()
            );
        }
        let mut trailing = body.clone();
        trailing.push(slot);
        assert_eq!(
            read_trial(&trial_vector::envelope(&trailing), LIMITS),
            Err(ArchiveRefusal::TrailingBytes)
        );
        for bad in [0, 3, 255] {
            let mut malformed = body.clone();
            *malformed.last_mut().ok_or(())? = bad;
            assert_eq!(
                read_trial(&trial_vector::envelope(&malformed), LIMITS),
                Err(ArchiveRefusal::InvalidSlot)
            );
        }
    }
    Ok(())
}

#[test]
fn attributed_nested_run_records_survive_a_fresh_process() -> Result<(), ()> {
    let table = run_fixture::table(vec![
        run_fixture::binding("first", Origin::HandWritten, |_| TrialConclusion::Passed)?,
        run_fixture::binding("second", Origin::HandWritten, |_| TrialConclusion::Passed)?,
    ])?;
    let limits = RunArchiveLimits::declared(ArchiveLimits::declared(32768, 16384), 2);
    for clock in [SYNTHETIC, DECLARED_MONOTONIC] {
        let report = run_all(
            &table.view(),
            &SelectionPlan::of(Selection::All),
            &fixture::invocation_with(clock),
        );
        let record = retain_run(&report, limits).map_err(|_| ())?;
        for row in record.census() {
            let ArchivedDisposition::Selected(trial) = row.disposition() else {
                return Err(());
            };
            assert_eq!(trial.clock_attribution(), clock.attribution());
            assert_eq!(
                process::round_trip(trial.encoded(), "archive::process::child_loads_trial")
                    .map_err(|_| ())?,
                trial.encoded()
            );
        }
        let encoded = process::round_trip(record.encoded(), "archive::process::child_loads_run")
            .map_err(|_| ())?;
        assert_eq!(read_run(&encoded, limits).map_err(|_| ())?, record);
    }
    Ok(())
}

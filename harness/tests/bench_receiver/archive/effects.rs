//! Actual clock outcomes and caller counters distinguish retention from execution.

use super::super::fixture as receiver;
use super::{
    LIMITS, fixture,
    types::{ReportVector, RowVector},
    wire,
};
use macroonz_harness::bench::archive::{ArchivedBenchOutcome, read_report, retain_report};
use macroonz_harness::bench::{
    BenchBinding, WorkJudgment, WorkJudgmentInput, WorkRecorder, WorkRecordingRefusal,
};
use macroonz_harness::clock::{ClockAttribution, ClockReadRefusal, HarnessClock};
use macroonz_harness::descriptor::Provenance;
use macroonz_harness::report::TrialConclusion;
use macroonz_harness::report::archive::{ArchivedClockFailure, ArchivedMeasurement};
use macroonz_harness::runner::Invocation;
use std::error::Error;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

static MODE: AtomicU64 = AtomicU64::new(0);
static READS: AtomicUsize = AtomicUsize::new(0);
static CALLS: AtomicU64 = AtomicU64::new(0);

fn varied_clock() -> Result<u64, ClockReadRefusal> {
    let at = READS.fetch_add(1, Ordering::SeqCst);
    let closing = !at.is_multiple_of(2);
    match MODE.load(Ordering::SeqCst) {
        0 => [100, 101, 110, 112, 120, 123, 130, 134, 140, 145, 150, 156]
            .get(at)
            .copied()
            .ok_or(ClockReadRefusal::Refused),
        1 => Ok(50),
        2 => Err(ClockReadRefusal::Refused),
        3 if closing => Err(ClockReadRefusal::Refused),
        4 => std::panic::resume_unwind(Box::new("declared opening unwind control")),
        5 if closing => std::panic::resume_unwind(Box::new("declared closing unwind control")),
        3 | 5 => Ok(100),
        _ => [100, 99, 101, 100, 102, 101, 103, 102, 104, 103, 105, 104]
            .get(at)
            .copied()
            .ok_or(ClockReadRefusal::Refused),
    }
}

fn expected(mode: u64, sample: u64) -> (ArchivedMeasurement, Vec<u8>) {
    assert!(sample < 6);
    match mode {
        0 | 1 => {
            let value = if mode == 0 {
                sample.saturating_add(1)
            } else {
                0
            };
            let mut bytes = vec![0];
            bytes.extend_from_slice(&value.to_be_bytes());
            (ArchivedMeasurement::Observed(value), bytes)
        }
        2 => (
            ArchivedMeasurement::Failed(ArchivedClockFailure::OpeningRefused),
            vec![2, 0],
        ),
        3 => (
            ArchivedMeasurement::Failed(ArchivedClockFailure::ClosingRefused),
            vec![2, 1],
        ),
        4 => (
            ArchivedMeasurement::Failed(ArchivedClockFailure::OpeningUnwound),
            vec![2, 2],
        ),
        5 => (
            ArchivedMeasurement::Failed(ArchivedClockFailure::ClosingUnwound),
            vec![2, 3],
        ),
        _ => {
            let opened = sample.saturating_add(100);
            let closed = sample.saturating_add(99);
            let mut bytes = vec![2, 4];
            bytes.extend_from_slice(&opened.to_be_bytes());
            bytes.extend_from_slice(&closed.to_be_bytes());
            (
                ArchivedMeasurement::Failed(ArchivedClockFailure::Regressed { opened, closed }),
                bytes,
            )
        }
    }
}

#[test]
fn every_actual_measurement_outcome_keeps_sample_order_and_separate_attribution()
-> Result<(), Box<dyn Error>> {
    let row = receiver::lawful_row().map_err(|e| format!("{e:?}"))?;
    let attachment = receiver::lawful_attachment(
        receiver::measured,
        receiver::planted_worse,
        receiver::lawful_judge,
    )
    .map_err(|e| format!("{e:?}"))?;
    let preflight = receiver::preflight_with_clock(
        row.preflight(),
        receiver::preflight_passes,
        receiver::target(),
        HarnessClock::fallible_as(
            || Err(ClockReadRefusal::Refused),
            ClockAttribution::Monotonic,
        ),
    )
    .map_err(|e| format!("{e:?}"))?;
    let binding = BenchBinding::bound(row, attachment, preflight).map_err(|e| format!("{e:?}"))?;
    for mode in 0..7 {
        for (attribution, slot) in [
            (ClockAttribution::Unspecified, 0),
            (ClockAttribution::Synthetic, 1),
            (ClockAttribution::Monotonic, 2),
        ] {
            MODE.store(mode, Ordering::SeqCst);
            READS.store(0, Ordering::SeqCst);
            let report = fixture::report(
                vec![binding.clone()],
                Provenance::Unproduced,
                HarnessClock::fallible_as(varied_clock, attribution),
            )?;
            let trial = fixture::preflights(&report)?
                .into_iter()
                .next()
                .ok_or("trial")?;
            let mut vector_row = RowVector::qualified(trial, &[2, 4, 8]);
            let expected: Vec<_> = (0..6).map(|sample| expected(mode, sample)).collect();
            vector_row.secondary = wire::secondary(
                &vector_row.measured,
                &vector_row.judgment,
                slot,
                &expected
                    .iter()
                    .map(|(_, bytes)| bytes.clone())
                    .collect::<Vec<_>>(),
            );
            let retained = retain_report(&report, LIMITS).map_err(|e| format!("retain: {e:?}"))?;
            assert_eq!(
                retained.encoded(),
                ReportVector::declared(vec![vector_row]).encoded()
            );
            let reading = retained.readings().first().ok_or("reading")?;
            assert_eq!(
                reading.preflight().clock_attribution(),
                ClockAttribution::Monotonic
            );
            assert_eq!(
                reading.preflight().measurement(),
                ArchivedMeasurement::Failed(ArchivedClockFailure::OpeningRefused)
            );
            let ArchivedBenchOutcome::Qualified { secondary, .. } = reading.outcome() else {
                return Err("clock changed work qualification".into());
            };
            assert_eq!(secondary.clock_attribution(), attribution);
            assert_eq!(
                secondary.measurements(),
                expected
                    .iter()
                    .map(|(measurement, _)| *measurement)
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                READS.load(Ordering::SeqCst),
                if mode == 2 || mode == 4 { 6 } else { 12 }
            );
        }
    }
    Ok(())
}

fn counted_preflight(_invocation: &Invocation) -> TrialConclusion {
    CALLS.fetch_add(1, Ordering::SeqCst);
    TrialConclusion::Passed
}

fn counted_measured(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    CALLS.fetch_add(1, Ordering::SeqCst);
    receiver::measured(size, recorder)
}

fn counted_worse(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    CALLS.fetch_add(1, Ordering::SeqCst);
    receiver::planted_worse(size, recorder)
}

fn counted_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    CALLS.fetch_add(1, Ordering::SeqCst);
    receiver::lawful_judge(input)
}

fn counted_clock() -> u64 {
    CALLS.fetch_add(1, Ordering::SeqCst);
    0
}

#[test]
fn retaining_and_loading_never_reinvoke_any_execution_seat() -> Result<(), Box<dyn Error>> {
    CALLS.store(0, Ordering::SeqCst);
    let binding = receiver::binding(
        counted_measured,
        counted_worse,
        counted_judge,
        counted_preflight,
    )
    .map_err(|e| format!("{e:?}"))?;
    let report = fixture::report(
        vec![binding],
        Provenance::Unproduced,
        HarnessClock::reading_as(counted_clock, ClockAttribution::Synthetic),
    )?;
    let calls = CALLS.load(Ordering::SeqCst);
    assert_eq!(calls, 36);
    let record = retain_report(&report, LIMITS).map_err(|e| format!("retain: {e:?}"))?;
    drop(report);
    let mut bytes = record.encoded().to_vec();
    let loaded = read_report(&bytes, LIMITS).map_err(|e| format!("read: {e:?}"))?;
    bytes.fill(0);
    assert_eq!(loaded, record);
    assert!(read_report(&bytes, LIMITS).is_err());
    assert_eq!(CALLS.load(Ordering::SeqCst), calls);
    Ok(())
}

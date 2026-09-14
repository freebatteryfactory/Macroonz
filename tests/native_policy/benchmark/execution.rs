//! Actual work executes while preflight and work refusals keep timing from granting success.

use super::{
    declaration,
    specimen::{self, mapped},
};
use crate::presentation_formats::{self, field};
use macroonz::harness::bench::{
    BenchOutcome, BenchRunRefusal, BenchStage, PrimaryWorkPhase, SecondaryObservationRefusal,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal, bench_verdict, run_all,
};
use macroonz::harness::clock::{ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading};
use macroonz::presentation;

#[test]
fn actual_work_control_and_judge_execute_before_secondary_measurements() -> Result<(), String> {
    super::fixture::reset();
    let table = declaration::table(vec![declaration::binding(
        "count",
        super::fixture::measured,
        super::fixture::worse,
        super::fixture::judge,
        super::fixture::preflight,
    )?])?;
    let report = mapped(run_all(
        &table,
        &declaration::invocation(HarnessClock::unavailable()),
    ))?;
    mapped(bench_verdict(&report))?;
    assert_eq!(super::fixture::calls(), (15, 6, 2, 1));
    let [reading] = report.readings() else {
        return Err("missing row".to_owned());
    };
    let BenchOutcome::Qualified {
        measured,
        planted_worse,
        secondary,
        ..
    } = reading.outcome()
    else {
        return Err("work did not qualify".to_owned());
    };
    let counts = |curve: &macroonz::harness::bench::WorkCurve| {
        curve
            .points()
            .iter()
            .flat_map(|point| point.counts().iter().map(|count| count.count()))
            .collect::<Vec<_>>()
    };
    assert_eq!(counts(measured), [8, 16, 32]);
    assert_eq!(counts(planted_worse), [32, 128, 512]);
    assert_eq!(secondary.work(), measured);
    assert_eq!(
        secondary.measurements(),
        &[MeasurementReading::Unavailable; 6]
    );
    Ok(())
}

#[test]
fn failed_preflight_executes_neither_work_control_nor_judge() -> Result<(), String> {
    super::fixture::reset();
    let table = declaration::table(vec![declaration::binding(
        "wrong",
        super::fixture::measured,
        super::fixture::worse,
        super::fixture::judge,
        super::fixture::refused_preflight,
    )?])?;
    let report = mapped(run_all(
        &table,
        &declaration::invocation(HarnessClock::unavailable()),
    ))?;
    assert_eq!(super::fixture::calls(), (0, 0, 0, 0));
    assert_eq!(
        report
            .readings()
            .first()
            .ok_or("missing row")?
            .outcome()
            .stage(),
        BenchStage::PreflightRefused
    );
    assert!(bench_verdict(&report).is_err());
    Ok(())
}

fn unrecordable(_size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    let unknown = WorkObservationRef::named("outside.benchmark", "undeclared")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(unknown, 1)
}

#[test]
fn incomplete_primary_recording_publishes_no_report() -> Result<(), String> {
    let table = declaration::table(vec![declaration::binding(
        "unrecordable",
        unrecordable,
        specimen::worse,
        specimen::judge,
        specimen::preflight,
    )?])?;
    let refusal = run_all(
        &table,
        &declaration::invocation(HarnessClock::unavailable()),
    )
    .err()
    .ok_or("unrecordable work published a report")?;
    assert!(matches!(
        refusal,
        BenchRunRefusal::WorkNotRecorded {
            phase: PrimaryWorkPhase::Measured,
            ..
        }
    ));
    let shown = presentation::benchmark_refusal(&refusal);
    presentation_formats::agree(&shown)?;
    let value = mapped(serde_json::from_str(&shown.json()))?;
    assert_eq!(field(&value, "/kind")?, "benchmark-run-refusal");
    assert_eq!(field(&value, "/record/kind")?, "work-not-recorded");
    assert_eq!(field(&value, "/record/value/phase")?, "measured");
    assert_eq!(
        field(&value, "/record/value/refusal/kind")?,
        "unknown-observation"
    );
    assert_eq!(
        field(&value, "/record/value/refusal/value/stem")?,
        "undeclared"
    );
    Ok(())
}

fn fails_after_primary(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    if super::fixture::MEASURED.get() >= 6 {
        unrecordable(size, recorder)
    } else {
        super::fixture::measured(size, recorder)
    }
}

#[test]
fn secondary_work_refusal_remains_an_execution_error() -> Result<(), String> {
    super::fixture::reset();
    let table = declaration::table(vec![declaration::binding(
        "drifts",
        fails_after_primary,
        specimen::worse,
        specimen::judge,
        specimen::preflight,
    )?])?;
    let refusal = run_all(
        &table,
        &declaration::invocation(HarnessClock::unavailable()),
    )
    .err()
    .ok_or("secondary refusal published a report")?;
    assert!(matches!(
        refusal,
        BenchRunRefusal::SecondaryWorkRefused {
            refusal: SecondaryObservationRefusal::Warmup(_),
            ..
        }
    ));
    let shown = presentation::benchmark_refusal(&refusal);
    presentation_formats::agree(&shown)?;
    let value = mapped(serde_json::from_str(&shown.json()))?;
    assert_eq!(field(&value, "/record/kind")?, "secondary-work-refused");
    assert_eq!(field(&value, "/record/value/refusal/kind")?, "warmup");
    assert_eq!(
        field(&value, "/record/value/refusal/value/kind")?,
        "unknown-observation"
    );
    Ok(())
}

#[test]
fn clock_failure_does_not_replace_work_qualification() -> Result<(), String> {
    let table = declaration::table(vec![declaration::binding(
        "clock",
        specimen::measured,
        specimen::worse,
        specimen::judge,
        specimen::preflight,
    )?])?;
    let clock = HarnessClock::fallible(|| Err(ClockReadRefusal::Refused));
    let report = mapped(run_all(&table, &declaration::invocation(clock)))?;
    mapped(bench_verdict(&report))?;
    let BenchOutcome::Qualified { secondary, .. } =
        report.readings().first().ok_or("missing row")?.outcome()
    else {
        return Err("missing qualification".to_owned());
    };
    assert_eq!(
        secondary.measurements(),
        &[MeasurementReading::Failed(ClockFailure::OpeningRefused); 6]
    );
    Ok(())
}

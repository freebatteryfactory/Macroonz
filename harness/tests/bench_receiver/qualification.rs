//! Claim: correctness, planted-worse sensitivity, and measured-work judgment gate timing in that order.
//! Subject: the public receiver's primary qualification road.
//! Population: one lawful linear workload and hostile controls that erase or reverse each qualifying distinction.
//! Reversal: wrong work, an identical control, a vacuous judge, and an undisclosed gap all withhold timing.
//! Denominator: every primary stage and the lawful reading's complete curves.
//! Evidence ceiling: receiver ordering and owner-judge sensitivity, not an adopter performance result.
//! Retained regression: clock-before-work, vacuous control, favorable-time rescue, or incomplete qualified evidence.

use super::{fixture, support::*};
use std::sync::atomic::Ordering;

fn assert_work_point(
    point: &macroonz_harness::bench::WorkCurvePoint,
    input_size: u64,
    observation: WorkObservationRef,
    count: u64,
) {
    assert_eq!(point.input_size(), input_size);
    assert_eq!(point.counts().len(), 1usize);
    for found in point.counts() {
        assert_eq!(found.observation(), observation);
        assert_eq!(found.count(), count);
    }
}

#[test]
fn lawful_receiver_retains_complete_primary_and_secondary_readings() -> Result<(), BenchRoadFailure>
{
    let table = fixture::lawful_table()?;
    let report = run_all(&table, &fixture::invocation())?;
    assert_eq!(report.denominator(), table.bindings().len());
    bench_verdict(&report)?;
    fixture::render(&report);
    let reading = first_reading(&report)?;
    let binding = table
        .bindings()
        .first()
        .ok_or(BenchRoadFailure::MissingReading)?;
    assert_eq!(reading.row(), binding.row());
    assert_eq!(reading.row().formula(), binding.row().formula());
    assert_eq!(reading.row().complexity(), binding.row().complexity());
    let BenchOutcome::Qualified {
        measured,
        planted_worse,
        judgment,
        secondary,
        ..
    } = reading.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert_eq!(
        measured.points().len(),
        binding.row().input_sizes().sizes().len()
    );
    assert_eq!(planted_worse.points().len(), measured.points().len());
    let observation = WorkObservationRef::named("harness.bench.consumer", "unit-work")?;
    let [measured_two, measured_four, measured_eight] = measured.points() else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert_work_point(measured_two, 2u64, observation, 4u64);
    assert_work_point(measured_four, 4u64, observation, 8u64);
    assert_work_point(measured_eight, 8u64, observation, 16u64);
    let [worse_two, worse_four, worse_eight] = planted_worse.points() else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert_work_point(worse_two, 2u64, observation, 8u64);
    assert_work_point(worse_four, 4u64, observation, 32u64);
    assert_work_point(worse_eight, 8u64, observation, 128u64);
    assert!(judgment.qualifies());
    assert_eq!(secondary.work(), measured);
    let expected_measurements = binding
        .row()
        .input_sizes()
        .sizes()
        .iter()
        .flat_map(|_| 0..binding.row().budgets().samples())
        .count();
    assert_eq!(secondary.measurements().len(), expected_measurements);
    assert!(secondary.judgment().qualifies());
    assert!(
        secondary
            .measurements()
            .iter()
            .all(|measurement| matches!(measurement, MeasurementReading::Observed(duration) if duration.nanoseconds() > 0u64))
    );
    Ok(())
}

#[test]
fn planted_worse_and_judge_controls_are_non_vacuous() -> Result<(), BenchRoadFailure> {
    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let same_callable = fixture::binding(
        fixture::measured,
        fixture::measured,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let same_callable = run_all(
        &table_with(same_callable)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&same_callable)?.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    let Err(verdict) = bench_verdict(&same_callable) else {
        return Err(BenchRoadFailure::MissingVerdictRefusal);
    };
    assert_eq!(verdict.row(), first_reading(&same_callable)?.row().key());
    assert_eq!(verdict.stage(), BenchStage::PlantedWorseNotDistinguished);

    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let always_satisfy = fixture::binding(
        fixture::measured,
        fixture::planted_worse,
        always_satisfy,
        fixture::preflight_passes,
    )?;
    let always_satisfy = run_all(
        &table_with(always_satisfy)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&always_satisfy)?.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);

    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let gap_not_distinguished = fixture::binding(
        fixture::measured,
        fixture::planted_worse,
        refuse_worse_without_gap,
        fixture::preflight_passes,
    )?;
    let gap_not_distinguished = run_all(
        &table_with(gap_not_distinguished)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&gap_not_distinguished)?.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);

    let always_refuse = fixture::binding(
        fixture::measured,
        fixture::planted_worse,
        always_refuse,
        fixture::preflight_passes,
    )?;
    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let always_refuse = run_all(
        &table_with(always_refuse)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&always_refuse)?.outcome().stage(),
        BenchStage::PrimaryWorkRefused
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    Ok(())
}

#[test]
fn damaged_measured_work_cannot_be_rescued_by_favorable_time() -> Result<(), BenchRoadFailure> {
    PRIMARY_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let binding = fixture::binding(
        zeroed_measured_work,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let report = run_all(
        &table_with(binding)?,
        &fixture::invocation_with(HarnessClock::reading(primary_counted_clock)),
    )?;
    let reading = first_reading(&report)?;
    assert_eq!(reading.outcome().stage(), BenchStage::PrimaryWorkRefused);
    let BenchOutcome::PrimaryWorkRefused { judgment, .. } = reading.outcome() else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert!(matches!(judgment.measured(), WorkConclusion::Refused(_)));
    assert!(matches!(
        judgment.planted_worse(),
        WorkConclusion::Refused(_)
    ));
    assert_eq!(judgment.gap(), WorkGapStanding::Distinguished);
    assert_eq!(PRIMARY_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    Ok(())
}

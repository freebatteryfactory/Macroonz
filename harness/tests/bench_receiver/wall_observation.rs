//! Claim: clock readings are secondary observations and unavailable is not observed zero.
//! Subject: qualified reports produced under different caller-declared clocks.
//! Population: one lawful table under fast, slow, unavailable, and zero-valued clocks.
//! Reversal: clock values move while primary work and judgments must remain equal.
//! Denominator: every timed sample in the fixture's complete input axis.
//! Evidence ceiling: typed separation of work and local clock readings, not host timing stability.
//! Retained regression: clock-dependent qualification or collapsing unavailable into zero.

use super::{fixture, support::*};

#[test]
fn wall_readings_change_without_changing_primary_qualification() -> Result<(), BenchRoadFailure> {
    let table = fixture::lawful_table()?;
    let fast = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(fast_clock)),
    )?;
    let slow = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(slow_clock)),
    )?;
    bench_verdict(&fast)?;
    bench_verdict(&slow)?;
    let BenchOutcome::Qualified {
        measured: fast_work,
        planted_worse: fast_worse,
        judgment: fast_judgment,
        secondary: fast_secondary,
        ..
    } = first_reading(&fast)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    let BenchOutcome::Qualified {
        measured: slow_work,
        planted_worse: slow_worse,
        judgment: slow_judgment,
        secondary: slow_secondary,
        ..
    } = first_reading(&slow)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert_eq!(fast_work, slow_work);
    assert_eq!(fast_worse, slow_worse);
    assert_eq!(fast_judgment, slow_judgment);
    assert!(fast_judgment.qualifies());
    assert_eq!(fast_secondary.work(), slow_secondary.work());
    assert_eq!(fast_secondary.judgment(), slow_secondary.judgment());
    assert_ne!(fast_secondary.measurements(), slow_secondary.measurements());
    Ok(())
}

#[test]
fn unavailable_wall_readings_are_not_observed_zero() -> Result<(), BenchRoadFailure> {
    let table = fixture::lawful_table()?;
    let unavailable = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::unavailable()),
    )?;
    let zero = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(zero_clock)),
    )?;
    bench_verdict(&unavailable)?;
    bench_verdict(&zero)?;
    let BenchOutcome::Qualified {
        secondary: unavailable_secondary,
        ..
    } = first_reading(&unavailable)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    let BenchOutcome::Qualified {
        secondary: zero_secondary,
        ..
    } = first_reading(&zero)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert!(
        unavailable_secondary
            .measurements()
            .iter()
            .all(|reading| *reading == MeasurementReading::Unavailable)
    );
    assert!(zero_secondary.measurements().iter().all(|reading| matches!(
        reading,
        MeasurementReading::Observed(duration) if duration.nanoseconds() == 0u64
    )));
    Ok(())
}

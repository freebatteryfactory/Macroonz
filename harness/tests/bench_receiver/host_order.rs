//! Claim: target admission and correctness preflight run before every benchmark callable and clock read.
//! Subject: `run_all` admission, primary recording, and secondary refusal behavior.
//! Population: target/toolchain mismatches, a refused preflight, recorder failures, and timed-pass drift.
//! Reversal: counters make any early caller-code execution observable and hostile recorders force each refusal road.
//! Denominator: every callable class the receiver may invoke for the affected rows.
//! Evidence ceiling: local process ordering and typed refusal priority, not cross-process scheduling.
//! Retained regression: partial execution before admission, recorder errors becoming reports, or timed drift becoming qualification.

use super::{fixture, support::*};
use std::sync::atomic::Ordering;

#[test]
fn refused_preflight_withholds_every_benchmark_callable_and_clock() -> Result<(), BenchRoadFailure>
{
    PREFLIGHT_MEASURED_CALLS.store(0u64, Ordering::SeqCst);
    PREFLIGHT_WORSE_CALLS.store(0u64, Ordering::SeqCst);
    PREFLIGHT_JUDGE_CALLS.store(0u64, Ordering::SeqCst);
    PREFLIGHT_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let binding = fixture::binding(
        preflight_counted_measured,
        preflight_counted_worse,
        preflight_counted_judge,
        preflight_refuses,
    )?;
    let report = run_all(
        &table_with(binding)?,
        &fixture::invocation_with(HarnessClock::reading(preflight_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&report)?.outcome().stage(),
        BenchStage::PreflightRefused
    );
    assert_eq!(PREFLIGHT_MEASURED_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(PREFLIGHT_WORSE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(PREFLIGHT_JUDGE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(PREFLIGHT_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    Ok(())
}

#[test]
fn target_mismatch_refuses_before_any_benchmark_caller_code() -> Result<(), BenchRoadFailure> {
    reset_target_counters();
    let (table, expected_refusal_row) = table_with_foreign_preflight(
        "complete-target-prevalidation",
        TargetBinding::bound(
            TargetTriple::declared("foreign-target"),
            ToolchainIdentity::declared("1.98.0"),
        ),
    )?;
    let expected_benchmark_target = fixture::target().target().clone();
    let target_result = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(target_counted_clock)),
    );
    assert!(matches!(
        target_result,
        Err(BenchRunRefusal::PreflightTargetMismatch {
            row,
            mismatch: BenchTargetMismatch::Target {
                benchmark,
                preflight,
            },
        }) if row == expected_refusal_row
            && benchmark == expected_benchmark_target
            && preflight == TargetTriple::declared("foreign-target")
    ));
    assert_target_callables_were_withheld();
    Ok(())
}

#[test]
fn toolchain_mismatch_refuses_before_any_benchmark_caller_code() -> Result<(), BenchRoadFailure> {
    reset_target_counters();
    let (table, expected_refusal_row) = table_with_foreign_preflight(
        "complete-toolchain-prevalidation",
        TargetBinding::bound(
            fixture::target().target().clone(),
            ToolchainIdentity::declared("foreign-toolchain"),
        ),
    )?;
    let expected_benchmark_toolchain = fixture::target().toolchain().clone();
    let toolchain_result = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(target_counted_clock)),
    );
    assert!(matches!(
        toolchain_result,
        Err(BenchRunRefusal::PreflightTargetMismatch {
            row,
            mismatch: BenchTargetMismatch::Toolchain {
                benchmark,
                preflight,
            },
        }) if row == expected_refusal_row
            && benchmark == expected_benchmark_toolchain
            && preflight == ToolchainIdentity::declared("foreign-toolchain")
    ));
    assert_target_callables_were_withheld();
    Ok(())
}

#[test]
fn recorder_and_secondary_failures_never_become_qualified_reports() -> Result<(), BenchRoadFailure>
{
    let unknown = fixture::binding(
        unknown_observation,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    assert!(matches!(
        run_all(&table_with(unknown)?, &fixture::invocation()),
        Err(BenchRunRefusal::WorkNotRecorded {
            phase: PrimaryWorkPhase::Measured,
            refusal: WorkRecordingRefusal::UnknownObservation(_),
            ..
        })
    ));

    let overflow = fixture::binding(
        overflowing_count,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    assert!(matches!(
        run_all(&table_with(overflow)?, &fixture::invocation()),
        Err(BenchRunRefusal::WorkNotRecorded {
            phase: PrimaryWorkPhase::Measured,
            refusal: WorkRecordingRefusal::CountOverflow { .. },
            ..
        })
    ));

    DRIFT_CALLS.store(0u64, Ordering::SeqCst);
    let drift = fixture::binding(
        drifting_measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let primary_calls = drift
        .row()
        .input_sizes()
        .sizes()
        .iter()
        .flat_map(|_| 0..drift.row().budgets().samples())
        .count();
    DRIFT_PRIMARY_CALLS.store(u64::try_from(primary_calls)?, Ordering::SeqCst);
    assert!(matches!(
        run_all(&table_with(drift)?, &fixture::invocation()),
        Err(BenchRunRefusal::SecondaryWorkRefused {
            refusal: SecondaryObservationRefusal::Judgment(_),
            ..
        })
    ));
    Ok(())
}

#[test]
fn planted_worse_amount_overflow_names_its_phase_and_input() -> Result<(), BenchRoadFailure> {
    let binding = BenchBinding::bound(
        fixture::row_with_axis(vec![2u64, u64::MAX])?,
        fixture::lawful_attachment(
            zeroed_measured_work,
            fixture::planted_worse,
            fixture::lawful_judge,
        )?,
        fixture::lawful_preflight(fixture::preflight_passes)?,
    )
    .map_err(BenchStampRefusal::from)?;
    assert!(matches!(
        run_all(&table_with(binding)?, &fixture::invocation()),
        Err(BenchRunRefusal::WorkNotRecorded {
            phase: PrimaryWorkPhase::PlantedWorse,
            refusal: WorkRecordingRefusal::AmountOverflow {
                input_size: u64::MAX,
                ..
            },
            ..
        })
    ));
    Ok(())
}

#[test]
fn timed_warmup_and_sample_refusals_remain_distinct() -> Result<(), BenchRoadFailure> {
    TIMED_WARMUP_CALLS.store(0u64, Ordering::SeqCst);
    let warmup = fixture::binding(
        refusing_timed_warmup,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let warmup_primary_calls = warmup
        .row()
        .input_sizes()
        .sizes()
        .iter()
        .flat_map(|_| 0..warmup.row().budgets().samples())
        .count();
    let warmup_primary_calls = u64::try_from(warmup_primary_calls)?;
    TIMED_WARMUP_FAILURE_AT.store(warmup_primary_calls, Ordering::SeqCst);
    assert!(matches!(
        run_all(&table_with(warmup)?, &fixture::invocation()),
        Err(BenchRunRefusal::SecondaryWorkRefused {
            refusal: SecondaryObservationRefusal::Warmup(WorkRecordingRefusal::UnknownObservation(
                _
            )),
            ..
        })
    ));

    TIMED_SAMPLE_CALLS.store(0u64, Ordering::SeqCst);
    let sample = fixture::binding(
        refusing_timed_sample,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let sample_primary_calls = sample
        .row()
        .input_sizes()
        .sizes()
        .iter()
        .flat_map(|_| 0..sample.row().budgets().samples())
        .count();
    let sample_primary_calls = u64::try_from(sample_primary_calls)?;
    TIMED_SAMPLE_FAILURE_AT.store(
        sample_primary_calls.saturating_add(u64::from(sample.row().budgets().warmups())),
        Ordering::SeqCst,
    );
    assert!(matches!(
        run_all(&table_with(sample)?, &fixture::invocation()),
        Err(BenchRunRefusal::SecondaryWorkRefused {
            refusal: SecondaryObservationRefusal::Sample(WorkRecordingRefusal::UnknownObservation(
                _
            )),
            ..
        })
    ));
    Ok(())
}

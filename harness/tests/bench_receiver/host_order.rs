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

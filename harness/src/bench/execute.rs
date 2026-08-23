//! Complete-table structural admission, primary work execution, secondary observation, and verdict reading.

use super::types::{
    BenchBinding, BenchInvocation, BenchOutcome, BenchReading, BenchReport, BenchRunRefusal,
    BenchTable, BenchTargetMismatch, BenchVerdictRefusal, PrimaryWorkPhase, SecondaryObservation,
    SecondaryObservationRefusal, WorkCurve, WorkJudgment, WorkJudgmentInput,
    WorkQualificationRefusal, WorkRecorder, WorkRecordingRefusal,
};
use crate::runner::{lens_verdict, run_one};

fn validate_table(table: &BenchTable, invocation: &BenchInvocation) -> Result<(), BenchRunRefusal> {
    for binding in table.bindings() {
        let row = binding.row();
        let preflight_target = binding.preflight().invocation().target();
        let mismatch = if preflight_target.target() != invocation.target().target() {
            Some(BenchTargetMismatch::Target {
                benchmark: invocation.target().target().clone(),
                preflight: preflight_target.target().clone(),
            })
        } else if preflight_target.toolchain() != invocation.target().toolchain() {
            Some(BenchTargetMismatch::Toolchain {
                benchmark: invocation.target().toolchain().clone(),
                preflight: preflight_target.toolchain().clone(),
            })
        } else {
            None
        };
        if let Some(mismatch) = mismatch {
            return Err(BenchRunRefusal::PreflightTargetMismatch {
                row: row.key(),
                mismatch,
            });
        }
    }
    Ok(())
}

fn record_curve(
    call: super::BenchCall,
    binding: &BenchBinding,
) -> Result<WorkCurve, WorkRecordingRefusal> {
    let row = binding.row();
    let mut points = Vec::new();
    for input_size in row.input_sizes().sizes().iter().copied() {
        let mut recorder = WorkRecorder::scoped(binding.attachment().observations());
        for _sample in 0..row.budgets().samples() {
            call(input_size, &mut recorder)?;
        }
        points.push(recorder.finish(input_size));
    }
    Ok(WorkCurve::recorded(points))
}

fn judge(binding: &BenchBinding, measured: &WorkCurve, planted_worse: &WorkCurve) -> WorkJudgment {
    let row = binding.row();
    let input = WorkJudgmentInput::over(
        row.formula(),
        row.complexity(),
        row.budgets(),
        measured,
        planted_worse,
    );
    (binding.attachment().judge().judge())(&input)
}

fn observe_secondary(
    binding: &BenchBinding,
    invocation: &BenchInvocation,
    planted_worse: &WorkCurve,
) -> Result<SecondaryObservation, SecondaryObservationRefusal> {
    let row = binding.row();
    let attachment = binding.attachment();
    let mut points = Vec::new();
    let mut measurements = Vec::new();
    for input_size in row.input_sizes().sizes().iter().copied() {
        for _warmup in 0..row.budgets().warmups() {
            let mut recorder = WorkRecorder::scoped(attachment.observations());
            (attachment.measured())(input_size, &mut recorder)
                .map_err(SecondaryObservationRefusal::Warmup)?;
        }
        let mut recorder = WorkRecorder::scoped(attachment.observations());
        for _sample in 0..row.budgets().samples() {
            let measurement = invocation.clock().begin();
            (attachment.measured())(input_size, &mut recorder)
                .map_err(SecondaryObservationRefusal::Sample)?;
            measurements.push(measurement.finish());
        }
        points.push(recorder.finish(input_size));
    }
    let curve = WorkCurve::recorded(points);
    let judgment = judge(binding, &curve, planted_worse);
    SecondaryObservation::recorded(curve, judgment, measurements)
}

fn execute_binding(
    binding: &BenchBinding,
    invocation: &BenchInvocation,
) -> Result<BenchReading, BenchRunRefusal> {
    let row = binding.row();
    let preflight = run_one(
        binding.preflight().binding(),
        binding.preflight().invocation(),
    );
    if lens_verdict(&preflight).is_err() {
        return Ok(BenchReading::recorded(
            row,
            invocation.target().clone(),
            preflight,
            BenchOutcome::PreflightRefused,
        ));
    }
    let measured = record_curve(binding.attachment().measured(), binding).map_err(|refusal| {
        BenchRunRefusal::WorkNotRecorded {
            row: row.key(),
            phase: PrimaryWorkPhase::Measured,
            refusal,
        }
    })?;
    let planted_worse =
        record_curve(binding.attachment().planted_worse(), binding).map_err(|refusal| {
            BenchRunRefusal::WorkNotRecorded {
                row: row.key(),
                phase: PrimaryWorkPhase::PlantedWorse,
                refusal,
            }
        })?;
    let judgment = judge(binding, &measured, &planted_worse);
    let outcome = match judgment.qualification() {
        Err(WorkQualificationRefusal::PlantedWorseNotDistinguished { .. }) => {
            BenchOutcome::PlantedWorseNotDistinguished {
                measured,
                planted_worse,
                judgment,
            }
        }
        Err(WorkQualificationRefusal::MeasuredRefused(_)) => BenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        },
        Ok(()) => {
            let secondary =
                observe_secondary(binding, invocation, &planted_worse).map_err(|refusal| {
                    BenchRunRefusal::SecondaryWorkRefused {
                        row: row.key(),
                        refusal,
                    }
                })?;
            BenchOutcome::Qualified {
                measured,
                planted_worse,
                judgment,
                secondary,
            }
        }
    };
    Ok(BenchReading::recorded(
        row,
        invocation.target().clone(),
        preflight,
        outcome,
    ))
}

/// Run one complete benchmark table after validating all host-declaration relationships.
///
/// # Authority
///
/// Work counts and the owner-bound relational judge decide qualification. The caller clock is read only after correctness, the planted-worse control, and primary work qualify; its measurement readings never reach the judge.
///
/// # Errors
///
/// Refuses a target/toolchain mismatch before any caller code runs. After structural admission, refuses incomplete primary recording or a secondary pass that no longer satisfies the same work judgment, and publishes no partial report.
pub fn run_all(
    table: &BenchTable,
    invocation: &BenchInvocation,
) -> Result<BenchReport, BenchRunRefusal> {
    validate_table(table, invocation)?;
    let readings = table
        .bindings()
        .iter()
        .map(|binding| execute_binding(binding, invocation))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BenchReport::recorded(
        table.name(),
        table.provenance(),
        readings,
    ))
}

/// Read the first non-qualified row from a complete benchmark report.
///
/// # Errors
///
/// Refuses the first reading outside [`BenchStage::Qualified`](super::BenchStage::Qualified), in authored table order.
pub fn bench_verdict(report: &BenchReport) -> Result<(), BenchVerdictRefusal> {
    for reading in report.readings() {
        let stage = reading.outcome().stage();
        if stage != super::BenchStage::Qualified {
            return Err(BenchVerdictRefusal::row_not_qualified(
                reading.row().key(),
                stage,
            ));
        }
    }
    Ok(())
}

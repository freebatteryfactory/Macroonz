//! The host road: admit the whole table, take each row through its stages, and publish one report.

use super::timed::timed_pass;
use super::types::{
    BenchBinding, BenchCall, BenchInvocation, BenchOutcome, BenchReading, BenchReport,
    BenchRunRefusal, BenchTable, BenchTargetMismatch, PrimaryWorkPhase, WorkCurve,
    WorkQualificationRefusal,
};
use super::work::{curve, judge};
use crate::runner::{lens_verdict, run_one};

/// Which target fact disagrees between one row's preflight and this run, where either does.
fn target_mismatch(
    binding: &BenchBinding,
    invocation: &BenchInvocation,
) -> Option<BenchTargetMismatch> {
    let preflight = binding.preflight().invocation().target();
    let run = invocation.target();
    if preflight.target() != run.target() {
        return Some(BenchTargetMismatch::Target {
            benchmark: run.target().clone(),
            preflight: preflight.target().clone(),
        });
    }
    if preflight.toolchain() != run.toolchain() {
        return Some(BenchTargetMismatch::Toolchain {
            benchmark: run.toolchain().clone(),
            preflight: preflight.toolchain().clone(),
        });
    }
    None
}

/// Check every row's target declaration before one line of caller code runs.
fn admit(table: &BenchTable, invocation: &BenchInvocation) -> Result<(), BenchRunRefusal> {
    for binding in table.bindings() {
        if let Some(mismatch) = target_mismatch(binding, invocation) {
            return Err(BenchRunRefusal::PreflightTargetMismatch {
                row: binding.row().key(),
                mismatch,
            });
        }
    }
    Ok(())
}

/// One primary curve, with the callable that refused named in the refusal.
fn primary(
    binding: &BenchBinding,
    call: BenchCall,
    phase: PrimaryWorkPhase,
) -> Result<WorkCurve, BenchRunRefusal> {
    curve(call, binding.row(), binding.attachment()).map_err(|refusal| {
        BenchRunRefusal::WorkNotRecorded {
            row: binding.row().key(),
            phase,
            refusal,
        }
    })
}

/// Take one row through preflight, both primary curves, the judgment, and the timed pass.
fn run_row(
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

    let attachment = binding.attachment();
    let measured = primary(binding, attachment.measured(), PrimaryWorkPhase::Measured)?;
    let planted_worse = primary(
        binding,
        attachment.planted_worse(),
        PrimaryWorkPhase::PlantedWorse,
    )?;
    let judgment = judge(row, attachment, &measured, &planted_worse);

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
            let secondary = timed_pass(binding, invocation, &planted_worse).map_err(|refusal| {
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

/// Run one whole table under the host facts the caller declared.
///
/// # Authority
///
/// Work counts and the owner's judge decide qualification.
/// The clock is read only after correctness, the control, and the primary work have all held, and its readings never reach the judge.
///
/// # Errors
///
/// Refuses a target or toolchain mismatch before any caller code runs.
/// After that, refuses incomplete primary recording or a timed pass that no longer satisfies the same judgment, and publishes no partial report.
pub fn run_all(
    table: &BenchTable,
    invocation: &BenchInvocation,
) -> Result<BenchReport, BenchRunRefusal> {
    admit(table, invocation)?;
    let readings = table
        .bindings()
        .iter()
        .map(|binding| run_row(binding, invocation))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BenchReport::recorded(
        table.name(),
        table.provenance(),
        readings,
    ))
}

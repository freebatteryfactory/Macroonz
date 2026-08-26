//! Claims over the two admission roads, anti-vacuity, and host-authored limits.

use super::support::{LaneFailure, binding, invocation, passes, world};
use macroonz_harness::clock::MeasurementReading;
use macroonz_harness::report::{
    EmptySelectionReason, HostTrialRecord, RunAttempt, SelectionOutcome, SkipReason,
    TrialConclusion,
};
use macroonz_harness::runner::{
    ReportRecordingRefusal, SeatFailure, SeatOutcome, SeatRefusal, Selection, SelectionPlan,
    lens_verdict, record_all, record_one, run_all, seat_verdict, trial_identity,
};
use std::collections::BTreeSet;

fn host_records(report: &macroonz_harness::report::RunReport) -> Vec<HostTrialRecord> {
    report
        .census()
        .iter()
        .filter_map(|accounting| accounting.disposition().report())
        .map(|report| {
            HostTrialRecord::recorded(
                report.trial(),
                report.attempt().clone(),
                report.measurement(),
            )
        })
        .collect()
}

/// Claim: in-process execution and host admission produce one report meaning from the same declared facts.
///
/// Reversal: a host supplies only attempts and measurements; the shared assembler must reproduce every derived seat exactly.
#[test]
fn in_process_and_host_roads_share_one_assembly() -> Result<(), LaneFailure> {
    let table = world(vec![binding("first", passes)?, binding("second", passes)?])?;
    let invocation = invocation();
    let selection = SelectionPlan::of(Selection::All);
    let executed = run_all(&table.view(), &selection, &invocation);
    let admitted = record_all(
        &table.view(),
        &selection,
        &invocation,
        host_records(&executed),
    )?;
    assert_eq!(admitted, executed);
    Ok(())
}

/// Claim: an external host cannot author duplicate, foreign, unselected, or missing evidence into a report.
///
/// Reversal: each malformed roster is offered directly and must refuse under the first relationship it violates.
#[test]
fn host_records_cannot_author_unseen_standing() -> Result<(), LaneFailure> {
    let first = binding("first", passes)?;
    let second = binding("second", passes)?;
    let outside = binding("outside", passes)?;
    let first_trial = trial_identity(first.row());
    let second_trial = trial_identity(second.row());
    let outside_trial = trial_identity(outside.row());
    let table = world(vec![first, second])?;
    let invocation = invocation();
    let all = SelectionPlan::of(Selection::All);
    let first_record = HostTrialRecord::recorded(
        first_trial,
        RunAttempt::Executed(TrialConclusion::Passed),
        MeasurementReading::Unavailable,
    );
    assert_eq!(
        record_all(
            &table.view(),
            &all,
            &invocation,
            vec![first_record.clone(), first_record.clone()],
        ),
        Err(ReportRecordingRefusal::DuplicateHostRecord(
            first_record.trial()
        ))
    );
    assert_eq!(
        record_all(
            &table.view(),
            &all,
            &invocation,
            vec![HostTrialRecord::recorded(
                outside_trial,
                RunAttempt::Executed(TrialConclusion::Passed),
                MeasurementReading::Unavailable,
            )],
        ),
        Err(ReportRecordingRefusal::TrialOutsideTable(outside_trial))
    );
    let first_only = SelectionPlan::of(Selection::ByTrialIds(BTreeSet::from([first_trial])));
    assert_eq!(
        record_all(
            &table.view(),
            &first_only,
            &invocation,
            vec![HostTrialRecord::recorded(
                second_trial,
                RunAttempt::Executed(TrialConclusion::Passed),
                MeasurementReading::Unavailable,
            )],
        ),
        Err(ReportRecordingRefusal::RecordForUnselectedTrial(
            second_trial
        ))
    );
    assert_eq!(
        record_all(&table.view(), &all, &invocation, vec![first_record]),
        Err(ReportRecordingRefusal::MissingSelectedRecord(second_trial))
    );
    Ok(())
}

/// Claim: an empty selection and a cached skip retain their distinct zero-evidence postures at the verdict boundary.
///
/// Reversal: neither posture may flatten into a passing trial.
#[test]
fn zero_work_and_cached_work_never_flatten_into_passes() -> Result<(), LaneFailure> {
    let single_binding = binding("cached", passes)?;
    let table_binding = binding("cached", passes)?;
    let table = world(vec![table_binding])?;
    let invocation = invocation();
    let chooses_none = Selection::ByTrialIds(BTreeSet::new());
    let ordinary = run_all(
        &table.view(),
        &SelectionPlan::of(chooses_none.clone()),
        &invocation,
    );
    assert_eq!(
        ordinary.selection(),
        SelectionOutcome::UnsatisfiedByEmptySelection
    );
    assert_eq!(
        seat_verdict(&ordinary),
        Err(SeatRefusal::NothingSelected {
            denominator: 1usize
        })
    );
    let reason = EmptySelectionReason::AskingWhatTheWorldHolds;
    let admitted = run_all(
        &table.view(),
        &SelectionPlan::allowing_empty(chooses_none, reason),
        &invocation,
    );
    assert_eq!(
        seat_verdict(&admitted),
        Ok(SeatOutcome::NoWorkAsStated {
            reason,
            denominator: 1usize,
        })
    );
    let cached = record_one(
        &single_binding,
        &invocation,
        HostTrialRecord::recorded(
            trial_identity(single_binding.row()),
            RunAttempt::SkippedWithReason(SkipReason::SatisfiedByCachedExecution),
            MeasurementReading::Unavailable,
        ),
    )?;
    let Err(SeatRefusal::TrialFailed(failed)) = lens_verdict(&cached) else {
        return Err(LaneFailure::Missing("cached skip seat refusal"));
    };
    assert_eq!(
        failed.failure(),
        &SeatFailure::NotRun(SkipReason::SatisfiedByCachedExecution)
    );
    Ok(())
}

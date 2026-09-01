//! The one report assembler both admission roads walk.
//!
//! A complete table view and one selection plan are walked here once.
//! The caller supplies only how a selected binding reaches its admitted trial report; every census seat, row and execution revision, claim, table posture, selection outcome, profile, and target binding is derived here.

use super::resolve::{execution_key, execution_revisions, row_revision, trial_identity};
use super::select::admission;
use super::types::{Admission, Invocation, SelectionPlan, TrialBinding, TrialTableView};
use crate::clock::MeasurementReading;
use crate::report::{
    RunAttempt, RunReport, SelectionDisposition, SelectionOutcome, TrialAccounting, TrialReport,
    TrialRunStanding, attachment_replay_posture,
};

/// Admit one attempt under the standing derived from its binding and invocation.
pub(super) fn trial_report(
    binding: &TrialBinding,
    invocation: &Invocation,
    attempt: RunAttempt,
    measurement: MeasurementReading,
) -> TrialReport {
    let attachment = binding.attachment();
    let standing = TrialRunStanding::derived(
        execution_key(binding, invocation),
        attachment_replay_posture(
            attachment.subject_revision().posture(),
            attachment.check_revision().posture(),
        ),
    );
    TrialReport::recorded(standing, invocation.site(), attempt, measurement)
}

/// Assemble one complete report through the selected-row adapter its caller supplies.
///
/// The walk is over every binding the view presents, always, so the census carries one entry per row of the world whether this invocation named it or not.
/// The disposition is read before anything executes, so a row nobody ran can never appear as an attempt.
pub(super) fn run_report<E>(
    view: &TrialTableView<'_>,
    selection: &SelectionPlan,
    invocation: &Invocation,
    mut selected_report: impl FnMut(&TrialBinding) -> Result<TrialReport, E>,
) -> Result<RunReport, E> {
    let mut census = Vec::new();
    let mut selected = 0usize;
    for binding in view.bindings() {
        let row = binding.row();
        let trial = trial_identity(row);
        let disposition = match admission(selection.chooses(), row, trial) {
            Admission::Selected => {
                selected = selected.saturating_add(1);
                SelectionDisposition::selected(selected_report(binding)?)
            }
            Admission::NotSelected(reason) => SelectionDisposition::not_selected(trial, reason),
        };
        census.push(TrialAccounting::recorded(
            row_revision(row),
            execution_revisions(binding),
            row.claim(),
            disposition,
        ));
    }
    Ok(RunReport::recorded(
        census,
        view.posture(),
        SelectionOutcome::read(selection.expects(), selected),
        invocation.profile(),
        invocation.target().clone(),
    ))
}

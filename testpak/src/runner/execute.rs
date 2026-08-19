//! The two engine calls: one binding and an invocation become one trial report;
//! the complete view, a selection, and an invocation become one run report.
//!
//! Both roads are pure over their parameters. Nothing is read that was not
//! handed in, nothing is written anywhere, and nothing is kept: a report is a
//! value that leaves, and the engine holds no memory of the run that produced
//! it.

use super::catch::caught_conclusion;
use super::resolve::{row_revision, trial_identity};
use super::select::{Admission, admission};
use super::types::{Invocation, Selection, TrialBinding, TrialTableView};
use crate::descriptor::EncodeRefusal;
use crate::report::{
    RecordedDuration, RunAttempt, RunReport, SelectionDisposition, TrialAccounting, TrialReport,
};

/// Run one bound trial under one invocation.
///
/// # Authority
///
/// The attachment's callable is a pure map — invocation facts in, one
/// conclusion out — and this call is the trial boundary around it: a subject
/// panic is caught here and recorded as the finding it is, so a panicking
/// subject leaves a verdict rather than a dead process. Every other arm of
/// [`RunAttempt`] belongs to a seat that can establish it — a host that skipped
/// the trial, a watchdog that stopped it, a harness that failed around it — and
/// this engine states none of them because it establishes none of them.
///
/// # Bounds
///
/// The duration is the difference of two readings the invocation's own clock
/// returned, and a reading that does not move reads zero. Timing is recorded
/// and never concluded from: a trial that ran long still concludes whatever the
/// check concluded.
#[must_use]
pub fn run_one(binding: &TrialBinding, invocation: &Invocation) -> TrialReport {
    let clock = invocation.clock();
    let opened = clock.nanoseconds();
    let conclusion = caught_conclusion(binding.attachment(), invocation);
    let closed = clock.nanoseconds();
    TrialReport::recorded(
        trial_identity(binding.row()),
        invocation.site(),
        RunAttempt::Executed(conclusion),
        RecordedDuration::recorded(closed.saturating_sub(opened)),
    )
}

/// Run one selection over the complete world a view presents.
///
/// # Authority
///
/// The walk is over every binding the view presents, always, so the census
/// carries one entry per row of the world whether this invocation named it or
/// not: a caller narrows a run and never the denominator. The disposition is
/// read first and an execution happens only where the selection admitted one,
/// so a not-selected row can never be read as an attempt that failed.
///
/// The posture the report records is the view's own and the profile is the
/// invocation's, both recorded rather than restated. Comparing two reports and
/// reading claim coverage are the record home's operations over what this one
/// wrote.
///
/// # Errors
///
/// Refuses when a row's canonical bytes could not be written, so its revision
/// identity could not be derived — a length past the width the descriptor
/// home's row encoding declares, which is unreachable on every target this
/// crate is built for. The refusal is the whole report's rather than one
/// entry's: the census is the denominator, and a denominator carrying an entry
/// that cannot name its own row is a smaller world wearing the shape of the
/// complete one.
pub fn run_all(
    view: &TrialTableView<'_>,
    selection: &Selection,
    invocation: &Invocation,
) -> Result<RunReport, EncodeRefusal> {
    let census: Vec<TrialAccounting> = view
        .bindings()
        .map(|binding| accounted(binding, selection, invocation))
        .collect::<Result<_, _>>()?;
    Ok(RunReport::recorded(census, view.posture(), invocation.profile()))
}

/// One row of the denominator, and what this invocation did about it.
///
/// # Errors
///
/// Refuses exactly where [`row_revision`] does, and carries that refusal
/// unchanged.
fn accounted(
    binding: &TrialBinding,
    selection: &Selection,
    invocation: &Invocation,
) -> Result<TrialAccounting, EncodeRefusal> {
    let row = binding.row();
    let trial = trial_identity(row);
    let revision = row_revision(row)?;
    let disposition = match admission(selection, row, trial) {
        Admission::Selected => SelectionDisposition::selected(run_one(binding, invocation)),
        Admission::NotSelected(reason) => SelectionDisposition::not_selected(reason),
    };
    Ok(TrialAccounting::recorded(trial, revision, row.claim(), disposition))
}

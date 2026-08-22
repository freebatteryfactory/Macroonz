//! The two engine calls: one binding and an invocation become one trial report;
//! the complete view, a selection plan, and an invocation become one run report.
//!
//! Both roads are capture-free over explicit inputs and both return a report rather than retaining run state. Their callables and harness clock are supplied by the caller; this home does not relabel those external effects as purity, and the clock home alone owns source-unwind and elapsed-reading behavior.

use super::assemble::{run_report, trial_report};
use super::catch::caught_conclusion;
use super::types::{Invocation, SelectionPlan, TrialBinding, TrialTableView};
use crate::report::{RunAttempt, RunReport, TrialReport};
use core::convert::Infallible;

/// Run one bound trial under one invocation.
///
/// # Authority
///
/// The attachment's callable is a capture-free function pointer — invocation facts in, one conclusion out — and this call is the trial boundary around it. The pointer shape does not establish semantic purity. A subject panic is caught here and recorded as the finding it is, so a panicking subject leaves a verdict rather than a dead process. Every other arm of
/// [`RunAttempt`] belongs to a seat that can establish it — a host that skipped
/// the trial, a watchdog that stopped it, a harness that failed around it — and
/// this engine states none of them because it establishes none of them.
///
/// # Bounds
///
/// The invocation's own [`HarnessClock`](crate::clock::HarnessClock) opens before the subject and finishes afterwards. Its typed reading is recorded and never concluded from: unavailable or failed wall measurement cannot change what the check concluded, and an observed zero remains distinct from both.
#[must_use]
pub fn run_one(binding: &TrialBinding, invocation: &Invocation) -> TrialReport {
    let measurement = invocation.clock().begin();
    let conclusion = caught_conclusion(binding.attachment(), invocation);
    trial_report(
        binding,
        invocation,
        RunAttempt::Executed(conclusion),
        measurement.finish(),
    )
}

/// Run one selection plan over the complete world a view presents.
///
/// # Authority
///
/// Accounting has no refusal path: after selected callables return, every world produces a report. A row carries its canonical bytes from the moment it is built, so nothing on the accounting road can fail to name a row's revision, and a selection that matched nothing is a fact the report states rather than a reason to state no report at all. Ordinary clock-source unwind is retained as a failed measurement by the clock owner; arbitrary subject function pointers remain governed by the subject catch boundary.
///
/// The walk is over every binding the view presents, always, so the census
/// carries one entry per row of the world whether this invocation named it or
/// not: a caller narrows a run and never the denominator. The disposition is
/// read first and an execution happens only where the selection admitted one,
/// so a not-selected row can never be read as an attempt that failed.
///
/// The posture the report records is the view's own, the selection outcome is
/// the plan's expectation read against what the walk actually selected, and the
/// profile is the invocation's — all three recorded rather than restated.
/// Comparing two reports and reading claim coverage are the record home's
/// operations over what this one wrote.
#[must_use]
pub fn run_all(
    view: &TrialTableView<'_>,
    selection: &SelectionPlan,
    invocation: &Invocation,
) -> RunReport {
    match run_report(view, selection, invocation, |binding| {
        Ok::<TrialReport, Infallible>(run_one(binding, invocation))
    }) {
        Ok(report) => report,
        Err(never) => match never {},
    }
}

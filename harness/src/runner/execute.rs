//! The two engine calls: one binding becomes one trial report, and a whole view under a selection becomes one run report.
//!
//! Both roads are capture-free over explicit inputs, and both return a report rather than retaining run state.
//! The callables and the clock are the caller's; this home does not relabel their effects as purity, and source-unwind and elapsed-reading behaviour belong to the clock home alone.

use super::assemble::{run_report, trial_report};
use super::catch::caught_conclusion;
use super::types::{Invocation, SelectionPlan, TrialBinding, TrialTableView};
use crate::report::{RunAttempt, RunReport, TrialReport};
use core::convert::Infallible;

/// Run one bound trial under one invocation.
///
/// The attachment's callable is a capture-free function pointer, and this call is the trial boundary around it.
/// A subject panic is caught here and recorded as the finding it is, so a panicking subject leaves a verdict rather than a dead process.
/// Every other arm of [`RunAttempt`] belongs to a seat that can establish it — a host that skipped the trial, a watchdog that stopped it, a harness that failed around it — and this call states none of them because it establishes none of them.
///
/// The invocation's own [`HarnessClock`](crate::clock::HarnessClock) opens before the subject and finishes afterwards.
/// Its typed reading is recorded and never concluded from: an unavailable or failed wall measurement cannot change what the check concluded, and an observed zero stays distinct from both.
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
/// Accounting has no refusal path: once the selected callables return, every world produces a report.
/// A row carries its canonical bytes from the moment it is built, so nothing on the accounting road can fail to name a row's revision, and a selection that matched nothing is a fact the report states rather than a reason to state no report.
///
/// The posture the report records is the view's own, the selection outcome is the plan's expectation read against what the walk selected, and the profile and target binding are the invocation's — all recorded rather than restated.
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

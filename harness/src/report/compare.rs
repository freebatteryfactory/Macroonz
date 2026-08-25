//! Two reports in, a pure difference or an honest refusal out.
//!
//! It runs outside the runner and never grows the runner's memory: the baseline is the caller's to supply, both censuses are read through borrowed indexes, and the difference that comes back owns only identities and small typed facts.

use super::{
    Baseline, CensusDelta, ConclusionFlip, ExecutionRevisionChange, InvocationProfileChange,
    NotComparedReason, ReportComparison, ReportDiff, ReportExecutionDiff, ReportPopulationDiff,
    RowRevisionChange, RunReport, TargetBindingChange, TrialAccounting, TrialId,
};
use std::collections::BTreeMap;

/// Compare one report against a typed baseline.
///
/// A cross-posture pair is refused rather than compared: an authored world and a staged view have different denominators by construction, and a difference between them would read as change in the world when it is only change in what was overlaid.
/// Each refusal arm carries its own reason, so a caller can never mistake "no difference" for "nothing to compare against".
#[must_use]
pub fn compare(baseline: Baseline<'_>, current: &RunReport) -> ReportComparison {
    match baseline {
        Baseline::FirstRun => ReportComparison::NotCompared(NotComparedReason::FirstRun),
        Baseline::Unavailable(reason) => {
            ReportComparison::NotCompared(NotComparedReason::Unavailable(reason))
        }
        Baseline::Previous(previous) => against_previous(previous, current),
    }
}

/// The posture gate, then the difference.
fn against_previous(previous: &RunReport, current: &RunReport) -> ReportComparison {
    let left = previous.posture();
    let right = current.posture();
    if left == right {
        ReportComparison::Compared(diffed(previous, current))
    } else {
        ReportComparison::NotCompared(NotComparedReason::PostureMismatch { left, right })
    }
}

/// The difference itself: census membership, authored-row revisions, execution revisions, run standing, and outcome flips.
fn diffed(previous: &RunReport, current: &RunReport) -> ReportDiff {
    let before = indexed(previous);
    let after = indexed(current);
    let mut added: Vec<TrialId> = Vec::new();
    let mut removed: Vec<TrialId> = Vec::new();
    let mut revised: Vec<RowRevisionChange> = Vec::new();
    let mut execution_revisions: Vec<ExecutionRevisionChange> = Vec::new();
    let mut flips: Vec<ConclusionFlip> = Vec::new();

    for (trial, entry) in &after {
        match before.get(trial) {
            Some(prior) => {
                if prior.row() != entry.row() {
                    revised.push(RowRevisionChange::between(*trial, prior.row(), entry.row()));
                }
                if prior.revisions() != entry.revisions() {
                    execution_revisions.push(ExecutionRevisionChange::between(
                        *trial,
                        prior.revisions(),
                        entry.revisions(),
                    ));
                }
                let was = prior.disposition().outcome();
                let now = entry.disposition().outcome();
                if was != now {
                    flips.push(ConclusionFlip::between(*trial, was, now));
                }
            }
            None => added.push(*trial),
        }
    }
    for trial in before.keys() {
        if !after.contains_key(trial) {
            removed.push(*trial);
        }
    }

    let invocation = if previous.invocation() == current.invocation() {
        None
    } else {
        Some(InvocationProfileChange::between(
            previous.invocation(),
            current.invocation(),
        ))
    };
    let target = if previous.target() == current.target() {
        None
    } else {
        Some(TargetBindingChange::between(
            previous.target().clone(),
            current.target().clone(),
        ))
    };

    let population = ReportPopulationDiff::stated(
        added,
        removed,
        revised,
        CensusDelta::between(previous.denominator(), current.denominator()),
    );
    let execution = ReportExecutionDiff::stated(execution_revisions, flips, invocation, target);
    ReportDiff::stated(population, execution)
}

/// One report's census, indexed by trial identity for the walk.
///
/// Ordered rather than hashed, so the difference comes back in one deterministic order however the census was written.
fn indexed(report: &RunReport) -> BTreeMap<TrialId, &TrialAccounting> {
    report
        .census()
        .iter()
        .map(|entry| (entry.trial(), entry))
        .collect()
}

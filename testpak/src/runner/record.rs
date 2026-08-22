//! The external-host adapter: typed host observations become report evidence only after they join the table, selection, binding, and invocation that own their missing facts.

use super::assemble::{run_report, trial_report};
use super::resolve::trial_identity;
use super::select::{Admission, admission};
use super::types::{
    Invocation, ReportRecordingRefusal, SelectionPlan, TrialBinding, TrialTableView,
};
use crate::report::{HostTrialRecord, TrialReport};
use std::collections::{BTreeMap, BTreeSet};

/// Record one host-observed attempt under one bound trial and invocation.
///
/// # Errors
///
/// Refuses a host record that names a different semantic trial from the binding.
pub fn record_one(
    binding: &TrialBinding,
    invocation: &Invocation,
    record: HostTrialRecord,
) -> Result<TrialReport, ReportRecordingRefusal> {
    let expected = trial_identity(binding.row());
    let recorded = record.trial();
    if expected != recorded {
        return Err(ReportRecordingRefusal::TrialMismatch { expected, recorded });
    }
    let (_, attempt, measurement) = record.into_parts();
    Ok(trial_report(binding, invocation, attempt, measurement))
}

/// Record host observations over one selection and the complete table view it ran against.
///
/// # Errors
///
/// Refuses duplicate host records first, then a record outside the table, then a record for an unselected row, then the first selected row missing its host record. A lawful result always contains one census entry per table binding in table order.
pub fn record_all(
    view: &TrialTableView<'_>,
    selection: &SelectionPlan,
    invocation: &Invocation,
    records: Vec<HostTrialRecord>,
) -> Result<crate::report::RunReport, ReportRecordingRefusal> {
    let mut indexed = BTreeMap::new();
    let mut caller_order = Vec::new();
    for record in records {
        let trial = record.trial();
        if indexed.insert(trial, record).is_some() {
            return Err(ReportRecordingRefusal::DuplicateHostRecord(trial));
        }
        caller_order.push(trial);
    }

    let mut table = BTreeSet::new();
    let mut selected = BTreeSet::new();
    for binding in view.bindings() {
        let row = binding.row();
        let trial = trial_identity(row);
        table.insert(trial);
        if let Admission::Selected = admission(selection.chooses(), row, trial) {
            selected.insert(trial);
        }
    }
    for trial in caller_order {
        if !table.contains(&trial) {
            return Err(ReportRecordingRefusal::TrialOutsideTable(trial));
        }
        if !selected.contains(&trial) {
            return Err(ReportRecordingRefusal::RecordForUnselectedTrial(trial));
        }
    }

    run_report(view, selection, invocation, |binding| {
        let trial = trial_identity(binding.row());
        let record = indexed
            .remove(&trial)
            .ok_or(ReportRecordingRefusal::MissingSelectedRecord(trial))?;
        record_one(binding, invocation, record)
    })
}

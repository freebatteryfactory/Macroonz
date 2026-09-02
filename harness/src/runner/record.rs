//! The external-host adapter: typed host observations become evidence only after they join the binding, invocation, table, and selection that own their missing facts.
//!
//! A host knows which trial it ran, what the attempt did, and what the wall read.
//! Everything else on a report is derived here from the world the caller declared, so a host cannot author evidence it never observed.

use super::assemble::{run_report, trial_report};
use super::resolve::trial_identity;
use super::select::admission;
use super::types::{
    Admission, Invocation, ReportRecordingRefusal, SelectionPlan, TrialBinding, TrialTableView,
};
use crate::report::{HostTrialRecord, RunReport, TrialId, TrialReport};
use std::collections::{BTreeMap, BTreeSet};

/// Record one host-observed attempt under one bound trial and invocation.
///
/// # Errors
///
/// Refuses a host record naming a different semantic trial from the binding's.
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

/// Record host observations over one selection and the complete table view they ran against.
///
/// A lawful result carries one census entry per table binding, in table order, exactly as the in-process road does.
///
/// # Errors
///
/// Refuses duplicate host records first, then — in the order the caller handed the records over — a record naming no row of the table, then a record naming a row the selection passed over.
/// Refuses last the first selected row for which no host record was supplied.
pub fn record_all(
    view: &TrialTableView<'_>,
    selection: &SelectionPlan,
    invocation: &Invocation,
    records: Vec<HostTrialRecord>,
) -> Result<RunReport, ReportRecordingRefusal> {
    let mut host = HostRecords::indexed(records)?;
    host.admissible_against(view, selection)?;
    run_report(view, selection, invocation, |binding| {
        let trial = trial_identity(binding.row());
        let record = host
            .by_trial
            .remove(&trial)
            .ok_or(ReportRecordingRefusal::MissingSelectedRecord(trial))?;
        record_one(binding, invocation, record)
    })
}

/// The host's records under the trials they name, plus the order the caller handed them over in.
///
/// The order is kept because a refusal names the first record that did not hold, and "first" is the caller's word rather than the index's.
struct HostRecords {
    by_trial: BTreeMap<TrialId, HostTrialRecord>,
    order: Vec<TrialId>,
}

impl HostRecords {
    /// Index the caller's records.
    ///
    /// # Errors
    ///
    /// Refuses two records naming one trial, which is the one shape an index cannot hold.
    fn indexed(records: Vec<HostTrialRecord>) -> Result<Self, ReportRecordingRefusal> {
        let mut by_trial = BTreeMap::new();
        let mut order = Vec::new();
        for record in records {
            let trial = record.trial();
            if by_trial.insert(trial, record).is_some() {
                return Err(ReportRecordingRefusal::DuplicateHostRecord(trial));
            }
            order.push(trial);
        }
        Ok(Self { by_trial, order })
    }

    /// Check every recorded trial against the world and the selection before any of them is assembled.
    ///
    /// # Errors
    ///
    /// Refuses a record naming no row of the table, then a record naming a row this selection passed over.
    fn admissible_against(
        &self,
        view: &TrialTableView<'_>,
        selection: &SelectionPlan,
    ) -> Result<(), ReportRecordingRefusal> {
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
        for trial in self.order.iter().copied() {
            if !table.contains(&trial) {
                return Err(ReportRecordingRefusal::TrialOutsideTable(trial));
            }
            if !selected.contains(&trial) {
                return Err(ReportRecordingRefusal::RecordForUnselectedTrial(trial));
            }
        }
        Ok(())
    }
}

//! Host observations join their independently supplied execution and selection standing.

use super::assemble::{run_report, trial_report};
use super::resolve::trial_identity;
use super::select::admission;
use super::types::{
    Admission, Invocation, ReportRecordingRefusal, SelectionPlan, TrialBinding, TrialTableView,
};
use crate::clock::{ClockAttribution, MeasurementReading};
use crate::input::BoundInput;
use crate::report::{ExecutionInput, HostTrialRecord, RunAttempt, RunReport, TrialId, TrialReport};
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
    join_one(binding, invocation, record, None)
}

/// Record one host observation against an independently admitted typed invocation.
///
/// # Errors
///
/// Refuses a different trial, then mismatched input standing, then a contradictory input-budget observation.
pub fn record_input_one<Input>(
    binding: &TrialBinding<BoundInput<Input>>,
    invocation: &Invocation<BoundInput<Input>>,
    record: HostTrialRecord<ExecutionInput>,
) -> Result<TrialReport, ReportRecordingRefusal> {
    let input = *record.input();
    join_one(binding, invocation, record, Some(input))
}

fn join_one<Input, RecordedInput>(
    binding: &TrialBinding<Input>,
    invocation: &Invocation<Input>,
    record: HostTrialRecord<RecordedInput>,
    input: Option<ExecutionInput>,
) -> Result<TrialReport, ReportRecordingRefusal> {
    let expected = trial_identity(binding.row());
    let recorded = record.trial();
    if expected != recorded {
        return Err(ReportRecordingRefusal::TrialMismatch { expected, recorded });
    }
    if input != invocation.input_standing() {
        return Err(ReportRecordingRefusal::InputMismatch(recorded));
    }
    if let Some(reason) = invocation.input_budget_refusal()
        && (record.attempt() != &RunAttempt::SkippedWithReason(reason)
            || record.measurement() != MeasurementReading::Unavailable
            || record.clock_attribution() != ClockAttribution::Unspecified)
    {
        return Err(ReportRecordingRefusal::InputBudgetMismatch(recorded));
    }
    let (_, attempt, measurement, clock_attribution) = record.into_parts();
    Ok(trial_report(
        binding,
        invocation,
        attempt,
        measurement,
        clock_attribution,
    ))
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
    join_all(view, selection, invocation, records, record_one)
}

/// Record input-bearing host observations over a selection and its complete table.
///
/// # Errors
///
/// Applies [`record_all`]'s roster precedence, then admits selected rows in table order through [`record_input_one`].
pub fn record_input_all<Input>(
    view: &TrialTableView<'_, BoundInput<Input>>,
    selection: &SelectionPlan,
    invocation: &Invocation<BoundInput<Input>>,
    records: Vec<HostTrialRecord<ExecutionInput>>,
) -> Result<RunReport, ReportRecordingRefusal> {
    join_all(view, selection, invocation, records, record_input_one)
}

fn join_all<Input, RecordedInput>(
    view: &TrialTableView<'_, Input>,
    selection: &SelectionPlan,
    invocation: &Invocation<Input>,
    records: Vec<HostTrialRecord<RecordedInput>>,
    record_one: impl Fn(
        &TrialBinding<Input>,
        &Invocation<Input>,
        HostTrialRecord<RecordedInput>,
    ) -> Result<TrialReport, ReportRecordingRefusal>,
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
struct HostRecords<Input> {
    by_trial: BTreeMap<TrialId, HostTrialRecord<Input>>,
    order: Vec<TrialId>,
}

impl<Input> HostRecords<Input> {
    /// Index the caller's records.
    ///
    /// # Errors
    ///
    /// Refuses two records naming one trial, which is the one shape an index cannot hold.
    fn indexed(records: Vec<HostTrialRecord<Input>>) -> Result<Self, ReportRecordingRefusal> {
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
    fn admissible_against<InvocationInput>(
        &self,
        view: &TrialTableView<'_, InvocationInput>,
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

//! User text resolves to the execution-suite names already admitted by the complete table.

use super::SuiteSelectionRefusal;
use crate::harness::descriptor::ExecutionSuite;
use crate::harness::runner::{Selection, SelectionPlan, TrialTableView};
use std::collections::BTreeSet;

/// The distinct execution suites in a complete table view, ordered by their admitted names.
#[must_use]
pub fn suites<Input>(table: &TrialTableView<'_, Input>) -> BTreeSet<ExecutionSuite> {
    table
        .bindings()
        .map(|binding| binding.row().execution_suite())
        .collect()
}

/// Select admitted suites using independently supplied namespace and stem text.
///
/// # Errors
/// Refuses an empty request or the first unknown or repeated suite without returning a partial plan.
pub fn select_suites<Input>(
    table: &TrialTableView<'_, Input>,
    requested: &[(&str, &str)],
) -> Result<SelectionPlan, SuiteSelectionRefusal> {
    if requested.is_empty() {
        return Err(SuiteSelectionRefusal::Empty);
    }
    let available = suites(table);
    let mut selected = BTreeSet::new();
    for (position, &(namespace, stem)) in requested.iter().enumerate() {
        let found = available
            .iter()
            .copied()
            .find(|suite| {
                let name = suite.name();
                name.namespace().written() == namespace && name.stem().written() == stem
            })
            .ok_or(SuiteSelectionRefusal::Unknown { position })?;
        if !selected.insert(found) {
            return Err(SuiteSelectionRefusal::Duplicate { position });
        }
    }
    Ok(SelectionPlan::of(Selection::ByExecutionSuite(selected)))
}

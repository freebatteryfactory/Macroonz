//! The reduction owner's historical budget, execution-order and ceiling joins.

use super::super::{ArchivedReduction, ReductionArchiveRefusal};
use crate::generate::{ByteReducerExecution, ReductionHalt};
use std::collections::BTreeSet;

pub(super) fn joined(
    record: ArchivedReduction,
) -> Result<ArchivedReduction, ReductionArchiveRefusal> {
    let budget = record.budget.probes();
    let total = record.census.probes;
    if total > budget || (record.halt == ReductionHalt::BudgetExhausted && total != budget) {
        return Err(ReductionArchiveRefusal::AccountingMismatch);
    }
    let mut remaining = u64::from(budget);
    let mut names = BTreeSet::new();
    let mut ceiling = record.report_posture.meet(record.probe_posture);
    for reducer in &record.semantic_reducers {
        if !names.insert((reducer.name.namespace(), reducer.name.stem())) {
            return Err(ReductionArchiveRefusal::DuplicateReducer);
        }
        if remaining == 0 || reducer.probes != reducer.candidates.min(remaining) {
            return Err(ReductionArchiveRefusal::AccountingMismatch);
        }
        remaining = remaining.saturating_sub(reducer.probes);
        ceiling = ceiling.meet(reducer.posture);
    }
    let semantic = u64::from(budget).saturating_sub(remaining);
    if semantic > u64::from(total) {
        return Err(ReductionArchiveRefusal::AccountingMismatch);
    }
    match record.byte_reducer {
        ByteReducerExecution::Executed(_) if remaining == 0 => {
            return Err(ReductionArchiveRefusal::AccountingMismatch);
        }
        ByteReducerExecution::NotReachedBecauseBudgetSpent
            if remaining != 0 || record.halt != ReductionHalt::BudgetExhausted =>
        {
            return Err(ReductionArchiveRefusal::AccountingMismatch);
        }
        ByteReducerExecution::Executed(_) | ByteReducerExecution::NotReachedBecauseBudgetSpent => {}
    }
    if record.capsule.claimed_posture() != ceiling {
        return Err(ReductionArchiveRefusal::PostureMismatch);
    }
    if let Some(input) = record.capsule.key().input()
        && record.report_posture.meet(input.posture()) != record.report_posture
    {
        return Err(ReductionArchiveRefusal::PostureMismatch);
    }
    Ok(record)
}

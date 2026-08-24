//! The selection reading: what one invocation's selection says about one row of the complete world.
//!
//! A pure map from a selection and a row to a typed admission, read once per row before anything executes.
//! Reading it first is what keeps the two accounting axes apart: a row the selection passed over never reaches the execution road, so it can never be recorded as an attempt that failed.

use super::types::Selection;
use crate::descriptor::Row;
use crate::report::{NotSelectedReason, TrialId};

/// What one selection says about one row of the denominator.
pub(super) enum Admission {
    /// The selection named this row.
    Selected,
    /// The selection passed this row over, for a stated reason.
    NotSelected(NotSelectedReason),
}

/// Read one selection over one row of the world.
pub(super) fn admission(selection: &Selection, row: &Row, trial: TrialId) -> Admission {
    match passed_over(selection, row, trial) {
        None => Admission::Selected,
        Some(reason) => Admission::NotSelected(reason),
    }
}

/// Why this selection passes this row over, or nothing where it names the row.
///
/// The suite arm carries its own reason: a row left out for running under a seat this invocation did not run is a different fact from a row the selection simply did not name.
fn passed_over(selection: &Selection, row: &Row, trial: TrialId) -> Option<NotSelectedReason> {
    match selection {
        Selection::All => None,
        Selection::ByClaim(claims) => {
            (!claims.contains(&row.claim())).then_some(NotSelectedReason::OutsideSelection)
        }
        Selection::ByExecutionSuite(suites) => {
            (!suites.contains(&row.execution_suite())).then_some(NotSelectedReason::SuiteNotRun)
        }
        Selection::ByTrialIds(trials) => {
            (!trials.contains(&trial)).then_some(NotSelectedReason::OutsideSelection)
        }
        Selection::BySubjectRoute(routes) => {
            (!routes.contains(&row.subject())).then_some(NotSelectedReason::OutsideSelection)
        }
    }
}

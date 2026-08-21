//! The selection reading: what one invocation's selection says about one row of
//! the complete world.
//!
//! A pure map from a selection and a row to a typed admission, read once per
//! row before anything executes. Reading it first is what keeps the two axes
//! apart: a row the selection passed over never reaches the execution road at
//! all, so a not-selected row cannot be recorded as an attempt that failed.

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
///
/// The suite arm carries its own reason, because a row left out for running
/// under a seat this invocation did not run is a different fact from a row the
/// selection simply did not name.
pub(super) fn admission(selection: &Selection, row: &Row, trial: TrialId) -> Admission {
    match selection {
        Selection::All => Admission::Selected,
        Selection::ByClaim(claims) => admitted(
            claims.contains(&row.claim()),
            NotSelectedReason::OutsideSelection,
        ),
        Selection::ByExecutionSuite(suites) => admitted(
            suites.contains(&row.execution_suite()),
            NotSelectedReason::SuiteNotRun,
        ),
        Selection::ByTrialIds(trials) => {
            admitted(trials.contains(&trial), NotSelectedReason::OutsideSelection)
        }
        Selection::BySubjectRoute(routes) => admitted(
            routes.contains(&row.subject()),
            NotSelectedReason::OutsideSelection,
        ),
    }
}

/// One membership answer, as the typed admission it stands for.
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "a membership test over an already-informed set is the case where yes and no are the complete answer rather than two states wearing one name; the states this home does name are what the road hands back"
)]
fn admitted(named: bool, otherwise: NotSelectedReason) -> Admission {
    if named {
        Admission::Selected
    } else {
        Admission::NotSelected(otherwise)
    }
}

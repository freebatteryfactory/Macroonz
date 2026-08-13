//! The per-component disposition pass, and the refusal an established issue
//! list amounts to.
//!
//! The component roster is the quantifier. Every component the machine declares
//! is examined, in roster order, so "every component was examined" is a fact
//! about the loop rather than a claim about it. A component nobody disposed of
//! and a component disposed of twice are different findings and are reported as
//! two.
//!
//! Nothing here reaches a private field: the pass reads supplied selections and
//! omissions before any view exists. The roads that consume this pass live in
//! `type_guard.rs`, because building a complete view and building the refusal
//! body are both what must stay unreachable.

use super::{TriggerOmission, TriggerSelection, TriggerViewIssue};
use crate::planning::WRAPPER_COMPONENTS;

/// Every component the two lists leave undecided or dispose of twice, in roster
/// order.
pub(super) fn disposition_issues(
    selections: &[TriggerSelection],
    omissions: &[TriggerOmission],
) -> Vec<TriggerViewIssue> {
    let mut issues: Vec<TriggerViewIssue> = Vec::new();
    for component in WRAPPER_COMPONENTS {
        let selected = selections
            .iter()
            .filter(|selection| selection.component == component)
            .count();
        let omitted = omissions
            .iter()
            .filter(|omission| omission.component == component)
            .count();
        let disposed = selected.saturating_add(omitted);
        if disposed == 0 {
            issues.push(TriggerViewIssue::MissingComponentDisposition { component });
        } else if disposed > 1 {
            issues.push(TriggerViewIssue::DoubledComponent { component });
        }
    }
    issues
}

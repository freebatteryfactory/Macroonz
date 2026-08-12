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
//! omissions before any view exists. The road that consumes this pass lives in
//! `type_guard.rs`, because building a complete view is what must stay
//! unreachable.

use super::{TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue};
use crate::plane::AuthoringLimitProfile;
use crate::planning::WRAPPER_COMPONENTS;
use threadpak::refusal::{CompletionPosture, StopBound};
use threadpak::types::{NonEmptyBounded, NonEmptyBoundedConstruction, PositiveLimit};

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

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
pub(super) fn refused(issues: Vec<TriggerViewIssue>) -> Option<TriggerViewComposition> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(TriggerViewComposition::established(
        first,
        established.collect(),
    ))
}

impl TriggerViewComposition {
    /// The body a composition check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there.
    pub(super) fn established(first: TriggerViewIssue, rest: Vec<TriggerViewIssue>) -> Self {
        match NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        ) {
            Ok(issues) => Self {
                issues,
                posture: CompletionPosture::Complete,
            },
            Err(NonEmptyBoundedConstruction::OverLimit) => Self {
                issues: NonEmptyBounded::singleton(first),
                posture: CompletionPosture::EarlyStopped {
                    stopped_at: StopBound::DeclaredIssueBound,
                },
            },
        }
    }
}

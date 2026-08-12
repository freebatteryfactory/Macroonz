//! The three roads a planning refusal body is built through.
//!
//! Every seam in the plane that refuses while planning reaches one of these, so
//! no seam invents a body of its own shape. The one-issue road is total: the
//! declared bound admits an item by compile-time proof, so refusing never needs
//! an error road of its own. The co-establishing road is the one that can
//! overrun, and when it does the body keeps the first issue and reports that
//! enumeration stopped there — it never silently drops the remainder and never
//! claims a completeness it does not have.

use super::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};
use crate::plane::AuthoringLimitProfile;
use threadpak::refusal::{CompletionPosture, StopBound};
use threadpak::types::{NonEmptyBounded, NonEmptyBoundedConstruction, PositiveLimit};

impl ProjectionPlanning {
    /// The one-issue body, for a seam whose checks can establish exactly one
    /// issue. Total: the declared bound admits an item by compile-time proof, so
    /// refusing never needs an error road of its own.
    pub fn established(issue: ProjectionPlanningIssue) -> Self {
        Self {
            issues: NonEmptyBounded::singleton(issue),
            posture: CompletionPosture::Complete,
        }
    }

    /// The several-issue body, for a pass whose checks co-establish. When the
    /// supplied issues outrun the declared bound the body keeps the first and
    /// reports that enumeration stopped there — it never silently drops the
    /// remainder and never claims completeness it does not have.
    pub fn co_established(
        first: ProjectionPlanningIssue,
        rest: Vec<ProjectionPlanningIssue>,
    ) -> Self {
        match NonEmptyBounded::admitted_const(
            first.clone(),
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

    /// The body a bounded seam refuses with: the axis it overran, the magnitude
    /// it declared, and the count it observed.
    pub fn bound_exceeded(axis: BoundAxis, bound: usize, observed: usize) -> Self {
        Self::established(ProjectionPlanningIssue::BoundExceeded {
            axis,
            bound: u64::try_from(bound).unwrap_or(u64::MAX),
            observed: u64::try_from(observed).unwrap_or(u64::MAX),
        })
    }
}

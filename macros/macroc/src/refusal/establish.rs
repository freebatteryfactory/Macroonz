//! The three roads a planning refusal body is built through.
//!
//! Every seam in the plane that refuses while planning reaches one of these, so
//! no seam invents a body of its own shape. The one-issue road is total: the
//! declared bound admits an item by compile-time proof, so refusing never needs
//! an error road of its own. The co-establishing road is the one that can
//! overrun, and when it does the body carries what the declared bound holds and
//! names how many established issues stand outside it — it never silently drops
//! the remainder and never claims a completeness it does not have.

use super::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};
use crate::plane::AuthoringLimitProfile;
use threadpak::refusal::{CompletionPosture, StopBound};
use threadpak::types::{NonEmptyBounded, PositiveLimit};

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

    /// The several-issue body, for a pass whose checks co-establish.
    ///
    /// The caller arrives holding every issue its pass established, so the
    /// posture this road writes is about the REPORT and never about the pass:
    /// where the issues fit the declared bound the body carries all of them, and
    /// where they do not the body carries what the bound holds and names how
    /// many established issues stand outside it. It never silently drops the
    /// remainder and never claims a completeness it does not have.
    pub fn co_established(
        first: ProjectionPlanningIssue,
        rest: Vec<ProjectionPlanningIssue>,
    ) -> Self {
        let (issues, omitted) = NonEmptyBounded::admitted_prefix(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        );
        Self {
            issues,
            posture: CompletionPosture::examined_completely(omitted, StopBound::DeclaredIssueBound),
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

//! The refusal home's invariant nucleus: the three roads a planning refusal body
//! is built through, and the one road it is read back on.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's
//! claim structural rather than remembered. Every seam in the plane that refuses
//! while planning reaches one of these roads, so no seam invents a body of its
//! own shape — and none can, because the seat those roads fill is private and
//! this file is the only module that can name it. The one-issue road is total:
//! the declared bound admits an item by compile-time proof, so refusing never
//! needs an error road of its own. The co-establishing road is the one that can
//! overrun, and when it does the body carries what the declared bound holds and
//! names how many established issues stand outside it — it never silently drops
//! the remainder and never claims a completeness it does not have.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: a module beside `types.rs`, anywhere else in the
//! services, and any crate downstream cannot write the literal, and the compiler
//! says so with `E0451`. It does not exclude DESCENDANTS. A module declared
//! inside this one would construct the body as freely as these roads do, so a
//! `#[cfg(test)] mod` under the guard would reopen exactly what the guard closes
//! — which is why the reversals for this seat are compile-fail fixtures owned by
//! testpak, outside the crate, where the exclusion is total.

use super::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};
use crate::plane::{AuthoringLimitProfile, PlanningIssueLimit};
use threadpak::refusal::{AdmittedPrefix, StopBound};
use threadpak::types::PositiveLimit;

impl ProjectionPlanning {
    /// The one-issue body, for a seam whose checks can establish exactly one
    /// issue. Total: the declared bound admits an item by compile-time proof, so
    /// refusing never needs an error road of its own.
    pub fn established(issue: ProjectionPlanningIssue) -> Self {
        Self {
            body: AdmittedPrefix::carrying_one(issue),
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
        Self {
            body: AdmittedPrefix::examined_completely(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                StopBound::DeclaredIssueBound,
            ),
        }
    }

    /// The established issues and what this refusal says about its own coverage
    /// of them.
    ///
    /// Borrowed and never owned, for the reason band 00 borrows its carry: an
    /// owned body is a value a caller can seat under another refusal, which is
    /// the pairing the coupled seat exists to end.
    pub const fn body(&self) -> &AdmittedPrefix<ProjectionPlanningIssue, PlanningIssueLimit> {
        &self.body
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

//! The refusal home's invariant nucleus: the three roads a planning refusal body
//! is built through, and the one road it is read back on.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's
//! claim structural rather than remembered. Every seam in the plane that refuses
//! while planning reaches one of these roads, so no seam invents a body of its
//! own shape — and none can, because the seat those roads fill is private to the
//! `seat` module below and nothing else in the workspace is inside it. The
//! one-issue road is total: the declared bound admits an item by compile-time
//! proof, so refusing never needs an error road of its own. The co-establishing
//! road is the one that can overrun, and when it does the body carries what the
//! declared bound holds and names how many established issues stand outside it —
//! it never silently drops the remainder and never claims a completeness it does
//! not have.
//!
//! # Why the body is DECLARED here and not in `types.rs`
//!
//! Rust's privacy is MODULE-scoped. A seat declared in `types.rs` is private to
//! `types.rs`, which means every one of the dozens of other items in that file
//! is inside the wall — and the only remaining question is whether anybody wrote
//! a road out among them. That is a whole-file audit, and a whole-file audit is
//! not a claim a reader can settle by reading.
//!
//! So the declaration sits in the `seat` module below, whose entire content is
//! the body and the roads to it. The set of roads that reach this seat is now
//! the module rather than the file, and the module keeps that shape by
//! declaration: a `seat`
//! module carries its one record and inherent implementations of it, and nothing
//! else at all.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: the rest of this file, `types.rs` above it,
//! anywhere else in the services, and any crate downstream cannot write the
//! literal, and the compiler says so with `E0451`. It does not exclude
//! DESCENDANTS. A module declared inside the seat would construct the body as
//! freely as these roads do — which is why the reversals for this seat are
//! compile-fail fixtures owned by testpak, outside the crate, where the
//! exclusion is total, and why the law above refuses a nested module in a `seat`
//! module outright.
//!
//! # A private seat with a public mint is a fence with a loading dock
//!
//! The seat closes the LITERAL. It does not, by itself, close the MINT: a road
//! that takes an issue and hands back a refusal lets any holder of an issue
//! produce a body no pass established, and lets a holder clone the issues out of
//! [`ProjectionPlanning::body`] and reseat them through that road. The record it
//! produces is indistinguishable from one a seam returned, which is the whole
//! defect the private seat was supposed to end. So the roads below are
//! `pub(crate)` and the reader stays `pub`: writable and readable are different
//! permissions and this home still grants one of them, now at both halves.
//!
//! `pub(crate)` is this family's strongest reachable scope and not a compromise
//! taken for convenience. The other five collection families are each raised by
//! a pass living in the same `type_guard.rs`, so each of their mints reaches no
//! further than that file. This one is the plane's SHARED planning family —
//! every seam that refuses while planning returns it — so its establishing
//! passes live in `planning`, `origin_graph`, `pattern_stamp` and this home at
//! once, and the narrowest scope that reaches all of them is the crate. What
//! remains open is stated rather than implied: inside the services, any module
//! can still mint, and the module order `lib.rs` declares is what enumerates the
//! seams that do.

pub use seat::ProjectionPlanning;

mod seat {
    use super::super::{BoundAxis, ProjectionPlanningIssue};
    use crate::plane::{AuthoringLimitProfile, PlanningIssueLimit};
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The planning refusal family body.
    ///
    /// Independent members, no ladder, no primary issue, posture carried as an
    /// instance value. A body that stopped at its declared bound says so rather
    /// than implying no further defects exist.
    #[must_use = "a refusal family body carries every planning issue the pass established"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ProjectionPlanning {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue its seam
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's.
        ///
        /// Private, and that is the second half of the same claim. The coupled
        /// seat keeps a carry and its posture together; a PUBLIC seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one seam could write it into another
        /// seam's refusal. Read back through [`ProjectionPlanning::body`].
        ///
        /// The seat closes the literal and the mints close the road beside it. A
        /// private seat reached by a public generic constructor is a fence with a
        /// loading dock behind it: a caller holding an issue mints a refusal no
        /// pass raised, and a caller holding this borrow clones the issues out
        /// and seats them under a fresh one. Both roads are crate-internal for
        /// that reason, and the reason is stated where they are declared.
        body: AdmittedPrefix<ProjectionPlanningIssue, PlanningIssueLimit>,
    }

    impl ProjectionPlanning {
        /// The one-issue body, for a seam whose checks can establish exactly one
        /// issue. Total: the declared bound admits an item by compile-time
        /// proof, so refusing never needs an error road of its own.
        ///
        /// Crate-internal: a body exists only where a planning seam established
        /// the issue it carries.
        pub(crate) fn established(issue: ProjectionPlanningIssue) -> Self {
            Self {
                body: AdmittedPrefix::carrying_one(issue),
            }
        }

        /// The several-issue body, for a pass whose checks co-establish.
        ///
        /// The caller arrives holding every issue its pass established, so the
        /// posture this road writes is about the REPORT and never about the
        /// pass: where the issues fit the declared bound the body carries all of
        /// them, and where they do not the body carries what the bound holds and
        /// names how many established issues stand outside it. It never silently
        /// drops the remainder and never claims a completeness it does not have.
        ///
        /// Crate-internal, on the same terms as the one-issue road.
        pub(crate) fn co_established(
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

        /// The established issues and what this refusal says about its own
        /// coverage of them.
        ///
        /// Borrowed and never owned, for the reason band 00 borrows its carry:
        /// an owned body is a value a caller can seat under another refusal,
        /// which is the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<ProjectionPlanningIssue, PlanningIssueLimit> {
            &self.body
        }

        /// The body a bounded seam refuses with: the axis it overran, the
        /// magnitude it declared, and the count it observed.
        ///
        /// Crate-internal: it is the one-issue road under a spelling, and a
        /// spelling of a closed road is not an opening of it.
        pub(crate) fn bound_exceeded(axis: BoundAxis, bound: usize, observed: usize) -> Self {
            Self::established(ProjectionPlanningIssue::BoundExceeded {
                axis,
                bound: u64::try_from(bound).unwrap_or(u64::MAX),
                observed: u64::try_from(observed).unwrap_or(u64::MAX),
            })
        }
    }
}

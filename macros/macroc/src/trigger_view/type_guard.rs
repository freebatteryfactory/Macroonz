//! The trigger-view home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes exhaustive
//! disposition structural. A view is built HERE, after the disposition pass
//! agreed that every component was disposed of exactly once, so a view carrying
//! an undecided component is a value nobody can hold rather than a state a
//! reader has to notice. The refusal BODY is built here for the same reason and
//! by the same permission: its seat is private, so this file is the only module
//! in the workspace that can spell the literal, and every refusal that exists
//! came off the disposition pass.
//!
//! # Why the body is DECLARED here and not in `types.rs`
//!
//! Rust's privacy is MODULE-scoped, so a seat declared in `types.rs` puts every
//! other item in that file inside the wall and leaves "did anybody write a road
//! out?" as a whole-file audit. The body is therefore declared in the `seat`
//! module below, whose entire content is that record and inherent
//! implementations of it and nothing else — the module is the complete set of
//! roads that can reach the private seat.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: the rest of this file, `types.rs` above it,
//! `establish.rs` beside it, anywhere else in the services, and any crate
//! downstream cannot write the literal, and the compiler says so with `E0451`.
//! It does not exclude DESCENDANTS — a module declared inside the seat would
//! construct as freely as these roads do, which is why the reversals for this
//! seat are testpak's compile-fail fixtures and why the law above refuses a
//! nested module in a `seat` module outright.

use super::super::establish::disposition_issues;
use super::{TriggerOmission, TriggerSelection, TriggerViewIssue, WrapperTriggerView};
use crate::plane::{AuthoringLimitProfile, PlanId, WrapperComponentLimit};
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
fn refused(issues: Vec<TriggerViewIssue>) -> Option<TriggerViewComposition> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(TriggerViewComposition::established(
        first,
        established.collect(),
    ))
}

pub use seat::TriggerViewComposition;

mod seat {
    use super::super::TriggerViewIssue;
    use crate::plane::{AuthoringLimitProfile, TriggerViewIssueLimit};
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The trigger-view composition refusal family body.
    ///
    /// Independent members: several components may be undecided while another is
    /// doubled, and a caller repairing a view one component per attempt is a
    /// caller this seam failed.
    #[must_use = "a refusal family body carries every undisposed or doubled component"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TriggerViewComposition {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue the disposition
        /// pass established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's. The pass itself always
        /// covers every component, so the completion here never reports a halted
        /// examination.
        ///
        /// Private, and that is the second half of the same claim. The coupled
        /// seat keeps a carry and its posture together; a PUBLIC seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one pass could write it into another
        /// pass's refusal. Read back through [`TriggerViewComposition::body`].
        body: AdmittedPrefix<TriggerViewIssue, TriggerViewIssueLimit>,
    }

    impl TriggerViewComposition {
        /// The body a composition check refuses with.
        ///
        /// The disposition pass walks the whole component roster before a body
        /// exists, so the posture here is about the REPORT rather than the pass.
        /// Where every established issue fits the declared bound the body
        /// carries all of them; where it does not, the body carries what the
        /// bound holds and names how many established issues stand outside it.
        ///
        /// Reaches the guard file and no further — `pub(super)` from inside the
        /// seat is exactly the module-private reach this road had before the
        /// declaration moved, and the pass that raises it is beside it.
        pub(super) fn established(first: TriggerViewIssue, rest: Vec<TriggerViewIssue>) -> Self {
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
        pub const fn body(&self) -> &AdmittedPrefix<TriggerViewIssue, TriggerViewIssueLimit> {
            &self.body
        }
    }
}

impl WrapperTriggerView {
    /// Compose the view over one plan's decisions.
    ///
    /// # Errors
    ///
    /// Returns [`TriggerViewComposition`] naming every component nobody
    /// disposed of and every component disposed of twice. Both are reported
    /// together, and a component left undecided is refused rather than read as
    /// an omission: "nobody said" and "the owner said no" are different facts.
    pub fn composed(
        plan: PlanId,
        selections: Vec<TriggerSelection>,
        omissions: Vec<TriggerOmission>,
    ) -> Result<Self, TriggerViewComposition> {
        if let Some(refusal) = refused(disposition_issues(&selections, &omissions)) {
            return Err(refusal);
        }
        let observed = selections.len().saturating_add(omissions.len());
        let admitted = Bounded::admitted_const(
            selections,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .and_then(|bounded_selections| {
            Bounded::admitted_const(
                omissions,
                &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
            )
            .map(|bounded_omissions| (bounded_selections, bounded_omissions))
        });
        admitted
            .map(|(bounded_selections, bounded_omissions)| Self {
                plan,
                selections: bounded_selections,
                omissions: bounded_omissions,
            })
            .map_err(|_| {
                TriggerViewComposition::established(
                    TriggerViewIssue::SeatBoundExceeded {
                        bound: u64::try_from(WrapperComponentLimit::MAX).unwrap_or(u64::MAX),
                        observed: u64::try_from(observed).unwrap_or(u64::MAX),
                    },
                    Vec::new(),
                )
            })
    }

    /// The plan whose decisions this view summarizes.
    #[must_use]
    pub const fn plan(&self) -> PlanId {
        self.plan
    }

    /// Read the selected components and their citations.
    ///
    /// The order law applies: the disposition is keyed by component, so nothing
    /// identity-bearing is derived from the order this yields.
    pub fn selections(&self) -> impl Iterator<Item = &TriggerSelection> {
        self.selections.iter()
    }

    /// Read the omitted components and their citations.
    ///
    /// The order law applies exactly as it does for the selections.
    pub fn omissions(&self) -> impl Iterator<Item = &TriggerOmission> {
        self.omissions.iter()
    }

    /// The number of components disposed of — the component roster's
    /// cardinality, by construction.
    #[must_use]
    pub fn len(&self) -> usize {
        self.selections.len().saturating_add(self.omissions.len())
    }

    /// Always `false`: a view disposes of every component or does not exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.selections.is_empty() && self.omissions.is_empty()
    }
}

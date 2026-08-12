//! The trigger-view home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes exhaustive
//! disposition structural. A view is built HERE, after the disposition pass
//! agreed that every component was disposed of exactly once, so a view carrying
//! an undecided component is a value nobody can hold rather than a state a
//! reader has to notice.

use super::super::establish::{disposition_issues, refused};
use super::{
    TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue, WrapperTriggerView,
};
use crate::plane::{PlanId, WrapperComponentLimit};
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

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
        let admitted = Bounded::admitted_const(selections, &AdmittedLimit::under_ceiling())
            .and_then(|bounded_selections| {
                Bounded::admitted_const(omissions, &AdmittedLimit::under_ceiling())
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

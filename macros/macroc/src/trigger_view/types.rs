//! The trigger-view home's declarations: what a selection and an omission
//! carry, how a disposition fails, and the complete view itself.
//!
//! Declarations only. Every road that reaches a private field — the view's plan,
//! its selections, its omissions — lives in `type_guard.rs`, this file's own
//! child, which is what makes exhaustive disposition structural.

use crate::plane::{
    OwnerFactRef, PlanId, SelectionCitationLimit, TriggerViewIssueLimit, WrapperComponentLimit,
};
use crate::planning::WrapperComponent;
use threadpak::refusal::CompletionPosture;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

/// The owner facts one disposition cites — at least one, by shape.
pub type TriggerCitations = NonEmptyBounded<OwnerFactRef, SelectionCitationLimit>;

/// One component this plan composes, and the owner facts that selected it.
///
/// The citation seat is structurally non-empty: a selection that could carry no
/// citation would be a bare "yes", and a bare "yes" says a decision happened
/// without saying whose declaration decided it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TriggerSelection {
    /// The selected component.
    pub component: WrapperComponent,
    /// The owner facts that selected it.
    pub because: TriggerCitations,
}

/// One component this plan does not compose, and the owner facts that left it
/// out.
///
/// Same shape as a selection on purpose: an unexplained absence is the thing
/// this view abolishes, so absence carries its citations exactly like presence
/// does.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TriggerOmission {
    /// The omitted component.
    pub component: WrapperComponent,
    /// The owner facts that left it out.
    pub because: TriggerCitations,
}

/// How a trigger view fails to dispose of the component roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerViewIssue {
    /// A component appears in neither the selections nor the omissions.
    MissingComponentDisposition {
        /// The undecided component.
        component: WrapperComponent,
    },
    /// A component is disposed of more than once, whether twice one way or
    /// once each way.
    DoubledComponent {
        /// The doubled component.
        component: WrapperComponent,
    },
    /// A disposition seat outran its declared magnitude.
    ///
    /// Foreclosed on this seam's own route: an exhaustive disposition holds
    /// exactly one entry per component, and a list long enough to overrun the
    /// seat necessarily doubles a component, which the coverage pass reports
    /// first. The issue exists so the seat's construction has a truthful road
    /// rather than a fabricated one.
    SeatBoundExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// The trigger-view composition refusal family body.
///
/// Independent members: several components may be undecided while another is
/// doubled, and a caller repairing a view one component per attempt is a caller
/// this seam failed.
#[must_use = "a refusal family body carries every undisposed or doubled component"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TriggerViewComposition {
    /// The established issues — at least one, at most the declared bound.
    pub issues: NonEmptyBounded<TriggerViewIssue, TriggerViewIssueLimit>,
    /// Whether the body carries every issue the disposition pass established,
    /// or names how many stand outside the declared bound. The pass itself
    /// always covers every component, so this seat never reports a halted
    /// examination.
    pub posture: CompletionPosture,
}

/// The complete wrapper-trigger view over one plan.
///
/// Holding one is the proof of exhaustive disposition: every component in
/// [`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS) appears exactly
/// once, either selected with citations or omitted with citations. There is no
/// third list and no silent remainder.
#[must_use = "a complete view is the proof every wrapper component was disposed of once"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperTriggerView {
    plan: PlanId,
    selections: Bounded<TriggerSelection, WrapperComponentLimit>,
    omissions: Bounded<TriggerOmission, WrapperComponentLimit>,
}

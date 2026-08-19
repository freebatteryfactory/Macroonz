//! The trigger-view home's declarations: what a selection and an omission
//! carry, how a disposition fails, the complete view itself, and the two
//! magnitudes this home's capacities are governed by.
//!
//! Declarations only. Every road that reaches a private field — the view's plan,
//! its selections, its omissions, and the refusal body's one seat — lives in
//! `type_guard.rs`, this file's own child, which is what makes exhaustive
//! disposition structural.

use crate::plane::{OwnerFactRef, PlanId, WrapperComponentLimit};
use crate::planning::WrapperComponent;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many owner facts one wrapper-trigger
    /// selection or omission may cite.
    ///
    /// # Bounds
    ///
    /// Eight. A citation set is the declared reason one component was composed
    /// or left out, and a disposition standing on more than eight owner facts
    /// has stopped being one reason a reader can check — the repair is the
    /// owning home stating the fact the eight amount to, not a wider roster
    /// here.
    SelectionCitationLimit = 8,
    /// The magnitude governing how many issues one trigger-view refusal body may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Eight — the wrapper-component roster's own cardinality
    /// ([`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS)), because a
    /// component is either undisposed or doubled and never both, so the pass
    /// establishes at most one issue per component. It is not a number this home
    /// chose out of taste: a ninth issue would have to be a ninth COMPONENT, and
    /// the roster declares eight.
    TriggerViewIssueLimit = 8,
}

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

/// The trigger-view composition refusal family body, published from this file
/// and DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that
/// reach its seat.
pub use guard::TriggerViewComposition;

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

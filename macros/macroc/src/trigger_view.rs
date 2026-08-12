//! The wrapper-trigger view: which host-wrapper components a plan selected,
//! which it left out, and on whose declared fact each way.
//!
//! # A derived summary, never a second truth table
//!
//! The view reads decisions the machine's owners already made and reports them
//! with citations — "the suspension wrapper was selected because the execution
//! posture permits PEND". It answers no capability question of its own. If this
//! view and an owner disagree, the owner is right and the view is broken; there
//! is no second table here to consult.
//!
//! # Absence is explained on the same footing as presence
//!
//! A selection cites at least one owner fact and so does an omission. Both are
//! decisions somebody's declaration caused, and a component that appears in
//! neither list is not "off by default" — it is undecided, which
//! [`WrapperTriggerView::composed`] refuses.
//!
//! # The two capabilities that are not modeled here
//!
//! Benchmark intent and the host-conformance requirement have no owner
//! declaration to cite yet. They are deliberately absent from this view rather
//! than modeled on a guess: their owner declarations land with the
//! qualification plane, under a named owner. Sequencing the work is not
//! deferring the architecture — a citation-free trigger would be exactly the
//! second truth table this view exists to refuse.

use crate::plane::{
    OwnerFactRef, PlanId, SelectionCitationLimit, TriggerViewIssueLimit, WrapperComponentLimit,
};
use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{
    AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, NonEmptyBoundedConstruction,
};

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
    /// Whether every component was examined.
    pub posture: CompletionPosture,
}

impl RefusalFamily for TriggerViewComposition {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl TriggerViewComposition {
    /// The body a composition check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there.
    fn established(first: TriggerViewIssue, rest: Vec<TriggerViewIssue>) -> Self {
        match NonEmptyBounded::admitted_const(first, rest) {
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

/// The complete wrapper-trigger view over one plan.
///
/// Holding one is the proof of exhaustive disposition: every component in
/// [`WRAPPER_COMPONENTS`] appears exactly once, either selected with citations
/// or omitted with citations. There is no third list and no silent remainder.
#[must_use = "a complete view is the proof every wrapper component was disposed of once"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperTriggerView {
    plan: PlanId,
    selections: Bounded<TriggerSelection, WrapperComponentLimit>,
    omissions: Bounded<TriggerOmission, WrapperComponentLimit>,
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
        let mut established = issues.into_iter();
        if let Some(first) = established.next() {
            return Err(TriggerViewComposition::established(
                first,
                established.collect(),
            ));
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

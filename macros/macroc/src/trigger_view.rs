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
    OwnerFactRef, SelectionCitationLimit, TriggerViewIssueLimit, WrapperComponentLimit,
};
use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
use crate::refusal::PlanIdentity;
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{Bounded, ConstLimit, NonEmptyBounded, NonEmptyBoundedConstruction};

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
    #[must_use]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperTriggerView {
    plan: PlanIdentity,
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
        plan: PlanIdentity,
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
        let admitted = Bounded::admitted_const(selections).and_then(|selections| {
            Bounded::admitted_const(omissions).map(|omissions| (selections, omissions))
        });
        admitted
            .map(|(selections, omissions)| Self {
                plan,
                selections,
                omissions,
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
    pub const fn plan(&self) -> PlanIdentity {
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

#[cfg(test)]
mod laws {
    use super::{
        TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue,
        WrapperTriggerView,
    };
    use crate::plane::{ExactIdentity, OwnerFactRef};
    use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::NonEmptyBounded;

    /// One owner fact, for laws that need a citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([tag; 32]),
            fact: ExactIdentity::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One selection of the named component, citing one owner fact.
    fn selection(component: WrapperComponent) -> TriggerSelection {
        TriggerSelection {
            component,
            because: NonEmptyBounded::singleton(owner_fact(90)),
        }
    }

    /// One omission of the named component, citing one owner fact.
    fn omission(component: WrapperComponent) -> TriggerOmission {
        TriggerOmission {
            component,
            because: NonEmptyBounded::singleton(owner_fact(92)),
        }
    }

    /// law: trigger.a-disposition-always-cites-an-owner-fact — a selection and
    /// an omission each carry at least one citation by shape, so a bare
    /// selection is unrepresentable rather than refused, and the citations read
    /// back whole.
    /// Owed reversal (red twin): a citation-free selection must not compile.
    #[test]
    fn a_disposition_always_cites_an_owner_fact() {
        let selected = selection(WrapperComponent::Admission);
        assert_eq!(selected.because.len(), 1);
        assert_eq!(*selected.because.first(), owner_fact(90));
        assert!(!selected.because.is_empty());

        let paired = TriggerSelection {
            component: WrapperComponent::Receipt,
            because: NonEmptyBounded::admitted_const(owner_fact(94), vec![owner_fact(96)])
                .unwrap_or_else(|_| NonEmptyBounded::singleton(owner_fact(94))),
        };
        assert_eq!(paired.because.iter().count(), 2);

        let left_out = omission(WrapperComponent::Explanation);
        assert_eq!(left_out.because.len(), 1);
        assert_eq!(*left_out.because.first(), owner_fact(92));
    }

    /// law: trigger.every-component-is-disposed-exactly-once — a composed view
    /// covers the whole component roster, an undecided component refuses under
    /// its own issue naming it, and a component disposed of twice refuses too.
    /// Owed reversal: a seam that treated an undecided component as omitted
    /// must break this law.
    #[test]
    fn every_component_is_disposed_exactly_once() {
        let plan = ExactIdentity::decoded([88; 32]);
        let selections: Vec<TriggerSelection> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .take(5)
            .map(selection)
            .collect();
        let omissions: Vec<TriggerOmission> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .skip(5)
            .map(omission)
            .collect();
        let composed = WrapperTriggerView::composed(plan, selections, omissions);
        assert!(composed.is_ok_and(|view| {
            view.len() == 8
                && !view.is_empty()
                && view.plan() == plan
                && view.selections().count() == 5
                && view.omissions().count() == 3
                && view
                    .selections()
                    .all(|selection| !selection.because.is_empty())
                && view
                    .omissions()
                    .all(|omission| !omission.because.is_empty())
        }));

        let undecided: Vec<TriggerSelection> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .filter(|component| *component != WrapperComponent::Cancellation)
            .map(selection)
            .collect();
        let refused = WrapperTriggerView::composed(plan, undecided, Vec::new());
        assert!(refused.is_err_and(|composition| matches!(
            composition.issues.first(),
            TriggerViewIssue::MissingComponentDisposition {
                component: WrapperComponent::Cancellation
            }
        )));

        let doubled: Vec<TriggerSelection> =
            WRAPPER_COMPONENTS.iter().copied().map(selection).collect();
        let twice = WrapperTriggerView::composed(
            plan,
            doubled,
            vec![omission(WrapperComponent::Observation)],
        );
        assert!(twice.is_err_and(|composition| matches!(
            composition.issues.first(),
            TriggerViewIssue::DoubledComponent {
                component: WrapperComponent::Observation
            }
        )));
    }

    /// law: trigger.the-view-family-is-an-issue-collection — the composition
    /// family declares the collection shape and elects no primary issue, and a
    /// view missing several dispositions reports all of them at once.
    /// Owed reversal (red twin): reporting only the first undecided component
    /// must break this law.
    #[test]
    fn the_view_family_is_an_issue_collection() {
        assert!(matches!(
            TriggerViewComposition::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(TriggerViewComposition::SELECTION_ORDER.is_empty());

        let refused = WrapperTriggerView::composed(
            ExactIdentity::decoded([89; 32]),
            vec![selection(WrapperComponent::Admission)],
            Vec::new(),
        );
        assert!(refused.is_err_and(|composition| composition.issues.len() == 7));
    }
}

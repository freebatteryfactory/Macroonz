//! The origin graph and the citation machinery: where a generated thing came
//! from, and which owner fact decided each step.
//!
//! # The structural orphan law
//!
//! Every generated-unit type in the plane carries an [`OriginTrail`], and a
//! trail is structurally non-empty. A generated node with no origin is
//! therefore unrepresentable rather than validated: there is no road that
//! produces one, so no check has to catch one.
//!
//! A source span is not an origin. A span says where bytes sat; an origin says
//! which authored declaration, which pattern instantiation, which profile
//! selection, and which rendering act stand between that declaration and this
//! unit. A generated unit that offers only a span has answered a different
//! question than the one asked.
//!
//! # What a trace is, and is not
//!
//! A [`DecisionTrace`] preserves selection order — the order the plane made the
//! decisions, never a sorted or prettified order. A check that did not run is
//! recorded as [`TraceDecision::NotRun`] and is never confused with one that ran
//! and passed. No protected source material enters a trace: entries name
//! subjects and owner facts by identity, and identities carry no spelling.

use crate::plane::{
    ExactIdentity, NonclaimSubject, OriginEdgeLimit, OriginNodeSubject, OwnerFactRef,
    TraceEntryLimit, TracedSubject,
};
use crate::refusal::{BoundAxis, ProjectionPlanning};
use threadpak::types::{ConstLimit, NonEmptyBounded};

/// The closed roster of origin relations — the ruled categories an edge may
/// stand for. Fourteen, and the roster is the vocabulary: an edge that means
/// something else is a law change, not a new string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OriginRelation {
    /// A declaration a person authored.
    AuthoredDeclaration,
    /// An authored pattern instantiated with typed arguments.
    PatternInstantiation,
    /// Meaning derived from admitted meaning.
    SemanticDerivation,
    /// A declaration fragment constructed from parts.
    FragmentConstruction,
    /// A link an author stated explicitly.
    ExplicitLink,
    /// A normalization step that changed form and not meaning.
    Normalization,
    /// A projection profile selected.
    ProfileSelection,
    /// A projection kind selected under that profile.
    ProjectionSelection,
    /// Wrapper components composed for a host.
    WrapperComposition,
    /// Typed material rendered into a target's syntax.
    Rendering,
    /// A rendered surface bound to a host contract.
    HostBinding,
    /// A test descriptor derived from an obligation.
    TestDerivation,
    /// A benchmark descriptor derived from a work formula.
    BenchmarkDerivation,
    /// A diagnostic derived from an observed disagreement.
    DiagnosticDerivation,
}

/// The declared origin-relation roster, in the order the plane states it.
pub const ORIGIN_RELATIONS: [OriginRelation; 14] = [
    OriginRelation::AuthoredDeclaration,
    OriginRelation::PatternInstantiation,
    OriginRelation::SemanticDerivation,
    OriginRelation::FragmentConstruction,
    OriginRelation::ExplicitLink,
    OriginRelation::Normalization,
    OriginRelation::ProfileSelection,
    OriginRelation::ProjectionSelection,
    OriginRelation::WrapperComposition,
    OriginRelation::Rendering,
    OriginRelation::HostBinding,
    OriginRelation::TestDerivation,
    OriginRelation::BenchmarkDerivation,
    OriginRelation::DiagnosticDerivation,
];

/// One edge of the origin graph: which node, under which relation, produced
/// which node. The edge is directed and names both ends by identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginEdge {
    /// The node the relation starts at.
    pub from: ExactIdentity<OriginNodeSubject>,
    /// The ruled relation this edge stands for.
    pub relation: OriginRelation,
    /// The node the relation produces.
    pub to: ExactIdentity<OriginNodeSubject>,
}

/// The origin trail every generated unit carries: a bounded, structurally
/// non-empty walk back to the authored material. Emptiness is not refused here
/// because emptiness cannot be built here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OriginTrail {
    edges: NonEmptyBounded<OriginEdge, OriginEdgeLimit>,
}

impl OriginTrail {
    /// The one-edge trail. Total: a unit with one origin edge always fits.
    #[must_use]
    pub fn from_edge(edge: OriginEdge) -> Self {
        Self {
            edges: NonEmptyBounded::singleton(edge),
        }
    }

    /// Draw a trail of several edges.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::OriginEdges`] when the
    /// walk outgrows the declared bound. A trail that does not fit refuses:
    /// truncating a trail is how an origin silently becomes a span.
    pub fn drawn(first: OriginEdge, rest: Vec<OriginEdge>) -> Result<Self, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest)
            .map(|edges| Self { edges })
            .map_err(|_| {
                ProjectionPlanning::bound_exceeded(
                    BoundAxis::OriginEdges,
                    OriginEdgeLimit::MAX,
                    observed,
                )
            })
    }

    /// The guaranteed first edge.
    #[must_use]
    pub fn first(&self) -> &OriginEdge {
        self.edges.first()
    }

    /// The number of edges drawn; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Always `false`: a trail holds at least one edge. The constant answer is
    /// the orphan law stated as a method.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

/// What the plane decided about one subject, and on whose fact.
///
/// A selection and an omission both cite an owner fact, because both are
/// decisions someone's declaration caused. A check that did not run cites
/// nothing, because there is nothing to cite — and it stays distinct from a
/// check that ran and omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceDecision {
    /// Selected, because this owner fact required it.
    SelectedBecause(OwnerFactRef),
    /// Omitted, because this owner fact excluded it.
    OmittedBecause(OwnerFactRef),
    /// The check did not run. Never a pass, never a fail.
    NotRun,
}

/// One recorded decision about one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceEntry {
    /// The subject decided about.
    pub subject: ExactIdentity<TracedSubject>,
    /// What was decided, and on whose fact.
    pub decision: TraceDecision,
}

/// The decision trace of one plan: the entries in selection order, bounded.
///
/// Order is the order the plane decided, preserved exactly. Reordering a trace
/// would make it an inventory rather than a record of what happened.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecisionTrace {
    entries: NonEmptyBounded<TraceEntry, TraceEntryLimit>,
}

impl DecisionTrace {
    /// The one-entry trace. Total: a plan that decided once always fits.
    #[must_use]
    pub fn from_entry(entry: TraceEntry) -> Self {
        Self {
            entries: NonEmptyBounded::singleton(entry),
        }
    }

    /// Record a trace of several entries, in selection order.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::TraceEntries`] when the
    /// trace outgrows the declared bound.
    pub fn recorded(first: TraceEntry, rest: Vec<TraceEntry>) -> Result<Self, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest)
            .map(|entries| Self { entries })
            .map_err(|_| {
                ProjectionPlanning::bound_exceeded(
                    BoundAxis::TraceEntries,
                    TraceEntryLimit::MAX,
                    observed,
                )
            })
    }

    /// The guaranteed first entry, in selection order.
    #[must_use]
    pub fn first(&self) -> &TraceEntry {
        self.entries.first()
    }

    /// The number of entries recorded; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Always `false`: a plan that decided nothing is not a plan.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// One thing a plan explicitly does not claim, and the owner fact that leaves
/// it unclaimed. Stated nonclaims are what keep a trace from reading as a
/// stronger promise than the plan made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonclaim {
    /// The subject the plan makes no claim about.
    pub unclaimed: ExactIdentity<NonclaimSubject>,
    /// The owner fact that leaves it unclaimed.
    pub because: OwnerFactRef,
}

#[cfg(test)]
mod laws {
    use super::{
        DecisionTrace, Nonclaim, ORIGIN_RELATIONS, OriginEdge, OriginRelation, OriginTrail,
        TraceDecision, TraceEntry,
    };
    use crate::plane::{ExactIdentity, OriginEdgeLimit, OwnerFactRef, TraceEntryLimit};
    use crate::refusal::{BoundAxis, ProjectionPlanningIssue};
    use threadpak::types::ConstLimit;

    /// The closed relation roster, proven closed by an exhaustive match.
    const fn relation_index(relation: OriginRelation) -> usize {
        match relation {
            OriginRelation::AuthoredDeclaration => 0,
            OriginRelation::PatternInstantiation => 1,
            OriginRelation::SemanticDerivation => 2,
            OriginRelation::FragmentConstruction => 3,
            OriginRelation::ExplicitLink => 4,
            OriginRelation::Normalization => 5,
            OriginRelation::ProfileSelection => 6,
            OriginRelation::ProjectionSelection => 7,
            OriginRelation::WrapperComposition => 8,
            OriginRelation::Rendering => 9,
            OriginRelation::HostBinding => 10,
            OriginRelation::TestDerivation => 11,
            OriginRelation::BenchmarkDerivation => 12,
            OriginRelation::DiagnosticDerivation => 13,
        }
    }

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([1; 32]),
            fact: ExactIdentity::decoded([2; 32]),
        }
    }

    /// One edge, for laws that need a trail.
    fn edge() -> OriginEdge {
        OriginEdge {
            from: ExactIdentity::decoded([3; 32]),
            relation: OriginRelation::AuthoredDeclaration,
            to: ExactIdentity::decoded([4; 32]),
        }
    }

    /// law: origin.relations-are-fourteen-and-closed — the ruled relation
    /// categories are a closed roster whose members are pairwise distinct and
    /// declared in one order.
    /// Owed reversal: adding a relation without placing it must break this law.
    #[test]
    fn relations_are_fourteen_and_closed() {
        assert_eq!(ORIGIN_RELATIONS.len(), 14);
        let indexes: Vec<usize> = ORIGIN_RELATIONS
            .iter()
            .copied()
            .map(relation_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: origin.a-generated-node-without-an-origin-is-unrepresentable — the
    /// trail seat is structurally non-empty, so the orphan case has no
    /// constructor to reach and no runtime check to pass.
    /// Owed reversal (red twin): a trail built from an empty edge list must not
    /// compile.
    #[test]
    fn a_generated_node_without_an_origin_is_unrepresentable() {
        let trail = OriginTrail::from_edge(edge());
        assert!(!trail.is_empty() && trail.len() == 1);
        assert!(matches!(
            trail.first().relation,
            OriginRelation::AuthoredDeclaration
        ));
    }

    /// law: origin.trails-refuse-rather-than-truncate — a walk past the declared
    /// bound refuses with the bound axis named, so an origin never quietly
    /// shortens into a span.
    /// Owed reversal: a constructor that truncated must break this law.
    #[test]
    fn trails_refuse_rather_than_truncate() {
        let overrun: Vec<OriginEdge> = core::iter::repeat_n(edge(), OriginEdgeLimit::MAX).collect();
        let refused = OriginTrail::drawn(edge(), overrun);
        assert!(refused.is_err_and(|planning| matches!(
            planning.issues.first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::OriginEdges,
                ..
            }
        )));
        let fits = OriginTrail::drawn(edge(), vec![edge()]);
        assert!(fits.is_ok_and(|trail| trail.len() == 2));
    }

    /// law: origin.not-run-is-not-passed — a check that did not run is a
    /// distinct recorded decision, and a decision that ran cites the owner fact
    /// that caused it.
    /// Owed reversal (red twin): collapsing `NotRun` into an omission must break
    /// this law.
    #[test]
    fn not_run_is_not_passed() {
        let selected = TraceEntry {
            subject: ExactIdentity::decoded([5; 32]),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let omitted = TraceEntry {
            subject: ExactIdentity::decoded([5; 32]),
            decision: TraceDecision::OmittedBecause(owner_fact()),
        };
        let not_run = TraceEntry {
            subject: ExactIdentity::decoded([5; 32]),
            decision: TraceDecision::NotRun,
        };
        assert_ne!(selected, omitted);
        assert_ne!(omitted, not_run);
        assert_ne!(selected, not_run);
    }

    /// law: origin.traces-keep-selection-order-and-a-declared-bound — the first
    /// entry recorded is the first entry held, and a trace past its bound
    /// refuses on the trace-entry axis.
    /// Owed reversal: a constructor that sorted entries must break this law.
    #[test]
    fn traces_keep_selection_order_and_a_declared_bound() {
        let first = TraceEntry {
            subject: ExactIdentity::decoded([6; 32]),
            decision: TraceDecision::NotRun,
        };
        let second = TraceEntry {
            subject: ExactIdentity::decoded([7; 32]),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let recorded = DecisionTrace::recorded(first, vec![second]);
        assert!(recorded.is_ok_and(|trace| trace.len() == 2 && *trace.first() == first));

        let overrun: Vec<TraceEntry> = core::iter::repeat_n(second, TraceEntryLimit::MAX).collect();
        let refused = DecisionTrace::recorded(first, overrun);
        assert!(refused.is_err_and(|planning| matches!(
            planning.issues.first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::TraceEntries,
                ..
            }
        )));
    }

    /// law: origin.nonclaims-cite-an-owner-fact — a stated nonclaim names the
    /// fact that leaves it unclaimed rather than standing as a bare disclaimer.
    /// Owed reversal: a nonclaim without a citation must break this law.
    #[test]
    fn nonclaims_cite_an_owner_fact() {
        let nonclaim = Nonclaim {
            unclaimed: ExactIdentity::decoded([8; 32]),
            because: owner_fact(),
        };
        assert_eq!(nonclaim.because, owner_fact());
    }
}

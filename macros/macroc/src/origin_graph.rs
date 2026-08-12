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
    NonclaimSubject, OriginEdgeLimit, OriginNodeSubject, OwnerFactRef, ProjectionIdentity,
    TraceEntryLimit, TracedSubject, encode_bytes, encode_length,
};
use crate::refusal::{BoundAxis, ProjectionPlanning};
use threadpak::types::{ConstLimit, NonEmptyBounded};

threadpak::closed_register! {
    /// The closed roster of origin relations — the settled categories an edge may
    /// stand for. Fourteen, and the roster is the vocabulary: an edge that means
    /// something else is a law change, not a new string.
    ///
    /// `ALL` is the roster in the order the plane states it, and `slot` is the
    /// byte a canonical encoding carries for a relation.
    pub enum OriginRelation {
        /// A declaration a person authored.
        AuthoredDeclaration = "authored-declaration", "a declaration a person authored";
        /// An authored pattern instantiated with typed arguments.
        PatternInstantiation = "pattern-instantiation",
            "an authored pattern instantiated with typed arguments";
        /// Meaning derived from admitted meaning.
        SemanticDerivation = "semantic-derivation", "meaning derived from admitted meaning";
        /// A declaration fragment constructed from parts.
        FragmentConstruction = "fragment-construction",
            "a declaration fragment constructed from parts";
        /// A link an author stated explicitly.
        ExplicitLink = "explicit-link", "a link an author stated explicitly";
        /// A normalization step that changed form and not meaning.
        Normalization = "normalization",
            "a normalization step that changed form and not meaning";
        /// A projection profile selected.
        ProfileSelection = "profile-selection", "a projection profile selected";
        /// A projection kind selected under that profile.
        ProjectionSelection = "projection-selection",
            "a projection kind selected under that profile";
        /// Wrapper components composed for a host.
        WrapperComposition = "wrapper-composition", "wrapper components composed for a host";
        /// Typed material rendered into a target's syntax.
        Rendering = "rendering", "typed material rendered into a target's syntax";
        /// A rendered surface bound to a host contract.
        HostBinding = "host-binding", "a rendered surface bound to a host contract";
        /// A test descriptor derived from an obligation.
        TestDerivation = "test-derivation", "a test descriptor derived from an obligation";
        /// A benchmark descriptor derived from a work formula.
        BenchmarkDerivation = "benchmark-derivation",
            "a benchmark descriptor derived from a work formula";
        /// A diagnostic derived from an observed disagreement.
        DiagnosticDerivation = "diagnostic-derivation",
            "a diagnostic derived from an observed disagreement";
    }
}

/// One edge of the origin graph: which node, under which relation, produced
/// which node. The edge is directed and names both ends by identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginEdge {
    /// The node the relation starts at.
    pub from: ProjectionIdentity<OriginNodeSubject>,
    /// The settled relation this edge stands for.
    pub relation: OriginRelation,
    /// The node the relation produces.
    pub to: ProjectionIdentity<OriginNodeSubject>,
}

impl OriginEdge {
    /// Append this edge's canonical bytes: the node it starts at, the relation
    /// slot, the node it produces.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.from.as_bytes(), into);
        into.push(self.relation.slot());
        encode_bytes(self.to.as_bytes(), into);
    }
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

    /// The edges, in the order the trail walks them.
    ///
    /// The walk order is the trail's own meaning — it is the path back to the
    /// authored material — so unlike a declared SET, an identity may be derived
    /// from it and this iteration is load-bearing.
    pub fn iter(&self) -> impl Iterator<Item = &OriginEdge> {
        self.edges.iter()
    }

    /// Append this trail's canonical bytes: the edge count, then every edge in
    /// walk order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(self.edges.len(), into);
        for edge in self.edges.iter() {
            edge.encode_into(into);
        }
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

impl TraceDecision {
    /// The decision's discriminant byte, written ahead of its citation so a
    /// selection can never encode as an omission over the same fact.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::SelectedBecause(_) => 0,
            Self::OmittedBecause(_) => 1,
            Self::NotRun => 2,
        }
    }

    /// Append this decision's canonical bytes: the discriminant, then the cited
    /// fact where one was cited.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::SelectedBecause(cited) | Self::OmittedBecause(cited) => {
                encode_bytes(&cited.citation_bytes(), into);
            }
            Self::NotRun => encode_bytes(&[], into),
        }
    }
}

/// One recorded decision about one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceEntry {
    /// The subject decided about.
    pub subject: ProjectionIdentity<TracedSubject>,
    /// What was decided, and on whose fact.
    pub decision: TraceDecision,
}

impl TraceEntry {
    /// Append this entry's canonical bytes: the subject, then the decision.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.subject.as_bytes(), into);
        self.decision.encode_into(into);
    }
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

    /// The entries, in selection order.
    ///
    /// Selection order is the trace's meaning, so an identity may be derived
    /// from it: two plans that decided the same things in a different order
    /// decided differently.
    pub fn iter(&self) -> impl Iterator<Item = &TraceEntry> {
        self.entries.iter()
    }

    /// Append this trace's canonical bytes: the entry count, then every entry in
    /// selection order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(self.entries.len(), into);
        for entry in self.entries.iter() {
            entry.encode_into(into);
        }
    }
}

/// One thing a plan explicitly does not claim, and the owner fact that leaves
/// it unclaimed. Stated nonclaims are what keep a trace from reading as a
/// stronger promise than the plan made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonclaim {
    /// The subject the plan makes no claim about.
    pub unclaimed: ProjectionIdentity<NonclaimSubject>,
    /// The owner fact that leaves it unclaimed.
    pub because: OwnerFactRef,
}

impl Nonclaim {
    /// Append this nonclaim's canonical bytes: the unclaimed subject, then the
    /// fact that leaves it unclaimed.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.unclaimed.as_bytes(), into);
        encode_bytes(&self.because.citation_bytes(), into);
    }
}

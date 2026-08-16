//! The origin-graph home's declarations: the settled relations an edge may stand
//! for, the edges and the trail they walk, what the plane decided and on whose
//! fact, and what a plan leaves unclaimed.
//!
//! Declarations only.
//! Every road that reaches a private field — the trail's edges, the trace's
//! entries — lives in `type_guard.rs`, this file's own child.
//! That is what makes the orphan law structural: there is no seam anywhere else
//! that can draw a trail.

use crate::plane::{
    NonclaimSubject, OriginEdgeLimit, OriginNodeSubject, OwnerFactRef, ProjectionIdentity,
    TraceEntryLimit, TracedSubject,
};
use threadpak::types::NonEmptyBounded;

#[path = "type_guard.rs"]
mod guard;

threadpak::closed_register! {
    /// The closed roster of origin relations — the settled categories an edge may
    /// stand for.
    ///
    /// The roster is the vocabulary: an edge that means something else is a law
    /// change, not a new string.
    /// A relation's `slot` is the byte a canonical encoding carries for it.
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
/// which node.
///
/// The edge is directed and names both ends by identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginEdge {
    /// The node the relation starts at.
    pub from: ProjectionIdentity<OriginNodeSubject>,
    /// The settled relation this edge stands for.
    pub relation: OriginRelation,
    /// The node the relation produces.
    pub to: ProjectionIdentity<OriginNodeSubject>,
}

/// The origin trail every generated unit carries: a bounded, structurally
/// non-empty, END-TO-END walk back to the authored material.
///
/// Emptiness is not refused here because emptiness cannot be built here.
/// Discontinuity is refused where the trail is drawn, because a sequence of
/// edges that does not join is two walks presented as one and no reader can tell
/// which end is provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OriginTrail {
    edges: NonEmptyBounded<OriginEdge, OriginEdgeLimit>,
}

/// What the plane decided about one subject, and on whose fact.
///
/// A selection and an omission both cite an owner fact, because both are
/// decisions someone's declaration caused.
/// A check that did not run cites nothing, because there is nothing to cite —
/// and it stays distinct from a check that ran and omitted.
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
    pub subject: ProjectionIdentity<TracedSubject>,
    /// What was decided, and on whose fact.
    pub decision: TraceDecision,
}

/// The decision trace of one plan: the entries in selection order, bounded.
///
/// # Ordering
///
/// The order is the order the plane decided, preserved exactly.
/// Reordering a trace would make it an inventory rather than a record of what
/// happened.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecisionTrace {
    entries: NonEmptyBounded<TraceEntry, TraceEntryLimit>,
}

/// One thing a plan explicitly does not claim, and the owner fact that leaves it
/// unclaimed.
///
/// Stated nonclaims are what keep a trace from reading as a stronger promise
/// than the plan made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonclaim {
    /// The subject the plan makes no claim about.
    pub unclaimed: ProjectionIdentity<NonclaimSubject>,
    /// The owner fact that leaves it unclaimed.
    pub because: OwnerFactRef,
}

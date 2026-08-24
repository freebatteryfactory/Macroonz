//! The origin home's declarations: the relations an edge may stand for, the trail those edges walk, what a plan decided and on whose fact, and what it leaves unclaimed.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes the orphan law structural: there is no other seam that can draw a trail.

use crate::bounded::{Empty, NonEmpty, Overflow};
use crate::identity::{self, Identity, OwnerFact};

#[path = "type_guard.rs"]
mod guard;

/// Edges one trail may draw.
///
/// A trail is the end-to-end walk from a generated unit back to authored material, one edge per act that stands between them, and a walk longer than this has stopped being provenance a reader can follow.
pub const ORIGIN_EDGE_LIMIT: usize = 64;

/// Entries one decision trace may record.
pub const TRACE_ENTRY_LIMIT: usize = 128;

/// What one edge of the origin graph stands for.
///
/// The roster is the vocabulary: an edge that means something else is a law change rather than a new string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OriginRelation {
    /// A declaration a person authored.
    AuthoredDeclaration,
    /// An authored pattern instantiated with typed arguments.
    PatternInstantiation,
    /// Meaning derived from meaning already established.
    SemanticDerivation,
    /// A link an author stated explicitly.
    ExplicitLink,
    /// A step that changed form and not meaning.
    Normalization,
    /// A projection profile selected.
    ProfileSelection,
    /// A projection kind selected under that profile.
    ProjectionSelection,
    /// Typed material rendered into a target's syntax.
    Rendering,
    /// A test descriptor derived from an obligation.
    TestDerivation,
    /// A benchmark descriptor derived from a work formula.
    BenchmarkDerivation,
    /// A diagnostic derived from an observed disagreement.
    DiagnosticDerivation,
}

/// One directed edge of the origin graph: which node, under which relation, produced which node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginEdge {
    /// The node the relation starts at.
    pub from: Identity<identity::OriginNode>,
    /// The relation this edge stands for.
    pub relation: OriginRelation,
    /// The node the relation produces.
    pub to: Identity<identity::OriginNode>,
}

/// The walk from one generated unit back to the material a person authored.
///
/// Edges are held in walk order, and there is always at least one of them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OriginTrail {
    edges: NonEmpty<OriginEdge, ORIGIN_EDGE_LIMIT>,
}

/// How drawing a trail refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrailError {
    /// The edges offered are not one walk.
    Discontinuous {
        /// The position of the first edge that does not start where the edge before it ended, counted from the walk's first edge.
        at: u32,
    },
    /// No edge was offered.
    Empty(Empty),
    /// More edges were offered than the trail admits.
    Overflow(Overflow),
}

/// What a plan decided about one subject, and on whose fact.
///
/// A selection and an omission each cite the fact that decided them, because both are decisions somebody's declaration caused; a check that did not run cites nothing and stays distinct from one that ran and omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceDecision {
    /// Selected, because this fact required it.
    SelectedBecause(OwnerFact),
    /// Omitted, because this fact excluded it.
    OmittedBecause(OwnerFact),
    /// The check did not run. Never a pass, never a fail.
    NotRun,
}

/// One recorded decision about one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceEntry {
    /// The subject decided about.
    pub subject: Identity<identity::Traced>,
    /// What was decided, and on whose fact.
    pub decision: TraceDecision,
}

/// One plan's decisions, in the order it made them.
///
/// # Ordering
///
/// Selection order is the trace's own meaning, so an identity may be derived from it: two plans that decided the same things in a different order decided differently.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecisionTrace {
    entries: NonEmpty<TraceEntry, TRACE_ENTRY_LIMIT>,
}

/// One thing a plan explicitly does not claim, and the fact that leaves it unclaimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonclaim {
    /// The subject the plan makes no claim about.
    pub unclaimed: Identity<identity::Nonclaim>,
    /// The fact that leaves it unclaimed.
    pub because: OwnerFact,
}

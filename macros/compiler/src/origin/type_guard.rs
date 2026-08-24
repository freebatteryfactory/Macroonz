//! The origin home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's two central claims structural.
//! A trail is drawn here and nowhere else, so a generated unit with no origin, and a trail whose edges do not join, are values that do not exist rather than shapes a reader has to watch for.
//! A trace is recorded here and nowhere else, in the order the decisions were made.

use super::{
    DecisionTrace, ORIGIN_EDGE_LIMIT, OriginEdge, OriginTrail, TRACE_ENTRY_LIMIT, TraceEntry,
    TrailError,
};
use crate::bounded::{NonEmpty, NonEmptyError};

impl OriginTrail {
    /// The one-edge trail, for a unit whose origin is a single act.
    #[must_use]
    pub fn from_edge(edge: OriginEdge) -> Self {
        Self {
            edges: NonEmpty::one(edge),
        }
    }

    /// Draws the walk these edges make, in the order they are offered.
    ///
    /// # Errors
    ///
    /// The two questions are dependent, and continuity is settled first: the ceiling is a fact about how long a WALK may be, so measuring a sequence that is not one would be measuring the wrong thing.
    /// Returns [`TrailError::Discontinuous`] naming the first edge that does not start where its predecessor ended, then [`TrailError::Empty`] or [`TrailError::Overflow`] when a joined walk does not fit the trail.
    /// A walk that does not fit refuses rather than truncating, because a truncated walk is how an origin silently becomes a span.
    pub fn drawn(offered: Vec<OriginEdge>) -> Result<Self, TrailError> {
        if let Some(at) = break_position(&offered) {
            return Err(TrailError::Discontinuous { at });
        }
        NonEmpty::new(offered)
            .map(|edges| Self { edges })
            .map_err(TrailError::from)
    }

    /// The edge the walk starts at, which every trail has.
    #[must_use]
    pub fn first(&self) -> &OriginEdge {
        self.edges.first()
    }

    /// The edges, in the order the trail walks them.
    ///
    /// # Ordering
    ///
    /// Walk order is the trail's own meaning — it is the path back to the authored material — so an identity may be derived from it.
    #[must_use]
    pub fn edges(&self) -> &NonEmpty<OriginEdge, ORIGIN_EDGE_LIMIT> {
        &self.edges
    }
}

/// The position of the first edge that does not start where its predecessor ended, or `None` where the sequence joins end to end.
///
/// Joining means identity equality and nothing looser: both ends of an edge are identities over the origin-node subject, so two nodes are one node exactly when their bytes agree.
/// The position counts from the walk's first edge, so it names the edge a caller must repair rather than the gap's own ordinal.
fn break_position(edges: &[OriginEdge]) -> Option<u32> {
    edges
        .iter()
        .zip(edges.iter().skip(1))
        .position(|(previous, edge)| edge.from != previous.to)
        .map(|gap| u32::try_from(gap.saturating_add(1)).unwrap_or(u32::MAX))
}

impl DecisionTrace {
    /// The one-entry trace, for a plan that decided once.
    #[must_use]
    pub fn from_entry(entry: TraceEntry) -> Self {
        Self {
            entries: NonEmpty::one(entry),
        }
    }

    /// Records these entries as the decisions a plan made, in the order they are offered.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyError`] when no entry is offered, or when more are offered than a trace admits.
    pub fn recorded(offered: Vec<TraceEntry>) -> Result<Self, NonEmptyError> {
        NonEmpty::new(offered).map(|entries| Self { entries })
    }

    /// The first decision the plan made, which every trace has.
    #[must_use]
    pub fn first(&self) -> &TraceEntry {
        self.entries.first()
    }

    /// The entries, in selection order; structurally at least one.
    #[must_use]
    pub fn entries(&self) -> &NonEmpty<TraceEntry, TRACE_ENTRY_LIMIT> {
        &self.entries
    }
}

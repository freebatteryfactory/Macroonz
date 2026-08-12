//! The origin-graph home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! two central claims structural. A trail is drawn HERE, so a generated unit
//! with no origin is a value nobody can build rather than a shape a check has to
//! catch. A trace is recorded HERE, in selection order, so a trace that had been
//! sorted into an inventory is likewise not a value that exists. There is no
//! other seam in the crate that can produce either one.

use super::{DecisionTrace, OriginEdge, OriginTrail, TraceEntry};
use crate::plane::{AuthoringLimitProfile, OriginEdgeLimit, TraceEntryLimit};
use crate::refusal::{BoundAxis, ProjectionPlanning};
use threadpak::types::{ConstLimit, NonEmptyBounded, PositiveLimit};

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
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
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
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
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
}

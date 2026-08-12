//! The origin-graph home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! central claims structural. A trail is drawn HERE, so a generated unit with no
//! origin is a value nobody can build rather than a shape a check has to catch,
//! and a trail whose edges do not join is refused at the one seam that can draw
//! one. A trace is recorded HERE, in selection order, so a trace that had been
//! sorted into an inventory is likewise not a value that exists. There is no
//! other seam in the crate that can produce either one.

use super::{DecisionTrace, OriginEdge, OriginTrail, TraceEntry};
use crate::plane::{AuthoringLimitProfile, OriginEdgeLimit, TraceEntryLimit};
use crate::refusal::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};
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
    /// # The two checks are dependent, and continuity runs first
    ///
    /// A trail is a WALK: each edge starts where the one before it ended, and
    /// following it backwards is what makes a generated unit's provenance
    /// readable. A sequence of edges that does not join is not a shorter walk, it
    /// is two walks presented as one — and whichever end a reader trusts, the
    /// other end is provenance nobody established. Accepting one would let a
    /// disconnected list receive canonical bytes as a provenance path, which is
    /// the exact defect the orphan law exists to prevent, one level up.
    ///
    /// So continuity is settled before the magnitude. The declared bound is a
    /// fact about how long a walk may be, and measuring a sequence that is not a
    /// walk would be measuring the wrong thing.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming
    /// [`ProjectionPlanningIssue::TrailDiscontinuous`] with the position of the
    /// first edge that does not join its predecessor, and
    /// [`BoundAxis::OriginEdges`] when a joined walk outgrows the declared
    /// bound. A trail that does not fit refuses: truncating a trail is how an
    /// origin silently becomes a span.
    pub fn drawn(first: OriginEdge, rest: Vec<OriginEdge>) -> Result<Self, ProjectionPlanning> {
        if let Some(at) = break_position(&first, &rest) {
            return Err(ProjectionPlanning::established(
                ProjectionPlanningIssue::TrailDiscontinuous { at },
            ));
        }
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

/// The position of the first edge that does not start where its predecessor
/// ended, or `None` where the sequence joins end to end.
///
/// Identity equality is what "joins" means here, and it is well defined without
/// interpretation: both ends of an edge are
/// [`ProjectionIdentity`](crate::plane::ProjectionIdentity) values over the
/// origin-node subject, so two nodes are the same node exactly when their bytes
/// are. Nothing is normalized, and no near-match is accepted.
///
/// The position counts from the trail's first edge, so it names the edge a
/// caller must repair rather than the gap's ordinal.
fn break_position(first: &OriginEdge, rest: &[OriginEdge]) -> Option<u32> {
    let mut previous = first;
    for (position, edge) in rest.iter().enumerate() {
        if edge.from != previous.to {
            return Some(u32::try_from(position.saturating_add(1)).unwrap_or(u32::MAX));
        }
        previous = edge;
    }
    None
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

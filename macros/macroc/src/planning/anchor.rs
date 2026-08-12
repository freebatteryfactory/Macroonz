//! What a plan hangs off, and what it therefore watches.
//!
//! Both answers are read off the same typed postures rather than chosen at a
//! call site. A plan caused by the machine's declaration fragments anchors on
//! the first of them and watches it; a plan caused by captured token material
//! anchors on the capture and watches THAT. Neither posture is dressed up as
//! the other, and neither is an absence, so an expansion-time plan states its
//! own footing instead of borrowing a linked artifact that does not exist yet.

use super::{CauseAnchoring, GraphAnchoring, InvalidationTrigger, ProjectionContext};
use crate::plane::TranscriptAnchoring;

impl CauseAnchoring {
    /// What a transcript derived under this cause is anchored to.
    ///
    /// A plan hangs off what caused it: the captured declaration where the cause
    /// IS the capture, and the first declared fragment where a caller holds the
    /// machine's own identities. The remaining fragments are inside the
    /// transcript's content rather than at its anchor, because an anchor names
    /// one thing.
    #[must_use]
    pub fn anchoring(&self) -> TranscriptAnchoring {
        match self {
            Self::Declarations(sources) => {
                TranscriptAnchoring::UnderOwnerIdentity(*sources.first().as_bytes())
            }
            Self::CapturedDeclaration(captured) => {
                TranscriptAnchoring::UnderProjectionIdentity(*captured.as_bytes())
            }
        }
    }
}

impl ProjectionContext {
    /// The invalidation trigger that watches whatever this context was caused
    /// by — the fragment where a caller holds one, and the captured declaration
    /// where the cause IS the capture.
    #[must_use]
    pub fn cause_trigger(&self) -> InvalidationTrigger {
        match &self.sources {
            CauseAnchoring::Declarations(sources) => {
                InvalidationTrigger::SourceDeclarationChanged {
                    watched: *sources.first(),
                }
            }
            CauseAnchoring::CapturedDeclaration(captured) => {
                InvalidationTrigger::CapturedDeclarationChanged { watched: *captured }
            }
        }
    }

    /// The invalidation trigger that watches whatever this context was decided
    /// against.
    #[must_use]
    pub const fn graph_trigger(&self) -> InvalidationTrigger {
        match self.graph {
            GraphAnchoring::ClosedGraph(graph) => {
                InvalidationTrigger::GraphIdentityChanged { watched: graph }
            }
            GraphAnchoring::CapturedDeclarationOnly(captured) => {
                InvalidationTrigger::CapturedDeclarationChanged { watched: captured }
            }
        }
    }
}

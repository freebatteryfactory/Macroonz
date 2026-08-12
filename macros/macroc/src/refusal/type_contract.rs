//! The refusal home's declarative surface: the family shape this home declares,
//! and the closed table its issue roster is read through.
//!
//! Both are declarations rather than computations. The family states its shape
//! and its selection order as constants; the issue roster states, per variant,
//! the slot an encoding writes for it. Nothing here decides anything — the
//! deciding is `establish.rs`.

use super::{ProjectionPlanning, ProjectionPlanningIssue};
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for ProjectionPlanning {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl ProjectionPlanningIssue {
    /// The issue kind's position in the declared roster, written ahead of the
    /// issue's own material so two kinds never encode alike.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::MissingOwnerFact { .. } => 0,
            Self::ContradictoryOwnerFacts { .. } => 1,
            Self::UnknownProjectionKind { .. } => 2,
            Self::ProfileUnsupported { .. } => 3,
            Self::BoundExceeded { .. } => 4,
            Self::MembershipIncomplete { .. } => 5,
            Self::OrphanGeneratedNode { .. } => 6,
            Self::MembershipDoubled { .. } => 7,
        }
    }
}

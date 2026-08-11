//! The planning refusal family: how the services say no while planning.
//!
//! Planning issues are independent and co-establishable — a plan may name an
//! unknown kind *and* exceed a declared bound in one pass — so the family takes
//! the machine's issue-collection shape: a bounded, non-empty collection over a
//! closed issue set, carrying its enumeration posture as an instance value. No
//! primary issue is ever elected, and a zero-issue refusal is unrepresentable.
//!
//! Every seam in the plane that can refuse returns this family body. The
//! universal refusal envelope is the publication form and is minted where
//! reasons are registered, which is the machine's business, not the plane's.

use crate::plane::{
    GeneratedUnitSubject, OwnerFactRef, PlanSubject, PlanningIssueLimit, ProfileVersion,
    ProjectionIdentity, ProjectionKindSubject, ProjectionProfileSubject,
};
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{NonEmptyBounded, NonEmptyBoundedConstruction};

/// The plan's declared bound axes. A bound refusal names which magnitude it
/// exceeded, so "too big" is never an unlocated word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundAxis {
    /// The source declarations one plan may name.
    Declarations,
    /// The outputs one plan may declare.
    Outputs,
    /// The entries one decision trace may record.
    TraceEntries,
    /// The diagnostics one pass may carry.
    Diagnostics,
    /// The edges one origin trail may draw.
    OriginEdges,
    /// The bytes one bounded projection may carry.
    Bytes,
}

/// The declared bound axes, in the order the plane states them.
pub const BOUND_AXES: [BoundAxis; 6] = [
    BoundAxis::Declarations,
    BoundAxis::Outputs,
    BoundAxis::TraceEntries,
    BoundAxis::Diagnostics,
    BoundAxis::OriginEdges,
    BoundAxis::Bytes,
];

/// The plan seats an owner fact can be missing from. Only seats a plan can
/// actually leave unfurnished appear: every other seat is structurally
/// required by the plan's own shape, so its absence is unrepresentable rather
/// than refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanSeat {
    /// The context's target binding, where the kind requires a bound host
    /// contract and the context is target-free.
    TargetBinding,
}

/// The pair of owner facts a contradiction stands between. Neither side is
/// elected as the offender: the disagreement is the fact, and naming one of
/// them as wrong would be a judgment the plane has no standing to make.
///
/// Boxed inside its issue because a refusal body travels by value through every
/// seam in the plane, and the rarest issue must not set the size of all of them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContradictionPair {
    /// The first constraining fact.
    pub left: OwnerFactRef,
    /// The second constraining fact.
    pub right: OwnerFactRef,
}

/// The closed planning issue set.
///
/// No issue is payload-free: an issue names what it observed, because a bare
/// variant makes the caller guess. Several of these are reachable only on the
/// decoded route — a plan authored through the typed seams cannot name an
/// unimplemented kind, cannot orphan a generated node, and cannot present an
/// incomplete membership, because each of those is a shape the typed road
/// cannot express.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectionPlanningIssue {
    /// A seat the kind requires is unfurnished.
    MissingOwnerFact {
        /// Which seat.
        seat: PlanSeat,
    },
    /// Two owner facts that decided this plan disagree.
    ContradictoryOwnerFacts {
        /// The disagreeing pair.
        between: Box<ContradictionPair>,
    },
    /// A decoded plan names a projection kind the plane does not implement.
    /// Unreachable on the typed route, where a kind is a type.
    UnknownProjectionKind {
        /// The named kind's identity.
        named: ProjectionIdentity<ProjectionKindSubject>,
    },
    /// The named profile and version admit no such projection.
    ProfileUnsupported {
        /// The profile.
        profile: ProjectionIdentity<ProjectionProfileSubject>,
        /// The profile version.
        version: ProfileVersion,
    },
    /// A declared magnitude was exceeded.
    BoundExceeded {
        /// Which magnitude.
        axis: BoundAxis,
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
    /// A declared sibling output is absent from the plan's membership. The
    /// output set is a firewall: a plan states its complete set or refuses,
    /// because a partially declared set is what silently drops a projection.
    MembershipIncomplete {
        /// The absent unit.
        absent: ProjectionIdentity<GeneratedUnitSubject>,
    },
    /// A generated node arrived with no origin edge. Unreachable on the typed
    /// route, where the trail seat is structurally non-empty.
    OrphanGeneratedNode {
        /// The orphaned node.
        node: ProjectionIdentity<GeneratedUnitSubject>,
    },
}

/// The planning refusal family body.
///
/// Independent members, no ladder, no primary issue, posture carried as an
/// instance value. A body that stopped at its declared bound says so rather
/// than implying no further defects exist.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionPlanning {
    /// The established issues — at least one, at most the declared bound.
    pub issues: NonEmptyBounded<ProjectionPlanningIssue, PlanningIssueLimit>,
    /// Whether every applicable check ran.
    pub posture: CompletionPosture,
}

impl RefusalFamily for ProjectionPlanning {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl ProjectionPlanning {
    /// The one-issue body, for a seam whose checks can establish exactly one
    /// issue. Total: the declared bound admits an item by compile-time proof, so
    /// refusing never needs an error road of its own.
    #[must_use]
    pub fn established(issue: ProjectionPlanningIssue) -> Self {
        Self {
            issues: NonEmptyBounded::singleton(issue),
            posture: CompletionPosture::Complete,
        }
    }

    /// The several-issue body, for a pass whose checks co-establish. When the
    /// supplied issues outrun the declared bound the body keeps the first and
    /// reports that enumeration stopped there — it never silently drops the
    /// remainder and never claims completeness it does not have.
    #[must_use]
    pub fn co_established(
        first: ProjectionPlanningIssue,
        rest: Vec<ProjectionPlanningIssue>,
    ) -> Self {
        match NonEmptyBounded::admitted_const(first.clone(), rest) {
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

    /// The body a bounded seam refuses with: the axis it overran, the magnitude
    /// it declared, and the count it observed.
    #[must_use]
    pub fn bound_exceeded(axis: BoundAxis, bound: usize, observed: usize) -> Self {
        Self::established(ProjectionPlanningIssue::BoundExceeded {
            axis,
            bound: u64::try_from(bound).unwrap_or(u64::MAX),
            observed: u64::try_from(observed).unwrap_or(u64::MAX),
        })
    }
}

/// The identity of one plan, as a bundle names its members. Carried here
/// because a bundle's membership and the planning family's issues are the two
/// places a plan is spoken of by identity rather than by value.
pub type PlanIdentity = ProjectionIdentity<PlanSubject>;

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
    ExactIdentity, GeneratedUnitSubject, OwnerFactRef, PlanSubject, PlanningIssueLimit,
    ProfileVersion, ProjectionKindSubject, ProjectionProfileSubject,
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
        named: ExactIdentity<ProjectionKindSubject>,
    },
    /// The named profile and version admit no such projection.
    ProfileUnsupported {
        /// The profile.
        profile: ExactIdentity<ProjectionProfileSubject>,
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
        absent: ExactIdentity<GeneratedUnitSubject>,
    },
    /// A generated node arrived with no origin edge. Unreachable on the typed
    /// route, where the trail seat is structurally non-empty.
    OrphanGeneratedNode {
        /// The orphaned node.
        node: ExactIdentity<GeneratedUnitSubject>,
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
pub type PlanIdentity = ExactIdentity<PlanSubject>;

#[cfg(test)]
mod laws {
    use super::{BOUND_AXES, BoundAxis, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
    use crate::plane::{ExactIdentity, PlanningIssueLimit};
    use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
    use threadpak::types::ConstLimit;

    /// The closed bound-axis roster, proven closed by an exhaustive match: a new
    /// axis stops compiling here until it is placed.
    const fn axis_index(axis: BoundAxis) -> usize {
        match axis {
            BoundAxis::Declarations => 0,
            BoundAxis::Outputs => 1,
            BoundAxis::TraceEntries => 2,
            BoundAxis::Diagnostics => 3,
            BoundAxis::OriginEdges => 4,
            BoundAxis::Bytes => 5,
        }
    }

    /// law: refusal.bound-axes-are-six-and-closed — the plan's declared
    /// magnitudes are a closed roster, each distinct.
    /// Owed reversal: adding an axis without placing it must break this law.
    #[test]
    fn bound_axes_are_six_and_closed() {
        assert_eq!(BOUND_AXES.len(), 6);
        let indexes: Vec<usize> = BOUND_AXES.iter().copied().map(axis_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: refusal.family-is-an-issue-collection — the planning family declares
    /// the collection shape and elects no primary issue, so its selection order
    /// is empty by law rather than by omission.
    /// Owed reversal (red twin): declaring `SingleCause` with a non-empty
    /// collection body must break this law.
    #[test]
    fn family_is_an_issue_collection() {
        assert!(matches!(
            ProjectionPlanning::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(ProjectionPlanning::SELECTION_ORDER.is_empty());
    }

    /// law: refusal.one-issue-body-is-total — a seam that establishes one issue
    /// builds its refusal without an error road of its own, so refusing is never
    /// the place a caller reaches for a panic.
    /// Owed reversal: a fallible one-issue road must break this law.
    #[test]
    fn one_issue_body_is_total() {
        let refusal = ProjectionPlanning::established(ProjectionPlanningIssue::MissingOwnerFact {
            seat: PlanSeat::TargetBinding,
        });
        assert_eq!(refusal.issues.len(), 1);
        assert!(matches!(refusal.posture, CompletionPosture::Complete));
        assert!(matches!(
            refusal.issues.first(),
            ProjectionPlanningIssue::MissingOwnerFact {
                seat: PlanSeat::TargetBinding
            }
        ));
    }

    /// law: refusal.co-established-issues-stay-whole-or-say-they-stopped — a
    /// body carrying several issues either covers them all or reports the
    /// declared bound that stopped it.
    /// Owed reversal (red twin): a body that dropped the remainder silently must
    /// break this law.
    #[test]
    fn co_established_issues_stay_whole_or_say_they_stopped() {
        let node = ExactIdentity::decoded([1; 32]);
        let whole = ProjectionPlanning::co_established(
            ProjectionPlanningIssue::OrphanGeneratedNode { node },
            vec![ProjectionPlanningIssue::MembershipIncomplete { absent: node }],
        );
        assert_eq!(whole.issues.len(), 2);
        assert!(matches!(whole.posture, CompletionPosture::Complete));

        let overrun: Vec<ProjectionPlanningIssue> = core::iter::repeat_n(
            ProjectionPlanningIssue::OrphanGeneratedNode { node },
            PlanningIssueLimit::MAX,
        )
        .collect();
        let stopped = ProjectionPlanning::co_established(
            ProjectionPlanningIssue::MembershipIncomplete { absent: node },
            overrun,
        );
        assert_eq!(stopped.issues.len(), 1);
        assert!(matches!(
            stopped.posture,
            CompletionPosture::EarlyStopped {
                stopped_at: StopBound::DeclaredIssueBound
            }
        ));
    }

    /// law: refusal.bound-refusals-name-their-magnitude — a bound refusal states
    /// the axis, the declared bound, and the observed count.
    /// Owed reversal: a payload-free bound cause must break this law.
    #[test]
    fn bound_refusals_name_their_magnitude() {
        let refusal = ProjectionPlanning::bound_exceeded(BoundAxis::Outputs, 32, 33);
        assert!(matches!(
            refusal.issues.first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::Outputs,
                bound: 32,
                observed: 33
            }
        ));
    }
}

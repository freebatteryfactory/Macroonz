//! The refusal home's declarations: the bound axes a plan can overrun, the plan
//! seats a fact can be missing from, the closed planning issue set, and the
//! family body they travel in.
//!
//! Declarations only. The body's one seat is private and its roads live in
//! `type_guard.rs`, this file's own child. Readable is not the same as writable:
//! a refusal body whose issues a caller could not read would be a refusal nobody
//! can act on, so the seat is read back through a borrow — and a refusal a caller
//! could WRITE would be a seam minting the plane's own answer, so there is no
//! literal anybody outside the nucleus can spell.

use crate::plane::{
    GeneratedUnitSubject, OwnerFactRef, PlanningIssueLimit, ProfileVersion, ProjectionIdentity,
    ProjectionKindSubject, ProjectionProfileSubject,
};
use threadpak::refusal::AdmittedPrefix;

#[path = "type_guard.rs"]
mod guard;

threadpak::closed_register! {
    /// The plan's declared bound axes. A bound refusal names which magnitude it
    /// exceeded, so "too big" is never an unlocated word.
    ///
    /// `ALL` is the roster in the order the plane states the axes, and `slot` is
    /// that order read back as a position — the position a canonical encoding of
    /// an issue carries, and the one a diagnostic names the axis by.
    pub enum BoundAxis {
        /// The source declarations one plan may name.
        Declarations = "declarations", "the source declarations one plan may name";
        /// The outputs one plan may declare.
        Outputs = "outputs", "the outputs one plan may declare";
        /// The entries one decision trace may record.
        TraceEntries = "trace-entries", "the entries one decision trace may record";
        /// The diagnostics one pass may carry.
        Diagnostics = "diagnostics", "the diagnostics one pass may carry";
        /// The edges one origin trail may draw.
        OriginEdges = "origin-edges", "the edges one origin trail may draw";
        /// The bytes one bounded projection may carry.
        Bytes = "bytes", "the bytes one bounded projection may carry";
    }
}

threadpak::closed_register! {
    /// The plan seats an owner fact can be missing from. Only seats a plan can
    /// actually leave unfurnished appear: every other seat is structurally
    /// required by the plan's own shape, so its absence is unrepresentable rather
    /// than refused.
    pub enum PlanSeat {
        /// The context's target binding, where the kind requires a bound host
        /// contract and the context is target-free.
        TargetBinding = "target-binding", "the context's target binding";
    }
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
    /// Two planned members stand under one rendered role. A membership is a SET
    /// over roles: the closure check matches a rendered unit to a planned member
    /// BY ROLE, so a role carrying two members leaves that match electing one of
    /// them and proving nothing about the other.
    MembershipDoubled {
        /// The doubled role's position in its kind's declared roster.
        role_slot: u32,
        /// How many members stood under it.
        observed: u32,
    },
    /// An origin trail's edges do not join: the edge at this position starts at
    /// a node the edge before it did not produce.
    ///
    /// A trail is a WALK back to authored material, and a walk with a gap in it
    /// is not a shorter walk — it is two walks presented as one, and whichever
    /// end a reader trusts, the other end is provenance nobody established. The
    /// position is carried because "the trail is broken" without a position is a
    /// finding an author cannot repair.
    ///
    /// Last in the roster on purpose: the declared order is what a canonical
    /// encoding writes down as a slot, so a new issue joins at the end and moves
    /// nobody else's byte.
    TrailDiscontinuous {
        /// The position of the edge that does not join its predecessor, counted
        /// from the trail's first edge.
        at: u32,
    },
}

/// The planning refusal family body.
///
/// Independent members, no ladder, no primary issue, posture carried as an
/// instance value. A body that stopped at its declared bound says so rather
/// than implying no further defects exist.
#[must_use = "a refusal family body carries every planning issue the pass established"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionPlanning {
    /// The established issues — at least one, at most the declared bound —
    /// together with whether the body carries every issue its seam established
    /// or names how many stand outside that bound. One seat rather than two,
    /// because a coverage claim seated beside its body is a claim that can be
    /// swapped for another body's.
    ///
    /// Private, and that is the second half of the same claim. The coupled seat
    /// keeps a carry and its posture together; a PUBLIC seat on a one-field
    /// record hands the whole record back as a literal, so any holder of a body
    /// built for one seam could write it into another seam's refusal. Read back
    /// through [`ProjectionPlanning::body`].
    body: AdmittedPrefix<ProjectionPlanningIssue, PlanningIssueLimit>,
}

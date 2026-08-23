//! The refusal home's declarations: the bound axes a plan can overrun, the plan
//! seats a fact can be missing from, the closed planning issue set, the
//! magnitude a body of those issues is bounded by, and the family body they
//! travel in.
//!
//! Declarations only.
//! The body itself is DECLARED in `type_guard.rs`'s `seat` module — this file's
//! own grandchild — and published from here, because Rust's privacy is
//! module-scoped and a seat declared in this file would be inside the wall with
//! every other item this file declares.
//!
//! `encode.rs` beside them writes what an issue and a body ARE as canonical
//! bytes, because a planning refusal travels inside a projection disposition and
//! a disposition enters an identity's preimage.
//!
//! Readable is not the same as writable: a refusal body whose issues a caller
//! could not read would be a refusal nobody can act on, so the seat is read back
//! through a borrow — and a refusal a caller could WRITE would be a seam minting
//! the plane's own answer, so there is no literal anybody outside the seat module
//! can spell and no mint anybody outside the crate can call.

use crate::plane::{
    GeneratedUnitSubject, OwnerFactRef, ProfileVersion, ProjectionIdentity, ProjectionKindSubject,
    ProjectionProfileSubject,
};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitude.
//
// This home's own row, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on the row below
// are this home's, declared beside the capacity it governs.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many issues one planning refusal body may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Thirty — the closed issue roster's own cardinality once each multi-seat
    /// issue is counted PER SEAT, because a body carries at most one issue per
    /// seat a pass can establish one at. Six single-seat issues, the
    /// missing-fact issue over its one plan seat, the discontinuity issue over
    /// the one break a trail is refused at, the bound issue over its six axes,
    /// and the doubled-output issue over the sixteen roles a membership at the
    /// output magnitude could double.
    ///
    /// It is not a number chosen for room: a thirty-first issue would have to be
    /// a thirty-first establishable seat, which is a change to the roster below
    /// rather than to this magnitude.
    ///
    /// The roster and this number are two statements of one fact, held together
    /// by whoever edits them. The sixth single-seat issue arrived after this
    /// number was written and the number did not move, so a body could carry one
    /// issue per seat and be refused at the last of them.
    PlanningIssueLimit = 30,
}

macroonz::closed_register! {
    /// The plan's declared bound axes.
    ///
    /// A bound refusal names which magnitude it exceeded, so "too big" is never
    /// an unlocated word.
    /// An axis's `slot` is the position a canonical encoding of an issue carries
    /// for it, and the one a diagnostic names the axis by.
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

/// The pair of owner facts a contradiction stands between.
///
/// Neither side is elected as the offender: the disagreement is the fact, and
/// naming one of them as wrong would be a judgment the plane has no standing to
/// make.
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
/// variant makes the caller guess.
/// Several of these are reachable only on the decoded route — a plan authored
/// through the typed seams cannot name an unimplemented kind, cannot orphan a
/// generated node, and cannot present an incomplete membership, because each of
/// those is a shape the typed road cannot express.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectionPlanningIssue {
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
    /// A declared sibling output is absent from the plan's membership.
    /// The output set is a firewall: a plan states its complete set or refuses,
    /// because a partially declared set is what silently drops a projection.
    MembershipIncomplete {
        /// The absent unit.
        absent: ProjectionIdentity<GeneratedUnitSubject>,
    },
    /// A generated node arrived with no origin edge.
    /// Unreachable on the typed route, where the trail seat is structurally
    /// non-empty.
    OrphanGeneratedNode {
        /// The orphaned node.
        node: ProjectionIdentity<GeneratedUnitSubject>,
    },
    /// Two planned members stand under one rendered role.
    /// A membership is a SET over roles: the closure check matches a rendered
    /// unit to a planned member BY ROLE, so a role carrying two members leaves
    /// that match electing one of them and proving nothing about the other.
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
    /// end a reader trusts, the other end is provenance nobody established.
    /// The position is carried because "the trail is broken" without a position
    /// is a finding an author cannot repair.
    TrailDiscontinuous {
        /// The position of the edge that does not join its predecessor, counted
        /// from the trail's first edge.
        at: u32,
    },
    /// A cause set names more source declarations than the invalidation trigger
    /// roster can watch.
    ///
    /// A plan may name up to the declared source magnitude, and one roster seat
    /// carries one identity.
    /// Where the two disagree there is no partial answer to give: a watch set
    /// covering the first declaration and no other reads exactly like a complete
    /// one, so a plan hanging off three declarations and watching one is CURRENT
    /// after two of its three causes changed.
    /// That is not a narrower claim than the roster can support — it is a false
    /// one, and the seam refuses rather than issuing it.
    ///
    /// Both counts are carried because the fact is the disagreement between
    /// them: an author repairing this needs to know how far past the profile the
    /// cause set reached, and "unwatchable" without the pair is a finding nobody
    /// can act on.
    ///
    /// Last in the roster on purpose: the declared order is what a canonical
    /// encoding writes down as a slot, so a new issue joins at the end and moves
    /// nobody else's byte.
    CauseSetUnwatchable {
        /// How many source declarations the cause set names.
        named: u32,
        /// How many of them the trigger roster can watch.
        watchable: u32,
    },
}

/// The planning refusal family body, published from this file and DECLARED in
/// `type_guard.rs`'s `seat` module, beside the only roads that reach its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a
/// private field is private to the module the declaration lands in, and every
/// other item this file declares would have been inside that wall.
pub use guard::ProjectionPlanning;

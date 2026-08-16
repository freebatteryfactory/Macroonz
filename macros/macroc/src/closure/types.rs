//! The closure home's declarations: what a renderer materialized, how a
//! rendering and its plan can disagree, and the proof that they do not.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this
//! file's own child, which is what makes "tokens are emitted only from a
//! closure" structural rather than reviewed.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ClosureId, GeneratedUnitSubject, MembershipLimit, OutputBytesSubject, PlanId, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, ProjectionProvenance, RenderedByteLimit,
    RenderedRole, RenderedUnitSubject,
};
use crate::planning::{MemberDestination, PlannedMembership};
use crate::token::GeneratedTree;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

/// How one rendering failed to materialize a unit at all.
///
/// Distinct from a closure disagreement: nothing has been compared yet.
/// Each way names a declared magnitude the renderer would have passed.
#[must_use = "a rendering refusal names the magnitude the renderer would have passed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderingRefusal {
    /// The rendered bytes exceed the declared magnitude.
    /// A renderer that would emit past it refuses rather than materializing
    /// part of a unit.
    BytesUnbounded,
    /// The rendering carries more units than the declared membership magnitude
    /// admits.
    UnitsUnbounded,
}

/// One unit a renderer actually materialized.
///
/// Every seat is the renderer's own answer, which is what a closure rebuilds
/// the plan's membership out of.
///
/// # Nonclaims
///
/// The Rust source text is not a member of the unit.
/// It is [`GeneratedTree::inspected`] — a projection of the tree, for a person.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderedUnit<R: RenderedRole> {
    role: R,
    identity: ProjectionIdentity<RenderedUnitSubject>,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    destination: MemberDestination,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    tree: GeneratedTree,
    bytes: Bounded<u8, RenderedByteLimit>,
    digest: ProjectionIdentity<OutputBytesSubject>,
}

/// Everything one renderer produced for one plan.
///
/// Structurally non-empty: a rendering that materialized nothing is not a
/// rendering, and a plan whose membership is non-empty can never close over
/// one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderedProjection<R: RenderedRole> {
    units: NonEmptyBounded<RenderedUnit<R>, MembershipLimit>,
}

/// How one rendering and the plan it claims to materialize disagree.
///
/// Every issue about a role names it, because "the membership is wrong" is not
/// an answer anybody can act on.
/// The issues about the whole reconstruction name none, because there is no
/// role to name and electing one would be a stand-in nobody established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClosureIssue<R: RenderedRole> {
    /// A role the plan declared was not rendered at all.
    MemberMissing {
        /// The planned role nothing materialized.
        role: R,
    },
    /// A role was rendered that the plan never declared — the output firewall's
    /// own reversal.
    MemberUnplanned {
        /// The rendered role nothing planned.
        role: R,
    },
    /// One role was rendered more than once.
    MemberDuplicated {
        /// The doubled role.
        role: R,
        /// How many units stood under it.
        observed: u32,
    },
    /// A rendered unit's origin trail is not the trail the plan declared.
    /// A generated unit that walks back somewhere else is orphaned from the
    /// declaration it claims to project.
    OriginOrphan {
        /// The role whose origin disagreed.
        role: R,
    },
    /// The digest a rendered unit carries is not the digest of the bytes it
    /// actually carries, taken under the contract the plan stated.
    DigestMismatch {
        /// The role whose digest disagreed.
        role: R,
    },
    /// A unit stood under the planned role and answered to a different semantic
    /// key: the right seat, filled by the wrong thing.
    SemanticKeyMismatch {
        /// The role whose semantic key disagreed.
        role: R,
    },
    /// A unit was rendered to a destination or under a profile the plan did not
    /// name.
    MaterializationMismatch {
        /// The role whose materialization disagreed.
        role: R,
    },
    /// The plan itself declared one role twice, independent of what was
    /// rendered: a membership carrying two members under one role makes the
    /// role-to-unit match elect one of them, and a proof that elected its own
    /// subject proves nothing.
    MemberPlannedTwice {
        /// The doubled role.
        role: R,
        /// How many members the plan declared under it.
        observed: u32,
    },
    /// The membership rebuilt out of the rendered units and the membership the
    /// plan declared are not the same set under this role.
    ///
    /// The final theorem, checked as sets rather than as first-per-role pairs:
    /// a walk comparing one member per role would agree about two memberships
    /// that differ in their second.
    MembershipDisagreement {
        /// The role the two sets disagree under.
        role: R,
    },
    /// The rebuild produced no member at all.
    ReconstructionEmpty,
    /// The rebuild produced members that will not declare as a complete output
    /// set.
    ReconstructionUndeclarable {
        /// How many members the rebuild produced.
        observed: u32,
    },
    /// The joined token tree the rendering amounts to outgrows the declared
    /// token magnitude.
    /// Established during the proof, because the closure owns the join.
    JoinedTreeUnbounded,
}

/// The closure refusal family body, published from this file and declared in
/// `type_guard.rs`'s `seat` module, beside the only roads that reach its seat.
pub use guard::ProjectionClosureRefusal;

/// The proof that what was rendered is what was planned.
///
/// Holding one means the membership was rebuilt out of the rendered units and
/// the rebuild equals the plan's declared membership role for role, key for
/// key, origin for origin, and digest for digest.
/// There is no partial closure.
///
/// **Tokens are emitted only from a value of this type.**
/// The road from a declaration to emitted tokens passes through here or it
/// does not exist.
#[must_use = "a closure is the proof that what was rendered is what was planned"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionClosure<R: RenderedRole> {
    plan: PlanId,
    reconstructed: PlannedMembership<R>,
    rendered: RenderedProjection<R>,
    emitted: GeneratedTree,
    emitted_digest: ProjectionIdentity<OutputBytesSubject>,
    identity: ClosureId,
    provenance: ProjectionProvenance,
}

//! The closure home's declarations: what a renderer materialized, how a
//! rendering and its plan can disagree, and the proof that they do not.
//!
//! Declarations only. Every road that reaches a private field — a rendered
//! unit's digest and bytes, a rendering's units, the proof's own seats — lives
//! in `type_guard.rs`, this file's own child. That is what makes "tokens are
//! emitted only from a closure" structural: there is no seam anywhere else that
//! can build one.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ClosureId, ClosureIssueLimit, GeneratedUnitSubject, MembershipLimit, OutputBytesSubject,
    PlanId, ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, ProjectionProvenance,
    RenderedByteLimit, RenderedRole, RenderedUnitSubject,
};
use crate::planning::{MemberDestination, PlannedMembership};
use crate::token::GeneratedTree;
use threadpak::refusal::AdmittedPrefix;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

/// How one rendering failed to materialize a unit at all.
///
/// Distinct from a closure disagreement: nothing has been compared yet. These
/// are the two ways the act of materializing refuses, and both are magnitudes.
#[must_use = "a rendering refusal names the magnitude the renderer would have passed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderingRefusal {
    /// The rendered bytes exceed the declared magnitude. A renderer that would
    /// emit past it refuses rather than materializing part of a unit.
    BytesUnbounded,
    /// The rendering carries more units than the declared membership magnitude
    /// admits.
    UnitsUnbounded,
}

/// One unit a renderer actually materialized.
///
/// Everything a closure needs to rebuild the plan's membership is here and is
/// the RENDERER's own answer: the role it rendered under, the semantic key it
/// answers to, where it lands, the profile it was rendered under, where it came
/// from, the token tree itself, and the digest over that tree's canonical bytes.
///
/// The Rust source text is not a member. It is
/// [`GeneratedTree::inspected`] — a projection of the tree, for a person.
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
/// rendering, and a plan whose membership is non-empty can never close over one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderedProjection<R: RenderedRole> {
    units: NonEmptyBounded<RenderedUnit<R>, MembershipLimit>,
}

/// How one rendering and the plan it claims to materialize disagree.
///
/// Every issue that is ABOUT a role names it, because "the membership is wrong"
/// is not an answer anybody can act on. The three that are about the whole
/// reconstruction — an empty rebuild, a rebuild that will not declare, and a
/// joined tree past its magnitude — name none, and that is the honest shape:
/// there is no role to name, and electing one to fill the seat would be exactly
/// the neighbouring-value repair this roster exists to refuse.
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
    /// A rendered unit's origin trail is not the trail the plan declared. A
    /// generated unit that walks back somewhere else is orphaned from the
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
    /// The PLAN declared one role twice. Independent of what was rendered: a
    /// membership carrying two members under one role makes the role-to-unit
    /// match elect one of them, and a proof that elected its own subject proves
    /// nothing.
    MemberPlannedTwice {
        /// The doubled role.
        role: R,
        /// How many members the plan declared under it.
        observed: u32,
    },
    /// The membership rebuilt out of the rendered units and the membership the
    /// plan declared are not the same SET under this role.
    ///
    /// The final theorem, checked as sets rather than as first-per-role pairs: a
    /// pairwise walk that compared one member per role would agree about two
    /// memberships that differ in their second.
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
    /// token magnitude. Established DURING the proof, because the closure owns
    /// the join.
    JoinedTreeUnbounded,
}

/// The closure refusal family body.
///
/// Independent members: a rendering may drop one role and orphan another in one
/// pass, and reporting one of them would leave a caller repairing a rendering
/// one role per attempt.
#[must_use = "a refusal family body carries every way the rendering and the plan disagree"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionClosureRefusal<R: RenderedRole> {
    /// The established issues — at least one, at most the declared bound —
    /// together with whether the body carries every issue the pass established
    /// or names how many stand outside that bound. One seat rather than two,
    /// because a coverage claim seated beside its body is a claim that can be
    /// swapped for another body's. The pass itself always covers every
    /// applicable role, so the completion here never reports a halted
    /// examination.
    pub report: AdmittedPrefix<ClosureIssue<R>, ClosureIssueLimit>,
}

/// The proof that what was rendered is what was planned.
///
/// Holding one means: the membership was rebuilt out of the rendered units, and
/// the rebuild equals the plan's declared membership role for role, key for key,
/// origin for origin, and digest for digest. There is no partial closure.
///
/// **Tokens are emitted only from a value of this type.** That is the whole
/// point of the type existing: the road from a declaration to emitted tokens
/// passes through here or it does not exist.
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

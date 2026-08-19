//! The closure home's declarations: what a renderer materialized, how a
//! rendering and its plan can disagree, the proof that they do not, the
//! partitioned emission that proof produces, and the receipt every projection
//! kind's road ends at.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this
//! file's own child, which is what makes "tokens are emitted only from a
//! closure, and only through the receipt that binds one" structural rather than
//! reviewed.

use crate::explanation_protocol::ProjectionExplanationView;
use crate::origin_graph::OriginTrail;
use crate::plane::{
    ByteRoleSubject, ClosedExpansionId, ClosureId, GeneratedUnitSubject, MembershipLimit,
    OutputBytesSubject, OwnerIdentityRef, PlanId, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, ProjectionProvenance, RenderedByteLimit, RenderedRole,
    RenderedUnitSubject,
};
use crate::planning::{
    EmissionPartition, MemberDestination, PlannedMembership, ProjectionKind, ProjectionPlan,
};
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

/// The tokens one emission carries, together with the digest of exactly those
/// bytes.
///
/// The two seats arrive together because the digest is taken over the tree at
/// the moment the tree is built, inside the proof: a value carrying a tree and a
/// digest that were produced by two separate acts could carry a digest of bytes
/// nobody emitted, which is the substitution the whole home exists to remove.
///
/// # Construction
///
/// There is no public road to one. Every value of this type is built by the
/// partitioning inside [`ProjectionClosure::proved`], which is why holding one
/// means the bytes it carries were proved.
#[must_use = "carried tokens are an emission's bytes and the digest of exactly those bytes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CarriedTokens {
    tree: GeneratedTree,
    digest: ProjectionIdentity<OutputBytesSubject>,
}

/// What one emission carries.
///
/// Two postures, and they are different facts rather than one with a missing
/// half. A partition the plan declared members into carries their joined tokens;
/// a partition the plan declared no member into carries nothing, and says so.
/// An empty token tree would read exactly like the first posture with an
/// unlucky rendering, which is the substitution this sum removes: "the test
/// carrier receives no cargo from this expansion" and "the test carrier receives
/// a cargo of no tokens" are answers to different questions.
#[must_use = "an emission either carries proved tokens or states that nothing was planned into it"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartitionCargo {
    /// The plan declared no member into this emission.
    NothingPlanned,
    /// The members the plan declared into this emission, joined in role-roster
    /// order, with the digest of exactly those bytes.
    Carried(CarriedTokens),
}

/// Everything one proved rendering JOINS, split across the emissions its members
/// declared.
///
/// # Authority
///
/// **The partition roster is the quantifier, exactly as the role roster is the
/// quantifier for the membership proof.** Every joined emission on
/// [`EmissionPartition`] has a seat here, filled by walking the rendered units
/// in ROLE-ROSTER order and reading each unit's own destination — so "every
/// rendered unit reached exactly one emission" is a fact about the walk rather
/// than a claim about it, and a unit cannot be dropped by the partitioning or
/// counted by two of it.
///
/// This is what makes the destination vocabulary load-bearing. A single joined
/// tree hands every rendered unit to the consumer's normal build whatever its
/// plan said about it, which is how a mutation-evaluation surface ends up
/// compiled beside the implementation it exists to be evaluated against. Here
/// that surface cannot reach the declaration-site seat at all: its role's
/// constant destination answer reads to the test carrier, and the reading is the
/// only road into a seat.
///
/// # Bounds
///
/// The publication emission has no seat here, and the absence is the honest
/// shape rather than a dropped delivery. Joining is what creates material that
/// exists nowhere else — three builds, three byte streams, none of them any
/// unit's own tree. Artifacts are never joined: two artifacts are two addresses,
/// and one stream claiming to be both is the one thing a publication must not
/// write. So a published artifact IS its rendered unit, at the address that
/// unit's destination names, and it is read as one
/// ([`ProjectionReceipt::published`]) rather than copied into a record that
/// would answer the same question a second time.
///
/// # Nonclaims
///
/// It claims nothing about the vehicles. Whether a carrier's shell has been
/// rendered, what it is named, and whether any target invokes it are the
/// consumption side's facts; whether a publication ever wrote an artifact is the
/// publication road's. This value is the proved cargo, and the receipt that
/// binds it states the absence of those addresses rather than inventing them.
#[must_use = "a partitioned emission is what one proved rendering delivers, split by delivery"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartitionedEmission {
    declaration_site: PartitionCargo,
    test_carrier: PartitionCargo,
    bench_carrier: PartitionCargo,
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
    ///
    /// A destination disagreement is a DELIVERY disagreement: the unit would be
    /// emitted into a build the plan never sent it to. It is caught here, before
    /// anything is split, because a rendering the plan disagrees with about
    /// where a member goes is not a rendering to partition — it is one to
    /// refuse.
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
    /// One emission's joined token tree outgrows the declared token magnitude.
    ///
    /// Established during the proof, because the closure owns the join.
    /// It names the emission it overran at: a caller told only that "the tree"
    /// is too wide does not know which delivery to cut, and the three joins are
    /// three different byte streams for three different builds.
    JoinedTreeUnbounded {
        /// The emission whose joined tree overran.
        partition: EmissionPartition,
    },
    /// Two rendered units are published to ONE address, so the artifact written
    /// second stands where the first stands and one byte role answers for two
    /// units.
    ///
    /// Established during the proof, because occupancy in the publication
    /// emission is occupancy by ADDRESS. The two units stand under different
    /// roles and carry different destinations, so every check before this one
    /// passes them: the address is the only seat at which they collide, and it
    /// is compared exactly once, here.
    ArtifactAddressDoubled {
        /// The role whose artifact would stand at an address already taken.
        role: R,
        /// The address both units are written under.
        byte_role: OwnerIdentityRef<ByteRoleSubject>,
    },
}

/// The closure refusal family body, published from this file and declared in
/// `type_guard.rs`'s `seat` module, beside the only roads that reach its seat.
pub use guard::ProjectionClosureRefusal;

/// The proof that what was rendered is what was planned, and the partitioned
/// emission that proof produced.
///
/// Holding one means the membership was rebuilt out of the rendered units and
/// the rebuild equals the plan's declared membership role for role, key for
/// key, origin for origin, and digest for digest — and that the rendering was
/// then split across the emissions its members declared, each emission joined in
/// role-roster order and each joined tree's digest committed to inside this
/// closure's own identity.
/// There is no partial closure.
///
/// **No token reaches a compiler except through a value proved here.**
/// The proved emission is this closure's material and is not handed out: the
/// public road to tokens is [`ProjectionReceipt`], which binds this closure to
/// the plan it was proved against and the explanation written over it, and reads
/// the emissions off it. A road that handed the tokens back from here would be a
/// road to emission that skips the receipt, which is the same as no receipt.
#[must_use = "a closure is the proof that what was rendered is what was planned"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionClosure<R: RenderedRole> {
    plan: PlanId,
    reconstructed: PlannedMembership<R>,
    rendered: RenderedProjection<R>,
    emission: PartitionedEmission,
    identity: ClosureId,
    provenance: ProjectionProvenance,
}

threadpak::closed_register! {
    /// What a receipt states about the addresses a delivered emission will
    /// eventually be reached by, at the seam a receipt is bound.
    ///
    /// # Authority
    ///
    /// **A roster of one, and the roster IS the statement.** An expansion holds
    /// no carrier name and no publication receipt, and neither absence is a gap
    /// somebody could fill: the shell a carrier's cargo rides is named where the
    /// shell is rendered, and a publication receipt is a human-committed act
    /// outside any expansion. Writing the posture down rather than leaving it
    /// implicit is what makes a second posture a law change at this roster
    /// instead of a seat somebody adds to a receipt.
    ///
    /// A receipt therefore states the absence and STANDS. It does not refuse
    /// over a value nobody at this seam could supply, and it does not mint a
    /// stand-in — an invented carrier name reads exactly like a published one,
    /// which is the substitution every posture roster in the plane exists to
    /// remove.
    ///
    /// The posture's slot rides in every receipt's transcript, so the day a
    /// second posture is admitted the receipts derived under it stand in a
    /// different name space from the receipts derived under this one.
    pub enum DeliveryAddressing {
        /// Neither address exists at this seam: no carrier has been named and
        /// nothing has been published. The cargo is proved and partitioned, and
        /// each address is minted by the road that owns it.
        UnmintedAtThisSeam = "unminted-at-this-seam",
            "no carrier has been named and nothing has been published at this seam";
    }
}

/// How binding one receipt refuses.
///
/// One way, because there is exactly one thing three values that were produced
/// separately can disagree about and this seam can check: whether the closure
/// was proved against the plan it is being bound to.
#[must_use = "a binding refusal names the two plans a receipt was asked to bind under one identity"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReceiptBindingRefusal {
    /// The closure proves a rendering against a DIFFERENT plan than the one
    /// handed in beside it.
    /// A receipt bound over the pair would name one plan and carry the proof of
    /// another, and every reading downstream would answer correctly about the
    /// wrong expansion.
    ClosureProvedAgainstAnotherPlan {
        /// The plan handed to the binding.
        planned: PlanId,
        /// The plan the closure was actually proved against.
        proved: PlanId,
    },
}

/// One closed expansion: everything one projection produced, bound under one
/// identity, with emission reachable only from here.
///
/// This is where every projection kind's road ends. A caller that walked the
/// steps itself holds a plan, a proved closure, and a complete explanation;
/// binding them here is what turns those three into one account, and the account
/// is the only value that hands tokens out.
///
/// # The one road to emitted tokens
///
/// A receipt cannot be held without a plan, a proved closure over that plan, and
/// a complete explanation view all having been produced and having agreed. The
/// closure is unforgeable and the view is unforgeable, so the binding cannot
/// assemble a receipt out of values that skipped a step — and the emissions are
/// read off the closure's own proved partitions, so what a caller emits is what
/// was proved rather than something joined afterwards.
///
/// # Inspection and emission
///
/// [`ProjectionReceipt::plan`] and [`ProjectionReceipt::closure`] are the SAME
/// values the emissions are read from. There is no parallel plan built for
/// inspection and no synthetic sibling built for emission, so "what does it say
/// it did" and "what did it do" cannot drift.
///
/// The receipt holds no tokens of its own. The partitioned emission belongs to
/// the CLOSURE, which built it as part of proving and committed to its digests
/// inside its own identity; this value borrows it. A receipt that had been
/// handed an emission alongside a closure could have been handed one the closure
/// never joined.
///
/// # Nonclaims
///
/// The explanation view carries no plan identity, so nothing here establishes
/// that the view was written over THIS plan rather than over another plan of the
/// same kind. The type parameter ties the three to one kind and the binding ties
/// the closure to the plan; the view's own subject is what the coverage proof
/// establishes, and this seat states the boundary rather than implying a
/// comparison nobody performed.
#[must_use = "a receipt is the whole account one projection produced, and the only road to tokens"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReceipt<K: ProjectionKind> {
    identity: ClosedExpansionId,
    provenance: ProjectionProvenance,
    plan: ProjectionPlan<K>,
    closure: ProjectionClosure<K::Rendered>,
    explanation: ProjectionExplanationView<K>,
}

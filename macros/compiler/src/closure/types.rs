//! The closure home's declarations: the ways a rendering and its plan disagree, the refusal that carries them, the proof, and the emission that proof partitions.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes the home's central claim structural: a joined delivery is built inside a proof and exists nowhere else.

use crate::bounded::Capped;
use crate::identity::{self, ClosureId, Identity, OwnerIdentity, PlanId, Provenance};
use crate::kind::{Destination, Role};
use crate::plan::Membership;
use crate::render::RenderedProjection;
use crate::token::GeneratedTree;

#[path = "type_guard.rs"]
mod guard;

/// Issues one closure refusal carries before it begins counting the rest.
///
/// Twice the outputs one plan may declare: at most one issue per planned seat, plus one per rendered unit nothing planned.
pub const CLOSURE_ISSUE_LIMIT: usize = 64;

/// The tokens one delivery carries, together with the digest of exactly those bytes.
///
/// The two arrive together because the digest is taken over the tree at the moment the tree is built, inside the proof.
///
/// # Construction
///
/// There is no public road to one: every value of this type is built by the partitioning inside [`Closure::proved`], which is why holding one means the bytes it carries were proved.
#[must_use = "carried tokens are a delivery's bytes and the digest of exactly those bytes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CarriedTokens {
    tree: GeneratedTree,
    digest: Identity<identity::OutputBytes>,
}

/// What one delivery carries.
///
/// "This delivery receives no cargo from this expansion" and "this delivery receives a cargo of no tokens" are answers to different questions, and an empty token tree would read exactly like the first.
#[must_use = "a delivery either carries proved tokens or states that nothing was planned into it"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartitionCargo {
    /// The plan declared no member into this delivery.
    NothingPlanned,
    /// The members the plan declared into this delivery, joined in roster order, with the digest of exactly those bytes.
    Carried(CarriedTokens),
}

/// Everything one proved rendering JOINS, split across the deliveries its seats declared.
///
/// # Authority
///
/// **The delivery roster is the quantifier**, exactly as the role roster is the quantifier for the membership proof: every joined delivery has a seat here, filled by walking the rendered units in ROSTER order and reading the delivery each unit's seat declares.
///
/// # Bounds
///
/// The publication delivery has no seat here, because artifacts are never joined: two artifacts are two addresses, and a published artifact IS its rendered unit at the address the plan named for it.
#[must_use = "a partitioned emission is what one proved rendering delivers, split by delivery"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartitionedEmission {
    declaration_site: PartitionCargo,
    test_carrier: PartitionCargo,
    bench_carrier: PartitionCargo,
}

/// One way a rendering and the plan it claims to materialize disagree.
///
/// Every issue about a seat names it; the issues about the whole reconstruction name none, because there is no seat to name and electing one would be a stand-in nobody established.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureIssue<R: Role> {
    /// A seat the plan declared was not rendered at all.
    MemberMissing {
        /// The planned seat nothing materialized.
        role: R,
    },
    /// A seat was rendered that the plan never declared — the output firewall's own reversal.
    MemberUnplanned {
        /// The rendered seat nothing planned.
        role: R,
    },
    /// One seat was rendered more than once.
    MemberDuplicated {
        /// The doubled seat.
        role: R,
        /// How many units stood under it.
        observed: u32,
    },
    /// A rendered unit walks back to an origin the plan did not declare, which orphans it from the declaration it claims to project.
    OriginOrphan {
        /// The seat whose origin disagreed.
        role: R,
    },
    /// The digest a rendered unit carries is not the digest of the bytes it actually carries, taken under the contract the plan stated.
    DigestMismatch {
        /// The seat whose digest disagreed.
        role: R,
    },
    /// A unit stood under the planned seat and answered to a different semantic key: the right seat, filled by the wrong thing.
    SemanticKeyMismatch {
        /// The seat whose semantic key disagreed.
        role: R,
    },
    /// A unit was rendered under a profile, or written to an address, the plan did not name.
    ///
    /// The delivery cannot disagree and is not checked: a delivery is the seat's own constant answer, so both sides read one value.
    MaterializationMismatch {
        /// The seat whose materialization disagreed.
        role: R,
    },
    /// The plan itself declared one seat twice, independent of what was rendered.
    ///
    /// Two members under one seat make the seat-to-unit match elect one of them, and a proof that elected its own subject proves nothing.
    MemberPlannedTwice {
        /// The doubled seat.
        role: R,
        /// How many members the plan declared under it.
        observed: u32,
    },
    /// The rebuilt membership and the planned one are not the same set under this seat.
    ///
    /// The final theorem, checked as sets: a walk comparing one member per seat would agree about two memberships that differ in their second.
    MembershipDisagreement {
        /// The seat the two sets disagree under.
        role: R,
    },
    /// The rebuild produced no member at all.
    ReconstructionEmpty,
    /// The rebuild produced members that will not declare as a complete output set.
    ReconstructionUndeclarable {
        /// How many members the rebuild produced.
        observed: u32,
    },
    /// One delivery's joined token tree outgrows the declared token magnitude.
    ///
    /// It names the delivery it overran at: a caller told only that "the tree" is too wide does not know which build to cut.
    JoinedTreeUnbounded {
        /// The delivery whose joined tree overran.
        destination: Destination,
    },
    /// Two rendered units are published to ONE address, so the artifact written second stands where the first stands.
    ///
    /// The two stand under different seats and carry different material, so every check before this one passes them.
    ArtifactAddressDoubled {
        /// The seat whose artifact would stand at an address already taken.
        role: R,
        /// The address both units are written under.
        address: OwnerIdentity,
    },
    /// A unit was rendered into the publication delivery with no address to write it to.
    ///
    /// A publication that elected an address for it would be minting the consumer's own fact.
    ArtifactAddressAbsent {
        /// The seat whose artifact has nowhere to be written.
        role: R,
    },
}

/// How closure says no.
///
/// Closure issues are independent and co-establishable, so the body carries every issue the pass established and says so where it kept only what fits.
/// No issue is elected as the primary one, and a body with nothing in it is unrepresentable.
#[must_use = "a closure refusal carries every way the rendering and the plan disagree"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureError<R: Role> {
    body: Capped<ClosureIssue<R>, CLOSURE_ISSUE_LIMIT>,
}

/// The proof that what was rendered is what was planned, and the emission that proof partitioned.
///
/// Holding one means the membership was rebuilt out of the rendered units and equals the plan's declared membership seat for seat, key for key, origin for origin, and digest for digest — and that the rendering was then split across the deliveries its seats declared, each joined delivery's digest committed to inside this closure's own identity.
///
/// **No token reaches a compiler except through a value proved here.**
/// The proved emission is this closure's material and is not handed out: the public road to tokens is the expansion that binds this proof to the plan it was proved against and the explanation written over the two.
#[must_use = "a closure is the proof that what was rendered is what was planned"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure<R: Role> {
    plan: PlanId,
    reconstructed: Membership<R>,
    rendered: RenderedProjection<R>,
    emission: PartitionedEmission,
    identity: ClosureId,
    provenance: Provenance,
}

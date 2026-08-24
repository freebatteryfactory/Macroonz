//! The plan home's declarations: the account a request walked in with, the context it is decided under, its complete output set, what invalidates it, the plan itself, and how planning refuses.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes the output firewall structural: a plan's declared set is whatever one of those roads admitted, and there is no other way in.

use crate::bounded::{Bounded, Capped, NonEmpty};
use crate::identity::{self, Identity, OwnerFact, OwnerIdentity, PlanId, Profile, Provenance};
use crate::kind::{Kind, Role};
use crate::origin::{DecisionTrace, Nonclaim, OriginTrail};
use core::marker::PhantomData;

#[path = "type_guard.rs"]
mod guard;

/// Captured declarations one account may name beside its own commitment.
///
/// A cause list cut to fit is byte for byte the shape of a complete one, so an account past this refuses rather than narrating a partial cause.
pub const DEPENDENCY_LIMIT: usize = 64;

/// Triggers one plan may watch.
///
/// The shared derivation alone reaches sixty-seven — the content commitment, up to sixty-four declared dependencies, the profile, and the generator — and a kind adds whatever its own anchors require on top, so the roster is wider than the derived part rather than exactly it.
pub const TRIGGER_LIMIT: usize = 128;

/// Outputs one plan may declare.
pub const MEMBERSHIP_LIMIT: usize = 32;

/// Nonclaims one plan may state.
pub const NONCLAIM_LIMIT: usize = 16;

/// Issues one planning refusal carries before it begins counting the rest.
///
/// One per doubled seat — sixteen, since doubling spends two members of a membership of thirty-two — one per bound axis, and one of each remaining kind.
pub const PLAN_ISSUE_LIMIT: usize = 32;

/// What a request MEANT: its kind's declared name over the content commitment it was meant for.
///
/// Two requests that meant the same thing derive one of these, whatever machinery would realize them, which is why this is the layer equivalence is compared at.
pub type Intent = Identity<identity::ProjectionIntent>;

/// The triggers one plan watches.
pub type InvalidationSet = NonEmpty<InvalidationTrigger, TRIGGER_LIMIT>;

/// The one account of the content a request walked in with: what that content IS, and what it declares it stands on.
///
/// Every reading of a request's content reads THIS value — the intent derived from it, the triggers that watch it, the declaration that caused it, the node it stands at — and none of them keeps a copy.
/// A second list of what content depends on would agree with this one until it did not, and nothing downstream could tell which of the two a plan was planned over.
///
/// # Nonclaims
///
/// It says nothing about whether the commitment is current, reachable, or admitted anywhere: it is the address the caller handed over, read exactly.
#[must_use = "the account is what a plan is planned over, and every reading reads it"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account<K: Kind> {
    commitment: Identity<identity::CapturedDeclaration>,
    dependencies: Bounded<Identity<identity::CapturedDeclaration>, DEPENDENCY_LIMIT>,
    kind: PhantomData<K>,
}

/// The exact facts every plan is decided under, whatever its kind.
///
/// What a plan was planned OVER is not here: that is the account's, and a context naming it too would be the second holder of one fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Context {
    profile: Profile,
    generator: Identity<identity::GeneratorVersion>,
}

/// One thing whose change makes a plan stale, and exactly which thing it watches.
///
/// A relevant change invalidates loudly and names what moved; a change no row watches — formatting, declaration order, an alias — touches nothing, because nothing watches those.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvalidationTrigger {
    /// A captured declaration the plan stands on.
    CapturedDeclaration {
        /// The watched capture.
        watched: Identity<identity::CapturedDeclaration>,
    },
    /// The profile the plan was decided under, at the version it was decided at.
    Profile {
        /// The watched profile.
        watched: Profile,
    },
    /// The generator that produced the plan.
    Generator {
        /// The watched generator version.
        watched: Identity<identity::GeneratorVersion>,
    },
    /// Anything else a consumer declared this plan watches.
    ///
    /// One row rather than a row per consumer noun: a mechanism profile, a work formula, and a fixture population are three consumers' facts, and a compiler that enumerated them would be minting vocabulary for meanings it does not own.
    Declared {
        /// The consumer's declared name for what moved.
        name: &'static str,
        /// The identity that moving is watched by.
        watched: OwnerIdentity,
    },
}

/// What the eventual digest of one member must satisfy, stated before a byte of it exists.
///
/// A plan holds no rendered bytes and therefore no digest of them: it names the member the digest must be anchored to, and closure recomputes the digest over the rendered bytes at [`Role::OutputBytes`](crate::identity::Role::OutputBytes) and compares.
/// A digest anchored anywhere else belongs to a different member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigestContract {
    /// The member identity the digest must be anchored to.
    pub anchored_to: Identity<identity::GeneratedUnit>,
}

/// One declared output of a plan — logical, and only logical.
///
/// What it IS, where it came from, who is expected to materialize it, the address a publication writes it to, and what its eventual digest must satisfy.
/// No rendered bytes and no digest of them: those are the rendering's facts and they live on the rendered unit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedOutput {
    /// What this member is, independently of any bytes.
    pub semantic_key: Identity<identity::GeneratedUnit>,
    /// Where it came from — a walk back to authored material, non-empty by its own shape.
    pub origin: OriginTrail,
    /// The profile expected to render it.
    pub expected_profile: Profile,
    /// The address a publication writes it to, where the member's seat is one that writes to an address.
    pub address: Option<OwnerIdentity>,
    /// What the eventual digest must satisfy.
    pub digest_contract: DigestContract,
}

/// One planned member: the seat it stands under, and the output planned there.
///
/// The seat is what closure matches on, so a rendering that produced the right NUMBER of units in the wrong seats is caught by the seat rather than passing a count.
/// It is also where the member's delivery is read from ([`Role::destination`]); a plan declares no delivery of its own, so two plans of one kind cannot disagree about which build compiles a seat's unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedMember<R: Role> {
    /// The seat this member stands under.
    pub role: R,
    /// The output planned there.
    pub output: PlannedOutput,
}

/// The complete declared output set of one plan — the output firewall.
///
/// The declared set is the whole set: a sibling that is not in it was not planned, and nothing downstream may materialize one.
/// Structurally non-empty, because a plan that would generate nothing is a disposition rather than a plan.
///
/// Every member's seat is in the kind's declared roster, because admission refuses one that is not: every walk downstream — encoding, proof, reconstruction, delivery — quantifies over that roster, and a member outside it would be a unit those walks never look at, held by a proof that claims the whole set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership<R: Role> {
    members: NonEmpty<PlannedMember<R>, MEMBERSHIP_LIMIT>,
}

/// Everything one plan decided, as the one value those seats travel in.
///
/// Five seats, in the order a plan's transcript writes them, and every one of them required: a construction that leaves one out stops compiling exactly where a missing argument would, and a seat added to a plan is added here and breaks every construction again.
#[must_use = "the decided seats are what one plan is planned from, whole"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanDecisions<R: Role> {
    /// The complete declared output set.
    pub membership: Membership<R>,
    /// The triggers whose change invalidates the plan.
    pub invalidation: InvalidationSet,
    /// The decisions that produced the plan, in selection order.
    pub trace: DecisionTrace,
    /// Where the plan itself came from, in walk order.
    pub origin: OriginTrail,
    /// What the plan explicitly does not claim.
    pub nonclaims: Bounded<Nonclaim, NONCLAIM_LIMIT>,
}

/// One plan: the complete output set of one request, named before any syntax exists.
///
/// Every seat is required, and the seats that could have been empty are shapes that cannot be — the output set, the watch set, the trace, and the trail are all structurally non-empty.
/// Only the nonclaims may be empty, because a plan that claims exactly what it does has none to state.
///
/// The account is not a copy of anything: it is the value the caller walked in with, moved into the plan, so the plan's own answer to "what were you planned over" is what its identity, its watch set, and its origin edges were all read off.
#[must_use = "a plan is the complete declared output set nothing may be rendered without"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan<K: Kind> {
    identity: PlanId,
    provenance: Provenance,
    account: Account<K>,
    context: Context,
    content: K::Content,
    membership: Membership<K::Role>,
    invalidation: InvalidationSet,
    trace: DecisionTrace,
    origin: OriginTrail,
    nonclaims: Bounded<Nonclaim, NONCLAIM_LIMIT>,
}

/// Which declared magnitude a plan overran.
///
/// A bound refusal names its axis, so "too big" is never an unlocated word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundAxis {
    /// The captured declarations one account may name.
    Declarations,
    /// The outputs one plan may declare.
    Outputs,
    /// The triggers one plan may watch.
    Triggers,
    /// The entries one decision trace may record.
    TraceEntries,
    /// The edges one origin trail may draw.
    OriginEdges,
}

/// The two facts a contradiction stands between.
///
/// Neither side is elected as the offender: the disagreement is the fact, and naming one of them wrong is a judgment this compiler has no standing to make.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContradictionPair {
    /// The first constraining fact.
    pub left: OwnerFact,
    /// The second constraining fact.
    pub right: OwnerFact,
}

/// One way planning refuses.
///
/// No issue is payload-free: an issue names what it observed, because a bare row makes the reader guess.
/// Several are reachable only where a plan arrives decoded rather than built through the roads here, since the typed roads cannot express an unimplemented kind, an orphaned unit, or an incomplete membership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanIssue {
    /// Two facts that decided this plan disagree.
    ContradictoryFacts {
        /// The disagreeing pair.
        between: ContradictionPair,
    },
    /// The plan names a kind this compiler was not handed an implementation of.
    UnknownKind {
        /// The named kind's identity.
        named: Identity<identity::ProjectionKind>,
    },
    /// The profile the request selected offers no such projection.
    ProfileUnsupported {
        /// The profile that offers it not.
        profile: Profile,
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
    /// A declared sibling output is absent from the membership.
    MembershipIncomplete {
        /// The absent unit.
        absent: Identity<identity::GeneratedUnit>,
    },
    /// A generated unit arrived with no origin.
    OrphanGeneratedNode {
        /// The orphaned unit.
        node: Identity<identity::GeneratedUnit>,
    },
    /// Two planned members stand under one seat.
    ///
    /// Closure matches a rendered unit to a planned member BY SEAT, so a seat carrying two members leaves that match electing one of them and proving nothing about the other.
    MembershipDoubled {
        /// The doubled seat's position in its kind's roster.
        role_slot: u16,
        /// How many members stood under it.
        observed: u32,
    },
    /// An origin trail's edges do not join: the edge at this position starts at a node the edge before it did not produce.
    ///
    /// A walk with a gap in it is not a shorter walk — it is two walks presented as one, and whichever end a reader trusts, the other end is provenance nobody established.
    TrailDiscontinuous {
        /// The position of the edge that does not join its predecessor, counted from the trail's first edge.
        at: u32,
    },
    /// A narrow one-trigger reading was asked of an account that names more than one declaration.
    ///
    /// A watch covering the first declaration and no other reads exactly like a complete one, so the reading refuses rather than issuing a claim about the declarations it dropped.
    CauseSetUnwatchable {
        /// How many declarations the account names.
        named: u32,
        /// How many of them the reading can watch.
        watchable: u32,
    },
    /// A planned member stands under a seat the kind's roster does not declare.
    ///
    /// The roster is the denominator of every downstream walk — encoding, proof, reconstruction, delivery.
    /// A member outside it would render, vanish from all of them, and leave the closure proving a set it never examined whole, so the member refuses at admission instead.
    MembershipForeign {
        /// The undeclared seat's own declared name.
        seat: &'static str,
    },
    /// An address was stated for a seat no publication act consumes.
    ///
    /// An address is a claim about where an artifact will be written, and only a seat delivering to a publication artifact ever writes to one.
    /// Stated anywhere else — a declaration site, a test carrier, or a seat outside the roster entirely — the address would still enter every identity while no act ever consumed it: a writable claim with no product act, which is exactly the shape this plan refuses to hold.
    AddressInert {
        /// The seat the address was stated for, by its own declared name.
        seat: &'static str,
    },
}

/// How planning says no.
///
/// Planning issues are independent and co-establishable — one pass may find a doubled seat and an overrun magnitude at once — so the body carries every issue the pass established, and says so where it kept only what fits.
/// No issue is elected as the primary one, and a body with nothing in it is unrepresentable.
#[must_use = "a planning refusal carries every issue the pass established"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanError {
    body: Capped<PlanIssue, PLAN_ISSUE_LIMIT>,
}

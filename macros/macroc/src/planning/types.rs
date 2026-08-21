//! The plan family's declarations: the entry account and the intent it names,
//! the shared context, the output firewall, the destination vocabulary and the
//! emission roster every destination reads to, the invalidation roster, the
//! sealed kind roster with its contents, its enumerated rows, and the record
//! that carries exactly one disposition per row, the plan itself, the bundle,
//! the disposition, the services' own expectation of the generated-support
//! schema identity, and the magnitudes this home's own capacities are governed
//! by.
//!
//! Declarations only.
//! Every road that reaches a private field — the account's addressing, the
//! membership's members, a plan's seats, a bundle's member set, the schema
//! expectation's bytes — lives in `type_guard.rs`, this file's own child.

use crate::origin_graph::{DecisionTrace, Nonclaim, OriginTrail};
use crate::plane::{
    AssumptionLimit, BundleSubject, ByteRoleSubject, CapturedDeclarationSubject,
    DerivedTypeSubject, DocumentedSubject, FacetLimit, FixturePopulationSubject,
    GeneratedUnitSubject, GeneratorVersionSubject, ImplementedContractSubject, MeasuredSubject,
    MechanismProfileSubject, MembershipLimit, NonclaimLimit, ObligationSubject, OwnerFactRef,
    OwnerIdentityRef, PatternArgumentLimit, PatternArgumentSubject, PatternInstanceSubject,
    PatternSubject, PlanId, PortSubject, ProfileVersion, ProjectionIdentity,
    ProjectionIntentSubject, ProjectionProfileSubject, ProjectionProvenance, ProjectionRole,
    RenderedRole, SchemaSubject, SoleRenderedUnit, WireContractSubject, WorkCurrencySubject,
    WorkFormulaSubject, WrapperComponentLimit,
};
use crate::question::ExplanationQuestion;
use crate::refusal::ProjectionPlanning;
use core::fmt::Debug;
use core::marker::PhantomData;
use threadpak::declaration::Facet;
use threadpak::declaration::types::{
    FragmentIdentityDomain, LinkedGraphDomain, ProjectionAudienceDomain,
    ProjectionConfigurationDomain, ProjectionTargetDomain,
};
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many source declarations one plan may name.
    ///
    /// # Bounds
    ///
    /// Sixty-four. A plan whose declared cause set outgrows this refuses rather
    /// than narrating a partial cause: a cause list cut to fit is byte-for-byte
    /// the shape of a complete one, so a reader would take the first
    /// sixty-four declarations for the whole account of what the plan stands on.
    SourceDeclarationLimit = 64,
    /// The magnitude governing how many invalidation triggers one plan may
    /// watch.
    ///
    /// # Bounds
    ///
    /// Nine — the trigger roster's own cardinality, because one trigger per kind
    /// is all that can be watched. It is not a number this home chose out of
    /// taste: a tenth watched trigger would have to be a tenth KIND of trigger,
    /// and the roster beside it declares nine.
    InvalidationLimit = 9,
    /// The magnitude governing how many member plans one bundle may hold.
    ///
    /// # Bounds
    ///
    /// Thirty-two. A bundle is what ONE publication boundary stages, checks, and
    /// materializes as a unit, and a boundary past thirty-two member plans has
    /// stopped being one publication — the repair is a second bundle, not a
    /// wider roster here.
    ///
    /// # Nonclaims
    ///
    /// It is its own family and not [`MembershipLimit`], even though the two
    /// numbers agree today. That one bounds the outputs ONE PLAN declares; this
    /// one bounds the plans one publication carries, and one family standing for
    /// both would be one authority answering two questions. The bound refusal
    /// both roads raise names [`BoundAxis::Outputs`](crate::refusal::BoundAxis),
    /// because in both cases what overran is what the road was going to
    /// materialize — a shared axis in the refusal grammar is not a shared
    /// magnitude.
    BundleMemberLimit = 32,
}

/// What a plan binds itself to at its target end.
///
/// Not an option: a target-free projection is a stated posture, not a missing
/// host contract, and the two must never read the same.
/// A plan whose kind needs a host and whose binding is target-free refuses
/// rather than defaulting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetBinding {
    /// Bound to one named host contract.
    HostContract(OwnerIdentityRef<ProjectionTargetDomain>),
    /// Deliberately bound to no host contract.
    TargetFree,
}

/// The declaration fragments one piece of owner content declares it stands on.
///
/// Possibly empty, and empty is a STATED fact rather than a missing one:
/// content that stands on nothing declares nothing, and the account carrying
/// this seat is required either way.
/// Bounded by the source-declaration magnitude — the magnitude a plan's causes
/// were bounded by when the causes were listed beside the plan.
/// The set moved into [`OwnerContentAccount`], and the bound moved with it,
/// because a second magnitude for one capacity is a second authority.
pub type SourceDeclarations =
    Bounded<OwnerIdentityRef<FragmentIdentityDomain>, SourceDeclarationLimit>;

/// The captured declarations one piece of captured owner content declares it
/// stands on.
///
/// The expansion-time twin of [`SourceDeclarations`], under the same magnitude:
/// where nothing has been linked, what content stands on is other captured token
/// material, named by the capture identity the plane derived for it.
pub type CapturedDependencies =
    Bounded<ProjectionIdentity<CapturedDeclarationSubject>, SourceDeclarationLimit>;

/// The triggers one plan watches.
pub type InvalidationSet = NonEmptyBounded<InvalidationTrigger, InvalidationLimit>;

/// What a plan was decided AGAINST at its graph end.
///
/// Not an option.
/// A plan decided against the machine's closed declaration graph says so and
/// names it; a plan decided at expansion time, where nothing has been linked and
/// there is no closed graph to name, says THAT — and names the captured
/// declaration it was decided against instead.
/// The two postures never read alike, and neither is a missing graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphAnchoring {
    /// Decided against the machine's closed declaration graph.
    ClosedGraph(OwnerIdentityRef<LinkedGraphDomain>),
    /// Decided against one captured declaration alone, with no closed graph in
    /// existence yet. The expansion-time posture, stated rather than implied.
    CapturedDeclarationOnly(ProjectionIdentity<CapturedDeclarationSubject>),
}

/// The ONE address a piece of owner content walked in the door carrying.
///
/// The same split, at the other end: the machine's declaration fragment where a
/// caller holds the linker's mint, and otherwise the exact token material one
/// expansion was handed.
/// A capture is a real address and is named as one; it is never dressed up as a
/// fragment the linker never minted.
///
/// The commitment is the OWNER's, at full width, and the services derive none of
/// it: a content address is machine meaning, and the plane reads machine meaning
/// rather than inventing an encoding for it.
///
/// # Bounds
///
/// It names ONE address and never a set.
/// What content stands ON is the dependency seat of [`OwnerContentAccount`],
/// which is the services' one account of content dependencies; a set here would
/// be that account's duplicate, and the anchor question would then be answered
/// by electing a member out of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CauseAnchoring {
    /// The machine's declaration-fragment identity for the content.
    Declaration(OwnerIdentityRef<FragmentIdentityDomain>),
    /// The captured declaration this plan was derived from.
    CapturedDeclaration(ProjectionIdentity<CapturedDeclarationSubject>),
}

/// What one piece of owner content is addressed by, together with what it
/// declares it stands on.
///
/// The POSTURE is the outer sum and the two seats ride inside it, so a linked
/// commitment can never be handed a captured dependency set and a captured one
/// can never be handed linked fragments.
/// At expansion time nothing has been linked, so there are no fragments for
/// captured content to stand on; where a caller holds the linker's mints, token
/// material is not what a fragment stands on.
/// A mixed pair is meaningless in both directions, so it is unrepresentable
/// rather than refused.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentAddressing {
    /// The linked posture: the machine's fragment identities, at both seats.
    Linked {
        /// The content's own commitment, as the owner supplied it.
        commitment: OwnerIdentityRef<FragmentIdentityDomain>,
        /// The commitments it declares it stands on.
        dependencies: SourceDeclarations,
    },
    /// The expansion-time posture: captured token material, at both seats.
    Captured {
        /// The captured declaration this content IS.
        commitment: ProjectionIdentity<CapturedDeclarationSubject>,
        /// The captures it declares it stands on.
        dependencies: CapturedDependencies,
    },
}

/// The typed entry account: the ONE account of owner content the services hold.
///
/// Three seats, and they arrive together because they are one fact about one
/// piece of content: the COMMITMENT the owner supplied at the door, the
/// DEPENDENCY SET that commitment declares it stands on — both carried by
/// [`ContentAddressing`] under one posture — and the KIND the content is content
/// for, carried as the type parameter so an account cannot be handed to a plan
/// of another kind.
///
/// # Authority
///
/// **One account, four readings, and no second account anywhere.**
/// Every one of the following reads THIS value; none of them keeps a copy of
/// what it read, and nothing beside it holds a second answer to the same
/// question:
///
/// 1. **semantic identity** — [`OwnerContentAccount::intent`] names what was
///    meant, and the account's canonical bytes are the first member of a plan's
///    transcript, so a plan over different content is a different plan;
/// 2. **invalidation dependencies** — the watch derivation
///    ([`ProjectionContext::watch_set`]) consumes the commitment and counts the
///    dependency set, and refuses where the trigger roster cannot represent it;
/// 3. **explanation facts** — [`OwnerContentAccount::commitment`] is what
///    answers "which declaration caused you";
/// 4. **origin edges** — [`OwnerContentAccount::origin_node`] and
///    [`OwnerContentAccount::dependency_edges`] are the account's contribution
///    to the origin graph.
///
/// A second list of what content depends on — beside a plan, inside a context,
/// or at a call site — would be a value that agrees with this one until it does
/// not, and nothing downstream could tell which of the two the plan was
/// actually planned over.
///
/// # Nonclaims
///
/// It claims nothing about whether the commitment is CURRENT, available, or
/// admitted: it is the address the owner handed over, read exactly, and the
/// services neither derived it nor checked it.
#[must_use = "the entry account is the one account of owner content, and every reading reads it"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnerContentAccount<K: ProjectionKind> {
    addressing: ContentAddressing,
    kind: PhantomData<K>,
}

/// The intent layer's identity: WHAT was meant — the kind's declared name and
/// the owner content commitment it was meant over, derived into thirty-two bytes
/// under the plane's own profile.
///
/// The first of the three identity layers.
/// The plan identity is derived over the entry account's bytes and everything
/// the plan decided beside them; the rendered-unit identity is derived over
/// bytes that do not exist yet when this one does.
///
/// # Authority
///
/// **Equality of the identity is equality of intent.** Two doors that meant the
/// same thing derive one of these, which is what door equivalence compares —
/// plan identities cannot be compared for that, since distinct doors are
/// required to carry distinct origins.
///
/// It is a derived identity in the full sense the plane means: thirty-two bytes
/// over a complete transcript, under the identity subject `projection-intent`
/// at role [`ProjectionRole::ProjectionIntent`], both of them roster seats the
/// plane declares. The preimage is [`OwnerContentAccount::intent_bytes`] — the
/// pair, written once, by the one road an account's own canonical bytes open
/// with — and the derivation is stated in full where it happens, on
/// [`OwnerContentAccount::intent`].
///
/// **The intent has its own version ladder and rides nobody else's.** The role
/// reads to [`PreimageFamily::ProjectionIntent`], whose profile
/// ([`PROJECTION_INTENT_IDENTITY_PROFILE`]) moves when the kind-and-commitment
/// grammar moves and at no other time. That is what makes the equality above
/// answerable across an upgrade: an intent that was renamed every time a
/// rendering shape, a delivery, or a token roster widened would compare unequal
/// between two doors that meant exactly the same thing, and the comparison would
/// report a difference in the machinery as a difference in meaning.
///
/// # Nonclaims
///
/// It commits to the PAIR and to nothing else: not to what the content declares
/// it stands on, not to the context a plan was decided under, not to any
/// decision a plan recorded, and not to the generator that would realize it.
/// An account's own bytes widen the pair by the dependency set and a plan's
/// transcript widens it again, so neither is reachable from these thirty-two
/// bytes — which is exactly why this is the layer two distinct doors are allowed
/// to agree at.
///
/// It is never a machine commitment, on the same terms every plane identity
/// states: where the machine needs one the machine mints it.
///
/// [`PreimageFamily::ProjectionIntent`]: crate::plane::PreimageFamily::ProjectionIntent
/// [`PROJECTION_INTENT_IDENTITY_PROFILE`]: crate::plane::PROJECTION_INTENT_IDENTITY_PROFILE
#[must_use = "an intent identity is what door equivalence compares"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionIntentId {
    identity: ProjectionIdentity<ProjectionIntentSubject>,
}

/// The exact identities every plan shares, whatever its kind: what it was
/// decided against, which profile at which version, which version of the
/// services produced it, and what it is bound to.
///
/// # Bounds
///
/// The content a plan was planned OVER is not here.
/// That is the entry account's fact ([`OwnerContentAccount`]) and it is stated
/// once: a context that also named the content would be the second account of
/// content dependencies, and the watch derivation would then be reading a copy
/// rather than the account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionContext {
    /// What this plan was decided against.
    pub graph: GraphAnchoring,
    /// The projection profile selected.
    pub profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// The version of the services that produced this plan.
    pub generator: ProjectionIdentity<GeneratorVersionSubject>,
    /// What the plan binds to at its target end.
    pub target: TargetBinding,
}

/// Which emission one planned member's tokens belong to once it is rendered.
///
/// # Authority
///
/// **The destination decides which emission a member's bytes reach, and it is
/// the only seat that decides it.** An expansion does not hand a compiler one
/// stream: the tokens the consumer's NORMAL build compiles, the cargo a test
/// target invokes later, the cargo a bench target invokes, and the bytes a
/// publication writes to a named address are four deliveries with four
/// audiences, and a member states which one it is for here.
/// A vocabulary that could not tell them apart would put every rendered unit
/// into the normal build whatever the delivery matrix said about it — which is
/// exactly how a mutation-evaluation surface ends up compiled beside the
/// implementation it is supposed to be evaluated against.
///
/// Every arm reads to exactly one [`EmissionPartition`]
/// ([`MemberDestination::partition`]), and that reading is total: there is no
/// member whose tokens belong nowhere and none whose tokens belong to two
/// emissions.
///
/// # Bounds
///
/// A CARRIER destination says where the member's tokens are compiled, not where
/// the carrier itself is written. The generated support shell is DEFINED at the
/// declaration site — that is what makes it reachable — and the member's tokens
/// ride inside it as deferred cargo the consumption target expands. Deferred
/// cargo and a spliced item are two emissions, so they are two destinations;
/// where the vehicle's own definition sits is the shell's fact and not this
/// member's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberDestination {
    /// Spliced into the declaration the plan was derived from — the expansion
    /// destination, where the rendered unit replaces or accompanies the
    /// caller's own item and the consumer's normal build compiles it.
    AtDeclarationSite,
    /// Written as a standalone artifact under the named byte role.
    AsArtifact {
        /// The byte role the artifact is written under.
        byte_role: OwnerIdentityRef<ByteRoleSubject>,
    },
    /// Carried as deferred cargo into the consumer's TEST target, which invokes
    /// the generated support shell and receives it.
    /// The normal build compiles none of it.
    IntoTestCarrier,
    /// Carried as deferred cargo into the consumer's BENCH target, on the same
    /// terms and through the same shell.
    /// Separate from the test carrier because the two targets are separate
    /// builds: a bench row delivered into a test target is a row nothing runs,
    /// and the normal-build tax the wall exists to refuse is exactly what a
    /// single carrier would reintroduce.
    IntoBenchCarrier,
}

threadpak::closed_register! {
    /// The closed roster of emissions one expansion partitions its rendered
    /// units across.
    ///
    /// One row per delivery an expansion can make, and a member reaches exactly
    /// one of them, through its own declared destination
    /// ([`MemberDestination::partition`]).
    ///
    /// # Authority
    ///
    /// **The roster is the quantifier for emission, exactly as the rendered-role
    /// roster is the quantifier for the membership proof.** A join that walked
    /// the rendered units without walking this roster would produce one stream
    /// out of members that belong to different builds, and every claim made
    /// about that stream would be a claim about a value nobody delivers.
    /// Adding a row is a law change here — and one the compiler collects, since
    /// every reading over the roster is a `match` that stops compiling until the
    /// new row says what it carries.
    ///
    /// # Nonclaims
    ///
    /// A row says which emission a member's tokens belong to. It says nothing
    /// about whether that emission was requested, whether the carrier exists,
    /// or whether anybody will invoke it: those are the consumption target's
    /// facts, and an expansion that claimed them would be claiming something
    /// about a build it has never seen.
    pub enum EmissionPartition {
        /// The tokens the declaration site expands into, which the consumer's
        /// normal build compiles.
        DeclarationSite = "declaration-site",
            "the tokens the consumer's normal build compiles at the declaration site";
        /// The deferred cargo the consumer's test target invokes.
        TestCarrier = "test-carrier",
            "the deferred cargo the consumer's test target invokes";
        /// The deferred cargo the consumer's bench target invokes.
        BenchCarrier = "bench-carrier",
            "the deferred cargo the consumer's bench target invokes";
        /// The standalone artifacts a publication writes, each under its own
        /// byte role.
        PublicationArtifact = "publication-artifact",
            "the standalone artifacts a publication writes, each under its own byte role";
    }
}

/// What the eventual rendered-byte digest of one member must satisfy — stated
/// before a single byte of it exists.
///
/// A plan is made BEFORE anything is rendered, and a digest of rendered bytes is
/// a fact about bytes that do not exist yet, so a plan carrying one would carry a
/// value nobody computed: a placeholder, or a digest smuggled in from a rendering
/// that already happened, which makes the closure check compare a value against
/// itself.
/// The plan states the CONTRACT instead: the role the digest will carry, and the
/// member identity it must be anchored to.
/// The closure check recomputes the digest from the rendered bytes under exactly
/// this contract and compares, and a digest anchored anywhere else belongs to a
/// different member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigestContract {
    /// The identity role the eventual digest will carry.
    pub role: ProjectionRole,
    /// The member identity the digest must be anchored to.
    pub anchored_to: ProjectionIdentity<GeneratedUnitSubject>,
}

/// One declared output of a plan — LOGICAL, and only logical.
///
/// What it IS (the semantic key), where it LANDS (the destination), where it
/// CAME FROM (the origin trail), who is expected to MATERIALIZE it (the
/// renderer's profile at its version), and what its eventual digest must satisfy
/// (the contract).
/// No rendered bytes and no rendered-byte digest: those are the rendering's facts
/// and they live on the rendered unit.
///
/// The origin seat is what makes a generated unit non-orphanable: there is no
/// output value in the plane that does not carry a trail back to authored
/// material.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedOutput {
    /// What this member is, independently of any bytes.
    pub semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    /// Where it lands.
    pub destination: MemberDestination,
    /// Where it came from. Structurally non-empty.
    pub origin: OriginTrail,
    /// The profile expected to render it.
    pub expected_profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub expected_profile_version: ProfileVersion,
    /// What the eventual digest must satisfy.
    pub digest_contract: DigestContract,
}

/// One planned member: the rendered role it stands for, and the logical output
/// under that role.
///
/// The role is what closure matches on.
/// A rendering that produced the right NUMBER of units in the wrong roles is
/// caught by the role rather than passing a count.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedMember<R: RenderedRole> {
    /// The rendered role this member plans.
    pub role: R,
    /// The logical output under that role.
    pub output: PlannedOutput,
}

/// The complete declared output set of one plan — the output firewall.
///
/// Structurally non-empty: a plan that would generate nothing is not a plan, it
/// is a disposition.
/// Bounded: a plan that would generate past the declared magnitude refuses rather
/// than materializing part of a set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedMembership<R: RenderedRole> {
    members: NonEmptyBounded<PlannedMember<R>, MembershipLimit>,
}

/// What makes a plan stale, and exactly which identity it watches.
///
/// A relevant change invalidates loudly and says which watched identity moved.
/// An irrelevant change — formatting, declaration order, an alias — matches no
/// trigger and touches nothing, because no trigger watches those.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    clippy::enum_variant_names,
    reason = "the shared word is the axis: every row names a thing that MOVED, and a roster of watched things without it would read as the things rather than as what happened to them"
)]
pub enum InvalidationTrigger {
    /// A source declaration this plan was derived from changed.
    SourceDeclarationChanged {
        /// The watched declaration.
        watched: OwnerIdentityRef<FragmentIdentityDomain>,
    },
    /// The captured declaration this plan was derived from changed.
    /// The expansion-time twin of the fragment trigger: where the cause IS the
    /// captured token material, that is what is watched.
    CapturedDeclarationChanged {
        /// The watched capture.
        watched: ProjectionIdentity<CapturedDeclarationSubject>,
    },
    /// The closed graph this plan was decided against changed.
    GraphIdentityChanged {
        /// The watched graph.
        watched: OwnerIdentityRef<LinkedGraphDomain>,
    },
    /// The projection profile changed.
    ProjectionProfileChanged {
        /// The watched profile.
        watched: ProjectionIdentity<ProjectionProfileSubject>,
    },
    /// The host contract this plan is bound to changed.
    TargetContractChanged {
        /// The watched contract.
        watched: OwnerIdentityRef<ProjectionTargetDomain>,
    },
    /// The version of the services that produced this plan changed.
    GeneratorVersionChanged {
        /// The watched generator version.
        watched: ProjectionIdentity<GeneratorVersionSubject>,
    },
    /// An admitted mechanism profile changed.
    MechanismProfileChanged {
        /// The watched mechanism profile.
        watched: OwnerIdentityRef<MechanismProfileSubject>,
    },
    /// A declared work formula changed.
    WorkFormulaChanged {
        /// The watched work formula.
        watched: OwnerIdentityRef<WorkFormulaSubject>,
    },
    /// A fixture population a descriptor ranges over changed.
    FixturePopulationChanged {
        /// The watched population.
        watched: OwnerIdentityRef<FixturePopulationSubject>,
    },
}

/// The seal on the projection-kind roster.
///
/// A value of this type is producible only inside the services, so a kind
/// declared anywhere else cannot satisfy [`ProjectionKind`].
/// The roster is closed because the explanation protocol is mandatory: a kind
/// nobody can explain is a kind that must not be planned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KindSeal(());

/// What a kind requires of its context's target binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetRequirement {
    /// The kind's plans are meaningless without a named host contract.
    BoundHostContract,
    /// The kind's plans stand under either binding.
    EitherBinding,
}

/// One projection kind: what it plans, what it must be able to explain, and
/// what it needs of its binding.
///
/// Sealed.
/// Implementing it is a law change, not an extension point — a frontend plugs in
/// through the machine's declaration path, never by inventing a projection kind
/// the plane cannot explain.
pub trait ProjectionKind {
    /// The seal. Only the services can produce a value of this type.
    const SEAL: KindSeal;

    /// The kind's declared stable name — its segment of a plan's transcript.
    ///
    /// Declared rather than taken from the Rust spelling, so a type rename does
    /// not rename every plan identity of that kind.
    const KIND_NAME: &'static str;

    /// The kind-specific facts a plan of this kind carries.
    type Content: Debug + Clone + PartialEq + Eq;

    /// The closed roster of rendered units plans of this kind materialize.
    ///
    /// Declared by the kind rather than discovered from a rendering, which is
    /// what lets the closure check ask "was every planned role rendered, and was
    /// anything rendered that no role planned?" before a token is emitted.
    type Rendered: RenderedRole;

    /// The questions this kind answers *beyond* [`UNIVERSAL_QUESTIONS`].
    /// The universal ones are not restated here — one roster, one home, and a
    /// kind that could drop a universal question by forgetting to list it does
    /// not exist.
    const KIND_QUESTIONS: &'static [ExplanationQuestion];

    /// What this kind requires of the context's target binding.
    const TARGET_REQUIREMENT: TargetRequirement;
}

/// The questions every kind answers, whatever it plans.
///
/// No kind ducks the protocol: this roster is added to every kind's own, so a
/// kind cannot narrow what it must be able to explain.
pub const UNIVERSAL_QUESTIONS: [ExplanationQuestion; 8] = [
    ExplanationQuestion::WhatAreYou,
    ExplanationQuestion::WhichOwnerRequired,
    ExplanationQuestion::WhichDeclarationCaused,
    ExplanationQuestion::WhichGraphAndProfile,
    ExplanationQuestion::WhichOutputIdentityAndDigest,
    ExplanationQuestion::WhatInvalidates,
    ExplanationQuestion::WhyWasRelatedProjectionNotGenerated,
    ExplanationQuestion::WhatRepairsARefusal,
];

/// Which direction a codec projection covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodecDirection {
    /// Typed value to canonical bytes.
    Encode,
    /// Canonical bytes to typed value.
    Decode,
    /// Both directions, planned together so neither can drift from the other.
    RoundTrip,
}

/// What a codec projection plans: the codec that reads and writes one schema's
/// canonical bytes.
///
/// It names the schema projected from, the byte role those bytes are written
/// under, the direction covered, and the owner facts the codec rests on.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecContent {
    /// The schema the codec is projected from.
    pub schema: OwnerIdentityRef<SchemaSubject>,
    /// The byte role the codec reads or writes.
    pub byte_role: OwnerIdentityRef<ByteRoleSubject>,
    /// The direction covered.
    pub direction: CodecDirection,
    /// The owner facts this projection assumes.
    pub assumptions: Bounded<OwnerFactRef, AssumptionLimit>,
}

/// One component a host wrapper may compose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperComponent {
    /// Admission of the incoming request.
    Admission,
    /// Decoding the request's bytes.
    Decode,
    /// Encoding the response's bytes.
    Encode,
    /// Cancellation carriage.
    Cancellation,
    /// Receipt emission.
    Receipt,
    /// Effect-intent dispatch.
    EffectDispatch,
    /// Observation delivery.
    Observation,
    /// Explanation delivery.
    Explanation,
}

/// The declared wrapper-component roster, in the order the plane states it.
///
/// The roster is what an exhaustive disposition is checked against: a view that
/// must decide every component reads this, so a component added here and nowhere
/// else stops compiling at the closure law rather than passing silently
/// undecided.
pub const WRAPPER_COMPONENTS: [WrapperComponent; 8] = [
    WrapperComponent::Admission,
    WrapperComponent::Decode,
    WrapperComponent::Encode,
    WrapperComponent::Cancellation,
    WrapperComponent::Receipt,
    WrapperComponent::EffectDispatch,
    WrapperComponent::Observation,
    WrapperComponent::Explanation,
];

/// What a host wrapper projection plans: the wrapper one host contract needs.
///
/// It names the contract the wrapper binds to, the components composed into it,
/// and the declared capability that selected them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostWrapperContent {
    /// The host contract the wrapper binds to.
    pub host_contract: OwnerIdentityRef<ProjectionTargetDomain>,
    /// The components composed — at least one, by shape.
    pub components: NonEmptyBounded<WrapperComponent, WrapperComponentLimit>,
    /// The declared capability that selected them.
    pub capability_basis: OwnerFactRef,
}

/// Which way a remote surface faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceDirection {
    /// The surface receives.
    Inbound,
    /// The surface sends.
    Outbound,
}

/// What a remote surface projection plans: the surface one declared port speaks
/// over a wire contract.
///
/// It names the port projected, the wire contract spoken, and which way the
/// surface faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemoteSurfaceContent {
    /// The port declaration projected.
    pub port: OwnerIdentityRef<PortSubject>,
    /// The wire contract spoken.
    pub wire_contract: OwnerIdentityRef<WireContractSubject>,
    /// Which way the surface faces.
    pub direction: SurfaceDirection,
}

/// Where the row material one descriptor projection's crossing carries comes
/// from.
///
/// A descriptor plan names the MACHINE's own facts — the obligation a descriptor
/// challenges, the unit a benchmark measures, the currency its envelope is
/// stated in — and stops there.
/// Everything a rendered ROW states about itself is the harness's declaration:
/// the claim, the aggregate seat, the roles and tags, the subject route, the
/// check reference, the population, the callables, the input-size axis, the
/// declared budgets, and the neutral complexity reference.
/// Those arrive from the CALLER as the crossing's own payload, and this seat is
/// where a plan says so.
///
/// # Authority
///
/// **A roster of one, and the roster IS the statement.** A generator that could
/// state a second source is a generator that sometimes invents its own row
/// material and then proves it, which is the one thing these services never do.
/// Writing the posture down rather than leaving it implicit is what makes a
/// second source a law change at this roster instead of a seat somebody adds to
/// a content record.
///
/// # Nonclaims
///
/// It claims nothing ABOUT the material: not that a payload was supplied, not
/// that one will be, and nothing about what any row in it says.
/// It states whose declaration the rows are, and a plan holds no rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowMaterialPosture {
    /// The rows arrive whole from the caller, as the crossing's own payload.
    CallerSupplied,
}

/// What one test-descriptor projection challenges, at the end where the
/// obligation is named.
///
/// Not an option, and the same split every other anchoring in this file states:
/// a descriptor planned where a caller HOLDS the machine's obligation identity
/// names it, and a descriptor planned at expansion time — where nothing has been
/// linked and the machine has minted no obligation for anybody to name — says
/// THAT, and names the captured declaration it was derived from instead.
///
/// # Authority
///
/// **An obligation identity is the MACHINE's mint, and an expansion holds
/// none.** [`OwnerIdentityRef`] has one production road and it takes a
/// commitment the machine already minted, so a seat that admitted only the
/// machine's identity made a test-descriptor plan unbuildable inside a
/// proc-macro — which made the wall's first crossing unreachable from the only
/// door that exists. Writing the posture down is what lets the carrier be
/// planned at expansion time without any seam inventing owner meaning to fill a
/// required seat.
///
/// The two postures never read alike, and neither is a missing obligation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObligationAnchoring {
    /// The machine's own obligation identity, as the caller that holds it
    /// supplied it.
    Declared(OwnerIdentityRef<ObligationSubject>),
    /// The captured declaration this descriptor was derived from, with no
    /// obligation identity in existence yet. The expansion-time posture, stated
    /// rather than implied.
    CapturedDeclarationOnly(ProjectionIdentity<CapturedDeclarationSubject>),
}

/// What a test descriptor projection plans: the descriptor that challenges one
/// declared obligation.
///
/// It names what it challenges, under the posture the caller could honestly
/// state ([`ObligationAnchoring`]), and it states where the descriptor's rows
/// come from.
///
/// # Bounds
///
/// There is no challenge-METHOD seat, and the absence is the honest shape rather
/// than a dropped fact.
/// The harness's closed descriptor field set has no method seat at all: a row
/// names its CHECK, and which mechanism that check runs under is the check's own
/// fact.
/// A method carried here would therefore reach no emitted seat of the crossing —
/// a value the plan decided and nothing read, which reads as a decision the plan
/// made about the rendering when the rendering never consults it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestDescriptorContent {
    /// What is challenged, under the posture the planning caller could state.
    pub obligation: ObligationAnchoring,
    /// Where the descriptor's rows come from.
    pub rows: RowMaterialPosture,
}

/// What a benchmark descriptor projection plans: the descriptor that measures one
/// declared work formula.
///
/// It names the unit measured and the work currency the envelope is stated in,
/// and it states where the bench rows come from.
///
/// # Bounds
///
/// There is no verified-CLAIM seat, on the same terms and for a second reason
/// besides.
/// The harness's bench row roster carries a NEUTRAL complexity reference —
/// a standalone public vocabulary never names a product type — so the reference
/// itself is part of the caller-supplied row material, and the claim a product's
/// own evidence home declares is mapped onto it at the PRODUCT's integration.
/// A product claim carried here would reach no emitted seat and would put the
/// product's vocabulary inside the plan a neutral crossing is planned from.
///
/// # Nonclaims
///
/// A benchmark is evidence about one realization, never a specification: an
/// envelope measured here says what that realization did, and nothing about what
/// any other realization must do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchmarkDescriptorContent {
    /// The unit measured.
    pub measured: OwnerIdentityRef<MeasuredSubject>,
    /// The named work currency the envelope is stated in.
    pub work_currency: OwnerIdentityRef<WorkCurrencySubject>,
    /// Where the bench rows come from.
    pub rows: RowMaterialPosture,
}

/// What a documentation projection plans: declared meaning written as prose for
/// one named audience.
///
/// It names the subject documented, the audience it is written for, and the
/// machine's semantic facets it covers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentationContent {
    /// The subject documented.
    pub subject: OwnerIdentityRef<DocumentedSubject>,
    /// The audience the projection is written for.
    pub audience: OwnerIdentityRef<ProjectionAudienceDomain>,
    /// The facets covered.
    pub facets: Bounded<Facet, FacetLimit>,
}

/// What an implementation projection plans: the implementation that realizes one
/// contract for one type.
///
/// It names the type derived for, the contract realized, and the owner facts the
/// implementation assumes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeriveImplContent {
    /// The type the implementation is derived for.
    pub derived_type: ProjectionIdentity<DerivedTypeSubject>,
    /// The contract it realizes.
    pub contract: ProjectionIdentity<ImplementedContractSubject>,
    /// The owner facts assumed.
    pub assumptions: Bounded<OwnerFactRef, AssumptionLimit>,
}

/// What a pattern stamp projection plans: declaration material stamped out of one
/// authored pattern.
///
/// It names the pattern, this instantiation of it, and the typed arguments
/// supplied — a string never becomes an argument here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternStampContent {
    /// The authored pattern.
    pub pattern: OwnerIdentityRef<PatternSubject>,
    /// This instantiation of it.
    pub instance: OwnerIdentityRef<PatternInstanceSubject>,
    /// The typed arguments supplied.
    pub arguments: Bounded<OwnerIdentityRef<PatternArgumentSubject>, PatternArgumentLimit>,
}

/// Declares the sealed projection-kind roster ONCE, and derives from that single
/// declaration everything the roster settles: each kind's zero-sized marker and
/// its sealed [`ProjectionKind`] implementation, the enumerated roster a reader
/// quantifies over, and the disposition record that carries exactly one answer
/// per row.
///
/// One row per kind.
/// A roster written a second time is a roster that agrees until one of the two is
/// edited — and the thing that would then disagree is what a door says happened
/// to a kind, which is the one place silence is not an answer. A kind added below
/// therefore grows the marker, the roster, and the record together, and stops the
/// compiler at every total reading over the roster until somebody says what
/// happens to it.
///
/// The SEAT column is the spelling the disposition record names a row by. It is
/// declared beside the kind rather than composed from the type's spelling,
/// because a field name composed from a Rust identifier is renamed by every
/// refactor of that identifier — the same reason the declared stable name beside
/// it is declared rather than taken from the spelling.
macro_rules! kinds {
    ($(
        $(#[$note:meta])*
        $name:ident = $declared:literal, $seat:ident => $content:ty, $rendered:ty,
            $requirement:expr, [$($question:expr),* $(,)?]
    );+ $(;)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl ProjectionKind for $name {
                const SEAL: KindSeal = KindSeal::admitted();
                const KIND_NAME: &'static str = $declared;
                type Content = $content;
                type Rendered = $rendered;
                const KIND_QUESTIONS: &'static [ExplanationQuestion] = &[$($question),*];
                const TARGET_REQUIREMENT: TargetRequirement = $requirement;
            }
        )+

        /// The sealed kind roster, enumerated: one row per kind the declaration
        /// above states, and no row for a kind it does not.
        ///
        /// # Authority
        ///
        /// **The roster is the quantifier for what a door says it did, exactly as
        /// the rendered-role roster is the quantifier for the membership proof.**
        /// A reader that walked the kinds it happened to remember would leave a
        /// kind undispositioned, and an undispositioned kind is silence — the one
        /// answer [`ProjectionDisposition`] does not have a variant for. The rows
        /// are emitted by the same declaration that declares the kinds, so the
        /// roster cannot be short.
        ///
        /// # Nonclaims
        ///
        /// A row names a kind. It says nothing about whether any door plans that
        /// kind, whether a plan of it can be made at a given seam, or what
        /// happened to it anywhere: those are a door's answers, and
        /// [`KindDispositions`] is where a door carries them.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[expect(
            clippy::enum_variant_names,
            reason = "the rows are the projection kinds themselves and carry the kinds' own type names, so the shared word is the vocabulary this roster is a roster OF rather than a prefix anybody chose for it"
        )]
        pub enum ProjectionKindRow {
            $( $(#[$note])* $name ),+
        }

        impl ProjectionKindRow {
            /// The complete roster, in the order the kind declaration states it.
            pub const ALL: &'static [Self] = &[$( Self::$name ),+];

            /// This row's kind's declared stable name.
            ///
            /// READ off the kind itself rather than spelled again here, so a
            /// roster row and the kind it names cannot disagree about what the
            /// kind is called.
            #[must_use]
            pub const fn declared_name(self) -> &'static str {
                match self {
                    $( Self::$name => <$name as ProjectionKind>::KIND_NAME ),+
                }
            }
        }

        /// What happened to EVERY kind of the sealed roster over one piece of
        /// owner content: one seat per row, and every one of them required.
        ///
        /// # Authority
        ///
        /// **Exactly one disposition per kind, by the record's shape rather than
        /// by a caller's care.** Every field is public and required, so a
        /// construction that leaves a kind unanswered stops compiling exactly
        /// where a missing field does, and a kind admitted to the roster breaks
        /// every construction again — which is the whole point of stating the
        /// roster once. Nothing here can carry two answers for one kind, and
        /// nothing here can carry none.
        ///
        /// # Bounds
        ///
        /// It says what HAPPENED and never what was produced. A generated kind's
        /// seat names the one output a disposition names ([`ProjectionDisposition`]);
        /// the terminal that produced it, the membership it declared, and the
        /// cargo it proved are the terminal's own answers, read off the terminal.
        #[must_use = "a disposition record is what happened to every kind of the sealed roster"]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct KindDispositions {
            $(
                #[doc = concat!("What happened to the `", $declared, "` projection.")]
                pub $seat: ProjectionDisposition
            ),+
        }

        impl KindDispositions {
            /// What happened to one kind's projection.
            ///
            /// Total over the closed roster: every row reads to exactly one seat,
            /// and a row admitted later stops the compiler here until somebody
            /// says which seat carries it.
            pub const fn under(&self, row: ProjectionKindRow) -> &ProjectionDisposition {
                match row {
                    $( ProjectionKindRow::$name => &self.$seat ),+
                }
            }
        }
    };
}

/// The rendered units one implementation projection materializes: each
/// contract's production implementation, and each production implementation's
/// mutation-evaluation copy.
///
/// The seats are role-distinct rather than positions in a list.
/// The machine's refusal home splits what a family declares across two
/// contracts — the family's shape and textual order, and the typed cause order —
/// so an implementation projection over such a declaration materializes one
/// production unit per contract, each under its own role; and one implementation
/// meaning is delivered as TWO surfaces, so each production role carries an
/// evaluation role beside it ([`RenderedImplementation::twin`]).
///
/// A rendering that produced two units and swapped them is not "the same set in
/// another order": it is two units under the wrong roles, and the closure check
/// says so.
///
/// # Authority
///
/// **The evaluation copy is a planned member, on exactly the terms the
/// production unit is.** The delivery matrix's second paved road carries the
/// mutation-evaluation surface across the wall, and a surface that stood outside
/// the declared membership would be material emitted past the output firewall:
/// the closure rebuilds the membership role by role, so a copy no role planned
/// is a copy the proof never looks at. Planning it here is what makes "nothing is
/// emitted that did not close" true of that crossing rather than true of the
/// production half alone.
///
/// # Bounds
///
/// A member under an evaluation role is delivered into the TEST CARRIER
/// ([`RenderedImplementation::destination`]) and never into the declaration
/// site: the copy rides the generated support shell as deferred cargo, and the
/// build that compiles it is the consumer's test target. The shell's own
/// definition is written at the declaration site — that is what makes it
/// reachable — but the copy's tokens are not, and the difference is the whole
/// point of the second paved road: an evaluation surface spliced beside the
/// production implementation is a selector-bearing copy inside the consumer's
/// normal build, which is the one delivery this pairing exists to avoid.
///
/// The production seats name the REFUSAL family's two contracts, because the
/// refusal-family derive is the one derive family admitted at the door today and
/// a roster that named contracts nobody derives would be a roster the closure
/// quantifies over for nothing. The GENERAL form — a role indexed by the contract
/// it realizes, crossed with the surface it delivers — is owed, and its opening
/// condition is named: the first derive family admitted beside the refusal
/// family's, which is the point at which this roster would otherwise grow a
/// second pair of family-specific spellings. Until then the specific roster is
/// what is true, and a general one would be a vocabulary standing for
/// declarations nobody makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    clippy::enum_variant_names,
    reason = "the shared word is this crate's central distinction: a RENDERED member is what a planned one answers to, the two rosters stand side by side, and dropping the word here would make a row of this one read as a row of that one"
)]
pub enum RenderedImplementation {
    /// The family contract's production implementation: the body shape and the
    /// textual selection order.
    RenderedFamilyImpl,
    /// The typed cause-order contract's production implementation.
    RenderedCauseOrderImpl,
    /// The family implementation's mutation-evaluation copy.
    RenderedFamilyEvaluation,
    /// The cause-order implementation's mutation-evaluation copy.
    RenderedCauseOrderEvaluation,
}

kinds! {
    /// Projects a schema into the codec that reads and writes its canonical
    /// bytes.
    CodecProjection = "codec-projection", codec => CodecContent, SoleRenderedUnit,
        TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichAssumptionsAndSpecializations];

    /// Projects a declared surface into the wrapper one host contract needs.
    HostWrapperProjection = "host-wrapper-projection", host_wrapper => HostWrapperContent,
        SoleRenderedUnit, TargetRequirement::BoundHostContract,
        [
            ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
            ExplanationQuestion::WhichRuntimeTracesCorrespond,
        ];

    /// Projects a port declaration into a remote surface over a wire contract.
    RemoteSurfaceProjection = "remote-surface-projection", remote_surface => RemoteSurfaceContent,
        SoleRenderedUnit, TargetRequirement::BoundHostContract,
        [ExplanationQuestion::WhichRuntimeTracesCorrespond];

    /// Projects a declared obligation into the descriptor that challenges it.
    TestDescriptorProjection = "test-descriptor-projection", test_descriptor =>
        TestDescriptorContent, SoleRenderedUnit, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichTestsChallenge];

    /// Projects a declared work formula into the descriptor that measures it.
    BenchmarkDescriptorProjection = "benchmark-descriptor-projection", benchmark_descriptor =>
        BenchmarkDescriptorContent, SoleRenderedUnit, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichBenchmarksMeasure];

    /// Projects declared meaning into prose for a named audience.
    DocumentationProjection = "documentation-projection", documentation => DocumentationContent,
        SoleRenderedUnit, TargetRequirement::EitherBinding, [];

    /// Projects a declared contract into the implementation that realizes it.
    DeriveImplProjection = "derive-impl-projection", derive_impl => DeriveImplContent,
        RenderedImplementation, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichAssumptionsAndSpecializations];

    /// Projects an authored pattern's instantiation into declaration material.
    PatternStampProjection = "pattern-stamp-projection", pattern_stamp => PatternStampContent,
        SoleRenderedUnit, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichTemplateOrPatternInstance];
}

/// One projection plan's own identity, and the record of how it was derived.
///
/// A plan's identity is derived under [`ProjectionRole::Plan`], anchored on the
/// entry account's own commitment ([`CauseAnchoring::anchoring`]), over a
/// content transcript.
/// The role reads to [`PreimageFamily::Plan`], so a plan's version ladder is the
/// plan grammar's own: it moves when the members below move, and a widening
/// anywhere else in the plane reaches it not at all.
///
/// # Ordering
///
/// The transcript commits to, in this order:
///
/// 1. the INTENT — the kind's declared name ([`ProjectionKind::KIND_NAME`]), the
///    owner content commitment, and the dependency set that commitment declares,
///    exactly as [`OwnerContentAccount`] holds them;
/// 2. the shared context — graph anchoring, profile and version, generator
///    identity, target binding;
/// 3. the complete logical membership, in role-roster order;
/// 4. the watch set, canonicalized;
/// 5. the decision trace, in selection order;
/// 6. the origin trail, in walk order;
/// 7. the nonclaims, canonicalized.
///
/// The watch set, the dependency set, and the nonclaims are SETS: each member is
/// encoded, the encodings are sorted, and the sorted sequence is written, so the
/// same members supplied in another order produce the same identity.
/// The trace and the trail are SEQUENCES — their order is their meaning — so they
/// are written in the order they hold.
///
/// # Authority
///
/// **The transcript commits to the SEMANTIC origin projection and to nothing
/// location-addressed.** Every member above is an owner identity, a plane
/// identity, a declared stable name, a typed discriminant, or a declared
/// magnitude. No span, no source coordinate, and no path is a member, and none
/// is reachable from one: the origin trail's edges name origin NODES by
/// identity, a decision trace's citations name a home and a fact by the names
/// their owner declared, and the token home's span handles and the diagnostics
/// home's site coordinates belong to the diagnostic rail, which no plan seat
/// carries. Spans are ephemeral by nature, which is exactly why a semantic
/// identity that admitted one would move for a reason nobody's meaning changed
/// by.
///
/// **The generator reaches this transcript through the seat a plan DECLARED it
/// at and through nowhere else.** [`ProjectionContext::generator`] names the
/// version of the services a plan was produced under, and member two above
/// writes it, so a plan says which producer decided it. What no member carries
/// is the generator as a transcript field every family would have shared: a
/// producer's rendered shape moving is a fact about the producer, and a plan
/// whose name moved with it would be a plan nobody could match against the one
/// they already hold.
///
/// # Nonclaims
///
/// The transcript does not commit to the kind-specific content's VALUES.
/// The kind is named, the owner content commitment and its dependency set are
/// carried at full width, and every plane-typed fact the kind content carries
/// that a plan actually turns on — the derived type, the realized contract, the
/// semantic keys — reaches the identity through the membership.
/// But [`ProjectionKind::Content`] is an owner-typed record with no canonical
/// byte encoding, and the plane declares none for it: an encoding of the
/// machine's semantic facets — the machine-owned vocabulary a kind content still
/// carries — would be a second answer to the machine's own encoding question,
/// which the services are forbidden to create.
/// Two plans of one kind, over one account, one context, and one membership,
/// differing only inside their kind content, therefore carry one plan identity —
/// the one place a plan transcript is narrower than the plan, and it is now
/// narrower by the kind content alone: the content a plan was planned OVER is
/// committed to.
///
/// [`PreimageFamily::Plan`]: crate::plane::PreimageFamily::Plan
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanDerivation {
    identity: PlanId,
    provenance: ProjectionProvenance,
}

/// Everything one plan DECIDED, as the one value those seats travel in.
///
/// Five seats, in the order a plan's transcript writes them: the complete
/// logical membership, the watch set, the decision trace, the origin trail, and
/// the nonclaims.
/// They arrive together because they were decided together — a membership
/// without the triggers that invalidate it, or a trail without the trace that
/// walked it, is half a decision — and because the road that takes them takes
/// the entry account and the shared context beside them.
///
/// # Authority
///
/// **Bundling settles nothing and defaults nothing.** Every field is required
/// and public, so a construction that leaves one out stops compiling exactly
/// where a missing argument used to, and a seat added to a plan is added here
/// and breaks every construction again.
/// What it removes is a call site stating eight positional facts, where seats of
/// one shape in a row are told apart by counting commas — and which is past the
/// arity the lint wall admits.
///
/// # Bounds
///
/// The kind-specific content is NOT one of these seats.
/// A plan's transcript does not commit to it ([`PlanDerivation`]), so a value
/// that grouped it with the seats the transcript does commit to would read as
/// though it were one of them.
#[must_use = "the decided seats are what one plan is planned from, whole"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanDecisions<R: RenderedRole> {
    /// The complete declared output set — the output firewall.
    pub membership: PlannedMembership<R>,
    /// The identities whose change invalidates the plan.
    pub invalidation: InvalidationSet,
    /// The decisions that produced the plan, in selection order.
    pub trace: DecisionTrace,
    /// Where the plan itself came from, in walk order.
    pub origin: OriginTrail,
    /// What the plan explicitly does not claim.
    pub nonclaims: Bounded<Nonclaim, NonclaimLimit>,
}

/// One projection plan: the shared spine on the generic.
///
/// Every seat is required, and the seats that could be empty are shapes that
/// cannot be: the cause set, the output set, the watch set, the trace, and the
/// trail are all structurally non-empty.
/// Only nonclaims may be empty, because a plan that claims exactly what it does
/// has none to state.
///
/// A plan carries its OWN identity, derived when it is planned; see
/// [`PlanDerivation`] for the transcript that identity commits to.
///
/// The account seat is the plan's copy of nothing: it is the account the caller
/// walked in with, moved into the plan, so the plan's own answer to "what were
/// you planned over" is the value the watch set, the intent, and the origin
/// edges were all read off.
#[must_use = "a plan is the complete declared output set nothing may be rendered without"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionPlan<K: ProjectionKind> {
    derivation: PlanDerivation,
    account: OwnerContentAccount<K>,
    context: ProjectionContext,
    content: K::Content,
    membership: PlannedMembership<K::Rendered>,
    invalidation: InvalidationSet,
    trace: DecisionTrace,
    origin: OriginTrail,
    nonclaims: Bounded<Nonclaim, NonclaimLimit>,
}

/// A set of plans materialized as one unit across one publication boundary.
///
/// Atomicity is the boundary's law and this type is where it is stated: the
/// members are staged as a unit, checked as a unit, and published as a unit.
/// A partial materialization is a refusal, never a partial success — half a set
/// of sibling projections is a set whose siblings disagree.
#[must_use = "a bundle plan is materialized whole or not at all"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionBundlePlan {
    bundle: ProjectionIdentity<BundleSubject>,
    members: NonEmptyBounded<PlanId, BundleMemberLimit>,
}

/// What happened to one projection that could have been generated.
///
/// Every kind that could apply gets one of these.
/// Silence is not a variant: where a projection is absent, the absence has a name
/// and, where a fact caused it, a citation.
#[must_use = "a disposition is what happened to a projection, and silence is not a variant"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionDisposition {
    /// It was generated, and this is the output.
    /// Boxed because a disposition travels by value beside every plan, and the
    /// largest answer must not set the size of the smaller ones.
    Generated {
        /// The generated unit.
        output: Box<PlannedOutput>,
    },
    /// It does not apply here, because of this owner fact.
    NotApplicable {
        /// The fact that makes it inapplicable.
        because: OwnerFactRef,
    },
    /// It was refused, and this is the refusal.
    Refused {
        /// The planning refusal.
        refusal: ProjectionPlanning,
    },
    /// The selected profile does not offer it, and this is the fact naming what
    /// that profile could not furnish.
    ///
    /// # Authority
    ///
    /// **The citation is the same shape [`ProjectionDisposition::NotApplicable`]
    /// already carries, and it is here for the same reason.** A profile named
    /// alone says that a decision happened without saying whose fact decided it,
    /// and a reader asking why one kind is unavailable under a profile that
    /// offers another kind is handed the standing and never the seat. The
    /// profile and its version say WHICH standing; the fact says what that
    /// standing could not fill.
    ///
    /// # Bounds
    ///
    /// One fact, and its own stable name states the CONJUNCTION where several
    /// seats are independently blocked — so no blocker is elected as the primary
    /// one and no seat is silently dropped.
    ///
    /// It is a citation and never a roster of blocked seats. A byte role, a work
    /// currency, a host contract, an audience, and a wire contract belong to
    /// five different semantic owners, and one enumerated seat vocabulary
    /// standing for all of them would be this home minting a shared word for
    /// facts it does not own — which is the second answer the citation shape
    /// exists to avoid.
    UnavailableUnderProfile {
        /// The profile that does not offer it.
        profile: ProjectionIdentity<ProjectionProfileSubject>,
        /// That profile's version.
        version: ProfileVersion,
        /// The fact naming what that profile could not furnish.
        because: OwnerFactRef,
    },
    /// Nobody asked for it.
    NotRequested,
    /// A configuration excluded it.
    ExcludedByConfiguration {
        /// The excluding configuration.
        configuration: OwnerIdentityRef<ProjectionConfigurationDomain>,
    },
}

/// The posture a checked-in schema expectation stands under while it is the
/// AUTHOR's word: hand-authored on both sides of the wall, claiming pair
/// coherence and nothing else.
///
/// A declared-bootstrap pair says "these two literals were written together by
/// somebody who meant them to agree". It does not say the literal is the
/// identity the harness's root schema declaration derives to.
///
/// # Bounds
///
/// **No value of this crate carries it.** It is the posture the first pair
/// stood under before anything had been derived, and there is no road back to
/// it: a pin is rewritten from a derivation from here on, so a hand-authored
/// expectation is a value the seat that takes [`VerifiedDerived`] cannot hold.
/// It is declared because the parameter it inhabits tells two postures apart at
/// compile time, and that is what made the flip a change of type rather than an
/// edit of bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredBootstrap;

/// The posture a checked-in schema expectation stands under once it has been
/// DERIVED from the harness's root schema declaration and copied here.
///
/// This crate's one expectation stands here. The bytes came off
/// `GeneratedSupportSchema::published()?.identity()?` in the home that owns the
/// declaration, and what keeps them current is that home's own currency lane
/// rather than anything on this side — these services cannot derive the value,
/// because the declaration lives in a crate they do not depend on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerifiedDerived;

/// The services' OWN expectation of the generated-support schema identity,
/// tagged by the posture the value stands under.
///
/// This is the whole of what these services own about the harness's schema:
/// **"I know how to emit against generated-support schema X."** The harness owns
/// the schema, its root declaration, the identity derived from that
/// declaration's canonical bytes, and every disposal route for a pair that does
/// not agree. This side owns one fact and holds it independently.
///
/// # Authority
///
/// **Independence across upgrade time is the whole mechanism.** The two values
/// live in two crates and are rewritten together, in one git-visible
/// human-committed change, when the harness's declaration moves. The comparison
/// the harness's gate performs therefore detects a version-mixed consumer, a
/// partial rewrite, or a hand edit to one side.
///
/// **It is NEVER derived during an invocation from the harness's supplied or
/// current id.** There is no constructor that takes a supplied identity, and
/// there is no public constructor at all: taking the harness's id as the input
/// to this side's expectation would rebuild a comparison of a value with itself,
/// which detects nothing at any cost.
///
/// # Nonclaims
///
/// A jointly stale pair — the schema changed and neither literal was rewritten,
/// so two old values still agree — is OUTSIDE this expectation's claim. It dies
/// at the compiler, where a changed constructor shape is an ordinary type error,
/// or in the harness's currency lane, where the current schema's id is derived
/// and both published spellings are required to equal it. The disposal routes
/// belong to the side that owns the mailbox.
///
/// # Bounds
///
/// The posture parameter DEFAULTS to [`VerifiedDerived`], which is the posture
/// this crate's one expectation actually stands in, so the default states where
/// the crate is rather than hiding a choice. [`DeclaredBootstrap`] has no
/// inhabitant and no road back, so a seat spelled without a posture takes a
/// derived expectation and cannot be handed a hand-authored one.
#[must_use = "the expectation is the one fact these services hold about the harness's schema"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedGeneratedSupportSchemaId<Posture = VerifiedDerived> {
    bytes: [u8; 32],
    _posture: PhantomData<Posture>,
}

/// The services' checked-in expectation of the generated-support schema
/// identity.
///
/// These thirty-two bytes were DERIVED, from the harness's own published
/// declaration through that home's own road, and copied here. They are not a
/// sentence and do not read as one: a reader who dumps them sees a digest, which
/// is what a value nobody authored looks like.
///
/// Written in DECIMAL, which is the base the harness's gate matches in and the
/// base its own published constant is written in. The rendering road turns these
/// bytes into the roster the gate's arm carries, and an unsuffixed integer has
/// exactly one rendering — so the base here is what makes the two crates'
/// spellings comparable by eye as well as by the currency lane.
///
/// # Authority
///
/// **An all-zero address is forbidden here.** Zeros are the value every
/// uninitialized, defaulted, or forgotten seat also carries, so a zero
/// expectation would compare equal to every other forgotten one.
///
/// This side cannot derive the value itself — the declaration it names lives in
/// a crate these services do not depend on — so what stands behind it is the
/// harness's currency lane, which derives the identity from the current
/// declaration and requires both published spellings to equal it. That lane is
/// the reason this constant can be trusted as CURRENT rather than merely
/// coherent with its twin.
pub const EXPECTED_GENERATED_SUPPORT_SCHEMA_ID: ExpectedGeneratedSupportSchemaId<VerifiedDerived> =
    ExpectedGeneratedSupportSchemaId::derived([
        113, 22, 215, 27, 201, 83, 45, 177, 228, 123, 154, 255, 239, 17, 99, 56, 150, 45, 78, 145,
        144, 250, 75, 10, 60, 33, 74, 147, 17, 187, 77, 147,
    ]);

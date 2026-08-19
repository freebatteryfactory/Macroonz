//! The plan family's declarations: the entry account and the intent it names,
//! the shared context, the output firewall, the invalidation roster, the sealed
//! kind roster and its contents, the plan itself, the bundle, the disposition,
//! and the services' own expectation of the generated-support schema identity.
//!
//! Declarations only.
//! Every road that reaches a private field — the account's addressing, the
//! membership's members, a plan's seats, a bundle's member set, the schema
//! expectation's bytes — lives in `type_guard.rs`, this file's own child.

use crate::origin_graph::{DecisionTrace, Nonclaim, OriginTrail};
use crate::plane::{
    AssumptionLimit, BundleMemberLimit, BundleSubject, ByteRoleSubject, CapturedDeclarationSubject,
    DerivedTypeSubject, DocumentedSubject, FacetLimit, FixturePopulationSubject,
    GeneratedUnitSubject, GeneratorVersionSubject, ImplementedContractSubject, InvalidationLimit,
    MeasuredSubject, MechanismProfileSubject, MembershipLimit, NonclaimLimit, ObligationSubject,
    OwnerFactRef, OwnerIdentityRef, PatternArgumentLimit, PatternArgumentSubject,
    PatternInstanceSubject, PatternSubject, PlanId, PortSubject, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, ProjectionProvenance, ProjectionRole,
    RenderedRole, SchemaSubject, SoleRenderedUnit, SourceDeclarationLimit, WireContractSubject,
    WorkCurrencySubject, WorkFormulaSubject, WrapperComponentLimit, static_bytes,
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
use threadpak::evidence::{Method, VerifiedClaim};
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

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

/// The intent layer's identity: WHAT was meant, as the pair that means it — the
/// kind's declared name and the owner content commitment it was meant over.
///
/// The first of the three identity layers.
/// The plan identity is derived over it and everything the plan decided beside
/// it; the rendered-unit identity is derived over bytes that do not exist yet
/// when this one does.
///
/// # Authority
///
/// **The pair is carried exactly, and equality of the pair is equality of
/// intent.** Two doors that meant the same thing carry one of these, which is
/// what door equivalence compares — plan identities cannot be compared for that,
/// since distinct doors are required to carry distinct origins.
///
/// # Nonclaims
///
/// **It is NOT a derived identity and never stands where one is required.**
/// It is not thirty-two bytes, it is not a member of either identity family, and
/// it never anchors a transcript: the plane's identity subjects and roles are
/// sealed rosters, and neither carries a seat for the intent layer, so a
/// digested spelling of this pair is unwritable here rather than approximated.
/// [`OwnerContentAccount::intent_bytes`] is the canonical preimage a digested
/// spelling would be derived over, written already, so admitting an intent
/// subject and role to the plane's rosters is the whole of what that promotion
/// costs.
#[must_use = "an intent identity is what door equivalence compares"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionIntentId {
    kind: &'static str,
    content: CauseAnchoring,
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

/// Where one planned member lands once it is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberDestination {
    /// Spliced into the declaration the plan was derived from — the expansion
    /// destination, where the rendered unit replaces or accompanies the
    /// caller's own item.
    AtDeclarationSite,
    /// Written as a standalone artifact under the named byte role.
    AsArtifact {
        /// The byte role the artifact is written under.
        byte_role: OwnerIdentityRef<ByteRoleSubject>,
    },
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

/// What a test descriptor projection plans: the descriptor that challenges one
/// declared obligation.
///
/// It names the obligation and the method the challenge is made by.
/// The method is the machine's own verification vocabulary — a compile refusal is
/// one challenge kind among several, never the universal one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestDescriptorContent {
    /// The obligation challenged.
    pub obligation: OwnerIdentityRef<ObligationSubject>,
    /// The challenge method.
    pub challenge: Method,
}

/// What a benchmark descriptor projection plans: the descriptor that measures one
/// declared work formula.
///
/// It names the unit measured, the work currency the envelope is stated in, and
/// the claim the envelope stands for.
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
    /// The claim the envelope stands for.
    pub claim: VerifiedClaim,
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

/// Declares one projection kind: a zero-sized marker plus its sealed
/// [`ProjectionKind`] implementation.
macro_rules! kinds {
    ($(
        $(#[$note:meta])*
        $name:ident = $declared:literal => $content:ty, $rendered:ty, $requirement:expr,
            [$($question:expr),* $(,)?]
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
    };
}

/// The rendered units one implementation projection materializes.
///
/// The two seats are role-distinct rather than positions in a list.
/// The machine's refusal home splits what a family declares across two
/// contracts — the family's shape and textual order, and the typed cause order —
/// so an implementation projection over such a declaration materializes one unit
/// per contract, each under its own role.
///
/// A rendering that produced two units and swapped them is not "the same set in
/// another order": it is two units under the wrong roles, and the closure check
/// says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderedImplementation {
    /// The family contract's implementation: the body shape and the textual
    /// selection order.
    RenderedFamilyImpl,
    /// The typed cause-order contract's implementation.
    RenderedCauseOrderImpl,
}

kinds! {
    /// Projects a schema into the codec that reads and writes its canonical
    /// bytes.
    CodecProjection = "codec-projection" => CodecContent, SoleRenderedUnit,
        TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichAssumptionsAndSpecializations];

    /// Projects a declared surface into the wrapper one host contract needs.
    HostWrapperProjection = "host-wrapper-projection" => HostWrapperContent, SoleRenderedUnit,
        TargetRequirement::BoundHostContract,
        [
            ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
            ExplanationQuestion::WhichRuntimeTracesCorrespond,
        ];

    /// Projects a port declaration into a remote surface over a wire contract.
    RemoteSurfaceProjection = "remote-surface-projection" => RemoteSurfaceContent, SoleRenderedUnit,
        TargetRequirement::BoundHostContract,
        [ExplanationQuestion::WhichRuntimeTracesCorrespond];

    /// Projects a declared obligation into the descriptor that challenges it.
    TestDescriptorProjection = "test-descriptor-projection" => TestDescriptorContent,
        SoleRenderedUnit, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichTestsChallenge];

    /// Projects a declared work formula into the descriptor that measures it.
    BenchmarkDescriptorProjection = "benchmark-descriptor-projection" =>
        BenchmarkDescriptorContent, SoleRenderedUnit, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichBenchmarksMeasure];

    /// Projects declared meaning into prose for a named audience.
    DocumentationProjection = "documentation-projection" => DocumentationContent, SoleRenderedUnit,
        TargetRequirement::EitherBinding, [];

    /// Projects a declared contract into the implementation that realizes it.
    DeriveImplProjection = "derive-impl-projection" => DeriveImplContent, RenderedImplementation,
        TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichAssumptionsAndSpecializations];

    /// Projects an authored pattern's instantiation into declaration material.
    PatternStampProjection = "pattern-stamp-projection" => PatternStampContent, SoleRenderedUnit,
        TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichTemplateOrPatternInstance];
}

/// One projection plan's own identity, and the record of how it was derived.
///
/// A plan's identity is derived under [`ProjectionRole::Plan`], anchored on the
/// entry account's own commitment ([`CauseAnchoring::anchoring`]), over a
/// content transcript.
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
/// # Nonclaims
///
/// The transcript does not commit to the kind-specific content's VALUES.
/// The kind is named, the owner content commitment and its dependency set are
/// carried at full width, and every plane-typed fact the kind content carries
/// that a plan actually turns on — the derived type, the realized contract, the
/// semantic keys — reaches the identity through the membership.
/// But [`ProjectionKind::Content`] is an owner-typed record with no canonical
/// byte encoding, and the plane declares none for it: an encoding of the
/// machine's verification methods, semantic facets, and verified claims would be
/// a second answer to the machine's own encoding question, which the services are
/// forbidden to create.
/// Two plans of one kind, over one account, one context, and one membership,
/// differing only inside their kind content, therefore carry one plan identity —
/// the one place a plan transcript is narrower than the plan, and it is now
/// narrower by the kind content alone: the content a plan was planned OVER is
/// committed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanDerivation {
    identity: PlanId,
    provenance: ProjectionProvenance,
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
    /// The selected profile does not offer it.
    UnavailableUnderProfile {
        /// The profile that does not offer it.
        profile: ProjectionIdentity<ProjectionProfileSubject>,
        /// That profile's version.
        version: ProfileVersion,
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
/// somebody who meant them to agree".
/// It does not say the literal is the identity the harness's root schema
/// declaration actually derives to — nothing has derived that yet, because the
/// publication operation that derives it does not exist until a toolchain runs.
/// What the comparison over a bootstrap pair still detects is real: a
/// version-mixed consumer, a partial publication, and a hand edit to one side.
/// What it cannot detect is stated where the expectation is declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredBootstrap;

/// The posture a checked-in schema expectation stands under once the publication
/// operation has DERIVED it from the harness's root schema declaration under a
/// receipt.
///
/// Declared here because the type parameter it inhabits exists to tell the two
/// postures apart at compile time, and a posture parameter with one inhabitant
/// tells nothing apart.
/// No value of this crate carries it yet: the flip from
/// [`DeclaredBootstrap`] to this posture is itself a receipted, human-committed
/// publication act at the first toolchain contact, and writing it before that
/// act would be claiming the derivation happened.
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
/// live in two crates and are written by ONE explicit publication operation at
/// schema-change time, git-visible and human-committed under a receipt. The
/// comparison the harness's gate performs therefore detects a version-mixed
/// consumer, a partial publication, or a hand edit to one side.
///
/// **It is NEVER derived during an invocation from the harness's supplied or
/// current id.** There is no constructor that takes a supplied identity, and
/// there is no public constructor at all: taking the harness's id as the input
/// to this side's expectation would rebuild a comparison of a value with itself,
/// which detects nothing at any cost.
///
/// # Nonclaims
///
/// A jointly stale pair — the schema changed and publication never ran, so two
/// old literals still agree — is OUTSIDE this expectation's claim. It dies at
/// the compiler, where a changed constructor shape is an ordinary type error, or
/// in the harness's conformance lane, where the current schema's id is derived
/// and checked against the published literal. The disposal routes belong to the
/// side that owns the mailbox.
///
/// # Bounds
///
/// The posture parameter DEFAULTS to [`DeclaredBootstrap`], which is the posture
/// this crate's one expectation actually stands in today, so the default states
/// where the crate is rather than hiding a choice. Moving it to
/// [`VerifiedDerived`] is part of the same receipted publication act that
/// rewrites the literal, and a reader who wants the posture spelled at a use site
/// writes it.
#[must_use = "the expectation is the one fact these services hold about the harness's schema"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedGeneratedSupportSchemaId<Posture = DeclaredBootstrap> {
    bytes: [u8; 32],
    _posture: PhantomData<Posture>,
}

/// The phrase both sides of the wall spell their DECLARED-BOOTSTRAP literal
/// from.
///
/// It is fifty-six bytes and an identity is thirty-two, so the literal is this
/// phrase's first thirty-two bytes — the cut is stated here rather than left for
/// a reader to infer, and both sides perform the same cut on the same phrase.
///
/// # Bounds
///
/// The cut drops the phrase's tail, so the LITERAL alone does not spell the
/// version or the posture: the posture is carried by the type parameter
/// [`DeclaredBootstrap`] and the version by this phrase. A later bootstrap
/// phrase must therefore differ inside its first thirty-two bytes, or two
/// versions' bootstrap pairs would be spelled identically and the comparison
/// that exists to catch a version-mixed consumer would open for one.
pub const GENERATED_SUPPORT_SCHEMA_DECLARED_BOOTSTRAP: &str =
    "threadpak-generated-support-schema-v0-declared-bootstrap";

/// The services' checked-in expectation of the generated-support schema
/// identity, in the declared-bootstrap posture.
///
/// The bytes are the ASCII `threadpak-generated-support-sche` — the first
/// thirty-two of [`GENERATED_SUPPORT_SCHEMA_DECLARED_BOOTSTRAP`], and the exact
/// literal the harness's own published side carries until the first toolchain
/// contact. Two hand-authored sides, one phrase, one cut: that is what "pair
/// coherence" means while the posture is [`DeclaredBootstrap`].
///
/// Readable ASCII is deliberate. A reader who dumps these bytes sees a sentence
/// rather than a digest and knows immediately that nothing derived them.
///
/// # Authority
///
/// **An all-zero address is forbidden here and is not what a bootstrap looks
/// like.** Zeros are the value every uninitialized, defaulted, or forgotten seat
/// also carries, so a zero expectation would compare equal to every other
/// forgotten one and would read as a derived identity that happened to be
/// unlucky. This value is unmistakably authored, which is exactly the claim the
/// posture makes.
///
/// It is written by the publication operation and by nothing else once that
/// operation exists; until then it is the hand-authored first pair, and the flip
/// to [`VerifiedDerived`] is a receipted publication act rather than an edit.
pub const EXPECTED_GENERATED_SUPPORT_SCHEMA_ID: ExpectedGeneratedSupportSchemaId =
    ExpectedGeneratedSupportSchemaId::declared(static_bytes(
        GENERATED_SUPPORT_SCHEMA_DECLARED_BOOTSTRAP,
    ));

//! The plan family's declarations: the shared context, the output firewall, the
//! invalidation roster, the sealed kind roster and its contents, the plan
//! itself, the bundle, and the disposition.
//!
//! Declarations only. Every road that reaches a private field — the membership's
//! members, a plan's seven seats, a bundle's member set — lives in
//! `type_guard.rs`, this file's own child.

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
    WorkCurrencySubject, WorkFormulaSubject, WrapperComponentLimit,
};
use crate::question::ExplanationQuestion;
use crate::refusal::ProjectionPlanning;
use core::fmt::Debug;
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
/// host contract, and the two must never read the same. A plan whose kind needs
/// a host and whose binding is target-free refuses rather than defaulting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetBinding {
    /// Bound to one named host contract.
    HostContract(OwnerIdentityRef<ProjectionTargetDomain>),
    /// Deliberately bound to no host contract.
    TargetFree,
}

/// The source declarations one plan names as its cause.
pub type SourceDeclarations =
    NonEmptyBounded<OwnerIdentityRef<FragmentIdentityDomain>, SourceDeclarationLimit>;

/// The triggers one plan watches.
pub type InvalidationSet = NonEmptyBounded<InvalidationTrigger, InvalidationLimit>;

/// What a plan was decided AGAINST at its graph end.
///
/// Not an option. A plan decided against the machine's closed declaration graph
/// says so and names it; a plan decided at expansion time, where nothing has
/// been linked and there is no closed graph to name, says THAT — and names the
/// captured declaration it was decided against instead. The two postures never
/// read alike, and neither is a missing graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphAnchoring {
    /// Decided against the machine's closed declaration graph.
    ClosedGraph(OwnerIdentityRef<LinkedGraphDomain>),
    /// Decided against one captured declaration alone, with no closed graph in
    /// existence yet. The expansion-time posture, stated rather than implied.
    CapturedDeclarationOnly(ProjectionIdentity<CapturedDeclarationSubject>),
}

/// What CAUSED a plan.
///
/// The same split, at the other end: the machine's declaration fragments where a
/// caller holds them, and otherwise the exact token material one expansion was
/// handed. A capture is a real cause and is named as one; it is never dressed up
/// as a fragment the linker never minted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CauseAnchoring {
    /// The machine's declaration fragments — at least one, by shape.
    Declarations(SourceDeclarations),
    /// The captured declaration this plan was derived from.
    CapturedDeclaration(ProjectionIdentity<CapturedDeclarationSubject>),
}

/// The exact identities every plan shares, whatever its kind: what it was
/// decided against, which profile at which version, what caused it, which
/// version of the services produced it, and what it is bound to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionContext {
    /// What this plan was decided against.
    pub graph: GraphAnchoring,
    /// The projection profile selected.
    pub profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// What caused this plan.
    pub sources: CauseAnchoring,
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
/// # Why a plan carries a contract and never a digest
///
/// A plan is made BEFORE anything is rendered. A digest of rendered bytes is a
/// fact about bytes, and those bytes do not exist yet, so a plan that carried
/// one would be carrying a value nobody computed — either a placeholder, or a
/// digest smuggled in from a rendering that already happened, which makes the
/// closure check compare a value against itself.
///
/// So the plan states the CONTRACT: the role the digest will carry, and the
/// member identity it must be anchored to. The closure check recomputes the
/// digest from the rendered bytes under exactly this contract and compares. A
/// digest anchored anywhere else belongs to a different member.
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
/// (the contract). No rendered bytes and no rendered-byte digest: those are the
/// rendering's facts and they live on the rendered unit.
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
/// The role is what closure matches on. A rendering that produced the right
/// NUMBER of units in the wrong roles is caught by the role rather than passing
/// a count.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedMember<R: RenderedRole> {
    /// The rendered role this member plans.
    pub role: R,
    /// The logical output under that role.
    pub output: PlannedOutput,
}

/// The complete declared output set of one plan — the output firewall.
///
/// Structurally non-empty: a plan that would generate nothing is not a plan,
/// it is a disposition. Bounded: a plan that would generate past the declared
/// magnitude refuses rather than materializing part of a set.
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
    /// The captured declaration this plan was derived from changed. The
    /// expansion-time twin of the fragment trigger: where the cause IS the
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
/// declared anywhere else cannot satisfy [`ProjectionKind`]. The roster is
/// closed because the explanation protocol is mandatory: a kind nobody can
/// explain is a kind that must not be planned.
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
/// Sealed. Implementing it is a law change, not an extension point — a frontend
/// plugs in through the machine's declaration path, never by inventing a
/// projection kind the plane cannot explain.
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

    /// The questions this kind answers *beyond* [`UNIVERSAL_QUESTIONS`]. The
    /// universal ones are not restated here — one roster, one home, and a kind
    /// that could drop a universal question by forgetting to list it does not
    /// exist.
    const KIND_QUESTIONS: &'static [ExplanationQuestion];

    /// What this kind requires of the context's target binding.
    const TARGET_REQUIREMENT: TargetRequirement;
}

/// The questions every kind answers, whatever it plans. No kind ducks the
/// protocol: this roster is added to every kind's own, so a kind cannot narrow
/// what it must be able to explain.
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

/// A codec projection's facts: which schema, which byte role, which direction.
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
/// The roster is what an exhaustive disposition is checked against: a view
/// that must decide every component reads this, so a component added here and
/// nowhere else stops compiling at the closure law rather than passing
/// silently undecided.
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

/// A host wrapper's facts: which host contract, which components were selected,
/// and on whose declared capability.
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

/// A remote surface's facts: which port, which wire contract, which direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemoteSurfaceContent {
    /// The port declaration projected.
    pub port: OwnerIdentityRef<PortSubject>,
    /// The wire contract spoken.
    pub wire_contract: OwnerIdentityRef<WireContractSubject>,
    /// Which way the surface faces.
    pub direction: SurfaceDirection,
}

/// A test descriptor's facts: which obligation it challenges, and by which
/// method. The method is the machine's own verification vocabulary — a compile
/// refusal is one challenge kind among several, never the universal one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestDescriptorContent {
    /// The obligation challenged.
    pub obligation: OwnerIdentityRef<ObligationSubject>,
    /// The challenge method.
    pub challenge: Method,
}

/// A benchmark descriptor's facts: what is measured, in which work currency,
/// and which claim the envelope stands for. A benchmark is evidence about one
/// realization, never a specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchmarkDescriptorContent {
    /// The unit measured.
    pub measured: OwnerIdentityRef<MeasuredSubject>,
    /// The named work currency the envelope is stated in.
    pub work_currency: OwnerIdentityRef<WorkCurrencySubject>,
    /// The claim the envelope stands for.
    pub claim: VerifiedClaim,
}

/// A documentation projection's facts: what is documented, for which audience,
/// over which of the machine's semantic facets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentationContent {
    /// The subject documented.
    pub subject: OwnerIdentityRef<DocumentedSubject>,
    /// The audience the projection is written for.
    pub audience: OwnerIdentityRef<ProjectionAudienceDomain>,
    /// The facets covered.
    pub facets: Bounded<Facet, FacetLimit>,
}

/// An implementation projection's facts: which type, which contract, and which
/// owner facts the implementation assumes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeriveImplContent {
    /// The type the implementation is derived for.
    pub derived_type: ProjectionIdentity<DerivedTypeSubject>,
    /// The contract it realizes.
    pub contract: ProjectionIdentity<ImplementedContractSubject>,
    /// The owner facts assumed.
    pub assumptions: Bounded<OwnerFactRef, AssumptionLimit>,
}

/// A pattern stamp's facts: which authored pattern, which instantiation, and
/// the typed arguments supplied. A string never becomes an argument here.
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
/// Two seats, and they are role-distinct rather than positions in a list. The
/// machine's refusal home splits what a family declares across two contracts —
/// the family's shape and textual order, and the typed cause order — so an
/// implementation projection over such a declaration materializes one unit per
/// contract, each under its own role.
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
/// # The plan transcript, and its one stated boundary
///
/// A plan's identity is derived under [`ProjectionRole::Plan`], anchored on
/// whatever CAUSED the plan ([`CauseAnchoring::anchoring`]), over a content
/// transcript that commits to, in this order:
///
/// 1. the kind's declared name ([`ProjectionKind::KIND_NAME`]);
/// 2. the shared context — graph anchoring, profile and version, cause
///    anchoring, generator identity, target binding;
/// 3. the complete logical membership, in role-roster order;
/// 4. the watch set, canonicalized (see below);
/// 5. the decision trace, in selection order;
/// 6. the origin trail, in walk order;
/// 7. the nonclaims, canonicalized.
///
/// The watch set and the nonclaims are SETS: each member is encoded, the
/// encodings are sorted, and the sorted sequence is written. The same members
/// supplied in another order therefore produce the same identity, which is what
/// a set means. The trace and the trail are SEQUENCES — their order is their
/// meaning — so they are written in the order they hold.
///
/// **What the transcript does NOT commit to: the kind-specific content's
/// values.** The kind is named, and every plane-typed fact the content carries
/// that a plan actually turns on — the derived type, the realized contract, the
/// semantic keys — reaches the identity through the membership. But
/// [`ProjectionKind::Content`] is an owner-typed record with no canonical byte
/// encoding, and the plane declares none for it: an encoding of the machine's
/// verification methods, semantic facets, and verified claims would be a second
/// answer to the machine's own encoding question, which the services are
/// forbidden to create. Two plans of one kind, over one context and one
/// membership, differing only inside their kind content, therefore carry one
/// plan identity. That is stated here rather than implied, and it is the one
/// place a plan transcript is narrower than the plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanDerivation {
    identity: PlanId,
    provenance: ProjectionProvenance,
}

/// One projection plan: the shared spine on the generic.
///
/// Every seat is required, and the seats that could be empty are shapes that
/// cannot be: the cause set, the output set, the watch set, the trace, and the
/// trail are all structurally non-empty. Only nonclaims may be empty, because a
/// plan that claims exactly what it does has none to state.
///
/// A plan carries its OWN identity, derived when it is planned. See
/// [`PlanDerivation`] for the transcript that identity commits to.
#[must_use = "a plan is the complete declared output set nothing may be rendered without"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionPlan<K: ProjectionKind> {
    derivation: PlanDerivation,
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
/// members are staged as a unit, checked as a unit, and published as a unit. A
/// partial materialization is a refusal, never a partial success — half a set
/// of sibling projections is a set whose siblings disagree.
#[must_use = "a bundle plan is materialized whole or not at all"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionBundlePlan {
    bundle: ProjectionIdentity<BundleSubject>,
    members: NonEmptyBounded<PlanId, BundleMemberLimit>,
}

/// What happened to one projection that could have been generated.
///
/// Every kind that could apply gets one of these. Silence is not a variant:
/// where a projection is absent, the absence has a name and, where a fact
/// caused it, a citation.
#[must_use = "a disposition is what happened to a projection, and silence is not a variant"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionDisposition {
    /// It was generated, and this is the output. Boxed because a disposition
    /// travels by value beside every plan, and the largest answer must not set
    /// the size of the five smaller ones.
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

//! The plan family: what the services decide before anything is rendered.
//!
//! # A family on a generic, never a mega-record
//!
//! One shared spine — [`ProjectionPlan`] — carries what every plan carries:
//! the shared exact identities, the complete declared output set, what
//! invalidates it, why it was decided that way, where it came from, and what it
//! does not claim. What differs by kind rides [`ProjectionKind::Content`], so a
//! new kind adds a content type rather than another optional seat on a record
//! everyone shares. The kind roster is sealed: a kind is admitted here or it
//! does not exist, because a kind the plane cannot explain is a kind the plane
//! must not plan.
//!
//! # Plan before render, and no partial output
//!
//! A plan states its complete membership up front. That is the output firewall:
//! the declared set is the whole set, and a sibling that is not in it was not
//! planned. Materializing a bundle is atomic at the publication boundary —
//! [`ProjectionBundlePlan`] names its members, and a partial materialization is
//! a refusal, never a partial success.
//!
//! # Absence is explained
//!
//! Where a projection was not generated, [`ProjectionDisposition`] says which
//! kind of absence it was and on whose fact. Silence is not one of the
//! variants, because silence is what the disposition exists to abolish.

use crate::origin_graph::{DecisionTrace, Nonclaim, OriginTrail};
use crate::plane::{
    AssumptionLimit, BundleMemberLimit, BundleSubject, ByteRoleSubject, DerivedTypeSubject,
    DocumentedSubject, ExactIdentity, FacetLimit, FixturePopulationSubject, GeneratedUnitSubject,
    GeneratorVersionSubject, ImplementedContractSubject, InvalidationLimit, MeasuredSubject,
    MechanismProfileSubject, MembershipLimit, NonclaimLimit, ObligationSubject, OutputBytesSubject,
    OwnerFactRef, PatternArgumentLimit, PatternArgumentSubject, PatternInstanceSubject,
    PatternSubject, PortSubject, ProfileVersion, ProjectionProfileSubject, SchemaSubject,
    SourceDeclarationLimit, WireContractSubject, WorkCurrencySubject, WorkFormulaSubject,
    WrapperComponentLimit,
};
use crate::question::ExplanationQuestion;
use crate::refusal::{
    BoundAxis, PlanIdentity, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue,
};
use core::fmt::Debug;
use threadpak::declaration::types::{
    FragmentIdentityDomain, LinkedGraphDomain, ProjectionAudienceDomain,
    ProjectionConfigurationDomain, ProjectionTargetDomain,
};
use threadpak::declaration::{DeclarationGraph, Facet};
use threadpak::evidence::{Method, VerifiedClaim};
use threadpak::types::{Bounded, ConstLimit, NonEmptyBounded};

// ---------------------------------------------------------------------------
// The shared context.
// ---------------------------------------------------------------------------

/// What a plan binds itself to at its target end.
///
/// Not an option: a target-free projection is a stated posture, not a missing
/// host contract, and the two must never read the same. A plan whose kind needs
/// a host and whose binding is target-free refuses rather than defaulting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetBinding {
    /// Bound to one named host contract.
    HostContract(ExactIdentity<ProjectionTargetDomain>),
    /// Deliberately bound to no host contract.
    TargetFree,
}

/// The source declarations one plan names as its cause.
pub type SourceDeclarations =
    NonEmptyBounded<ExactIdentity<FragmentIdentityDomain>, SourceDeclarationLimit>;

/// The triggers one plan watches.
pub type InvalidationSet = NonEmptyBounded<InvalidationTrigger, InvalidationLimit>;

/// The exact identities every plan shares, whatever its kind: which closed
/// graph, which profile at which version, which declarations caused it, which
/// version of the services produced it, and what it is bound to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionContext {
    /// The closed declaration graph this plan was decided against.
    pub graph: ExactIdentity<LinkedGraphDomain>,
    /// The projection profile selected.
    pub profile: ExactIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// The declarations that caused this plan — at least one, by shape.
    pub sources: SourceDeclarations,
    /// The version of the services that produced this plan.
    pub generator: ExactIdentity<GeneratorVersionSubject>,
    /// What the plan binds to at its target end.
    pub target: TargetBinding,
}

impl ProjectionContext {
    /// Read the machine's own closed-graph commitment into the plane. This is
    /// the production road: the services observe the identity the linker minted
    /// and never mint one of their own.
    #[must_use]
    pub fn graph_of(graph: &DeclarationGraph) -> ExactIdentity<LinkedGraphDomain> {
        ExactIdentity::of_commitment(graph.linked())
    }

    /// The one-declaration cause set. Total: one cause always fits.
    #[must_use]
    pub fn one_source(first: ExactIdentity<FragmentIdentityDomain>) -> SourceDeclarations {
        NonEmptyBounded::singleton(first)
    }

    /// Name several declarations as the cause of one plan.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Declarations`] when the
    /// cause set outgrows the declared bound. A partial cause set is refused,
    /// not trimmed: an explanation that names some of its causes is wrong about
    /// all of them.
    pub fn declared_sources(
        first: ExactIdentity<FragmentIdentityDomain>,
        rest: Vec<ExactIdentity<FragmentIdentityDomain>>,
    ) -> Result<SourceDeclarations, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest).map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                SourceDeclarationLimit::MAX,
                observed,
            )
        })
    }
}

// ---------------------------------------------------------------------------
// The output firewall.
// ---------------------------------------------------------------------------

/// One declared output of a plan: what it is, what bytes it will be, and where
/// it came from.
///
/// The origin seat is what makes a generated unit non-orphanable: there is no
/// output value in the plane that does not carry a trail back to authored
/// material.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutputIdentity {
    /// The generated unit's own identity.
    pub unit: ExactIdentity<GeneratedUnitSubject>,
    /// The canonical bytes that unit's rendering commits to.
    pub digest: ExactIdentity<OutputBytesSubject>,
    /// Where the unit came from. Structurally non-empty.
    pub origin: OriginTrail,
}

/// The complete declared output set of one plan — the output firewall.
///
/// Structurally non-empty: a plan that would generate nothing is not a plan,
/// it is a disposition. Bounded: a plan that would generate past the declared
/// magnitude refuses rather than materializing part of a set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedMembership {
    outputs: NonEmptyBounded<OutputIdentity, MembershipLimit>,
}

impl PlannedMembership {
    /// The one-output membership. Total: one output always fits.
    #[must_use]
    pub fn from_output(output: OutputIdentity) -> Self {
        Self {
            outputs: NonEmptyBounded::singleton(output),
        }
    }

    /// Declare the complete output set.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Outputs`] when the set
    /// outgrows the declared bound.
    pub fn declared(
        first: OutputIdentity,
        rest: Vec<OutputIdentity>,
    ) -> Result<Self, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest)
            .map(|outputs| Self { outputs })
            .map_err(|_| {
                ProjectionPlanning::bound_exceeded(
                    BoundAxis::Outputs,
                    MembershipLimit::MAX,
                    observed,
                )
            })
    }

    /// The guaranteed first output.
    #[must_use]
    pub fn first(&self) -> &OutputIdentity {
        self.outputs.first()
    }

    /// The number of outputs declared; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outputs.len()
    }

    /// Always `false`: a plan declaring no output is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Read the declared outputs, the guaranteed first one ahead of the rest.
    ///
    /// The order law applies and is not weakened here: a declared output SET is
    /// order-insensitive, so nothing identity-bearing may be derived from the
    /// order this yields. A plan identity computed over these outputs
    /// canonicalizes by an owner-declared order or key first, and testpak owes
    /// the permutation hostile — the same outputs supplied in another order
    /// must yield the same plan and the same output identities.
    pub fn iter(&self) -> impl Iterator<Item = &OutputIdentity> {
        self.outputs.iter()
    }
}

// ---------------------------------------------------------------------------
// Invalidation.
// ---------------------------------------------------------------------------

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
        watched: ExactIdentity<FragmentIdentityDomain>,
    },
    /// The closed graph this plan was decided against changed.
    GraphIdentityChanged {
        /// The watched graph.
        watched: ExactIdentity<LinkedGraphDomain>,
    },
    /// The projection profile changed.
    ProjectionProfileChanged {
        /// The watched profile.
        watched: ExactIdentity<ProjectionProfileSubject>,
    },
    /// The host contract this plan is bound to changed.
    TargetContractChanged {
        /// The watched contract.
        watched: ExactIdentity<ProjectionTargetDomain>,
    },
    /// The version of the services that produced this plan changed.
    GeneratorVersionChanged {
        /// The watched generator version.
        watched: ExactIdentity<GeneratorVersionSubject>,
    },
    /// An admitted mechanism profile changed.
    MechanismProfileChanged {
        /// The watched mechanism profile.
        watched: ExactIdentity<MechanismProfileSubject>,
    },
    /// A declared work formula changed.
    WorkFormulaChanged {
        /// The watched work formula.
        watched: ExactIdentity<WorkFormulaSubject>,
    },
    /// A fixture population a descriptor ranges over changed.
    FixturePopulationChanged {
        /// The watched population.
        watched: ExactIdentity<FixturePopulationSubject>,
    },
}

impl InvalidationTrigger {
    /// The one-trigger watch set. Total: one trigger always fits.
    #[must_use]
    pub fn one_watched(trigger: Self) -> InvalidationSet {
        NonEmptyBounded::singleton(trigger)
    }

    /// Watch several identities.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Declarations`] when the
    /// watch set outgrows the trigger roster's own cardinality — more triggers
    /// than there are kinds of trigger means one kind was stated twice.
    pub fn watched(first: Self, rest: Vec<Self>) -> Result<InvalidationSet, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest).map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                InvalidationLimit::MAX,
                observed,
            )
        })
    }
}

// ---------------------------------------------------------------------------
// The sealed kind roster and its contents.
// ---------------------------------------------------------------------------

/// The seal on the projection-kind roster.
///
/// A value of this type is producible only inside the services, so a kind
/// declared anywhere else cannot satisfy [`ProjectionKind`]. The roster is
/// closed because the explanation protocol is mandatory: a kind nobody can
/// explain is a kind that must not be planned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KindSeal(());

impl KindSeal {
    /// The seal, admitted only within the services.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
}

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

    /// The kind-specific facts a plan of this kind carries.
    type Content: Debug + Clone + PartialEq + Eq;

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
    pub schema: ExactIdentity<SchemaSubject>,
    /// The byte role the codec reads or writes.
    pub byte_role: ExactIdentity<ByteRoleSubject>,
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
    pub host_contract: ExactIdentity<ProjectionTargetDomain>,
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
    pub port: ExactIdentity<PortSubject>,
    /// The wire contract spoken.
    pub wire_contract: ExactIdentity<WireContractSubject>,
    /// Which way the surface faces.
    pub direction: SurfaceDirection,
}

/// A test descriptor's facts: which obligation it challenges, and by which
/// method. The method is the machine's own verification vocabulary — a compile
/// refusal is one challenge kind among several, never the universal one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestDescriptorContent {
    /// The obligation challenged.
    pub obligation: ExactIdentity<ObligationSubject>,
    /// The challenge method.
    pub challenge: Method,
}

/// A benchmark descriptor's facts: what is measured, in which work currency,
/// and which claim the envelope stands for. A benchmark is evidence about one
/// realization, never a specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchmarkDescriptorContent {
    /// The unit measured.
    pub measured: ExactIdentity<MeasuredSubject>,
    /// The named work currency the envelope is stated in.
    pub work_currency: ExactIdentity<WorkCurrencySubject>,
    /// The claim the envelope stands for.
    pub claim: VerifiedClaim,
}

/// A documentation projection's facts: what is documented, for which audience,
/// over which of the machine's semantic facets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentationContent {
    /// The subject documented.
    pub subject: ExactIdentity<DocumentedSubject>,
    /// The audience the projection is written for.
    pub audience: ExactIdentity<ProjectionAudienceDomain>,
    /// The facets covered.
    pub facets: Bounded<Facet, FacetLimit>,
}

/// An implementation projection's facts: which type, which contract, and which
/// owner facts the implementation assumes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeriveImplContent {
    /// The type the implementation is derived for.
    pub derived_type: ExactIdentity<DerivedTypeSubject>,
    /// The contract it realizes.
    pub contract: ExactIdentity<ImplementedContractSubject>,
    /// The owner facts assumed.
    pub assumptions: Bounded<OwnerFactRef, AssumptionLimit>,
}

/// A pattern stamp's facts: which authored pattern, which instantiation, and
/// the typed arguments supplied. A string never becomes an argument here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternStampContent {
    /// The authored pattern.
    pub pattern: ExactIdentity<PatternSubject>,
    /// This instantiation of it.
    pub instance: ExactIdentity<PatternInstanceSubject>,
    /// The typed arguments supplied.
    pub arguments: Bounded<ExactIdentity<PatternArgumentSubject>, PatternArgumentLimit>,
}

/// Declares one projection kind: a zero-sized marker plus its sealed
/// [`ProjectionKind`] implementation.
macro_rules! kinds {
    ($(
        $(#[$note:meta])*
        $name:ident => $content:ty, $requirement:expr, [$($question:expr),* $(,)?]
    );+ $(;)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl ProjectionKind for $name {
                const SEAL: KindSeal = KindSeal::admitted();
                type Content = $content;
                const KIND_QUESTIONS: &'static [ExplanationQuestion] = &[$($question),*];
                const TARGET_REQUIREMENT: TargetRequirement = $requirement;
            }
        )+
    };
}

kinds! {
    /// Projects a schema into the codec that reads and writes its canonical
    /// bytes.
    CodecProjection => CodecContent, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichAssumptionsAndSpecializations];

    /// Projects a declared surface into the wrapper one host contract needs.
    HostWrapperProjection => HostWrapperContent, TargetRequirement::BoundHostContract,
        [
            ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
            ExplanationQuestion::WhichRuntimeTracesCorrespond,
        ];

    /// Projects a port declaration into a remote surface over a wire contract.
    RemoteSurfaceProjection => RemoteSurfaceContent, TargetRequirement::BoundHostContract,
        [ExplanationQuestion::WhichRuntimeTracesCorrespond];

    /// Projects a declared obligation into the descriptor that challenges it.
    TestDescriptorProjection => TestDescriptorContent, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichTestsChallenge];

    /// Projects a declared work formula into the descriptor that measures it.
    BenchmarkDescriptorProjection => BenchmarkDescriptorContent,
        TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichBenchmarksMeasure];

    /// Projects declared meaning into prose for a named audience.
    DocumentationProjection => DocumentationContent, TargetRequirement::EitherBinding, [];

    /// Projects a declared contract into the implementation that realizes it.
    DeriveImplProjection => DeriveImplContent, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichAssumptionsAndSpecializations];

    /// Projects an authored pattern's instantiation into declaration material.
    PatternStampProjection => PatternStampContent, TargetRequirement::EitherBinding,
        [ExplanationQuestion::WhichTemplateOrPatternInstance];
}

// ---------------------------------------------------------------------------
// The plan itself.
// ---------------------------------------------------------------------------

/// One projection plan: the shared spine on the generic.
///
/// Every seat is required, and the seats that could be empty are shapes that
/// cannot be: the cause set, the output set, the watch set, the trace, and the
/// trail are all structurally non-empty. Only nonclaims may be empty, because a
/// plan that claims exactly what it does has none to state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionPlan<K: ProjectionKind> {
    context: ProjectionContext,
    content: K::Content,
    membership: PlannedMembership,
    invalidation: InvalidationSet,
    trace: DecisionTrace,
    origin: OriginTrail,
    nonclaims: Bounded<Nonclaim, NonclaimLimit>,
}

impl<K: ProjectionKind> ProjectionPlan<K> {
    /// Plan one projection.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`PlanSeat::TargetBinding`] when the
    /// kind's plans are meaningless without a host contract and the context is
    /// target-free. The binding is not defaulted: guessing a host is how a
    /// wrapper ends up bound to a contract nobody declared.
    pub fn planned(
        context: ProjectionContext,
        kind_content: K::Content,
        membership: PlannedMembership,
        invalidation: InvalidationSet,
        trace: DecisionTrace,
        origin: OriginTrail,
        nonclaims: Bounded<Nonclaim, NonclaimLimit>,
    ) -> Result<Self, ProjectionPlanning> {
        match (K::TARGET_REQUIREMENT, context.target) {
            (TargetRequirement::BoundHostContract, TargetBinding::TargetFree) => Err(
                ProjectionPlanning::established(ProjectionPlanningIssue::MissingOwnerFact {
                    seat: PlanSeat::TargetBinding,
                }),
            ),
            (TargetRequirement::BoundHostContract | TargetRequirement::EitherBinding, _) => {
                Ok(Self {
                    context,
                    content: kind_content,
                    membership,
                    invalidation,
                    trace,
                    origin,
                    nonclaims,
                })
            }
        }
    }

    /// The shared exact identities this plan was decided under.
    #[must_use]
    pub const fn context(&self) -> &ProjectionContext {
        &self.context
    }

    /// The kind-specific facts.
    #[must_use]
    pub const fn content(&self) -> &K::Content {
        &self.content
    }

    /// The complete declared output set.
    #[must_use]
    pub const fn membership(&self) -> &PlannedMembership {
        &self.membership
    }

    /// The identities whose change invalidates this plan.
    #[must_use]
    pub const fn invalidation(&self) -> &InvalidationSet {
        &self.invalidation
    }

    /// The decisions that produced this plan, in selection order.
    #[must_use]
    pub const fn trace(&self) -> &DecisionTrace {
        &self.trace
    }

    /// Where this plan itself came from.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// What this plan explicitly does not claim.
    #[must_use]
    pub const fn nonclaims(&self) -> &Bounded<Nonclaim, NonclaimLimit> {
        &self.nonclaims
    }

    /// The questions plans of this kind answer: the universal roster plus the
    /// kind's own. This is the set an explanation view must fill every seat of.
    #[must_use]
    pub fn applicable_questions() -> Vec<ExplanationQuestion> {
        UNIVERSAL_QUESTIONS
            .iter()
            .copied()
            .chain(K::KIND_QUESTIONS.iter().copied())
            .collect()
    }
}

/// A set of plans materialized as one unit across one publication boundary.
///
/// Atomicity is the boundary's law and this type is where it is stated: the
/// members are staged as a unit, checked as a unit, and published as a unit. A
/// partial materialization is a refusal, never a partial success — half a set
/// of sibling projections is a set whose siblings disagree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionBundlePlan {
    bundle: ExactIdentity<BundleSubject>,
    members: NonEmptyBounded<PlanIdentity, BundleMemberLimit>,
}

impl ProjectionBundlePlan {
    /// The one-member bundle. Total: one member always fits.
    #[must_use]
    pub fn of_one(bundle: ExactIdentity<BundleSubject>, member: PlanIdentity) -> Self {
        Self {
            bundle,
            members: NonEmptyBounded::singleton(member),
        }
    }

    /// Name the complete member set of one bundle.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Outputs`] when the member
    /// set outgrows the declared bound.
    pub fn materialized(
        bundle: ExactIdentity<BundleSubject>,
        first: PlanIdentity,
        rest: Vec<PlanIdentity>,
    ) -> Result<Self, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest)
            .map(|members| Self { bundle, members })
            .map_err(|_| {
                ProjectionPlanning::bound_exceeded(
                    BoundAxis::Outputs,
                    BundleMemberLimit::MAX,
                    observed,
                )
            })
    }

    /// The bundle's own identity.
    #[must_use]
    pub const fn bundle(&self) -> ExactIdentity<BundleSubject> {
        self.bundle
    }

    /// The number of member plans; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Always `false`: an empty bundle is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Dispositions.
// ---------------------------------------------------------------------------

/// What happened to one projection that could have been generated.
///
/// Every kind that could apply gets one of these. Silence is not a variant:
/// where a projection is absent, the absence has a name and, where a fact
/// caused it, a citation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionDisposition {
    /// It was generated, and this is the output.
    Generated {
        /// The generated unit.
        output: OutputIdentity,
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
        profile: ExactIdentity<ProjectionProfileSubject>,
        /// That profile's version.
        version: ProfileVersion,
    },
    /// Nobody asked for it.
    NotRequested,
    /// A configuration excluded it.
    ExcludedByConfiguration {
        /// The excluding configuration.
        configuration: ExactIdentity<ProjectionConfigurationDomain>,
    },
}

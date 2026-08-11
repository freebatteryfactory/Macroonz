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

use crate::explanation_protocol::ExplanationQuestion;
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

#[cfg(test)]
mod laws {
    use super::{
        BenchmarkDescriptorProjection, CodecProjection, DeriveImplContent, DeriveImplProjection,
        DocumentationProjection, HostWrapperContent, HostWrapperProjection, InvalidationTrigger,
        OutputIdentity, PatternStampProjection, PlannedMembership, ProjectionBundlePlan,
        ProjectionContext, ProjectionDisposition, ProjectionKind, ProjectionPlan,
        RemoteSurfaceProjection, TargetBinding, TargetRequirement, TestDescriptorProjection,
        UNIVERSAL_QUESTIONS, WrapperComponent,
    };
    use crate::explanation_protocol::{EXPLANATION_QUESTIONS, ExplanationQuestion};
    use crate::origin_graph::{
        DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
    };
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use crate::refusal::{PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
    use threadpak::types::{Bounded, NonEmptyBounded};

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([10; 32]),
            fact: ExactIdentity::decoded([11; 32]),
        }
    }

    /// One origin trail, for laws that need a generated unit.
    fn trail() -> OriginTrail {
        OriginTrail::from_edge(OriginEdge {
            from: ExactIdentity::decoded([12; 32]),
            relation: OriginRelation::SemanticDerivation,
            to: ExactIdentity::decoded([13; 32]),
        })
    }

    /// One declared output.
    fn output() -> OutputIdentity {
        OutputIdentity {
            unit: ExactIdentity::decoded([14; 32]),
            digest: ExactIdentity::decoded([15; 32]),
            origin: trail(),
        }
    }

    /// One shared context, under the binding the caller names.
    fn context(target: TargetBinding) -> ProjectionContext {
        ProjectionContext {
            graph: ExactIdentity::decoded([16; 32]),
            profile: ExactIdentity::decoded([17; 32]),
            profile_version: ProfileVersion::declared(3),
            sources: ProjectionContext::one_source(ExactIdentity::decoded([18; 32])),
            generator: ExactIdentity::decoded([19; 32]),
            target,
        }
    }

    /// The implementation-projection content, for the complete-plan law.
    fn derive_content() -> DeriveImplContent {
        DeriveImplContent {
            derived_type: ExactIdentity::decoded([20; 32]),
            contract: ExactIdentity::decoded([21; 32]),
            assumptions: Bounded::empty(),
        }
    }

    /// The trace the complete-plan law records.
    fn trace() -> DecisionTrace {
        DecisionTrace::from_entry(TraceEntry {
            subject: ExactIdentity::decoded([22; 32]),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        })
    }

    /// law: planning.a-complete-plan-constructs-through-checked-seams — every
    /// seat is furnished through the plane's own seams, and the resulting plan
    /// carries its cause set, output set, watch set, trace, and trail.
    /// Owed reversal (red twin): omitting any seat must not compile.
    #[test]
    fn a_complete_plan_constructs_through_checked_seams() {
        let planned = ProjectionPlan::<DeriveImplProjection>::planned(
            context(TargetBinding::TargetFree),
            derive_content(),
            PlannedMembership::from_output(output()),
            InvalidationTrigger::one_watched(InvalidationTrigger::GraphIdentityChanged {
                watched: ExactIdentity::decoded([16; 32]),
            }),
            trace(),
            trail(),
            Bounded::empty(),
        );
        assert!(planned.is_ok_and(|plan| {
            plan.membership().len() == 1
                && !plan.membership().is_empty()
                && plan.invalidation().len() == 1
                && plan.trace().len() == 1
                && plan.origin().len() == 1
                && plan.nonclaims().is_empty()
                && plan.context().profile_version.position() == 3
                && !plan.membership().first().origin.is_empty()
        }));
    }

    /// law: planning.several-outputs-and-nonclaims-ride-the-same-plan — a plan
    /// may declare several outputs and state what it does not claim, and both
    /// bounded seats hold what was put in them.
    /// Owed reversal: a membership seam that dropped a sibling must break this
    /// law.
    #[test]
    fn several_outputs_and_nonclaims_ride_the_same_plan() {
        let nonclaims = Bounded::admitted_const(vec![Nonclaim {
            unclaimed: ExactIdentity::decoded([23; 32]),
            because: owner_fact(),
        }])
        .map_err(|_| ());
        let membership = PlannedMembership::declared(output(), vec![output()]).map_err(|_| ());
        let built = nonclaims.and_then(|nonclaims| {
            membership.and_then(|membership| {
                ProjectionPlan::<DeriveImplProjection>::planned(
                    context(TargetBinding::TargetFree),
                    derive_content(),
                    membership,
                    InvalidationTrigger::one_watched(
                        InvalidationTrigger::GeneratorVersionChanged {
                            watched: ExactIdentity::decoded([19; 32]),
                        },
                    ),
                    trace(),
                    trail(),
                    nonclaims,
                )
                .map_err(|_| ())
            })
        });
        assert!(
            built.is_ok_and(|plan| plan.membership().len() == 2 && plan.nonclaims().len() == 1)
        );
    }

    /// law: planning.a-declared-output-set-reads-back-whole — the membership
    /// seam holds every sibling put into it and hands them all back on a
    /// read-only pass: two distinct outputs go in, two distinct outputs come
    /// out, and the membership is unconsumed — the second read sees the same
    /// set as the first.
    ///
    /// The order law this read carries: the declared output set is
    /// order-insensitive, so nothing identity-bearing is derived from the order
    /// observed here; identity-bearing generation canonicalizes by an
    /// owner-declared order or key first. testpak owes the permutation hostile.
    ///
    /// Owed reversal: a membership seam that dropped or aliased a sibling must
    /// break this law.
    #[test]
    fn a_declared_output_set_reads_back_whole() {
        let sibling = OutputIdentity {
            unit: ExactIdentity::decoded([31; 32]),
            digest: ExactIdentity::decoded([32; 32]),
            origin: trail(),
        };
        let membership = PlannedMembership::declared(output(), vec![sibling]);
        assert!(membership.is_ok_and(|membership| {
            let units: Vec<[u8; 32]> = membership.iter().map(|out| *out.unit.as_bytes()).collect();
            units == vec![[14_u8; 32], [31_u8; 32]]
                && membership.iter().count() == 2
                && membership.len() == 2
                && !membership.is_empty()
        }));
    }

    /// law: planning.a-host-bound-kind-refuses-a-target-free-context — a kind
    /// whose plans are meaningless without a host contract refuses rather than
    /// defaulting to one, and names the seat.
    /// Owed reversal: defaulting the binding must break this law.
    #[test]
    fn a_host_bound_kind_refuses_a_target_free_context() {
        assert!(matches!(
            HostWrapperProjection::TARGET_REQUIREMENT,
            TargetRequirement::BoundHostContract
        ));
        let refused = ProjectionPlan::<HostWrapperProjection>::planned(
            context(TargetBinding::TargetFree),
            HostWrapperContent {
                host_contract: ExactIdentity::decoded([24; 32]),
                components: NonEmptyBounded::singleton(WrapperComponent::Admission),
                capability_basis: owner_fact(),
            },
            PlannedMembership::from_output(output()),
            InvalidationTrigger::one_watched(InvalidationTrigger::TargetContractChanged {
                watched: ExactIdentity::decoded([24; 32]),
            }),
            trace(),
            trail(),
            Bounded::empty(),
        );
        assert!(refused.is_err_and(|planning| matches!(
            planning.issues.first(),
            ProjectionPlanningIssue::MissingOwnerFact {
                seat: PlanSeat::TargetBinding
            }
        )));
    }

    /// The closed trigger roster, proven closed by an exhaustive match.
    const fn trigger_index(trigger: &InvalidationTrigger) -> usize {
        match trigger {
            InvalidationTrigger::SourceDeclarationChanged { .. } => 0,
            InvalidationTrigger::GraphIdentityChanged { .. } => 1,
            InvalidationTrigger::ProjectionProfileChanged { .. } => 2,
            InvalidationTrigger::TargetContractChanged { .. } => 3,
            InvalidationTrigger::GeneratorVersionChanged { .. } => 4,
            InvalidationTrigger::MechanismProfileChanged { .. } => 5,
            InvalidationTrigger::WorkFormulaChanged { .. } => 6,
            InvalidationTrigger::FixturePopulationChanged { .. } => 7,
        }
    }

    /// law: planning.invalidation-triggers-are-eight-and-each-watches-an-identity
    /// — the roster is closed at eight, its members are pairwise distinct, and
    /// each names the exact identity whose change invalidates.
    /// Owed reversal: a payload-free trigger must break this law.
    #[test]
    fn invalidation_triggers_are_eight_and_each_watches_an_identity() {
        let triggers = [
            InvalidationTrigger::SourceDeclarationChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::GraphIdentityChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::ProjectionProfileChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::TargetContractChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::MechanismProfileChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::WorkFormulaChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::FixturePopulationChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
        ];
        assert_eq!(triggers.len(), 8);
        let indexes: Vec<usize> = triggers.iter().map(trigger_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// The closed disposition roster, proven closed by an exhaustive match.
    fn disposition_index(disposition: &ProjectionDisposition) -> usize {
        match disposition {
            ProjectionDisposition::Generated { .. } => 0,
            ProjectionDisposition::NotApplicable { .. } => 1,
            ProjectionDisposition::Refused { .. } => 2,
            ProjectionDisposition::UnavailableUnderProfile { .. } => 3,
            ProjectionDisposition::NotRequested => 4,
            ProjectionDisposition::ExcludedByConfiguration { .. } => 5,
        }
    }

    /// law: planning.every-absence-has-a-named-disposition — all six
    /// dispositions are constructible and pairwise distinct, and none of them
    /// is silence.
    /// Owed reversal: dropping a disposition must break this law.
    #[test]
    fn every_absence_has_a_named_disposition() {
        let dispositions = [
            ProjectionDisposition::Generated { output: output() },
            ProjectionDisposition::NotApplicable {
                because: owner_fact(),
            },
            ProjectionDisposition::Refused {
                refusal: ProjectionPlanning::established(
                    ProjectionPlanningIssue::MissingOwnerFact {
                        seat: PlanSeat::TargetBinding,
                    },
                ),
            },
            ProjectionDisposition::UnavailableUnderProfile {
                profile: ExactIdentity::decoded([26; 32]),
                version: ProfileVersion::declared(1),
            },
            ProjectionDisposition::NotRequested,
            ProjectionDisposition::ExcludedByConfiguration {
                configuration: ExactIdentity::decoded([27; 32]),
            },
        ];
        assert_eq!(dispositions.len(), 6);
        let indexes: Vec<usize> = dispositions.iter().map(disposition_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: planning.a-bundle-names-its-members-and-refuses-a-partial-set — a
    /// bundle holds at least one member by shape and refuses past its declared
    /// bound rather than publishing part of a set.
    /// Owed reversal (red twin): an empty bundle must not compile.
    #[test]
    fn a_bundle_names_its_members_and_refuses_a_partial_set() {
        let bundle = ProjectionBundlePlan::materialized(
            ExactIdentity::decoded([28; 32]),
            ExactIdentity::decoded([29; 32]),
            vec![ExactIdentity::decoded([30; 32])],
        );
        assert!(bundle.is_ok_and(|plan| plan.len() == 2 && !plan.is_empty()));
        let single = ProjectionBundlePlan::of_one(
            ExactIdentity::decoded([28; 32]),
            ExactIdentity::decoded([29; 32]),
        );
        assert_eq!(single.bundle(), ExactIdentity::decoded([28; 32]));
    }

    /// law: planning.no-kind-ducks-the-explanation-protocol — every kind names
    /// every universal question, states its own questions without repeating one,
    /// and the eight kinds together reach all fourteen questions.
    /// Owed reversal: a kind declaring an empty applicable set must break this
    /// law.
    #[test]
    fn no_kind_ducks_the_explanation_protocol() {
        let rosters: [Vec<ExplanationQuestion>; 8] = [
            ProjectionPlan::<CodecProjection>::applicable_questions(),
            ProjectionPlan::<HostWrapperProjection>::applicable_questions(),
            ProjectionPlan::<RemoteSurfaceProjection>::applicable_questions(),
            ProjectionPlan::<TestDescriptorProjection>::applicable_questions(),
            ProjectionPlan::<BenchmarkDescriptorProjection>::applicable_questions(),
            ProjectionPlan::<DocumentationProjection>::applicable_questions(),
            ProjectionPlan::<DeriveImplProjection>::applicable_questions(),
            ProjectionPlan::<PatternStampProjection>::applicable_questions(),
        ];
        for roster in &rosters {
            assert!(
                UNIVERSAL_QUESTIONS
                    .iter()
                    .all(|question| roster.contains(question))
            );
            assert!(roster.iter().enumerate().all(|(position, question)| {
                roster
                    .iter()
                    .skip(position.saturating_add(1))
                    .all(|other| other != question)
            }));
        }
        assert!(
            EXPLANATION_QUESTIONS
                .iter()
                .all(|question| rosters.iter().any(|roster| roster.contains(question)))
        );
    }
}

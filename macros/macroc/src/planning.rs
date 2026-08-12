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
    AssumptionLimit, BundleMemberLimit, BundleSubject, ByteRoleSubject, CapturedDeclarationSubject,
    DerivedTypeSubject, DocumentedSubject, FacetLimit, FixturePopulationSubject,
    GeneratedUnitSubject, GeneratorVersionSubject, ImplementedContractSubject, InvalidationLimit,
    MeasuredSubject, MechanismProfileSubject, MembershipLimit, NonclaimLimit, ObligationSubject,
    OwnerFactRef, OwnerIdentityRef, PatternArgumentLimit, PatternArgumentSubject,
    PatternInstanceSubject, PatternSubject, PlanId, PortSubject, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, ProjectionProvenance, ProjectionRole,
    ProjectionTranscript, RenderedRole, RenderedRoleSeal, SchemaSubject, SoleRenderedUnit,
    SourceDeclarationLimit, TranscriptAnchoring, WireContractSubject, WorkCurrencySubject,
    WorkFormulaSubject, WrapperComponentLimit, encode_bytes, encode_length,
};
use crate::question::ExplanationQuestion;
use crate::refusal::{BoundAxis, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
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
    HostContract(OwnerIdentityRef<ProjectionTargetDomain>),
    /// Deliberately bound to no host contract.
    TargetFree,
}

impl TargetBinding {
    /// Append this binding's canonical bytes: the posture's discriminant, then
    /// the contract where one is named. Target-free is written as a posture and
    /// never as an absent contract, exactly as the type states it.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::HostContract(contract) => {
                into.push(0);
                encode_bytes(contract.as_bytes(), into);
            }
            Self::TargetFree => {
                into.push(1);
                encode_bytes(&[], into);
            }
        }
    }
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

impl GraphAnchoring {
    /// Append this anchoring's canonical bytes: the posture's discriminant, then
    /// the identity it names.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::ClosedGraph(graph) => {
                into.push(0);
                encode_bytes(graph.as_bytes(), into);
            }
            Self::CapturedDeclarationOnly(captured) => {
                into.push(1);
                encode_bytes(captured.as_bytes(), into);
            }
        }
    }
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

impl CauseAnchoring {
    /// Append this cause's canonical bytes: the posture's discriminant, then
    /// every declaration it names, in the order the cause set was declared.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::Declarations(sources) => {
                into.push(0);
                encode_length(sources.len(), into);
                for source in sources.iter() {
                    encode_bytes(source.as_bytes(), into);
                }
            }
            Self::CapturedDeclaration(captured) => {
                into.push(1);
                encode_length(1, into);
                encode_bytes(captured.as_bytes(), into);
            }
        }
    }

    /// What a transcript derived under this cause is anchored to.
    ///
    /// A plan hangs off what caused it: the captured declaration where the cause
    /// IS the capture, and the first declared fragment where a caller holds the
    /// machine's own identities. The remaining fragments are inside the
    /// transcript's content rather than at its anchor, because an anchor names
    /// one thing.
    #[must_use]
    pub fn anchoring(&self) -> TranscriptAnchoring {
        match self {
            Self::Declarations(sources) => {
                TranscriptAnchoring::UnderOwnerIdentity(*sources.first().as_bytes())
            }
            Self::CapturedDeclaration(captured) => {
                TranscriptAnchoring::UnderProjectionIdentity(*captured.as_bytes())
            }
        }
    }
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

impl ProjectionContext {
    /// Read the machine's own closed-graph commitment into the plane. This is
    /// the production road for a caller that HOLDS a closed graph: the services
    /// observe the identity the linker minted and never mint one of their own.
    #[must_use]
    pub fn graph_of(graph: &DeclarationGraph) -> GraphAnchoring {
        GraphAnchoring::ClosedGraph(OwnerIdentityRef::of_commitment(graph.linked()))
    }

    /// The one-declaration cause set. Total: one cause always fits.
    #[must_use]
    pub fn one_source(first: OwnerIdentityRef<FragmentIdentityDomain>) -> SourceDeclarations {
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
        first: OwnerIdentityRef<FragmentIdentityDomain>,
        rest: Vec<OwnerIdentityRef<FragmentIdentityDomain>>,
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

    /// The invalidation trigger that watches whatever this context was caused
    /// by — the fragment where a caller holds one, and the captured declaration
    /// where the cause IS the capture.
    #[must_use]
    pub fn cause_trigger(&self) -> InvalidationTrigger {
        match &self.sources {
            CauseAnchoring::Declarations(sources) => {
                InvalidationTrigger::SourceDeclarationChanged {
                    watched: *sources.first(),
                }
            }
            CauseAnchoring::CapturedDeclaration(captured) => {
                InvalidationTrigger::CapturedDeclarationChanged { watched: *captured }
            }
        }
    }

    /// The invalidation trigger that watches whatever this context was decided
    /// against.
    #[must_use]
    pub const fn graph_trigger(&self) -> InvalidationTrigger {
        match self.graph {
            GraphAnchoring::ClosedGraph(graph) => {
                InvalidationTrigger::GraphIdentityChanged { watched: graph }
            }
            GraphAnchoring::CapturedDeclarationOnly(captured) => {
                InvalidationTrigger::CapturedDeclarationChanged { watched: captured }
            }
        }
    }

    /// Append this context's canonical bytes: what it was decided against, the
    /// profile and its version, what caused it, the generator identity, and the
    /// target binding, each at full width and in that order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        self.graph.encode_into(into);
        encode_bytes(self.profile.as_bytes(), into);
        into.extend_from_slice(&self.profile_version.position().to_be_bytes());
        self.sources.encode_into(into);
        encode_bytes(self.generator.as_bytes(), into);
        self.target.encode_into(into);
    }
}

// ---------------------------------------------------------------------------
// The output firewall.
// ---------------------------------------------------------------------------

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

impl MemberDestination {
    /// Append this destination's canonical bytes: the discriminant, then the
    /// byte role where one is named.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::AtDeclarationSite => {
                into.push(0);
                encode_bytes(&[], into);
            }
            Self::AsArtifact { byte_role } => {
                into.push(1);
                encode_bytes(byte_role.as_bytes(), into);
            }
        }
    }
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

impl DigestContract {
    /// The contract binding one member's digest to that member.
    #[must_use]
    pub const fn over(anchored_to: ProjectionIdentity<GeneratedUnitSubject>) -> Self {
        Self {
            role: ProjectionRole::OutputBytes,
            anchored_to,
        }
    }

    /// Append this contract's canonical bytes: the role slot the digest will
    /// carry, then the member identity it must be anchored to.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.role.slot());
        encode_bytes(self.anchored_to.as_bytes(), into);
    }
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

impl PlannedOutput {
    /// Append this output's canonical bytes: the semantic key, the destination,
    /// the origin trail in walk order, the expected profile and its version, and
    /// the digest contract — everything a plan states about one member, and no
    /// rendered byte, because a plan has none.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.semantic_key.as_bytes(), into);
        self.destination.encode_into(into);
        self.origin.encode_into(into);
        encode_bytes(self.expected_profile.as_bytes(), into);
        into.extend_from_slice(&self.expected_profile_version.position().to_be_bytes());
        self.digest_contract.encode_into(into);
    }
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

impl<R: RenderedRole> PlannedMember<R> {
    /// Append this member's canonical bytes: the rendered role's slot, then the
    /// logical output.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.role.slot().to_be_bytes());
        self.output.encode_into(into);
    }
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

impl<R: RenderedRole> PlannedMembership<R> {
    /// The one-member membership. Total: one member always fits.
    #[must_use]
    pub fn from_member(member: PlannedMember<R>) -> Self {
        Self {
            members: NonEmptyBounded::singleton(member),
        }
    }

    /// The complete output set of a kind whose roster is fixed by its own shape
    /// — a *total structural* constructor.
    ///
    /// # Why the complete set has a road with no refusal on it
    ///
    /// Some kinds decide their membership at runtime and declare it through
    /// [`PlannedMembership::declared`], which reads a count and may refuse.
    /// Others do not: a shape that fixes exactly which roles it materializes
    /// knows the whole set before anything runs, and there is no count to read.
    ///
    /// The arity is `N`, a compile-time constant, so the bound is settled by
    /// const evaluation and this road returns no `Result`. That is the whole
    /// point. The seam that stood here handed such a caller a `Result` it could
    /// not fail, and the caller — having no honest value for a case that cannot
    /// happen — repaired it with a ONE-MEMBER membership. A complete set that
    /// silently became one member is a plan that declared a smaller output set
    /// than the shape fixed, and the closure check downstream then proved the
    /// smaller claim.
    ///
    /// # What this road does NOT settle
    ///
    /// It settles the magnitude, not the distinctness of the roles: nothing here
    /// stops a caller passing one role twice. That is deliberate rather than
    /// overlooked. A caller of this road names its roles literally, so a doubled
    /// role is visible at the call site, and the closure check reads the PLAN's
    /// own count per role independently and refuses before anything is emitted.
    /// The checked road, whose roles arrive at runtime, refuses a doubled role
    /// itself.
    #[must_use]
    pub fn complete<const N: usize>(first: PlannedMember<R>, rest: [PlannedMember<R>; N]) -> Self {
        Self {
            members: NonEmptyBounded::from_array(first, rest),
        }
    }

    /// Declare the complete output set.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Outputs`] when the set
    /// outgrows the declared bound, and naming
    /// [`ProjectionPlanningIssue::MembershipDoubled`] for every role two members
    /// stand under. The second check is here rather than downstream because a
    /// doubled role is a defect in the DECLARATION of the set: the closure check
    /// matches by role, so a membership that reaches it doubled has already made
    /// that match elect one member and ignore the other.
    pub fn declared(
        first: PlannedMember<R>,
        rest: Vec<PlannedMember<R>>,
    ) -> Result<Self, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        let members = NonEmptyBounded::admitted_const(first, rest).map_err(|_| {
            ProjectionPlanning::bound_exceeded(BoundAxis::Outputs, MembershipLimit::MAX, observed)
        })?;
        let declared = Self { members };
        let mut doubled: Vec<ProjectionPlanningIssue> = Vec::new();
        for role in R::ROLES {
            let count = declared.count_under(*role);
            if count > 1 {
                doubled.push(ProjectionPlanningIssue::MembershipDoubled {
                    role_slot: role.slot(),
                    observed: u32::try_from(count).unwrap_or(u32::MAX),
                });
            }
        }
        let mut established = doubled.into_iter();
        match established.next() {
            Some(issue) => Err(ProjectionPlanning::co_established(
                issue,
                established.collect(),
            )),
            None => Ok(declared),
        }
    }

    /// The guaranteed first member.
    #[must_use]
    pub fn first(&self) -> &PlannedMember<R> {
        self.members.first()
    }

    /// The member planned under one role, where one is.
    #[must_use]
    pub fn under(&self, role: R) -> Option<&PlannedMember<R>> {
        self.members.iter().find(|member| member.role == role)
    }

    /// How many members are planned under one role. Two is a defect
    /// [`PlannedMembership::declared`] refuses; the membership itself never
    /// elects one of them.
    #[must_use]
    pub fn count_under(&self, role: R) -> usize {
        self.members_under(role).count()
    }

    /// Every member planned under one role, in declaration order.
    ///
    /// The road a COMPLETE-SET comparison walks. Comparing two memberships by
    /// their first member per role would agree about two sets that differ in
    /// their second, which is exactly what a doubled role produces.
    pub fn members_under(&self, role: R) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members
            .iter()
            .filter(move |member| member.role == role)
    }

    /// Whether two memberships name the same members under one role, as sets:
    /// the same count, member for member.
    #[must_use]
    pub fn agrees_under(&self, other: &Self, role: R) -> bool {
        let mine: Vec<&PlannedMember<R>> = self.members_under(role).collect();
        let theirs: Vec<&PlannedMember<R>> = other.members_under(role).collect();
        mine == theirs
    }

    /// The number of members declared; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Always `false`: a plan declaring no output is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Read the declared members, the guaranteed first one ahead of the rest.
    ///
    /// The order law applies and is not weakened here: a declared output SET is
    /// order-insensitive, so nothing identity-bearing may be derived from the
    /// order this yields. Every member identity is derived from its ROLE and its
    /// anchor, never from its position in this iteration, so the same members
    /// supplied in another order yield the same plan.
    pub fn iter(&self) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members.iter()
    }

    /// Append this membership's canonical bytes, in the kind's declared ROLE
    /// ROSTER order.
    ///
    /// Roster order and never declaration order: a declared output set is
    /// order-insensitive, so the same members supplied in another order must
    /// encode identically. Every member standing under a role is written, not
    /// just the first, so a membership that doubled a role encodes differently
    /// from one that did not — the closure check reports that as a defect, and
    /// the encoding must not hide it before the check runs.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(R::ROLES.len(), into);
        for role in R::ROLES {
            into.extend_from_slice(&role.slot().to_be_bytes());
            let under: Vec<&PlannedMember<R>> = self
                .members
                .iter()
                .filter(|member| member.role == *role)
                .collect();
            encode_length(under.len(), into);
            for member in under {
                member.encode_into(into);
            }
        }
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

impl InvalidationTrigger {
    /// The trigger kind's discriminant byte, written ahead of the identity it
    /// watches so two kinds watching the same bytes never encode alike.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::SourceDeclarationChanged { .. } => 0,
            Self::CapturedDeclarationChanged { .. } => 1,
            Self::GraphIdentityChanged { .. } => 2,
            Self::ProjectionProfileChanged { .. } => 3,
            Self::TargetContractChanged { .. } => 4,
            Self::GeneratorVersionChanged { .. } => 5,
            Self::MechanismProfileChanged { .. } => 6,
            Self::WorkFormulaChanged { .. } => 7,
            Self::FixturePopulationChanged { .. } => 8,
        }
    }

    /// Append this trigger's canonical bytes: the kind, then the watched
    /// identity at full width.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let watched: &[u8; 32] = match self {
            Self::SourceDeclarationChanged { watched } => watched.as_bytes(),
            Self::CapturedDeclarationChanged { watched } => watched.as_bytes(),
            Self::GraphIdentityChanged { watched } => watched.as_bytes(),
            Self::ProjectionProfileChanged { watched } => watched.as_bytes(),
            Self::TargetContractChanged { watched } => watched.as_bytes(),
            Self::GeneratorVersionChanged { watched } => watched.as_bytes(),
            Self::MechanismProfileChanged { watched } => watched.as_bytes(),
            Self::WorkFormulaChanged { watched } => watched.as_bytes(),
            Self::FixturePopulationChanged { watched } => watched.as_bytes(),
        };
        encode_bytes(watched, into);
    }

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

impl RenderedRole for RenderedImplementation {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[Self::RenderedFamilyImpl, Self::RenderedCauseOrderImpl];

    fn slot(self) -> u32 {
        match self {
            Self::RenderedFamilyImpl => 0,
            Self::RenderedCauseOrderImpl => 1,
        }
    }

    fn described(self) -> &'static str {
        match self {
            Self::RenderedFamilyImpl => "the family contract's implementation",
            Self::RenderedCauseOrderImpl => "the typed cause order's implementation",
        }
    }
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

// ---------------------------------------------------------------------------
// The plan itself.
// ---------------------------------------------------------------------------

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

impl PlanDerivation {
    /// The plan's identity.
    #[must_use]
    pub const fn identity(&self) -> PlanId {
        self.identity
    }

    /// The record of how that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        &self.provenance
    }
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

/// Append one declared SET's canonical bytes: every member encoded, the
/// encodings sorted, the sorted sequence written length-prefixed.
///
/// Sorting the ENCODINGS rather than the members is what lets a set be
/// canonicalized without an `Ord` the plane refuses to declare: the plane ranks
/// nothing, and a byte order over finished encodings is not a ranking of the
/// values — it is a spelling rule for a collection whose order carries no
/// meaning.
fn encode_set<'member, T: 'member, Encode>(
    members: impl Iterator<Item = &'member T>,
    encode: Encode,
    into: &mut Vec<u8>,
) where
    Encode: Fn(&T, &mut Vec<u8>),
{
    let mut encoded: Vec<Vec<u8>> = members
        .map(|member| {
            let mut bytes = Vec::new();
            encode(member, &mut bytes);
            bytes
        })
        .collect();
    encoded.sort_unstable();
    encode_length(encoded.len(), into);
    for member in &encoded {
        encode_bytes(member, into);
    }
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
        membership: PlannedMembership<K::Rendered>,
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
                let mut claim = Vec::new();
                encode_bytes(K::KIND_NAME.as_bytes(), &mut claim);
                context.encode_into(&mut claim);
                membership.encode_into(&mut claim);
                encode_set(
                    invalidation.iter(),
                    InvalidationTrigger::encode_into,
                    &mut claim,
                );
                trace.encode_into(&mut claim);
                origin.encode_into(&mut claim);
                encode_set(nonclaims.iter(), Nonclaim::encode_into, &mut claim);
                let (identity, provenance) =
                    PlanId::derived_with_provenance(ProjectionTranscript::under(
                        ProjectionRole::Plan,
                        context.sources.anchoring(),
                        &claim,
                        0,
                    ));
                Ok(Self {
                    derivation: PlanDerivation {
                        identity,
                        provenance,
                    },
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

    /// This plan's own identity.
    #[must_use]
    pub const fn identity(&self) -> PlanId {
        self.derivation.identity()
    }

    /// How this plan's identity was derived — the record lives here, once,
    /// rather than inside the identity value it explains.
    #[must_use]
    pub const fn derivation(&self) -> &PlanDerivation {
        &self.derivation
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
    pub const fn membership(&self) -> &PlannedMembership<K::Rendered> {
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
    bundle: ProjectionIdentity<BundleSubject>,
    members: NonEmptyBounded<PlanId, BundleMemberLimit>,
}

impl ProjectionBundlePlan {
    /// The one-member bundle. Total: one member always fits.
    #[must_use]
    pub fn of_one(bundle: ProjectionIdentity<BundleSubject>, member: PlanId) -> Self {
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
        bundle: ProjectionIdentity<BundleSubject>,
        first: PlanId,
        rest: Vec<PlanId>,
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
    pub const fn bundle(&self) -> ProjectionIdentity<BundleSubject> {
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

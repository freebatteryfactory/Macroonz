//! The plan family's invariant nucleus: every road that reaches a private field,
//! and every smart constructor that settles a declared magnitude.
//!
//! Declared inside `types.rs` as its own child.
//! The output firewall is here: a membership's members are unreachable except
//! through the roads below, so a plan's declared output set is whatever one of
//! them admitted and nothing else.
//! The entry account is here for the same reason: its addressing is unreachable
//! except through the roads below, so the ONE account of owner content is
//! whatever walked in one of these doors, and the four readings all read it.
//! The total structural constructors — [`PlannedMembership::complete`] and the
//! one-member roads — carry no refusal at all, because a set fixed by a shape has
//! no runtime count to read and therefore no error branch for a caller to fill
//! with a shorter set.
//!
//! The services' expectation of the generated-support schema identity is here
//! too, and it has NO public constructor: the only value of that type anybody
//! outside this crate can reach is the checked-in constant, which is exactly the
//! independence the two-sided pin is made of.

use super::super::encode::encode_set;
use super::{
    BundleMemberLimit, CapturedDependencies, CauseAnchoring, ContentAddressing, DeclaredBootstrap,
    DigestContract, EmissionPartition, ExpectedGeneratedSupportSchemaId, GraphAnchoring,
    InvalidationLimit, InvalidationSet, InvalidationTrigger, KindSeal, OwnerContentAccount,
    PlanDecisions, PlanDerivation, PlannedMember, PlannedMembership, ProjectionBundlePlan,
    ProjectionContext, ProjectionIntentId, ProjectionKind, ProjectionPlan, SourceDeclarationLimit,
    SourceDeclarations, TargetBinding, TargetRequirement, UNIVERSAL_QUESTIONS,
};
use crate::origin_graph::{DecisionTrace, Nonclaim, OriginTrail};
use crate::plane::{
    AuthoringLimitProfile, BundleSubject, CapturedDeclarationSubject, GeneratedUnitSubject,
    MembershipLimit, NonclaimLimit, OwnerIdentityRef, PlanId, ProjectionIdentity,
    ProjectionProvenance, ProjectionRole, ProjectionTranscript, RenderedRole, encode_length,
};
use crate::question::ExplanationQuestion;
use crate::refusal::{BoundAxis, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
use core::marker::PhantomData;
use threadpak::declaration::DeclarationGraph;
use threadpak::declaration::types::FragmentIdentityDomain;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

impl ProjectionContext {
    /// Read the machine's own closed-graph commitment into the plane.
    ///
    /// The road for a caller that HOLDS a closed graph: the services observe the
    /// identity the linker minted and never mint one of their own.
    #[must_use]
    pub fn graph_of(graph: &DeclarationGraph) -> GraphAnchoring {
        GraphAnchoring::ClosedGraph(OwnerIdentityRef::of_commitment(graph.linked()))
    }
}

impl ContentAddressing {
    /// The ONE address this addressing names, whichever posture it stands under.
    ///
    /// The one place the posture is turned into an address, so the account's
    /// reading, the plan's anchor, and the canonical encoding all take the same
    /// answer from the same road rather than three matches that agree until one
    /// of them is edited.
    #[must_use]
    pub const fn commitment(&self) -> CauseAnchoring {
        match self {
            Self::Linked { commitment, .. } => CauseAnchoring::Declaration(*commitment),
            Self::Captured { commitment, .. } => CauseAnchoring::CapturedDeclaration(*commitment),
        }
    }

    /// How many commitments this addressing declares its content stands on.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        match self {
            Self::Linked { dependencies, .. } => dependencies.len(),
            Self::Captured { dependencies, .. } => dependencies.len(),
        }
    }
}

impl<K: ProjectionKind> OwnerContentAccount<K> {
    /// The linked account of content that stands on nothing.
    ///
    /// Total: the empty dependency set fits every declared magnitude, so there
    /// is no count to read and no refusal to return.
    /// Content that stands on nothing is a stated fact and not an absence — the
    /// account is required either way, and a caller with no dependencies still
    /// walks in one door rather than skipping the door.
    #[must_use]
    pub fn linked(commitment: OwnerIdentityRef<FragmentIdentityDomain>) -> Self {
        Self {
            addressing: ContentAddressing::Linked {
                commitment,
                dependencies: Bounded::empty(),
            },
            kind: PhantomData,
        }
    }

    /// The linked account, over the dependency set the OWNER declared.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Declarations`] when the
    /// declared dependency set outgrows the source-declaration magnitude.
    /// A partial dependency set is refused, not trimmed: an account that names
    /// some of what its content stands on is wrong about all of it, and every
    /// one of the four readings would then be reading a shorter world.
    pub fn linked_over(
        commitment: OwnerIdentityRef<FragmentIdentityDomain>,
        dependencies: Vec<OwnerIdentityRef<FragmentIdentityDomain>>,
    ) -> Result<Self, ProjectionPlanning> {
        let observed = dependencies.len();
        let dependencies: SourceDeclarations = Bounded::admitted_const(
            dependencies,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                SourceDeclarationLimit::MAX,
                observed,
            )
        })?;
        Ok(Self {
            addressing: ContentAddressing::Linked {
                commitment,
                dependencies,
            },
            kind: PhantomData,
        })
    }

    /// The expansion-time account of captured content that stands on nothing.
    ///
    /// Total, on the same terms as [`OwnerContentAccount::linked`].
    #[must_use]
    pub fn captured(commitment: ProjectionIdentity<CapturedDeclarationSubject>) -> Self {
        Self {
            addressing: ContentAddressing::Captured {
                commitment,
                dependencies: Bounded::empty(),
            },
            kind: PhantomData,
        }
    }

    /// The expansion-time account, over the captures the owner declared it
    /// stands on.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`BoundAxis::Declarations`] when the
    /// declared dependency set outgrows the source-declaration magnitude, on the
    /// same terms as [`OwnerContentAccount::linked_over`].
    pub fn captured_over(
        commitment: ProjectionIdentity<CapturedDeclarationSubject>,
        dependencies: Vec<ProjectionIdentity<CapturedDeclarationSubject>>,
    ) -> Result<Self, ProjectionPlanning> {
        let observed = dependencies.len();
        let dependencies: CapturedDependencies = Bounded::admitted_const(
            dependencies,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                SourceDeclarationLimit::MAX,
                observed,
            )
        })?;
        Ok(Self {
            addressing: ContentAddressing::Captured {
                commitment,
                dependencies,
            },
            kind: PhantomData,
        })
    }

    /// What this content is addressed by, and what it declares it stands on.
    ///
    /// The one road to both seats, borrowed: the account is read by everything
    /// and rewritten by nothing.
    #[must_use]
    pub const fn addressing(&self) -> &ContentAddressing {
        &self.addressing
    }

    /// The ONE address the owner supplied at the door.
    ///
    /// The reading a plan's anchor, its explanation's causing-declaration
    /// answer, and its cause trigger are all taken from — one value, read three
    /// times, rather than three seats that could disagree.
    #[must_use]
    pub const fn commitment(&self) -> CauseAnchoring {
        self.addressing.commitment()
    }

    /// How many commitments this content declares it stands on.
    ///
    /// Read by the watch derivation, which compares it against what the trigger
    /// roster can represent and refuses where the two disagree.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.addressing.dependency_count()
    }

    /// How many commitments a watch set over this account would have to cover:
    /// the content's own, plus every commitment it stands on.
    ///
    /// One number, so the derivation that compares it against the roster's
    /// capacity and the refusal that reports it cannot disagree about what was
    /// counted.
    #[must_use]
    pub fn watched_commitment_count(&self) -> usize {
        self.dependency_count().saturating_add(1)
    }

    /// What was MEANT: the kind and the commitment, derived into the intent
    /// layer's own identity.
    ///
    /// # The intent transcript
    ///
    /// This is a mint site, so its content grammar is stated here in full, the
    /// way [`crate::plane::ProjectionTranscript`] requires of every mint site.
    /// The identity is derived under the identity subject `projection-intent`
    /// at role [`ProjectionRole::ProjectionIntent`], ROOTED, at roster position
    /// zero, over exactly the bytes [`OwnerContentAccount::intent_bytes`] hands
    /// back:
    ///
    /// ```text
    /// content = bytes(kind_name) || posture_byte || bytes(commitment)
    /// ```
    ///
    /// where `kind_name` is [`ProjectionKind::KIND_NAME`], and the posture byte
    /// and the commitment at its full thirty-two bytes are
    /// [`CauseAnchoring`]'s own canonical spelling — so a linked commitment and
    /// a captured one never derive alike even where their bytes coincide.
    /// An independent reader holding the pair and this paragraph re-derives
    /// these thirty-two bytes and needs nothing else.
    ///
    /// The role reads to the intent's own preimage family, so the version in
    /// that context is the kind-and-commitment grammar's own. Two accounts that
    /// meant the same thing therefore keep deriving one identity across every
    /// change to the machinery that would realize it — a rendered token roster,
    /// a delivery, a generator's rendered shape — because none of those is a
    /// member of this preimage or a segment of this context.
    ///
    /// # Ordering
    ///
    /// Rooted, and deliberately so.
    /// The preimage already carries the commitment at full width, so anchoring
    /// the derivation on that same commitment would write it twice into one
    /// derivation — a double entry that separates nothing, since two intents
    /// over one commitment are already separated by the kind name inside the
    /// content.
    ///
    /// # Bounds
    ///
    /// Not `const`: the identity is a BLAKE3 derivation, which is not a const
    /// evaluation. The road this replaced carried the pair by value and could
    /// be; what a reader is handed instead is thirty-two bytes that stand
    /// wherever a derived identity is required.
    #[must_use]
    pub fn intent(&self) -> ProjectionIntentId {
        ProjectionIntentId::derived_over(&self.intent_bytes())
    }
}

impl ProjectionIntentId {
    /// The intent identity over one canonical intent preimage.
    ///
    /// Private, and the one road to a value of this type: an intent is a fact
    /// about an ACCOUNT, so [`OwnerContentAccount::intent`] is where it is
    /// derived, and a caller holding thirty-two bytes of its own has no road
    /// that turns them into one.
    fn derived_over(preimage: &[u8]) -> Self {
        Self {
            identity: ProjectionIdentity::derived(ProjectionTranscript::rooted(
                ProjectionRole::ProjectionIntent,
                preimage,
                0,
            )),
        }
    }

    /// The intent identity's thirty-two bytes, borrowed for comparison and for
    /// rendering.
    ///
    /// This is the whole public surface, and it replaced the pair's two
    /// readers: a digest cannot hand back the kind name or the commitment it
    /// committed to, and a road that claimed to would be reading a preimage the
    /// value does not carry.
    /// A caller that needs the pair itself reads the ACCOUNT, which holds it.
    ///
    /// One-way by the absence of its inverse, exactly as the plane's own
    /// identities are: no road anywhere takes bytes and returns an intent
    /// identity.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        self.identity.as_bytes()
    }
}

impl<Posture> ExpectedGeneratedSupportSchemaId<Posture> {
    /// The expectation's thirty-two bytes, borrowed for the one lawful use: the
    /// generated support shell splices them into the tokens it carries across
    /// the wall, and the harness's own gate compares them against its published
    /// literal before releasing anything into type checking.
    ///
    /// One-way by the absence of its inverse: no road anywhere takes bytes and
    /// returns an expectation, so a supplied identity can never become this
    /// side's expectation of it.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl ExpectedGeneratedSupportSchemaId<DeclaredBootstrap> {
    /// The hand-authored first pair's road, crate-internal and const.
    ///
    /// Crate-internal because the value it makes is a CLAIM about a schema this
    /// crate does not own: one checked-in constant states it, the publication
    /// operation rewrites that constant under a receipt when the schema changes,
    /// and a caller that could mint another would be making the same claim
    /// somewhere nobody publishes.
    pub(crate) const fn declared(bytes: [u8; 32]) -> Self {
        Self {
            bytes,
            _posture: PhantomData,
        }
    }
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
}

impl<R: RenderedRole> PlannedMembership<R> {
    /// The one-member membership. Total: one member always fits.
    #[must_use]
    pub fn from_member(member: PlannedMember<R>) -> Self {
        Self {
            members: NonEmptyBounded::singleton(member),
        }
    }

    /// The complete output set of a kind whose roster is fixed by its own shape —
    /// a *total structural* constructor.
    ///
    /// A kind that decides its membership at runtime declares it through
    /// [`PlannedMembership::declared`], which reads a count and may refuse.
    /// A shape that fixes exactly which roles it materializes knows the whole set
    /// before anything runs: the arity is `N`, a compile-time constant, so the
    /// bound is settled by const evaluation and this road returns no `Result`.
    /// A caller then has no error branch to fill for a case that cannot happen,
    /// and a complete set that quietly became one member would be a plan
    /// declaring a smaller output set than its shape fixed — which everything
    /// downstream would go on to prove correctly.
    ///
    /// # Nonclaims
    ///
    /// It settles the magnitude, not the distinctness of the roles: nothing here
    /// stops a caller passing one role twice.
    /// That is deliberate — a caller of this road names its roles literally, so a
    /// doubled role is visible at the call site, and the closure check reads the
    /// PLAN's own count per role independently and refuses before anything is
    /// emitted.
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
    /// stand under.
    /// The second check is here rather than downstream because a doubled role is
    /// a defect in the DECLARATION of the set: the closure check matches by role,
    /// so a membership that reaches it doubled has already made that match elect
    /// one member and ignore the other.
    pub fn declared(
        first: PlannedMember<R>,
        rest: Vec<PlannedMember<R>>,
    ) -> Result<Self, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        let members = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| {
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

    /// How many members are planned under one role.
    ///
    /// Two is a defect [`PlannedMembership::declared`] refuses; the membership
    /// itself never elects one of them.
    #[must_use]
    pub fn count_under(&self, role: R) -> usize {
        self.members_under(role).count()
    }

    /// Every member planned under one role, in declaration order.
    ///
    /// The road a COMPLETE-SET comparison walks.
    /// Comparing two memberships by their first member per role would agree about
    /// two sets that differ in their second, which is exactly what a doubled role
    /// produces.
    pub fn members_under(&self, role: R) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members
            .iter()
            .filter(move |member| member.role == role)
    }

    /// Every member this plan declared into one emission, in declaration order.
    ///
    /// The reading that routes: a consumption target asking which members its
    /// carrier receives, and an emission asking which members its join walks,
    /// both take this road rather than reading a destination and deciding what
    /// it means. The partition of a member is its DESTINATION's own constant
    /// answer ([`MemberDestination::partition`]), so this road elects nothing
    /// and interprets nothing.
    ///
    /// [`MemberDestination::partition`]: super::MemberDestination::partition
    pub fn members_in(
        &self,
        partition: EmissionPartition,
    ) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members
            .iter()
            .filter(move |member| member.output.destination.partition() == partition)
    }

    /// How many members this plan declared into one emission.
    ///
    /// Zero is a stated answer and not an absence: a plan that declared nothing
    /// into a partition has an unoccupied emission there, which is a different
    /// fact from an emission that carries no bytes.
    #[must_use]
    pub fn count_in(&self, partition: EmissionPartition) -> usize {
        self.members_in(partition).count()
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
    /// # Ordering
    ///
    /// A declared output SET is order-insensitive, so nothing identity-bearing
    /// may be derived from the order this yields.
    /// Every member identity is derived from its ROLE and its anchor, never from
    /// its position in this iteration, so the same members supplied in another
    /// order yield the same plan.
    pub fn iter(&self) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members.iter()
    }

    /// Append this membership's canonical bytes, in the kind's declared ROLE
    /// ROSTER order.
    ///
    /// Roster order and never declaration order: a declared output set is
    /// order-insensitive, so the same members supplied in another order must
    /// encode identically.
    /// Every member standing under a role is written, not just the first, so a
    /// membership that doubled a role encodes differently from one that did not —
    /// the closure check reports that as a defect, and the encoding must not hide
    /// it before the check runs.
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

impl InvalidationTrigger {
    /// How many source declarations one
    /// [`InvalidationTrigger::SourceDeclarationChanged`] seat watches.
    ///
    /// DECLARED and not derived: Rust cannot count a variant's seats, so nothing
    /// reads this number off the roster.
    /// What holds it true is where it sits and what consumes it.
    /// It sits in the roster's own implementation, so the edit that gives the
    /// seat a second watched identity is an edit to the lines around it, and
    /// [`InvalidationTrigger::watching_source_declarations`] takes exactly this
    /// many identities and destructures them into the seat, so the number and the
    /// seat cannot disagree without failing to compile.
    /// Raise it to two and the pattern no longer matches the array; give the
    /// variant a second watched seat and the construction is missing a field.
    /// This line and that road are the one place that changes when the seat's
    /// shape changes.
    pub(crate) const WATCHED_SOURCE_DECLARATIONS: usize = 1;

    /// The source-declaration watch, built from exactly the identities the seat
    /// carries.
    ///
    /// The array's arity IS
    /// [`InvalidationTrigger::WATCHED_SOURCE_DECLARATIONS`], which is what makes
    /// the caller's refusal and this construction one statement instead of two:
    /// a caller holding more declarations than the seat watches refuses before
    /// reaching here, naming both counts, and a caller holding exactly that many
    /// hands the whole set over and elects nothing out of it.
    pub(crate) const fn watching_source_declarations(
        declared: [OwnerIdentityRef<FragmentIdentityDomain>; Self::WATCHED_SOURCE_DECLARATIONS],
    ) -> Self {
        let [watched] = declared;
        Self::SourceDeclarationChanged { watched }
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
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                InvalidationLimit::MAX,
                observed,
            )
        })
    }
}

impl KindSeal {
    /// The seal, admitted only within the services.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
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

impl<K: ProjectionKind> ProjectionPlan<K> {
    /// Plan one projection, over the entry account the content walked in with.
    ///
    /// The account arrives first because it is what was MEANT: the kind and the
    /// owner content commitment are the intent, the context is what that intent
    /// was decided under, and the decisions are the record of the decision.
    /// The account is moved in rather than read and dropped, so the plan's own
    /// answer to "what were you planned over" is the value its identity, its
    /// watch set, and its origin edges were all derived from.
    ///
    /// # Bounds
    ///
    /// The five decided seats arrive as ONE [`PlanDecisions`] value rather than
    /// as five positions, and the bundling settles nothing: the value's fields
    /// are all required, so a caller that leaves one out stops compiling exactly
    /// where a missing argument used to, and a seat added to a plan is added to
    /// [`PlanDecisions`] and breaks every construction again.
    /// What it removes is a call site stating eight positional facts — the arity
    /// past which the lint wall refuses, and past which a reader tells two seats
    /// of one shape apart by counting commas.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`PlanSeat::TargetBinding`] when the
    /// kind's plans are meaningless without a host contract and the context is
    /// target-free.
    /// The binding is not defaulted: guessing a host is how a wrapper ends up
    /// bound to a contract nobody declared.
    pub fn planned(
        account: OwnerContentAccount<K>,
        context: ProjectionContext,
        kind_content: K::Content,
        decisions: PlanDecisions<K::Rendered>,
    ) -> Result<Self, ProjectionPlanning> {
        // Destructured at the door rather than read field by field further
        // down: every seat the bundle carries is moved into the plan below, so
        // a seat added to the bundle and forgotten here fails to compile at
        // this pattern instead of arriving unwritten.
        let PlanDecisions {
            membership,
            invalidation,
            trace,
            origin,
            nonclaims,
        } = decisions;
        match (K::TARGET_REQUIREMENT, context.target) {
            (TargetRequirement::BoundHostContract, TargetBinding::TargetFree) => Err(
                ProjectionPlanning::established(ProjectionPlanningIssue::MissingOwnerFact {
                    seat: PlanSeat::TargetBinding,
                }),
            ),
            (TargetRequirement::BoundHostContract | TargetRequirement::EitherBinding, _) => {
                let mut claim = Vec::new();
                account.encode_into(&mut claim);
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
                        account.anchoring(),
                        &claim,
                        0,
                    ));
                Ok(Self {
                    derivation: PlanDerivation {
                        identity,
                        provenance,
                    },
                    account,
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

    /// The entry account this plan was planned over.
    ///
    /// The plan's one account of its content, handed back whole: a reader asking
    /// what invalidates it, what caused it, or what it stands on reads the seats
    /// of this value rather than a summary of them.
    #[must_use]
    pub const fn account(&self) -> &OwnerContentAccount<K> {
        &self.account
    }

    /// What this plan MEANT: the intent layer's identity over the kind and the
    /// owner content commitment.
    ///
    /// The comparison door equivalence is stated over — never plan identity,
    /// which contains origin and is required to differ between distinct doors.
    ///
    /// Read off the plan's own account, so the intent a plan reports and the
    /// intent its transcript opened with are one derivation rather than two.
    /// Not `const`, for the reason [`OwnerContentAccount::intent`] states: the
    /// identity is derived, and the digest is not a const evaluation.
    #[must_use]
    pub fn intent(&self) -> ProjectionIntentId {
        self.account.intent()
    }

    /// How this plan's identity was derived.
    ///
    /// The record lives here, once, rather than inside the identity value it
    /// explains.
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
    /// kind's own.
    ///
    /// This is the set an explanation view must fill every seat of.
    #[must_use]
    pub fn applicable_questions() -> Vec<ExplanationQuestion> {
        UNIVERSAL_QUESTIONS
            .iter()
            .copied()
            .chain(K::KIND_QUESTIONS.iter().copied())
            .collect()
    }
}

impl ProjectionBundlePlan {
    /// The one-member bundle. Total: one member always fits.
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
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map(|members| Self { bundle, members })
        .map_err(|_| {
            ProjectionPlanning::bound_exceeded(BoundAxis::Outputs, BundleMemberLimit::MAX, observed)
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

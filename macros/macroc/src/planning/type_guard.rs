//! The plan family's invariant nucleus: every road that reaches a private field,
//! and every smart constructor that settles a declared magnitude.
//!
//! Declared inside `types.rs` as its own child.
//! The output firewall is here: a membership's members are unreachable except
//! through the roads below, so a plan's declared output set is whatever one of
//! them admitted and nothing else.
//! The total structural constructors — [`PlannedMembership::complete`] and the
//! one-member roads — carry no refusal at all, because a set fixed by a shape has
//! no runtime count to read and therefore no error branch for a caller to fill
//! with a shorter set.

use super::super::encode::encode_set;
use super::{
    DigestContract, GraphAnchoring, InvalidationSet, InvalidationTrigger, KindSeal, PlanDerivation,
    PlannedMember, PlannedMembership, ProjectionBundlePlan, ProjectionContext, ProjectionKind,
    ProjectionPlan, SourceDeclarations, TargetBinding, TargetRequirement, UNIVERSAL_QUESTIONS,
};
use crate::origin_graph::{DecisionTrace, Nonclaim, OriginTrail};
use crate::plane::{
    AuthoringLimitProfile, BundleMemberLimit, BundleSubject, GeneratedUnitSubject,
    InvalidationLimit, MembershipLimit, NonclaimLimit, OwnerIdentityRef, PlanId,
    ProjectionIdentity, ProjectionProvenance, ProjectionRole, ProjectionTranscript, RenderedRole,
    SourceDeclarationLimit, encode_bytes, encode_length,
};
use crate::question::ExplanationQuestion;
use crate::refusal::{BoundAxis, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
use threadpak::declaration::DeclarationGraph;
use threadpak::declaration::types::FragmentIdentityDomain;
use threadpak::types::{Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

impl ProjectionContext {
    /// Read the machine's own closed-graph commitment into the plane.
    ///
    /// The road for a caller that HOLDS a closed graph: the services observe the
    /// identity the linker minted and never mint one of their own.
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
    /// cause set outgrows the declared bound.
    /// A partial cause set is refused, not trimmed: an explanation that names
    /// some of its causes is wrong about all of them.
    pub fn declared_sources(
        first: OwnerIdentityRef<FragmentIdentityDomain>,
        rest: Vec<OwnerIdentityRef<FragmentIdentityDomain>>,
    ) -> Result<SourceDeclarations, ProjectionPlanning> {
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                SourceDeclarationLimit::MAX,
                observed,
            )
        })
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
    /// Plan one projection.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming [`PlanSeat::TargetBinding`] when the
    /// kind's plans are meaningless without a host contract and the context is
    /// target-free.
    /// The binding is not defaulted: guessing a host is how a wrapper ends up
    /// bound to a contract nobody declared.
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

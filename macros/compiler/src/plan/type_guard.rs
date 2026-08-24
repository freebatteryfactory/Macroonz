//! The plan home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's two central claims structural.
//! A membership's members are unreachable except through the roads below, so a plan's declared output set is whatever one of them admitted; an account's seats are unreachable the same way, so the one account of a request's content is whatever walked in one of these doors.

use super::super::encode::encode_set;
use super::{
    Account, BoundAxis, Context, Intent, InvalidationSet, InvalidationTrigger, MEMBERSHIP_LIMIT,
    Membership, PLAN_ISSUE_LIMIT, Plan, PlanDecisions, PlanError, PlanIssue, PlannedMember,
    TRIGGER_LIMIT,
};
use crate::bounded::{Bounded, Capped, Capping, NonEmpty, Overflow};
use crate::identity::{
    self, GENERATOR, Identity, PlanId, Profile, Provenance, Transcript, encode_bytes,
};
use crate::kind::{Destination, Kind, Role};
use crate::origin::{DecisionTrace, Nonclaim, OriginTrail, TrailError};
use core::marker::PhantomData;

impl<K: Kind> Account<K> {
    /// The account of content that stands on nothing.
    pub fn over(commitment: Identity<identity::CapturedDeclaration>) -> Self {
        Self {
            commitment,
            dependencies: Bounded::empty(),
            kind: PhantomData,
        }
    }

    /// The account of content that stands on the captures the caller declares.
    ///
    /// The set is canonicalized here — ordered by identity, exact repeats dropped — so two callers declaring one set in two orders reach one plan.
    ///
    /// # Errors
    ///
    /// Returns the planning refusal naming [`BoundAxis::Declarations`] where the declared set outgrows [`DEPENDENCY_LIMIT`](super::DEPENDENCY_LIMIT).
    pub fn standing_on(
        commitment: Identity<identity::CapturedDeclaration>,
        mut dependencies: Vec<Identity<identity::CapturedDeclaration>>,
    ) -> Result<Self, PlanError> {
        dependencies.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        dependencies.dedup();
        Bounded::new(dependencies)
            .map(|admitted| Self {
                commitment,
                dependencies: admitted,
                kind: PhantomData,
            })
            .map_err(|overflow| PlanError::bounded(BoundAxis::Declarations, overflow))
    }

    /// The one address the caller supplied at the door.
    ///
    /// The reading a plan's anchor, its causing-declaration answer, and its own trigger are all taken from — one value read three times rather than three seats that could disagree.
    #[must_use]
    pub const fn commitment(&self) -> Identity<identity::CapturedDeclaration> {
        self.commitment
    }

    /// The captures this content declares it stands on, in canonical order.
    #[must_use]
    pub fn dependencies(&self) -> &[Identity<identity::CapturedDeclaration>] {
        self.dependencies.as_slice()
    }

    /// What was MEANT: the kind's declared name over the content commitment, derived into the intent layer's own identity.
    ///
    /// The preimage is [`Account::intent_bytes`], derived at [`Role::ProjectionIntent`](crate::identity::Role::ProjectionIntent), rooted, at position zero.
    /// Rooted deliberately: the preimage already carries the commitment at full width, so anchoring on that same commitment would write it twice into one derivation and separate nothing.
    #[must_use]
    pub fn intent(&self) -> Intent {
        Intent::derived(Transcript::rooted(
            identity::Role::ProjectionIntent,
            &self.intent_bytes(),
            0,
        ))
    }
}

impl Context {
    /// The context a request is decided under: the profile it selected, and the generator answering.
    ///
    /// The generator is this crate's own and is derived here rather than supplied, so a plan cannot be told a producer it was not produced by.
    /// The derivation is the one [`GENERATOR_VERSION_PROFILE`](crate::identity::GENERATOR_VERSION_PROFILE) states: the declared name framed, then the shape position in four big-endian bytes, rooted at position zero.
    #[must_use]
    pub fn under(profile: Profile) -> Self {
        let mut material = Vec::new();
        encode_bytes(GENERATOR.name().as_bytes(), &mut material);
        material.extend_from_slice(&GENERATOR.shape().position().to_be_bytes());
        Self {
            profile,
            generator: Identity::derived(Transcript::rooted(
                identity::Role::GeneratorVersion,
                &material,
                0,
            )),
        }
    }

    /// The profile this context selected.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// The generator answering under it.
    #[must_use]
    pub const fn generator(&self) -> Identity<identity::GeneratorVersion> {
        self.generator
    }

    /// Every trigger one plan's own facts require, as a set.
    ///
    /// The shared half of any plan's invalidation, derived from the seats this context declares and the commitments the account names rather than listed at a plan site.
    /// A kind adds whatever its own anchors require on top, through [`InvalidationTrigger::Declared`].
    ///
    /// Exact repeats are dropped before construction: a repeat would be written twice by the transcript's set encoding, so two plans watching the same things would carry two identities depending only on whether a call site remembered to skip it.
    ///
    /// # Errors
    ///
    /// Returns the planning refusal naming [`BoundAxis::Triggers`] where the derived set outgrows [`TRIGGER_LIMIT`].
    pub fn watch_set<K: Kind>(&self, account: &Account<K>) -> Result<InvalidationSet, PlanError> {
        // Exhaustive on purpose: a seat added to the context stops compiling HERE
        // until somebody decides whether it is watched, so the watch set cannot
        // fall a seat behind the context it is derived from.
        let Self { profile, generator } = self;
        let (first, mut rest) = account.caused_by();
        let shared = [
            InvalidationTrigger::Profile { watched: *profile },
            InvalidationTrigger::Generator {
                watched: *generator,
            },
        ];
        for trigger in shared {
            if trigger != first && !rest.contains(&trigger) {
                rest.push(trigger);
            }
        }
        InvalidationTrigger::watched(first, rest)
    }
}

impl InvalidationTrigger {
    /// The one-trigger watch set. Total: one trigger always fits.
    #[must_use]
    pub fn one_watched(trigger: Self) -> InvalidationSet {
        NonEmpty::one(trigger)
    }

    /// Watches these triggers, the first one and the rest.
    ///
    /// Several triggers of one row are lawful where they watch distinct things.
    ///
    /// # Errors
    ///
    /// Returns the planning refusal naming [`BoundAxis::Triggers`] where the set outgrows [`TRIGGER_LIMIT`].
    pub fn watched(first: Self, rest: Vec<Self>) -> Result<InvalidationSet, PlanError> {
        let offered = rest.len().saturating_add(1);
        let mut triggers = vec![first];
        triggers.extend(rest);
        NonEmpty::new(triggers).map_err(|_| {
            PlanError::bounded(
                BoundAxis::Triggers,
                Overflow {
                    capacity: TRIGGER_LIMIT,
                    offered,
                },
            )
        })
    }
}

/// The issue one member raises where its seat is absent from the kind's declared roster.
fn foreign<R: Role>(member: &PlannedMember<R>) -> Option<PlanIssue> {
    (!R::ALL.contains(&member.role)).then(|| PlanIssue::MembershipForeign {
        seat: member.role.name(),
    })
}

impl<R: Role> Membership<R> {
    /// The one-member output set.
    ///
    /// # Errors
    ///
    /// Returns one [`PlanIssue::MembershipForeign`] where the member's seat is absent from the kind's declared roster.
    /// The roster is every downstream walk's denominator, so a member outside it is refused here rather than admitted, rendered, and dropped from a proof that claims the whole set.
    pub fn from_member(member: PlannedMember<R>) -> Result<Self, PlanError> {
        match foreign(&member) {
            Some(issue) => Err(PlanError::over(issue, Vec::new())),
            None => Ok(Self {
                members: NonEmpty::one(member),
            }),
        }
    }

    /// Declares the complete output set, the first member and the rest.
    ///
    /// # Errors
    ///
    /// Returns the planning refusal naming [`BoundAxis::Outputs`] where the set outgrows [`MEMBERSHIP_LIMIT`], one [`PlanIssue::MembershipForeign`] per member whose seat the kind's roster does not declare, and one [`PlanIssue::MembershipDoubled`] per seat two members stand under.
    /// Both checks are here rather than downstream because each is a defect in the DECLARATION of the set: closure matches by seat over the roster, so a membership that reaches it doubled has already made that match elect one member and ignore the other, and one that reaches it with a foreign seat holds a member no walk will ever look at.
    pub fn declared(
        first: PlannedMember<R>,
        rest: Vec<PlannedMember<R>>,
    ) -> Result<Self, PlanError> {
        let offered = rest.len().saturating_add(1);
        let mut offering = vec![first];
        offering.extend(rest);
        let declared = NonEmpty::new(offering)
            .map(|admitted| Self { members: admitted })
            .map_err(|_| {
                PlanError::bounded(
                    BoundAxis::Outputs,
                    Overflow {
                        capacity: MEMBERSHIP_LIMIT,
                        offered,
                    },
                )
            })?;
        let mut established: Vec<PlanIssue> = declared
            .members
            .iter()
            .filter_map(|member| foreign(member))
            .collect();
        established.extend(R::ALL.iter().filter_map(|role| declared.doubling(*role)));
        let mut walked = established.into_iter();
        match walked.next() {
            Some(issue) => Err(PlanError::over(issue, walked.collect())),
            None => Ok(declared),
        }
    }

    /// The issue one seat raises where two members stand under it.
    fn doubling(&self, role: R) -> Option<PlanIssue> {
        let observed = self.count_under(role);
        (observed > 1).then(|| PlanIssue::MembershipDoubled {
            role_slot: role.slot(),
            observed: u32::try_from(observed).unwrap_or(u32::MAX),
        })
    }

    /// The guaranteed first member.
    #[must_use]
    pub fn first(&self) -> &PlannedMember<R> {
        self.members.first()
    }

    /// The declared members, the guaranteed first one ahead of the rest.
    ///
    /// # Ordering
    ///
    /// A declared output set is order-insensitive, and nothing identity-bearing is derived from this order: the canonical encoding walks the kind's roster instead, so the same members declared in another order reach one plan.
    #[must_use]
    pub fn members(&self) -> &NonEmpty<PlannedMember<R>, MEMBERSHIP_LIMIT> {
        &self.members
    }

    /// The member planned under one seat, where one is.
    #[must_use]
    pub fn under(&self, role: R) -> Option<&PlannedMember<R>> {
        self.members().iter().find(|member| member.role == role)
    }

    /// Every member planned under one seat, in declaration order.
    ///
    /// The road a complete-set comparison walks: comparing two memberships by their first member per seat would agree about two sets that differ in their second, which is exactly what a doubled seat produces.
    pub fn members_under(&self, role: R) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members()
            .iter()
            .filter(move |member| member.role == role)
    }

    /// How many members are planned under one seat.
    #[must_use]
    pub fn count_under(&self, role: R) -> usize {
        self.members_under(role).count()
    }

    /// Every member this plan declared into one delivery, in declaration order.
    ///
    /// The reading that routes, and it elects nothing: a member's delivery is its seat's own constant answer ([`Role::destination`]), so a join asking which members it emits and a consumption target asking which cargo it receives take one answer rather than two that agree until one is edited.
    pub fn members_to(&self, destination: Destination) -> impl Iterator<Item = &PlannedMember<R>> {
        self.members()
            .iter()
            .filter(move |member| member.role.destination() == destination)
    }

    /// How many members this plan declared into one delivery.
    ///
    /// Zero is a stated answer rather than an absence: a plan that declared nothing into a delivery has an unoccupied one there, which is a different fact from a delivery that carries no bytes.
    #[must_use]
    pub fn count_to(&self, destination: Destination) -> usize {
        self.members_to(destination).count()
    }

    /// Whether two memberships name the same members under one seat, as sets.
    #[must_use]
    pub fn agrees_under(&self, other: &Self, role: R) -> bool {
        let mine: Vec<&PlannedMember<R>> = self.members_under(role).collect();
        let theirs: Vec<&PlannedMember<R>> = other.members_under(role).collect();
        mine == theirs
    }

    /// The number of members declared; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.members.count()
    }
}

impl PlanError {
    /// The refusal one established issue makes.
    pub fn of(issue: PlanIssue) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(issue)),
        }
    }

    /// The refusal a pass whose checks co-establish makes.
    ///
    /// The caller arrives holding every issue its pass established, so the posture the body writes is about the REPORT and never about the pass: where the issues fit it carries all of them, and where they do not it carries what fits and counts the rest.
    pub fn over(first: PlanIssue, rest: Vec<PlanIssue>) -> Self {
        Self {
            body: Capped::first_n(first, rest.into_iter()),
        }
    }

    /// The refusal a magnitude makes: the axis that was overrun, and the two counts the overflow already carries.
    pub fn bounded(axis: BoundAxis, overflow: Overflow) -> Self {
        Self::of(PlanIssue::BoundExceeded {
            axis,
            bound: u64::try_from(overflow.capacity).unwrap_or(u64::MAX),
            observed: u64::try_from(overflow.offered).unwrap_or(u64::MAX),
        })
    }

    /// The refusal a trail that could not be drawn makes, over the unit it was to be drawn for.
    ///
    /// The unit is the caller's to name because a trail refusal carries none, and an origin that cannot be drawn at all is that unit standing with no origin.
    pub fn over_trail(node: Identity<identity::GeneratedUnit>, refusal: TrailError) -> Self {
        match refusal {
            TrailError::Discontinuous { at } => Self::of(PlanIssue::TrailDiscontinuous { at }),
            TrailError::Empty(_) => Self::of(PlanIssue::OrphanGeneratedNode { node }),
            TrailError::Overflow(overflow) => Self::bounded(BoundAxis::OriginEdges, overflow),
        }
    }

    /// The first issue the pass established, which every refusal has.
    #[must_use]
    pub fn first_issue(&self) -> &PlanIssue {
        self.body.items().first()
    }

    /// Every issue this refusal carries, in the order the pass established them; structurally at least one.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<PlanIssue, PLAN_ISSUE_LIMIT> {
        self.body.items()
    }

    /// Whether this refusal carries every issue its pass established.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}

impl<K: Kind> Plan<K> {
    /// Plans one projection over the account the content walked in with.
    ///
    /// The account arrives first because it is what was MEANT; the context is what that intent was decided under, and the decisions are the record of the decision.
    /// Total: everything that could refuse refused where it was declared, so a plan is assembled out of values that already hold.
    pub fn planned(
        account: Account<K>,
        decided_under: Context,
        content: K::Content,
        decisions: PlanDecisions<K::Role>,
    ) -> Self {
        // Destructured at the door: every seat the bundle carries is moved into
        // the plan below, so a seat added to the bundle and forgotten here fails
        // to compile at this pattern instead of arriving unwritten.
        let PlanDecisions {
            membership,
            invalidation,
            trace,
            origin,
            nonclaims,
        } = decisions;
        let mut claim = Vec::new();
        account.encode_into(&mut claim);
        decided_under.encode_into(&mut claim);
        membership.encode_into(&mut claim);
        encode_set(
            invalidation.iter(),
            InvalidationTrigger::encode_into,
            &mut claim,
        );
        trace.encode_into(&mut claim);
        origin.encode_into(&mut claim);
        encode_set(nonclaims.iter(), Nonclaim::encode_into, &mut claim);
        let (derived, provenance) = PlanId::derived_with_provenance(Transcript::under(
            identity::Role::Plan,
            account.anchoring(),
            &claim,
            0,
        ));
        Self {
            identity: derived,
            provenance,
            account,
            context: decided_under,
            content,
            membership,
            invalidation,
            trace,
            origin,
            nonclaims,
        }
    }

    /// This plan's own identity.
    #[must_use]
    pub const fn identity(&self) -> PlanId {
        self.identity
    }

    /// The record of how that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &Provenance {
        &self.provenance
    }

    /// The account this plan was planned over, whole.
    ///
    /// A reader asking what invalidates it, what caused it, or what it stands on reads the seats of this value rather than a summary of them.
    pub const fn account(&self) -> &Account<K> {
        &self.account
    }

    /// What this plan MEANT.
    ///
    /// The comparison equivalence is stated over — never the plan identity, which carries origin and is required to differ between distinct requests.
    /// Read off the plan's own account, so the intent a plan reports and the intent its transcript opened with are one derivation rather than two.
    #[must_use]
    pub fn intent(&self) -> Intent {
        self.account.intent()
    }

    /// The exact facts this plan was decided under.
    #[must_use]
    pub const fn context(&self) -> &Context {
        &self.context
    }

    /// The kind-specific facts.
    #[must_use]
    pub const fn content(&self) -> &K::Content {
        &self.content
    }

    /// The complete declared output set.
    #[must_use]
    pub const fn membership(&self) -> &Membership<K::Role> {
        &self.membership
    }

    /// The triggers whose change invalidates this plan; structurally at least one.
    #[must_use]
    pub fn invalidation(&self) -> &InvalidationSet {
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
    pub fn nonclaims(&self) -> &[Nonclaim] {
        self.nonclaims.as_slice()
    }
}

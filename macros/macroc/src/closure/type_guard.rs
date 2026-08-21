//! The closure home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's
//! central claim structural.
//! A rendered unit's digest is taken here, over the tree's own canonical bytes,
//! so a renderer cannot hand in a digest of bytes it did not emit.
//! A closure is built here, after the reconstruction agreed and over the
//! emissions this file splits, joins, and keeps, so the exact token stream each
//! build receives is inside what was proved rather than assembled afterwards.
//! A closed expansion is built here too, and it is the only value that hands
//! those emissions out: the closure's own road to them is crate-internal, so a
//! caller reaches tokens through the account that binds the plan, the proof, and
//! the explanation, or it does not reach them.
//! No other seam in the crate produces any of these values.
//! The refusal body is built here by the same permission: its seat is private,
//! so this file is the only module in the workspace that can spell the literal,
//! and every refusal that exists came off the per-role pass.
//!
//! Rust's privacy is module-scoped, so a seat declared in `types.rs` would put
//! every other item in that file inside the wall.
//! The body is therefore declared in the `seat` module below, whose entire
//! content is that record and inherent implementations of it — the module is
//! the complete set of roads that can reach the private seat.
//!
//! # Nonclaims
//!
//! A private seat excludes every sibling — `types.rs` above it, `prove.rs`
//! beside it, anywhere else in the services, and any crate downstream — and the
//! compiler says so with `E0451`.
//! It does not exclude descendants: a module declared inside a guard would
//! construct as freely as these roads do, so the reversal for these seats is a
//! compile-fail fixture testpak owns.

use super::super::prove::examined;
use super::{
    CarriedTokens, ClosedExpansion, ClosureIssue, DeliveryAddressing, ExpansionBindingRefusal,
    PartitionCargo, PartitionedEmission, ProjectionClosure, RenderedProjection, RenderedUnit,
    RenderingRefusal,
};
use crate::explanation_protocol::ProjectionExplanationView;
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, ByteRoleSubject, ClosedExpansionId, ClosureId, GeneratedUnitSubject,
    OutputBytesSubject, OwnerIdentityRef, PlanId, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, ProjectionProvenance, ProjectionRole, ProjectionTranscript,
    RenderedRole, RenderedUnitSubject, encode_bytes, encode_length,
};
use crate::planning::{
    DigestContract, EmissionPartition, MemberDestination, PlannedMember, PlannedMembership,
    PlannedOutput, ProjectionKind, ProjectionPlan,
};
use crate::question::EXPLANATION_PROTOCOL_VERSION;
use crate::token::GeneratedTree;
use threadpak::types::{AdmittedLimit, Bounded, NonEmptyBounded, PositiveLimit};

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
///
/// One road for every pass in [`ProjectionClosure::proved`], so no pass can
/// establish issues and then walk on past them.
fn refused<R: RenderedRole>(issues: Vec<ClosureIssue<R>>) -> Option<ProjectionClosureRefusal<R>> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ProjectionClosureRefusal::established(
        first,
        established.collect(),
    ))
}

pub use seat::ProjectionClosureRefusal;

mod seat {
    use super::super::{ClosureIssue, ClosureIssueLimit};
    use crate::plane::{AuthoringLimitProfile, RenderedRole};
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The closure refusal family body.
    ///
    /// Independent members: a rendering may drop one role and orphan another in
    /// one pass, and reporting one of them would leave a caller repairing a
    /// rendering one role per attempt.
    #[must_use = "a refusal family body carries every way the rendering and the plan disagree"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ProjectionClosureRefusal<R: RenderedRole> {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue the pass
        /// established or names how many stand outside that bound.
        /// One seat rather than two, because a coverage claim seated beside its
        /// body is a claim that can be swapped for another body's.
        ///
        /// Private for the second half of the same claim: a public seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one pass could write it into another
        /// pass's refusal.
        /// Read back through [`ProjectionClosureRefusal::body`].
        body: AdmittedPrefix<ClosureIssue<R>, ClosureIssueLimit>,
    }

    impl<R: RenderedRole> ProjectionClosureRefusal<R> {
        /// The body a closure check refuses with.
        ///
        /// The per-role pass walks the kind's whole roster before a body
        /// exists, so the posture here is about the report rather than the
        /// pass: where every established issue fits the declared bound the body
        /// carries all of them, and where it does not, the body carries what
        /// the bound holds and names how many stand outside it.
        /// Never a silent drop.
        ///
        /// Reaches the guard file and no further.
        pub(super) fn established(first: ClosureIssue<R>, rest: Vec<ClosureIssue<R>>) -> Self {
            Self {
                body: AdmittedPrefix::examined_completely(
                    first,
                    rest,
                    &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                    StopBound::DeclaredIssueBound,
                ),
            }
        }

        /// The established issues and what this refusal says about its own
        /// coverage of them.
        ///
        /// Borrowed and never owned, for the reason band 00 borrows its carry:
        /// an owned body is a value a caller can seat under another refusal,
        /// which is the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<ClosureIssue<R>, ClosureIssueLimit> {
            &self.body
        }
    }
}

impl<R: RenderedRole> RenderedUnit<R> {
    /// Materialize one rendered unit from the tree a renderer produced.
    ///
    /// The digest is taken here, over the tree's own canonical bytes, under the
    /// contract's anchor.
    /// No caller supplies one, so a renderer cannot hand in a digest of bytes
    /// it did not emit.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::BytesUnbounded`] when the rendered bytes
    /// exceed the declared magnitude.
    pub fn materialized(
        role: R,
        semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
        destination: MemberDestination,
        profile: ProjectionIdentity<ProjectionProfileSubject>,
        profile_version: ProfileVersion,
        origin: OriginTrail,
        tree: GeneratedTree,
    ) -> Result<Self, RenderingRefusal> {
        let raw = tree.canonical_bytes();
        let digest = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::OutputBytes,
            &semantic_key,
            &raw,
            role.slot(),
        ));
        let identity = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::RenderedUnit,
            &semantic_key,
            &raw,
            role.slot(),
        ));
        let bytes = Bounded::admitted_const(
            raw,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| RenderingRefusal::BytesUnbounded)?;
        Ok(Self {
            role,
            identity,
            semantic_key,
            destination,
            profile,
            profile_version,
            origin,
            tree,
            bytes,
            digest,
        })
    }

    /// The role this unit was rendered under.
    #[must_use]
    pub const fn role(&self) -> R {
        self.role
    }

    /// This rendered unit's own identity.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<RenderedUnitSubject> {
        self.identity
    }

    /// The semantic key this unit answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.semantic_key
    }

    /// Which delivery this unit was rendered into.
    ///
    /// The renderer's own answer, taken from the role's constant answer at the
    /// moment the unit was materialized, and the seat the emission is split by:
    /// a unit reaches an emission through this value and through nothing else,
    /// so no seam decides a delivery a second time.
    #[must_use]
    pub const fn destination(&self) -> MemberDestination {
        self.destination
    }

    /// The digest over this unit's canonical bytes.
    #[must_use]
    pub const fn digest(&self) -> ProjectionIdentity<OutputBytesSubject> {
        self.digest
    }

    /// Where this unit came from.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The token tree this unit is.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }

    /// The unit's canonical bytes.
    pub fn bytes(&self) -> impl Iterator<Item = &u8> {
        self.bytes.iter()
    }

    /// How many canonical bytes the unit carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the unit rendered nothing at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The membership row this unit reconstructs — the renderer's own answer to
    /// what it materialized, in exactly the shape a plan states it.
    #[must_use]
    pub fn reconstructed(&self) -> PlannedMember<R> {
        PlannedMember {
            role: self.role,
            output: PlannedOutput {
                semantic_key: self.semantic_key,
                destination: self.destination,
                origin: self.origin.clone(),
                expected_profile: self.profile,
                expected_profile_version: self.profile_version,
                digest_contract: DigestContract::over(self.semantic_key),
            },
        }
    }

    /// The digest recomputed from the bytes this unit actually carries, under
    /// one stated contract.
    ///
    /// The closure compares this against [`RenderedUnit::digest`]: a digest
    /// that does not survive being recomputed under the plan's contract is a
    /// digest of something else.
    #[must_use]
    pub fn digest_under(&self, contract: DigestContract) -> ProjectionIdentity<OutputBytesSubject> {
        let raw: Vec<u8> = self.bytes.iter().copied().collect();
        ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            contract.role,
            &contract.anchored_to,
            &raw,
            self.role.slot(),
        ))
    }

    /// Append this unit's canonical bytes: the role it stood under, its own
    /// identity, the semantic key it answers to, where it landed, the profile
    /// and version it was rendered under, where it came from, and the digest of
    /// the bytes it carries.
    ///
    /// The rendered bytes themselves are not written, and do not need to be:
    /// the digest is derived over them at full width, so a byte that changed
    /// changes the digest and therefore this encoding.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.role.slot().to_be_bytes());
        encode_bytes(self.identity.as_bytes(), into);
        encode_bytes(self.semantic_key.as_bytes(), into);
        self.destination.encode_into(into);
        encode_bytes(self.profile.as_bytes(), into);
        into.extend_from_slice(&self.profile_version.position().to_be_bytes());
        self.origin.encode_into(into);
        encode_bytes(self.digest.as_bytes(), into);
    }
}

impl<R: RenderedRole> RenderedProjection<R> {
    /// The one-unit rendering. Total: one unit always fits.
    #[must_use]
    pub fn of_one(unit: RenderedUnit<R>) -> Self {
        Self {
            units: NonEmptyBounded::singleton(unit),
        }
    }

    /// The rendering of a roster fixed by its own shape — a *total structural*
    /// constructor, as [`PlannedMembership::complete`] is one.
    ///
    /// A renderer that knows before it starts exactly which roles it will
    /// materialize has no runtime count to read, so there is no refusal here to
    /// swallow and no shorter rendering to fall back to.
    #[must_use]
    pub fn complete<const N: usize>(first: RenderedUnit<R>, rest: [RenderedUnit<R>; N]) -> Self {
        Self {
            units: NonEmptyBounded::from_array(first, rest),
        }
    }

    /// The several-unit rendering.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::UnitsUnbounded`] when the rendering outgrows
    /// the declared membership magnitude.
    pub fn materialized(
        first: RenderedUnit<R>,
        rest: Vec<RenderedUnit<R>>,
    ) -> Result<Self, RenderingRefusal> {
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map(|units| Self { units })
        .map_err(|_| RenderingRefusal::UnitsUnbounded)
    }

    /// The rendered units, in the order the renderer produced them.
    pub fn units(&self) -> impl Iterator<Item = &RenderedUnit<R>> {
        self.units.iter()
    }

    /// How many units were rendered; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.units.len()
    }

    /// Always `false`: an empty rendering is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// The one unit rendered under a role, where exactly one was.
    #[must_use]
    pub fn under(&self, role: R) -> Option<&RenderedUnit<R>> {
        self.units.iter().find(|unit| unit.role() == role)
    }

    /// How many units were rendered under one role.
    #[must_use]
    pub fn count_under(&self, role: R) -> usize {
        self.units.iter().filter(|unit| unit.role() == role).count()
    }

    /// Every unit this rendering materialized into one emission, in role-roster
    /// order.
    ///
    /// The reading the join walks and a consumption target routes by. A unit's
    /// emission is its DESTINATION's own constant answer, so this road elects
    /// nothing and interprets nothing.
    ///
    /// # Ordering
    ///
    /// Role order, never rendering order: the roster is declared and the
    /// renderer's own sequencing is not, so what is emitted is stable under a
    /// renderer that happened to produce its units in another order.
    /// EVERY unit standing under a role is yielded, not the first one — a
    /// rendering that doubled a role is a rendering the proof refuses, and a
    /// reading that quietly dropped the second unit would hide the doubling from
    /// anybody looking here instead.
    pub fn units_in(&self, partition: EmissionPartition) -> impl Iterator<Item = &RenderedUnit<R>> {
        R::ROLES.iter().flat_map(move |role| {
            self.units.iter().filter(move |unit| {
                unit.role() == *role && unit.destination().partition() == partition
            })
        })
    }

    /// How many units this rendering materialized into one emission.
    #[must_use]
    pub fn count_in(&self, partition: EmissionPartition) -> usize {
        self.units_in(partition).count()
    }
}

impl CarriedTokens {
    /// The tokens one emission carries, with the digest taken here over exactly
    /// those bytes.
    ///
    /// Private to the guard, and the one road: no caller supplies a digest, so
    /// an emission cannot carry the digest of bytes it does not carry.
    ///
    /// The digest is anchored on the PLAN and positioned at the emission's own
    /// roster slot, so two emissions of one plan that happened to join to the
    /// same bytes are still two digests — which is what keeps an expansion's
    /// declaration-site answer from standing in for its carrier's.
    fn joined(plan: PlanId, partition: EmissionPartition, tree: GeneratedTree) -> Self {
        let raw = tree.canonical_bytes();
        let digest = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::OutputBytes,
            &plan,
            &raw,
            u32::from(partition.slot()),
        ));
        Self { tree, digest }
    }

    /// The tokens themselves.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }

    /// The digest of exactly these bytes, as the proving closure's identity
    /// commits to it.
    #[must_use]
    pub const fn digest(&self) -> ProjectionIdentity<OutputBytesSubject> {
        self.digest
    }

    /// Append these tokens' canonical bytes: the digest at full width.
    ///
    /// The tokens themselves are not written and do not need to be: the digest
    /// is derived over them at full width, so a byte that changed changes the
    /// digest and therefore this encoding.
    fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.digest.as_bytes(), into);
    }
}

impl PartitionCargo {
    /// The tokens this emission carries, where it carries any.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing where the plan declared no member into this
    /// emission. That is a stated posture rather than a missing value: an empty
    /// token tree is what an emission carries when a rendering produced no
    /// bytes for it, and an unoccupied emission is one nothing was ever planned
    /// into. This road never turns the second into the first.
    #[must_use]
    pub const fn tokens(&self) -> Option<&GeneratedTree> {
        match self {
            Self::NothingPlanned => None,
            Self::Carried(carried) => Some(carried.tree()),
        }
    }

    /// Append this cargo's canonical bytes: the posture's discriminant, then the
    /// digest where tokens are carried.
    ///
    /// The posture rides ahead of the material, so an emission nothing was
    /// planned into never encodes as one that carries bytes.
    fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::NothingPlanned => {
                into.push(0);
                encode_bytes(&[], into);
            }
            Self::Carried(carried) => {
                into.push(1);
                carried.encode_into(into);
            }
        }
    }
}

/// The cargo one emission of one rendering carries.
///
/// Private to the guard, with one caller: the partitioning inside
/// [`ProjectionClosure::proved`]. The join is a step inside the proof, so there
/// is no second road to a joined tree the closure identity says nothing about.
fn joined_cargo<R: RenderedRole>(
    plan: PlanId,
    rendered: &RenderedProjection<R>,
    partition: EmissionPartition,
) -> Result<PartitionCargo, ClosureIssue<R>> {
    let mut tokens = Vec::new();
    let mut occupied = false;
    for unit in rendered.units_in(partition) {
        occupied = true;
        tokens.extend(unit.tree().tokens().cloned());
    }
    if !occupied {
        return Ok(PartitionCargo::NothingPlanned);
    }
    let tree = GeneratedTree::assembled(tokens)
        .map_err(|_| ClosureIssue::JoinedTreeUnbounded { partition })?;
    Ok(PartitionCargo::Carried(CarriedTokens::joined(
        plan, partition, tree,
    )))
}

/// The address every published unit of one rendering stands at, checked for a
/// collision.
///
/// One address, one artifact. Two units written under one byte role would put
/// the second unit's bytes where the first unit's bytes are, and the address
/// would then name two units while answering for one.
///
/// Private to the guard, with one caller, on the terms the join has: this is a
/// step inside the proof.
fn published_addresses_agree<R: RenderedRole>(
    rendered: &RenderedProjection<R>,
) -> Result<(), ClosureIssue<R>> {
    let mut taken: Vec<OwnerIdentityRef<ByteRoleSubject>> = Vec::new();
    for unit in rendered.units_in(EmissionPartition::PublicationArtifact) {
        // The publication emission IS the units whose destination names an
        // address — that is the reading that put them here — so the address is
        // read off that same destination rather than looked up beside it. The
        // other arms do not reach this emission, so there is no second case for
        // this walk to answer.
        if let MemberDestination::AsArtifact { byte_role } = unit.destination() {
            if taken.contains(&byte_role) {
                return Err(ClosureIssue::ArtifactAddressDoubled {
                    role: unit.role(),
                    byte_role,
                });
            }
            taken.push(byte_role);
        }
    }
    Ok(())
}

impl PartitionedEmission {
    /// Split one proved rendering across the emissions its members declared.
    ///
    /// Private to the guard, with one caller: [`ProjectionClosure::proved`].
    ///
    /// # Ordering
    ///
    /// The partition roster is the quantifier: every joined emission is built,
    /// in roster order, whether or not anything was planned into it — so an
    /// emission that carries nothing says so rather than being left out of the
    /// walk.
    ///
    /// # Errors
    ///
    /// Returns [`ClosureIssue::JoinedTreeUnbounded`] naming the emission whose
    /// joined tree outgrew the declared token magnitude.
    fn over<R: RenderedRole>(
        plan: PlanId,
        rendered: &RenderedProjection<R>,
    ) -> Result<Self, ClosureIssue<R>> {
        Ok(Self {
            declaration_site: joined_cargo(plan, rendered, EmissionPartition::DeclarationSite)?,
            test_carrier: joined_cargo(plan, rendered, EmissionPartition::TestCarrier)?,
            bench_carrier: joined_cargo(plan, rendered, EmissionPartition::BenchCarrier)?,
        })
    }

    /// What the declaration site expands into — the tokens the consumer's normal
    /// build compiles.
    pub const fn declaration_site(&self) -> &PartitionCargo {
        &self.declaration_site
    }

    /// The deferred cargo the consumer's test target invokes.
    pub const fn test_carrier(&self) -> &PartitionCargo {
        &self.test_carrier
    }

    /// The deferred cargo the consumer's bench target invokes.
    pub const fn bench_carrier(&self) -> &PartitionCargo {
        &self.bench_carrier
    }

    /// The cargo one joined emission carries.
    ///
    /// Exhaustive over the roster on purpose: a partition added to
    /// [`EmissionPartition`] stops compiling HERE until somebody says what it
    /// carries, so no emission can be admitted and left unrouted.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing for the publication emission, which is not
    /// joined: a published artifact is its rendered unit at the address that
    /// unit's destination names, and it is read as one
    /// ([`ClosedExpansion::published`]).
    #[must_use]
    pub const fn joined(&self, partition: EmissionPartition) -> Option<&PartitionCargo> {
        match partition {
            EmissionPartition::DeclarationSite => Some(&self.declaration_site),
            EmissionPartition::TestCarrier => Some(&self.test_carrier),
            EmissionPartition::BenchCarrier => Some(&self.bench_carrier),
            EmissionPartition::PublicationArtifact => None,
        }
    }

    /// Append this emission's canonical bytes: the joined emissions, in
    /// partition-roster order, each written as its slot and its cargo.
    ///
    /// The published artifacts are not written here and are not missing: a
    /// closure's transcript already commits to every rendered unit at full
    /// width — its identity, its semantic key, its DESTINATION with the byte
    /// role inside it, and its digest — so an artifact written to another
    /// address is already a different closure.
    fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(EmissionPartition::ALL.len(), into);
        for partition in EmissionPartition::ALL {
            into.push(partition.slot());
            match self.joined(partition) {
                Some(cargo) => cargo.encode_into(into),
                // An emission that is not joined at all is its own posture, and
                // it is written as one: it must not encode as an emission
                // nothing was planned into, because the publication emission
                // carrying artifacts and the test carrier carrying nothing are
                // different facts about different deliveries.
                None => {
                    into.push(2);
                    encode_bytes(&[], into);
                }
            }
        }
    }
}

impl<R: RenderedRole> ProjectionClosure<R> {
    /// Prove the closure between one plan's membership and one rendering.
    ///
    /// # Construction
    ///
    /// The identity is derived under [`ProjectionRole::Closure`], anchored on
    /// the plan's own identity, over a content transcript that commits to the
    /// complete closure claim, in this order:
    ///
    /// 1. the explanation protocol version
    ///    ([`EXPLANATION_PROTOCOL_VERSION`]) — a closure claims a rendering
    ///    answers a protocol, and a claim made under a different protocol is a
    ///    different claim;
    /// 2. the full planned membership, in role-roster order — every semantic
    ///    key, destination, origin trail, expected profile and version, and
    ///    digest contract the plan declared;
    /// 3. the role roster's own length;
    /// 4. for every role in roster order: the role slot, how many units stood
    ///    under it, and the unit that did — its identity, semantic key,
    ///    destination, profile and version, origin trail, and digest;
    /// 5. the partitioned emission — every joined emission, in partition-roster
    ///    order, as its slot, its posture, and the digest of exactly the bytes
    ///    it carries.
    ///
    /// So the identity names the whole agreement rather than a sample of it.
    ///
    /// The last member is why the emission is inside the proof: the emissions
    /// are built here, from the rendered units in role-roster order and split by
    /// the delivery each unit's destination declares, and the closure keeps
    /// them.
    /// The bytes a caller emits into any build are the bytes this identity
    /// names, and a rendering that moved one member to another delivery is a
    /// different closure rather than the same one emitted differently.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionClosureRefusal`] naming every role the two disagree
    /// at, the emission whose joined tree outgrew its magnitude, or the address
    /// two published units stand at.
    /// Every disagreement of one pass is reported together: a caller repairing a
    /// rendering one role per attempt is a caller the check failed.
    pub fn proved(
        plan: PlanId,
        planned: &PlannedMembership<R>,
        rendered: RenderedProjection<R>,
    ) -> Result<Self, ProjectionClosureRefusal<R>> {
        let (issues, rebuilt) = examined(planned, &rendered);
        if let Some(refusal) = refused(issues) {
            return Err(refusal);
        }

        let observed = u32::try_from(rebuilt.len()).unwrap_or(u32::MAX);
        let mut rows = rebuilt.into_iter();
        let Some(first) = rows.next() else {
            return Err(ProjectionClosureRefusal::established(
                ClosureIssue::ReconstructionEmpty,
                Vec::new(),
            ));
        };
        let reconstructed = PlannedMembership::declared(first, rows.collect()).map_err(|_| {
            ProjectionClosureRefusal::established(
                ClosureIssue::ReconstructionUndeclarable { observed },
                Vec::new(),
            )
        })?;

        // The theorem, stated over the whole set: role by role, the rebuild and
        // the plan hold the same members. Every check above is about one seat;
        // this one is about the collection, which a first-per-role walk could
        // never establish.
        let disagreements: Vec<ClosureIssue<R>> = R::ROLES
            .iter()
            .copied()
            .filter(|role| !reconstructed.agrees_under(planned, *role))
            .map(|role| ClosureIssue::MembershipDisagreement { role })
            .collect();
        if let Some(refusal) = refused(disagreements) {
            return Err(refusal);
        }

        // The publication emission's occupancy is an occupancy by ADDRESS, so
        // it is established here — after the roles agree and before anything is
        // joined, because two units at one address is a defect in what the
        // rendering would WRITE rather than in what it rendered.
        published_addresses_agree(&rendered)
            .map_err(|issue| ProjectionClosureRefusal::established(issue, Vec::new()))?;

        let emission = PartitionedEmission::over(plan, &rendered)
            .map_err(|issue| ProjectionClosureRefusal::established(issue, Vec::new()))?;

        let mut material: Vec<u8> = Vec::new();
        material.extend_from_slice(&EXPLANATION_PROTOCOL_VERSION.to_be_bytes());
        planned.encode_into(&mut material);
        encode_length(R::ROLES.len(), &mut material);
        for role in R::ROLES {
            material.extend_from_slice(&role.slot().to_be_bytes());
            encode_length(rendered.count_under(*role), &mut material);
            if let Some(unit) = rendered.under(*role) {
                unit.encode_into(&mut material);
            }
        }
        emission.encode_into(&mut material);
        let (identity, provenance) = ClosureId::derived_with_provenance(
            ProjectionTranscript::under_projection(ProjectionRole::Closure, &plan, &material, 0),
        );

        Ok(Self {
            plan,
            reconstructed,
            rendered,
            emission,
            identity,
            provenance,
        })
    }

    /// The membership rebuilt out of the rendered units.
    #[must_use]
    pub const fn reconstructed(&self) -> &PlannedMembership<R> {
        &self.reconstructed
    }

    /// What the renderer produced.
    #[must_use]
    pub const fn rendered(&self) -> &RenderedProjection<R> {
        &self.rendered
    }

    /// The emissions this closure proved, split by delivery, joined in
    /// role-roster order, and owned here.
    ///
    /// Crate-internal, with one caller: [`ClosedExpansion::bound`].
    /// This is the closure's own proof material, not a road to tokens — a caller
    /// that could read it here would be emitting off a proof without the plan it
    /// was proved against or the explanation written over it, which is the
    /// binding's whole reason to exist.
    /// Nothing joins the rendered units a second time, and the digests this
    /// closure's identity commits to are the digests of exactly these bytes.
    pub(crate) const fn emission(&self) -> &PartitionedEmission {
        &self.emission
    }

    /// The plan this closure was proved against.
    #[must_use]
    pub const fn plan(&self) -> PlanId {
        self.plan
    }

    /// This closure's own identity.
    /// Inspection and emission both read this value, so there is no second
    /// closure identity anywhere to disagree with.
    #[must_use]
    pub const fn identity(&self) -> ClosureId {
        self.identity
    }

    /// How this closure's identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        &self.provenance
    }
}

impl<K: ProjectionKind> ClosedExpansion<K> {
    /// Bind one closed expansion: the plan, the closure proved against it, and
    /// the explanation answered over the two.
    ///
    /// Public, and the road every projection kind's door terminates at. A caller
    /// that walked the steps itself arrives here with three unforgeable values
    /// and leaves with the one account emission is reachable from; a caller that
    /// skipped a step has nothing to hand in.
    ///
    /// # The three identities agree, or nothing is bound
    ///
    /// The values were produced separately and each carries the parentage it was
    /// produced under: the closure names the plan it was proved against, and the
    /// explanation names the plan and the closure it was answered over. All
    /// three comparisons are made here, in that order, and none of them is
    /// reconciled — a disagreement is a typed refusal naming both identities
    /// rather than an election between them.
    ///
    /// # The closed-expansion transcript
    ///
    /// The identity is derived under [`ProjectionRole::ClosedExpansion`],
    /// anchored on the CLOSURE's identity — because a closed expansion exists
    /// only where a closure does — over a content transcript of exactly two
    /// members:
    ///
    /// 1. the plan's identity, which already commits to the entry account (and
    ///    through it the kind), the context, and the complete declared
    ///    membership — so what was READ and what was DECIDED reach this
    ///    transcript through the value that owns them;
    /// 2. the explanation's identity, which already commits to the plan and the
    ///    closure it was answered over and to every typed answer it carries — so
    ///    what was EXPLAINED reaches this transcript the same way.
    ///
    /// Nothing else enters, and each absence is the no-double-entry law: the
    /// partitioned emission is inside the anchor (a closure's identity commits
    /// to its partition digests), and the kind is inside member one (a plan's
    /// identity commits to its intent), so a second spelling of either here
    /// would write one fact twice and let the two spellings drift.
    ///
    /// The explanation member is the one this transcript used to be missing.
    /// The explanation had no canonical name at all then, so a terminal could
    /// not commit to it and stated the boundary instead; now it has one, the
    /// boundary is closed, and two expansions differing only in which
    /// explanation they bound are two names.
    ///
    /// # Errors
    ///
    /// Returns [`ExpansionBindingRefusal::ClosureProvedAgainstAnotherPlan`]
    /// where the closure was proved against a plan other than the one handed in,
    /// [`ExpansionBindingRefusal::ExplanationAnsweredOverAnotherPlan`] where the
    /// explanation was answered over another plan of this kind, and
    /// [`ExpansionBindingRefusal::ExplanationAnsweredOverAnotherClosure`] where
    /// it was answered over another proof.
    /// Nothing is elected out of any of the three pairs: an expansion naming one
    /// plan while carrying another's proof, or another's explanation, would
    /// answer every question correctly about the wrong expansion.
    pub fn bound(
        plan: ProjectionPlan<K>,
        closure: ProjectionClosure<K::Rendered>,
        explanation: ProjectionExplanationView<K>,
    ) -> Result<Self, ExpansionBindingRefusal> {
        let planned = plan.identity();
        let proved = closure.plan();
        if planned != proved {
            return Err(ExpansionBindingRefusal::ClosureProvedAgainstAnotherPlan {
                planned,
                proved,
            });
        }
        let answered_over_plan = explanation.plan();
        if planned != answered_over_plan {
            return Err(
                ExpansionBindingRefusal::ExplanationAnsweredOverAnotherPlan {
                    planned,
                    answered: answered_over_plan,
                },
            );
        }
        let anchor = closure.identity();
        let answered_over_closure = explanation.closure();
        if anchor != answered_over_closure {
            return Err(
                ExpansionBindingRefusal::ExplanationAnsweredOverAnotherClosure {
                    proved: anchor,
                    answered: answered_over_closure,
                },
            );
        }
        let mut content = Vec::new();
        encode_bytes(planned.as_bytes(), &mut content);
        encode_bytes(explanation.identity().as_bytes(), &mut content);
        let (identity, provenance) =
            ClosedExpansionId::derived_with_provenance(ProjectionTranscript::under_projection(
                ProjectionRole::ClosedExpansion,
                &anchor,
                &content,
                0,
            ));
        Ok(Self {
            identity,
            provenance,
            plan,
            closure,
            explanation,
        })
    }

    /// This expansion's own identity: the name of the whole account.
    #[must_use]
    pub const fn identity(&self) -> ClosedExpansionId {
        self.identity
    }

    /// How that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        &self.provenance
    }

    /// The complete plan: account, context, content, membership, invalidation
    /// set, decision trace, origin trail, and nonclaims.
    pub const fn plan(&self) -> &ProjectionPlan<K> {
        &self.plan
    }

    /// The proof that what was rendered is what was planned.
    pub const fn closure(&self) -> &ProjectionClosure<K::Rendered> {
        &self.closure
    }

    /// The complete explanation over this kind's applicable questions.
    pub const fn explanation(&self) -> &ProjectionExplanationView<K> {
        &self.explanation
    }

    /// The emissions this expansion delivers, split by delivery.
    ///
    /// The CLOSURE's own proved value, borrowed rather than copied: this
    /// expansion keeps no second emission, so what is delivered is what was
    /// proved and there is no pair of values to drift apart.
    pub const fn emission(&self) -> &PartitionedEmission {
        self.closure.emission()
    }

    /// What the declaration site expands into — the tokens an expansion shell
    /// hands the compiler, and the only ones the consumer's normal build
    /// compiles.
    pub const fn declaration_site(&self) -> &PartitionCargo {
        self.emission().declaration_site()
    }

    /// The deferred cargo the consumer's test target invokes.
    pub const fn test_carrier(&self) -> &PartitionCargo {
        self.emission().test_carrier()
    }

    /// The deferred cargo the consumer's bench target invokes.
    pub const fn bench_carrier(&self) -> &PartitionCargo {
        self.emission().bench_carrier()
    }

    /// Every unit this expansion publishes as a standalone artifact, in
    /// role-roster order, each carrying the address its own destination names.
    ///
    /// Read off the proved rendering rather than copied into a record beside it:
    /// a published artifact IS its rendered unit at an address, and a second
    /// value restating that unit's tree, digest, and role would be a second
    /// answer to one question.
    pub fn published(&self) -> impl Iterator<Item = &RenderedUnit<K::Rendered>> {
        self.closure
            .rendered()
            .units_in(EmissionPartition::PublicationArtifact)
    }

    /// What this expansion states about the addresses its delivery will
    /// eventually be reached by.
    ///
    /// Read off the roster, which has one row: there is nothing here for a
    /// caller to choose and nothing for this seam to invent. A second row is
    /// admitted when the identity it names exists.
    #[must_use]
    pub const fn addressing(&self) -> DeliveryAddressing {
        DeliveryAddressing::UnmintedAtThisSeam
    }
}

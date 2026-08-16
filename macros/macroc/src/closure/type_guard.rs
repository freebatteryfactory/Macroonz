//! The closure home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's
//! central claim structural.
//! A rendered unit's digest is taken here, over the tree's own canonical bytes,
//! so a renderer cannot hand in a digest of bytes it did not emit.
//! A closure is built here, after the reconstruction agreed and over the joined
//! tree this file builds and keeps, so the exact token stream a caller emits is
//! inside what was proved rather than assembled afterwards.
//! No other seam in the crate produces either value.
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
use super::{ClosureIssue, ProjectionClosure, RenderedProjection, RenderedUnit, RenderingRefusal};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, ClosureId, GeneratedUnitSubject, OutputBytesSubject, PlanId,
    ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, ProjectionProvenance,
    ProjectionRole, ProjectionTranscript, RenderedRole, RenderedUnitSubject, encode_bytes,
    encode_length,
};
use crate::planning::{
    DigestContract, MemberDestination, PlannedMember, PlannedMembership, PlannedOutput,
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
    use super::super::ClosureIssue;
    use crate::plane::{AuthoringLimitProfile, ClosureIssueLimit, RenderedRole};
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

    /// The token tree the whole rendering is, in role-roster order.
    ///
    /// # Ordering
    ///
    /// Role order, never rendering order: the roster is declared and the
    /// renderer's own sequencing is not, so what is emitted is stable under a
    /// renderer that happened to produce its units in another order.
    ///
    /// Crate-internal, with one caller: [`ProjectionClosure::proved`].
    /// The join is a step inside the proof, so there is no second road to a
    /// joined tree the closure identity says nothing about.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::BytesUnbounded`] when the joined tree
    /// outgrows the declared token magnitude.
    pub(crate) fn joined_tree(&self) -> Result<GeneratedTree, RenderingRefusal> {
        let mut tokens = Vec::new();
        for role in R::ROLES {
            if let Some(unit) = self.under(*role) {
                tokens.extend(unit.tree().tokens().cloned());
            }
        }
        GeneratedTree::assembled(tokens).map_err(|_| RenderingRefusal::BytesUnbounded)
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
    /// 5. the digest of the emitted joined tree, at full width.
    ///
    /// So the identity names the whole agreement rather than a sample of it.
    ///
    /// The last member is why the emission is inside the proof: the joined tree
    /// is built here, from the rendered units in role-roster order, and the
    /// closure keeps it.
    /// The bytes a caller emits are the bytes this identity names.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionClosureRefusal`] naming every role the two disagree
    /// at.
    /// All of them are reported together: a caller repairing a rendering one
    /// role per attempt is a caller the check failed.
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

        let emitted = rendered.joined_tree().map_err(|_| {
            ProjectionClosureRefusal::established(ClosureIssue::JoinedTreeUnbounded, Vec::new())
        })?;
        let emitted_bytes = emitted.canonical_bytes();
        let emitted_digest = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::OutputBytes,
            &plan,
            &emitted_bytes,
            0,
        ));

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
        encode_bytes(emitted_digest.as_bytes(), &mut material);
        let (identity, provenance) = ClosureId::derived_with_provenance(
            ProjectionTranscript::under_projection(ProjectionRole::Closure, &plan, &material, 0),
        );

        Ok(Self {
            plan,
            reconstructed,
            rendered,
            emitted,
            emitted_digest,
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

    /// The token tree this closure proved, joined in role-roster order and
    /// owned here.
    ///
    /// The one road to emitted tokens.
    /// Nothing joins the rendered units a second time, and the digest this
    /// closure's identity commits to is the digest of exactly these bytes.
    #[must_use]
    pub const fn emitted(&self) -> &GeneratedTree {
        &self.emitted
    }

    /// The digest of the emitted joined tree, as this closure's identity commits
    /// to it.
    #[must_use]
    pub const fn emitted_digest(&self) -> ProjectionIdentity<OutputBytesSubject> {
        self.emitted_digest
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

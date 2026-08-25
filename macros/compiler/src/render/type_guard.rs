//! The render home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's central claim structural.
//! A unit's identity and the digest of its bytes are taken here, over the tree's own canonical bytes, under the semantic key the planned member declares — so a renderer cannot hand in a digest of bytes it did not emit, and cannot answer to a seat no plan declared.

use super::{Output, RENDERED_BYTE_LIMIT, RenderError, RenderedProjection, RenderedUnit};
use crate::bounded::{NonEmpty, NonEmptyError};
use crate::identity::{self, Identity, OwnerIdentity, Profile, Transcript};
use crate::kind::{Destination, Kind, Role};
use crate::origin::OriginTrail;
use crate::plan::{DigestContract, MEMBERSHIP_LIMIT, Plan, PlannedMember, PlannedOutput};
use crate::token::GeneratedTree;

impl<R: Role> RenderedUnit<R> {
    /// Materialize one planned member out of the tree a renderer produced.
    ///
    /// A rendered unit IS a planned member plus the bytes that answer it, so every fact the member already states is read off it rather than restated at the call — nothing here can pair one seat's key with another seat's origin.
    /// The digest and this unit's own identity are both taken here, over the tree's canonical bytes, under that key at that seat's roster position.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::BytesUnbounded`] where the rendered bytes pass [`RENDERED_BYTE_LIMIT`].
    pub fn materialized(
        planned: &PlannedMember<R>,
        tree: GeneratedTree,
    ) -> Result<Self, RenderError> {
        let material = tree.canonical_bytes();
        if material.len() > RENDERED_BYTE_LIMIT {
            return Err(RenderError::BytesUnbounded {
                role: planned.role.name(),
                bound: RENDERED_BYTE_LIMIT,
                observed: material.len(),
            });
        }
        let output = &planned.output;
        let position = u32::from(planned.role.slot());
        let digest = Identity::derived(Transcript::under_projection(
            identity::Role::OutputBytes,
            &output.semantic_key,
            &material,
            position,
        ));
        let derived = Identity::derived(Transcript::under_projection(
            identity::Role::RenderedUnit,
            &output.semantic_key,
            &material,
            position,
        ));
        Ok(Self {
            role: planned.role,
            identity: derived,
            semantic_key: output.semantic_key,
            profile: output.expected_profile,
            origin: output.origin.clone(),
            address: output.address,
            tree,
            digest,
        })
    }

    /// The seat this unit was rendered under.
    #[must_use]
    pub const fn role(&self) -> R {
        self.role
    }

    /// This rendered unit's own identity.
    #[must_use]
    pub const fn identity(&self) -> Identity<identity::RenderedUnit> {
        self.identity
    }

    /// The semantic key this unit answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> Identity<identity::GeneratedUnit> {
        self.semantic_key
    }

    /// Which delivery this unit lands in.
    ///
    /// Read off the seat and never stored: a delivery a unit could disagree with its own role about would be a second answer to a question the roster already answers.
    #[must_use]
    pub fn destination(&self) -> Destination {
        self.role.destination()
    }

    /// The profile this unit was rendered under.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// Where this unit came from.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The address a publication writes this unit to, where its seat is one that writes to an address.
    pub const fn address(&self) -> Option<OwnerIdentity> {
        self.address
    }

    /// The token tree this unit is.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }

    /// The digest over this unit's canonical bytes.
    #[must_use]
    pub const fn digest(&self) -> Identity<identity::OutputBytes> {
        self.digest
    }

    /// This unit's canonical bytes — the exact material the digest was taken over.
    ///
    /// Derived from the tree on every reading rather than kept beside it, so there is no second copy of one unit's bytes to disagree with the tree.
    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        self.tree.canonical_bytes()
    }

    /// The membership row this unit reconstructs — the renderer's own answer to what it materialized, in exactly the shape a plan states it.
    #[must_use]
    pub fn reconstructed(&self) -> PlannedMember<R> {
        PlannedMember {
            role: self.role,
            output: PlannedOutput {
                semantic_key: self.semantic_key,
                origin: self.origin.clone(),
                expected_profile: self.profile,
                address: self.address,
                digest_contract: DigestContract {
                    anchored_to: self.semantic_key,
                },
            },
        }
    }

    /// The digest recomputed from the bytes this unit carries, under one stated contract.
    ///
    /// A proof compares this against [`RenderedUnit::digest`]: a digest that does not survive being recomputed under the plan's own contract is a digest of something else.
    #[must_use]
    pub fn digest_under(&self, contract: DigestContract) -> Identity<identity::OutputBytes> {
        let material = self.tree.canonical_bytes();
        Identity::derived(Transcript::under_projection(
            identity::Role::OutputBytes,
            &contract.anchored_to,
            &material,
            u32::from(self.role.slot()),
        ))
    }
}

impl<R: Role> RenderedProjection<R> {
    /// The one-unit rendering. Total: one unit always fits.
    #[must_use]
    pub fn of_one(unit: RenderedUnit<R>) -> Self {
        Self {
            units: NonEmpty::one(unit),
        }
    }

    /// The several-unit rendering.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::NothingRendered`] where no unit was offered, and [`RenderError::UnitsUnbounded`] where the rendering outgrows [`MEMBERSHIP_LIMIT`] — the two counts read off the collection that refused rather than restated here.
    pub fn materialized(units: Vec<RenderedUnit<R>>) -> Result<Self, RenderError> {
        NonEmpty::new(units)
            .map(|admitted| Self { units: admitted })
            .map_err(|refusal| match refusal {
                NonEmptyError::Empty(_) => RenderError::NothingRendered,
                NonEmptyError::Overflow(overflow) => RenderError::UnitsUnbounded {
                    bound: overflow.capacity,
                    observed: overflow.offered,
                },
            })
    }

    /// The guaranteed first unit.
    #[must_use]
    pub fn first(&self) -> &RenderedUnit<R> {
        self.units.first()
    }

    /// The rendered units, in the order the renderer produced them; structurally at least one.
    #[must_use]
    pub fn units(&self) -> &NonEmpty<RenderedUnit<R>, MEMBERSHIP_LIMIT> {
        &self.units
    }

    /// The unit rendered under one seat, where one was.
    pub fn under(&self, role: R) -> Option<&RenderedUnit<R>> {
        self.units().iter().find(|unit| unit.role() == role)
    }

    /// Every unit rendered under one seat, in rendering order.
    ///
    /// The road a set comparison walks: comparing two renderings by their first unit per seat would agree about two renderings that differ in their second, which is exactly what a doubled seat produces.
    pub fn units_under(&self, role: R) -> impl Iterator<Item = &RenderedUnit<R>> {
        self.units().iter().filter(move |unit| unit.role() == role)
    }

    /// How many units were rendered under one seat.
    #[must_use]
    pub fn count_under(&self, role: R) -> usize {
        self.units_under(role).count()
    }

    /// Every unit this rendering materialized into one delivery, in ROSTER order.
    ///
    /// A unit reaches a delivery through its seat's own constant answer, so this road elects nothing and interprets nothing.
    ///
    /// # Ordering
    ///
    /// Roster order and never rendering order: the roster is declared and a renderer's own sequencing is not, so what a join writes is stable under a renderer that happened to produce its units in another order.
    /// Every unit standing under a seat is yielded rather than the first, because a rendering that doubled a seat is one the proof refuses and a reading that quietly dropped the second unit would hide the doubling from anybody looking here instead.
    pub fn units_to(&self, destination: Destination) -> impl Iterator<Item = &RenderedUnit<R>> {
        R::ALL
            .iter()
            .copied()
            .filter(move |role| role.destination() == destination)
            .flat_map(move |role| self.units_under(role))
    }

    /// How many units this rendering materialized into one delivery.
    #[must_use]
    pub fn count_to(&self, destination: Destination) -> usize {
        self.units_to(destination).count()
    }

    /// How many units were rendered; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.units.count()
    }
}

impl<'plan, K: Kind> Output<'plan, K> {
    /// The empty output one plan's renderer writes into.
    #[must_use]
    pub const fn over(plan: &'plan Plan<K>) -> Self {
        Self {
            plan,
            units: Vec::new(),
        }
    }

    /// Materialize the unit that fills one seat.
    ///
    /// Naming the seat is the whole call: the key the unit answers to, where it came from, the profile expected to render it, and the address it publishes to are that seat's planned member's, read here.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::SeatUnplanned`] where this plan declares no member under the seat, and [`RenderError::BytesUnbounded`] where the tokens pass [`RENDERED_BYTE_LIMIT`].
    pub fn unit(&mut self, role: K::Role, tree: GeneratedTree) -> Result<(), RenderError> {
        let plan = self.plan;
        let planned = plan
            .membership()
            .under(role)
            .ok_or_else(|| RenderError::SeatUnplanned { role: role.name() })?;
        let rendered = RenderedUnit::materialized(planned, tree)?;
        self.units.push(rendered);
        Ok(())
    }

    /// Everything the renderer wrote, as the rendering a proof closes over.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::NothingRendered`] where the renderer wrote no unit at all, and [`RenderError::UnitsUnbounded`] where it wrote past the membership magnitude.
    /// It does not answer for a seat left unfilled or filled twice: those are disagreements between this rendering and the plan, and the proof that compares the two is what states them.
    pub fn rendered(self) -> Result<RenderedProjection<K::Role>, RenderError> {
        RenderedProjection::materialized(self.units)
    }
}

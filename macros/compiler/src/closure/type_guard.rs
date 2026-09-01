//! The closure home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's central claim structural.
//! A closure is built here, after the reconstruction agreed and over the deliveries this file joins and keeps, so the exact token stream each build receives is inside what was proved rather than assembled afterwards.
//! The road from a closure to those deliveries is crate-internal, so a caller reaches tokens through the expansion that binds the plan, the proof, and the explanation, or it does not reach them.
//! No other seam in the crate produces any of these values.

use super::super::encode::claim;
use super::super::prove::{addressed, examined, units_to};
use super::{
    CLOSURE_ISSUE_LIMIT, CarriedTokens, Closure, ClosureError, ClosureIssue, PartitionCargo,
    PartitionedEmission,
};
use crate::bounded::{Capped, Capping, NonEmpty};
use crate::identity::{self, ClosureId, Identity, PlanId, Provenance, Transcript};
use crate::kind::{Destination, Kind, Role};
use crate::plan::{Membership, Plan, PlannedMember};
use crate::render::RenderedProjection;
use crate::token::GeneratedTree;

impl CarriedTokens {
    /// The tokens one delivery carries, with the digest taken here over exactly those bytes.
    ///
    /// Private to the guard, and the one road: no caller supplies a digest, so a delivery cannot carry the digest of bytes it does not carry.
    /// The digest is anchored on the PLAN and positioned at the delivery's own roster position, so two deliveries of one plan that happened to join to the same bytes are still two digests — which is what keeps an expansion's declaration-site answer from standing in for its carrier's.
    fn joined(plan: PlanId, destination: Destination, tree: GeneratedTree) -> Self {
        let raw = tree.canonical_bytes();
        let digest = Identity::derived(Transcript::under_projection(
            identity::Role::OutputBytes,
            &plan,
            &raw,
            delivery_position(destination),
        ));
        Self { tree, digest }
    }

    /// The tokens themselves.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }

    /// The digest of exactly these bytes, as the proving closure's identity commits to it.
    #[must_use]
    pub const fn digest(&self) -> Identity<identity::OutputBytes> {
        self.digest
    }
}

impl PartitionCargo {
    /// The tokens this delivery carries, where it carries any.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing where the plan declared no member into this delivery.
    /// That is a stated posture rather than a missing value, and this road never turns "nothing was planned here" into "a cargo of no tokens".
    #[must_use]
    pub const fn tokens(&self) -> Option<&GeneratedTree> {
        match self {
            Self::NothingPlanned => None,
            Self::Carried(carried) => Some(carried.tree()),
        }
    }
}

impl PartitionedEmission {
    /// Split one proved rendering across the deliveries its seats declared.
    ///
    /// Private to the guard, with one caller: [`Closure::proved`].
    ///
    /// # Ordering
    ///
    /// The delivery roster is the quantifier: every joined delivery is built whether or not anything was planned into it, so a delivery that carries nothing says so rather than being left out of the walk.
    ///
    /// # Errors
    ///
    /// Returns [`ClosureIssue::JoinedTreeUnbounded`] naming the delivery whose joined tree outgrew the declared token magnitude.
    fn over<R: Role>(
        plan: PlanId,
        rendered: &RenderedProjection<R>,
    ) -> Result<Self, ClosureIssue<R>> {
        Ok(Self {
            declaration_site: joined_cargo(plan, rendered, Destination::DeclarationSite)?,
            test_carrier: joined_cargo(plan, rendered, Destination::TestCarrier)?,
            bench_carrier: joined_cargo(plan, rendered, Destination::BenchCarrier)?,
        })
    }

    /// What the declaration site expands into — the tokens the consumer's normal build compiles.
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

    /// The cargo one joined delivery carries.
    ///
    /// Exhaustive over the roster on purpose: a delivery added to [`Destination`] stops compiling HERE until somebody says what it carries, so no delivery can be admitted and left unrouted.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing for the publication delivery, which is not joined: a published artifact is its rendered unit at the address the plan named for it, and it is read as one.
    #[must_use]
    pub const fn joined(&self, destination: Destination) -> Option<&PartitionCargo> {
        match destination {
            Destination::DeclarationSite => Some(&self.declaration_site),
            Destination::TestCarrier => Some(&self.test_carrier),
            Destination::BenchCarrier => Some(&self.bench_carrier),
            Destination::PublicationArtifact => None,
        }
    }
}

impl<R: Role> ClosureError<R> {
    /// The refusal one established issue makes.
    pub fn of(issue: ClosureIssue<R>) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(issue)),
        }
    }

    /// The refusal a pass whose checks co-establish makes.
    ///
    /// The caller arrives holding every issue its pass established, so the posture the body writes is about the REPORT and never about the pass: where the issues fit it carries all of them, and where they do not it carries what fits and counts the rest.
    pub fn over(first: ClosureIssue<R>, rest: Vec<ClosureIssue<R>>) -> Self {
        Self {
            body: Capped::first_n(first, rest.into_iter()),
        }
    }

    /// The first issue the pass established, which every refusal has.
    #[must_use]
    pub fn first_issue(&self) -> &ClosureIssue<R> {
        self.body.items().first()
    }

    /// Every issue this refusal carries, in the order the pass established them; structurally at least one.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<ClosureIssue<R>, CLOSURE_ISSUE_LIMIT> {
        self.body.items()
    }

    /// Whether this refusal carries every issue its pass established.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}

impl<R: Role> Closure<R> {
    /// Prove the closure between one plan's membership and one rendering.
    ///
    /// # Construction
    ///
    /// The identity is derived at [`Role::Closure`](crate::identity::Role::Closure), anchored on the plan's own identity, over the complete claim: the planned membership in roster order, the roster's own length, the identity and digest of the unit that stood under each seat, and every joined delivery's digest.
    /// So the identity names the whole agreement rather than a sample of it, and the bytes a caller emits into any build are bytes this identity names.
    ///
    /// # The two halves are one value
    ///
    /// The proof takes the PLAN, not a plan identity beside a membership.
    /// Separate arguments are separable: nothing in the types would stop a caller handing one plan's identity beside another's membership, and the closure would be born naming the first while proving the second.
    ///
    /// # Errors
    ///
    /// Returns [`ClosureError`] naming every seat the two disagree at, the delivery whose joined tree outgrew its magnitude, or the address two published units stand at.
    /// Every disagreement of one pass is reported together: a caller repairing a rendering one seat per attempt is a caller the check failed.
    pub fn proved<K: Kind<Role = R>>(
        plan: &Plan<K>,
        rendered: RenderedProjection<R>,
    ) -> Result<Self, ClosureError<R>> {
        let planned = plan.membership();
        let named = plan.identity();
        let (issues, rows) = examined(planned, &rendered);
        if let Some(refusal) = refused(issues) {
            return Err(refusal);
        }
        let reconstructed = rebuilt(rows)?;

        // The theorem, stated over the whole set: seat by seat, the rebuild and
        // the plan hold the same members. Every check above is about one seat;
        // this one is about the collection, which a first-per-seat walk could
        // never establish.
        let disagreements: Vec<ClosureIssue<R>> = R::ALL
            .iter()
            .copied()
            .filter(|role| !reconstructed.agrees_under(planned, *role))
            .map(|role| ClosureIssue::MembershipDisagreement { role })
            .collect();
        if let Some(refusal) = refused(disagreements) {
            return Err(refusal);
        }

        // Occupancy in the publication delivery is occupancy by ADDRESS, so it
        // is established after the seats agree and before anything is joined:
        // two units at one address is a defect in what the rendering would
        // WRITE rather than in what it rendered.
        if let Some(refusal) = refused(addressed(&rendered)) {
            return Err(refusal);
        }

        let emission = PartitionedEmission::over(named, &rendered).map_err(ClosureError::of)?;
        let material = claim(planned, &rendered, &emission);
        let (derived, provenance) = ClosureId::derived_with_provenance(
            Transcript::under_projection(identity::Role::Closure, &named, &material, 0),
        );
        Ok(Self {
            plan: named,
            reconstructed,
            rendered,
            emission,
            identity: derived,
            provenance,
        })
    }

    /// The membership rebuilt out of the rendered units.
    #[must_use]
    pub const fn reconstructed(&self) -> &Membership<R> {
        &self.reconstructed
    }

    /// What the renderer produced.
    #[must_use]
    pub const fn rendered(&self) -> &RenderedProjection<R> {
        &self.rendered
    }

    /// The deliveries this closure proved, joined in roster order and owned here.
    ///
    /// Crate-internal, with one caller: the binding that seals an expansion.
    /// This is the closure's own proof material rather than a road to tokens — a caller that could read it here would be emitting off a proof without the plan it was proved against or the explanation written over it, which is the binding's whole reason to exist.
    pub(crate) const fn emission(&self) -> &PartitionedEmission {
        &self.emission
    }

    /// The plan this closure was proved against.
    #[must_use]
    pub const fn plan(&self) -> PlanId {
        self.plan
    }

    /// This closure's own identity.
    #[must_use]
    pub const fn identity(&self) -> ClosureId {
        self.identity
    }

    /// The record of how that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &Provenance {
        &self.provenance
    }
}

/// The refusal one established issue list amounts to, or nothing where the list is empty.
///
/// One road for every pass in [`Closure::proved`], so no pass can establish issues and then walk on past them.
fn refused<R: Role>(issues: Vec<ClosureIssue<R>>) -> Option<ClosureError<R>> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ClosureError::over(first, established.collect()))
}

/// The rows the per-seat pass rebuilt, declared as a complete output set.
///
/// Reached only after that pass established nothing, so every seat holds at most one member and the two ways declaring can still fail are the two named here.
fn rebuilt<R: Role>(rows: Vec<PlannedMember<R>>) -> Result<Membership<R>, ClosureError<R>> {
    let observed = u32::try_from(rows.len()).unwrap_or(u32::MAX);
    let mut members = rows.into_iter();
    let Some(first) = members.next() else {
        return Err(ClosureError::of(ClosureIssue::ReconstructionEmpty));
    };
    Membership::declared(first, members.collect())
        .map_err(|_| ClosureError::of(ClosureIssue::ReconstructionUndeclarable { observed }))
}

/// The cargo one delivery of one rendering carries.
///
/// Private to the guard, with one caller: the partitioning inside [`Closure::proved`].
/// The join is a step inside the proof, so there is no second road to a joined tree the closure identity says nothing about.
fn joined_cargo<R: Role>(
    plan: PlanId,
    rendered: &RenderedProjection<R>,
    destination: Destination,
) -> Result<PartitionCargo, ClosureIssue<R>> {
    let mut joined: Option<GeneratedTree> = None;
    for unit in units_to(rendered, destination) {
        joined = Some(match joined {
            Some(tree) => tree
                .joined(unit.tree())
                .map_err(|_| ClosureIssue::JoinedTreeUnbounded { destination })?,
            None => unit.tree().clone(),
        });
    }
    let Some(tree) = joined else {
        return Ok(PartitionCargo::NothingPlanned);
    };
    Ok(PartitionCargo::Carried(CarriedTokens::joined(
        plan,
        destination,
        tree,
    )))
}

/// The position one delivery's joined tokens are digested at, inside one plan's own sequence.
///
/// Preimage material, so a row is APPENDED and never renumbered.
const fn delivery_position(destination: Destination) -> u32 {
    match destination {
        Destination::DeclarationSite => 0,
        Destination::TestCarrier => 1,
        Destination::BenchCarrier => 2,
        Destination::PublicationArtifact => 3,
    }
}

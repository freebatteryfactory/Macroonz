//! The compile-once interpreter's rapid loop: runtime SELECTION over an
//! evaluation surface, the mandatory no-mutation parity, and the typed
//! availability that stands where a crippled fake interpreter would.
//!
//! The walk over the declaration happened at generation time, over the
//! services' own typed capture. One evaluation copy compiles once carrying every
//! admitted mutation point, and everything in this file is selection among
//! them — never interpretation of arbitrary source, which would mint a second
//! meaning authority.
//!
//! # The surface arrives as data
//!
//! An evaluation surface reaches this lane as conforming DATA under the
//! producer-facing mutation-point roster the descriptor vocabulary publishes
//! ([`crate::descriptor::MUTATION_POINT_FIELDS`]). The runtime types are this
//! crate's own ([`MutationPoint`], [`EvaluationSurface`]) and nothing here
//! imports a generator: a producer emits against the published roster, and a
//! hand-authored surface under the same contract is equally lawful.
//!
//! # The mandatory parity
//!
//! Every evaluation surface contains the no-mutation mutant, and the evaluation
//! copy with no-mutation selected must agree with production over the same
//! inputs before any interpreted mutant is trusted. The trial is a
//! [`ParitySuite`] over the properties vocabulary, and its shared substrate is
//! stated rather than implied: the two roads stand on ONE DECLARATION and ONE
//! RENDERING ENGINE, named as the roster they are, so the parity is silent
//! about both. Declaring these two roads independent would be a claim nobody
//! could make honestly, and the property vocabulary keeps that claim a
//! deliberate arm rather than a place an empty roster arrives at. What the
//! parity proves is that the evaluation copy is faithful to the rendered
//! production surface — never that either matches the owner's intent.
//!
//! The law itself is [`crate::properties::parity`]'s. This file builds the suite
//! and reads the standing its conclusion states; it runs nothing.
//!
//! # Availability is typed
//!
//! Interpreted mutation is available exactly when a conforming surface exists
//! and the trust order has opened — a witness rejection demonstrated under the
//! adapter qualification trust is being opened under, then the parity. The two
//! typed facts are what the gate consumes ([`AdapterQualification`],
//! [`CompiledPressureWitness`]); a bare run is not evidence anybody can open
//! trust with, because a run stripped of its profile no longer says which tool,
//! which version posture, which output, and which grammar produced it at
//! exactly the moment those facts decide something. Absence is
//! [`InterpreterAvailability`], never a crippled interpreter that answers
//! anyway.
//!
//! The gate opens only on evidence somebody checked, and the vocabulary is what
//! makes that so: a qualification exists only over an adapter whose grammar a
//! party checked against the backend version its reading names, so an unchecked
//! chain has no qualification to offer and reaches this road holding nothing.
//! What the gate itself weighs is therefore the remaining question — whether
//! the witness in hand was shown under the qualification trust is being opened
//! under — and the answers name themselves.
//!
//! # Four axes stay four
//!
//! Whether the adapter reads its backend's output correctly is not whether a
//! property bit; whether a property bit is not how many mutants a run pressed;
//! and none of those three is the no-mutation parity. The qualification carries
//! the first, the witness the second, the run's census the third, and
//! [`ParityStanding`] the fourth — four facts on four seats, so no reader takes
//! one of them for another.

use super::types::{
    ActiveMutant, AdapterQualification, CompiledPressureWitness, EvaluationSurface,
    FamilyAttribution, InterpreterAvailability, MUTERPRATER_NAMESPACE, MappingPosture,
    MissingTrustEvidence, MutationIdentity, MutationPoint, MutationSite, MutationTarget,
    NO_MUTATION_PAIRING, PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE, ParityRefusal,
    ParityStanding, PlanRefusal, PlannedDamage, PlannedRun, PressureLane, ProofPlan,
    ScopedInvocation, SelectionRefusal,
};
use crate::descriptor::{MutationPointRef, NamespacedName};
use crate::properties::{
    Equivalence, ParitySuite, Road, RoadPairing, SharedSubstrate, SubstrateRef, SubstrateRoster,
};
use crate::runner::Selection;
use std::collections::BTreeSet;

/// What the two no-mutation parity roads both stand on.
///
/// # Authority
///
/// The honesty clause, stated as a value: agreement across a shared substrate is
/// SILENCE about that substrate. These two roads share the one declaration they
/// are both projected from and the rendering engine that renders them, so their
/// agreement is exactly as good as those two things are right and no better.
///
/// The two roads are never declared independent here, and could not honestly
/// be: they are projections of one declaration through one rendering engine, so
/// this lane names both foundations and takes the parity evidence that naming
/// affords.
///
/// # Errors
///
/// Refuses a substrate name that would not parse, then a roster the property
/// home refused — empty, or naming one substrate twice.
pub fn no_mutation_substrate() -> Result<SharedSubstrate, ParityRefusal> {
    let declaration = SubstrateRef::named(MUTERPRATER_NAMESPACE, PARITY_DECLARATION_SUBSTRATE)
        .map_err(ParityRefusal::NameNotParsed)?;
    let rendering = SubstrateRef::named(MUTERPRATER_NAMESPACE, PARITY_RENDERING_SUBSTRATE)
        .map_err(ParityRefusal::NameNotParsed)?;
    let roster = SubstrateRoster::declared(&[declaration, rendering])
        .map_err(ParityRefusal::SubstrateNotDeclared)?;
    Ok(SharedSubstrate::Standing(roster))
}

/// The suite the mandatory no-mutation parity is judged by.
///
/// The production road is the left one and the evaluation copy with no-mutation
/// selected is the right, under the pairing this lane declares — neither of the
/// property home's two named pairings describes it, so the pairing carries this
/// lane's own name and a disagreement reaches a fingerprint that tells it apart
/// from every other pair.
///
/// # Authority
///
/// The equivalence is the OWNER's, always: what sameness means for a meaning is
/// the owner's declaration, and no bound is demanded of either seat.
///
/// # Errors
///
/// Refuses a name this lane spells that would not parse, then a shared-substrate
/// roster the property home refused.
pub fn no_mutation_parity<Input, Meaning>(
    production: Road<Input, Meaning>,
    evaluation_copy: Road<Input, Meaning>,
    same: Equivalence<Meaning>,
) -> Result<ParitySuite<Input, Meaning>, ParityRefusal> {
    let pairing = NamespacedName::named(MUTERPRATER_NAMESPACE, NO_MUTATION_PAIRING)
        .map_err(ParityRefusal::NameNotParsed)?;
    Ok(ParitySuite::over(
        RoadPairing::Declared(pairing),
        production,
        evaluation_copy,
        same,
        no_mutation_substrate()?,
    ))
}

/// What the interpreted lane is available for, read from what the caller holds.
///
/// # Authority
///
/// The trust order, in its owner's order and no other: a conforming surface,
/// then a witness rejection demonstrated under the adapter qualification trust
/// is being opened under, then the mandatory parity. Each answer names itself,
/// so a caller never has to infer why the lane declined.
///
/// The evidence seat takes the two typed facts and never a bare run. A run
/// carries counts; what opens trust is a rejection SHOWN by a suite under a
/// tool somebody vouched for, and the qualification is what says which tool,
/// which version posture, which output, and which grammar that was. That
/// vouching is settled before this road runs: an [`AdapterQualification`] is
/// only ever built over a grammar checked against the backend version its
/// reading names, so the qualification seat here cannot be filled by an
/// unchecked chain and this road's whole question is whose evidence it is.
///
/// # Nonclaims
///
/// A wrap run that reported and killed nothing yields no witness at all — a
/// pass that caught no mutant has shown no property biting — so it reads as
/// [`MissingTrustEvidence::WrapEvidence`] exactly as a lane that never reported
/// does. A witness shown under another qualification is evidence about that
/// other adapter's reading, and it names itself rather than standing in.
#[must_use]
pub fn availability<'surface>(
    surface: Option<&'surface EvaluationSurface>,
    qualification: &AdapterQualification,
    witness: Option<&CompiledPressureWitness>,
    parity: ParityStanding,
) -> InterpreterAvailability<'surface> {
    let Some(conforming) = surface else {
        return InterpreterAvailability::NoConformingSurface;
    };
    let Some(shown) = witness else {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::WrapEvidence,
        };
    };
    if shown.qualification() != qualification {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::WitnessUnderAnotherQualification,
        };
    }
    match parity {
        ParityStanding::NotPassed => InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::NoMutationParity,
        },
        ParityStanding::Passed => InterpreterAvailability::Available {
            surface: conforming,
        },
    }
}

/// The target one interpreted point presses.
///
/// # Authority
///
/// The owner mapping is always [`MappingPosture::Mapped`] here, and structurally
/// so: the producer-facing mutation-point roster carries the owner claim as an
/// exactly-one field, so an interpreted point that named no owner is not a value
/// a producer can emit. Owner-unmapped is the external lane's posture, where a
/// coordinate arrives without a claim.
#[must_use]
pub fn interpreted_target(point: MutationPoint, family: FamilyAttribution) -> MutationTarget {
    MutationTarget::pressed(
        MutationIdentity::Interpreted(point.identity()),
        family,
        MutationSite::Declared(point.activation_site()),
        MappingPosture::Mapped(point.owner_claim()),
    )
}

/// What one point reads as under one active-mutant selection.
///
/// # Authority
///
/// The whole of what runtime selection does. Under the no-mutation mutant every
/// point reads as its declaration's own rendered bytes; under an active
/// selection exactly the selected point reads as the alternative it was selected
/// into, and every other point is untouched. Nothing here compiles, parses, or
/// evaluates anything.
///
/// # Errors
///
/// Refuses a point the surface does not carry, then an alternative index the
/// point's roster does not admit — which is reachable only when an index minted
/// against one surface is offered to another.
pub fn point_reading(
    surface: &EvaluationSurface,
    point: MutationPointRef,
    active: ActiveMutant,
) -> Result<&'static [u8], SelectionRefusal> {
    let Some(found) = surface.point(point) else {
        return Err(SelectionRefusal::NoSuchPoint(point));
    };
    let ActiveMutant::Active(selection) = active else {
        return Ok(found.original_operation());
    };
    if selection.point() != point {
        return Ok(found.original_operation());
    }
    let admitted = found.admitted_alternatives();
    let named = selection.alternative().position();
    admitted
        .get(named)
        .copied()
        .ok_or_else(|| SelectionRefusal::AlternativePastRoster {
            admitted: admitted.len(),
            named,
        })
}

/// Plan one interpreted pass over every damage the surface admits.
///
/// # Authority
///
/// A pure function of its arguments, and it spends nothing: one intended run per
/// point crossed with each alternative that point admits, each carrying the
/// claim-scoped selection it would run under and the budget it would spend. The
/// mutant budget is weighed against the roster before any of it is committed.
///
/// # Errors
///
/// Refuses a surface admitting no damage at all, then a roster larger than the
/// scope's mutant budget admits.
pub fn plan_pass(
    surface: &EvaluationSurface,
    scope: ScopedInvocation,
) -> Result<ProofPlan, PlanRefusal> {
    let budget = scope.budget();
    let mut runs: Vec<PlannedRun> = Vec::new();
    for point in surface.points() {
        let claims: BTreeSet<_> = BTreeSet::from([point.owner_claim()]);
        for selection in point.selections() {
            runs.push(PlannedRun::intended(
                PressureLane::InterpretedMutation,
                MutationIdentity::Interpreted(point.identity()),
                PlannedDamage::Alternative(selection.alternative()),
                Selection::ByClaim(claims.clone()),
                budget,
            ));
        }
    }
    ProofPlan::planned(scope, runs)
}

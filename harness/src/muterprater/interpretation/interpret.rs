//! The compile-once mutation receiver: stable selection, exact no-mutation parity, and admitted interpreted execution.
//!
//! Production is an ordinary callable with no directive.
//! The evaluation callable alone receives a surface-bound [`EvaluationDirective`], and its no-mutation answer must agree with production.
//!
//! Catalog posture and parity qualification are independent: a point-free surface is lawful, and no surface earns parity until its exact pair executes under [`EvaluationDirective::no_mutation`] and both reports and both meanings qualify.
//!
//! Generic suite pressure, exact compiled projection pressure, and interpreted activation are three different evidence routes.
//! [`availability`] requires the first two together, and the second already retains mandatory no-mutation parity and one surface-issued selection.
//! The active road can execute only that selection, reuses the exact input and witness, and admits evidence only after the evaluation callback reports a positive firing count.
//!
//! The caller's production, evaluation, and check function pointers keep their ordinary Rust effect and unwind ceilings.
//! This module records what they returned and delegates wall measurement to the clock owner; measurement posture never enters parity or mutation classification.

use super::types::{
    EvaluationPair, InterpretedExecutionRefusal, InterpretedMutationEvidence, InterpretedTrust,
    InterpreterAvailability, MUTERPRATER_NAMESPACE, MissingTrustEvidence, MutationWitness,
    NO_MUTATION_PAIRING, NoMutationObservationRefusal, NoMutationParityQualification,
    NoMutationParityReading, NoMutationParityStanding, NoMutationReports, NoMutationResults,
    PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE, ParityQualificationRefusal,
    ParityRefusal, RejectedNoMutationParity,
};
use crate::descriptor::NamespacedName;
use crate::muterprater::{
    ActivationEvidence, ActiveSelection, AdmittedAlternative, CompiledProjectionPressure,
    CompiledSuitePressure, DudPlant, EvaluationDirective, EvaluationSurface, FamilyAttribution,
    MappingPosture, MutationIdentity, MutationPoint, MutationReport, MutationSite, MutationTarget,
    PlanRefusal, PlannedDamage, PlannedRun, PressureLane, ProofPlan, ScopedInvocation,
    SelectionRefusal,
};
use crate::properties::{SharedSubstrate, SubstrateRef, SubstrateRoster, agreement};
use crate::report::{FindingCause, HostTrialRecord, RunAttempt, TrialConclusion};
use crate::runner::{
    Invocation, Selection, execution_key, lens_verdict, record_one, trial_identity,
};
use std::collections::BTreeSet;

/// The shared foundations the mandatory no-mutation comparison declares.
fn no_mutation_substrate() -> Result<SharedSubstrate, ParityRefusal> {
    let declaration = SubstrateRef::named(MUTERPRATER_NAMESPACE, PARITY_DECLARATION_SUBSTRATE)
        .map_err(ParityRefusal::NameNotParsed)?;
    let rendering = SubstrateRef::named(MUTERPRATER_NAMESPACE, PARITY_RENDERING_SUBSTRATE)
        .map_err(ParityRefusal::NameNotParsed)?;
    let roster = SubstrateRoster::declared(&[declaration, rendering])
        .map_err(ParityRefusal::SubstrateNotDeclared)?;
    Ok(SharedSubstrate::Standing(roster))
}

/// The finding cause one no-mutation disagreement carries.
fn parity_cause() -> Result<FindingCause, ParityRefusal> {
    let pairing = NamespacedName::named(MUTERPRATER_NAMESPACE, NO_MUTATION_PAIRING)
        .map_err(ParityRefusal::NameNotParsed)?;
    Ok(FindingCause::named(
        pairing.namespace().written(),
        pairing.stem().written(),
    ))
}

/// Resolve the exact alternative one active selection names through the discovery-owned surface guard.
pub(in crate::muterprater) fn selected_alternative(
    surface: &EvaluationSurface,
    selection: ActiveSelection,
) -> Result<(&MutationPoint, &AdmittedAlternative), SelectionRefusal> {
    surface.selected_alternative(selection)
}

/// Plan one interpreted pass over every admitted alternative on a surface.
///
/// A point-free surface yields [`PlanRefusal::NoRunPlanned`]: it stays a lawful parity surface and does not impersonate a mutation campaign.
///
/// # Errors
///
/// Refuses a surface admitting no active run, then a roster larger than the scope's mutant budget.
pub fn plan_pass(
    surface: &EvaluationSurface,
    scope: ScopedInvocation,
) -> Result<ProofPlan, PlanRefusal> {
    let budget = scope.budget();
    let mut runs = Vec::new();
    for point in surface.points() {
        let claims = BTreeSet::from([point.owner_claim()]);
        for alternative in point.admitted_alternatives() {
            runs.push(PlannedRun::intended(
                PressureLane::InterpretedMutation,
                MutationIdentity::Interpreted {
                    point: point.identity(),
                    alternative: alternative.identity(),
                },
                PlannedDamage::Alternative(alternative.identity()),
                Selection::ByClaim(claims.clone()),
                budget,
            ));
        }
    }
    ProofPlan::planned(scope, runs)
}

/// Execute production and the no-mutation evaluation over one exact input, and retain both meanings and both reports.
///
/// The same [`MutationWitness`] judges both meanings, and both observations join its exact trial binding.
/// The reading that comes back is evidence input rather than qualification; [`qualify_no_mutation`] decides whether it earned that standing.
///
/// # Errors
///
/// Refuses the shared-substrate declaration, then a production or evaluation observation that cannot join the witness binding.
pub fn observe_no_mutation<'pair, 'input, Input, Meaning>(
    pair: &'pair EvaluationPair<Input, Meaning>,
    witness: MutationWitness<Meaning>,
    input: &'input Input,
    invocation: &Invocation,
) -> Result<NoMutationParityReading<'pair, 'input, Input, Meaning>, NoMutationObservationRefusal> {
    let substrate = no_mutation_substrate().map_err(NoMutationObservationRefusal::Substrate)?;
    let cause = parity_cause().map_err(NoMutationObservationRefusal::Substrate)?;
    let trial = trial_identity(witness.binding().row());

    let production_measurement = invocation.clock().begin();
    let production = pair.production().evaluate(input);
    let production_conclusion = witness.conclude(&production);
    let production_report = record_one(
        witness.binding(),
        invocation,
        HostTrialRecord::recorded(
            trial,
            RunAttempt::Executed(production_conclusion),
            production_measurement.finish(),
        ),
    )
    .map_err(NoMutationObservationRefusal::ProductionReport)?;

    let evaluation_measurement = invocation.clock().begin();
    let observed = pair
        .evaluation()
        .evaluate(input, EvaluationDirective::no_mutation())
        .map_err(NoMutationObservationRefusal::EvaluationCall)?;
    let (evaluation, evaluation_firings) = observed.into_parts();
    let evaluation_conclusion = witness.conclude(&evaluation);
    let evaluation_report = record_one(
        witness.binding(),
        invocation,
        HostTrialRecord::recorded(
            trial,
            RunAttempt::Executed(evaluation_conclusion),
            evaluation_measurement.finish(),
        ),
    )
    .map_err(NoMutationObservationRefusal::EvaluationReport)?;

    let conclusion = agreement(pair.equivalence(), &production, &evaluation, cause);
    Ok(NoMutationParityReading::recorded(
        pair,
        witness,
        input,
        NoMutationResults::observed(production, evaluation, evaluation_firings),
        substrate,
        conclusion,
        NoMutationReports::recorded(production_report, evaluation_report),
    ))
}

/// Read whether one complete no-mutation comparison earned scoped qualification.
///
/// Qualification takes both reports concluding lawfully, zero activation under the no-mutation directive, and agreement under the owner-declared equivalence.
/// A rejection retains the entire reading and the exact stage that failed.
#[must_use]
pub fn qualify_no_mutation<'pair, 'input, Input, Meaning>(
    reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
) -> NoMutationParityStanding<'pair, 'input, Input, Meaning> {
    let cause = if lens_verdict(reading.production_report()).is_err() {
        Some(ParityQualificationRefusal::ProductionDidNotQualify)
    } else if lens_verdict(reading.evaluation_report()).is_err() {
        Some(ParityQualificationRefusal::EvaluationDidNotQualify)
    } else if reading.evaluation_firings() != 0 {
        Some(ParityQualificationRefusal::NoMutationActivated {
            firings: reading.evaluation_firings(),
        })
    } else if matches!(reading.conclusion(), TrialConclusion::Refused(_)) {
        Some(ParityQualificationRefusal::MeaningsDisagreed)
    } else {
        None
    };
    match cause {
        Some(refusal) => {
            NoMutationParityStanding::Rejected(RejectedNoMutationParity::rejected(refusal, reading))
        }
        None => {
            NoMutationParityStanding::Qualified(NoMutationParityQualification::qualified(reading))
        }
    }
}

/// Read whether interpreted evidence is available for one exact surface and selected projection.
///
/// Generic suite pressure establishes that the qualified external suite bites and carries no pair authority.
/// Exact projection pressure retains mandatory no-mutation parity, one exact pair, and one surface-issued selection.
/// A lawful point-free surface can still qualify parity, and it cannot produce projection pressure, so it never opens active trust.
#[must_use]
pub fn availability<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>(
    surface: Option<&'surface EvaluationSurface>,
    suite: Option<&'suite CompiledSuitePressure>,
    projection: Option<
        &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
    >,
) -> InterpreterAvailability<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
{
    let Some(surface) = surface else {
        return InterpreterAvailability::NoConformingSurface;
    };
    let Some(suite) = suite else {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledSuitePressure,
        };
    };
    let Some(projection) = projection else {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledProjectionPressure,
        };
    };
    let standing = projection.standing();
    if standing.pair().family() != surface.family()
        || standing.pair().surface() != surface.identity()
        || standing.selection().surface() != surface.identity()
    {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::ProjectionPressureForAnotherSurface,
        };
    }
    InterpreterAvailability::Available(InterpretedTrust::opened(surface, suite, projection))
}

/// Execute the one active selection opened trust retains, and admit it through the trial and mutation report spines.
///
/// The input and the evaluation callable are the exact ones the no-mutation qualification retains.
/// A positive firing count becomes an activation observation bound to that selection and witness; zero returns a [`DudPlant`] and no evidence exists.
///
/// # Errors
///
/// Refuses an invocation that does not reproduce the compiled projection's execution, a witness trial owned by another claim, an omitted evaluation branch, a callback report of zero firings, or an observation that could not join the witness binding.
pub fn execute_active<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>(
    trust: &InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>,
    invocation: &Invocation,
) -> Result<
    InterpretedMutationEvidence<
        'surface,
        'suite,
        'projection,
        'parity,
        'pair,
        'input,
        Input,
        Meaning,
    >,
    InterpretedExecutionRefusal,
> {
    let selection = trust.selection();
    let (point, alternative) = selected_alternative(trust.surface(), selection)
        .map_err(InterpretedExecutionRefusal::Selection)?;
    let parity = trust.parity();
    let pair = parity.reading().pair();
    let input = parity.reading().input();
    let witness = parity.reading().witness();
    let witness_claim = witness.binding().row().claim();
    if witness_claim != point.owner_claim() {
        return Err(InterpretedExecutionRefusal::WitnessForAnotherClaim {
            expected: point.owner_claim(),
            found: witness_claim,
        });
    }
    let execution = execution_key(witness.binding(), invocation);
    if trust.projection().standing().execution() != &execution {
        return Err(InterpretedExecutionRefusal::InvocationForAnotherExecution);
    }
    let trial = trial_identity(witness.binding().row());
    let measurement = invocation.clock().begin();
    let observed = pair
        .evaluation()
        .evaluate(
            input,
            EvaluationDirective::active(selection, point, alternative),
        )
        .map_err(InterpretedExecutionRefusal::EvaluationCall)?;
    let (meaning, firings) = observed.into_parts();
    let Some(activation) = ActivationEvidence::observed(selection, trial, firings) else {
        return Err(InterpretedExecutionRefusal::DudPlant(Box::new(
            DudPlant::unfired(selection, trial),
        )));
    };
    let conclusion = witness.conclude(&meaning);
    let report = record_one(
        witness.binding(),
        invocation,
        HostTrialRecord::recorded(
            trial,
            RunAttempt::Executed(conclusion.clone()),
            measurement.finish(),
        ),
    )
    .map_err(InterpretedExecutionRefusal::Report)?;
    let target = MutationTarget::pressed(
        MutationIdentity::Interpreted {
            point: point.identity(),
            alternative: alternative.identity(),
        },
        FamilyAttribution::Declared(alternative.family()),
        MutationSite::Declared(point.activation_site()),
        MappingPosture::Mapped(point.owner_claim()),
    );
    let mutation = MutationReport::interpreted(target, activation, &report);
    Ok(InterpretedMutationEvidence::admitted(
        trust.duplicate(),
        meaning,
        report,
        mutation,
    ))
}

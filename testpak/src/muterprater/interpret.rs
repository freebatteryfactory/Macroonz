//! The compile-once mutation receiver: stable selection, exact no-mutation parity, and admitted interpreted execution.
//!
//! Production is an ordinary callable with no control. The evaluation copy alone receives [`EvaluationControl`]. Point-catalog posture and parity qualification are independent: a point-free surface is lawful, and neither a point-free nor mutable surface earns parity until the exact production/evaluation pair executes under [`EvaluationControl::NoMutation`] and its reports and meanings qualify.
//!
//! Compiled source pressure and interpreted activation remain different evidence routes. [`availability`] joins them only when both name the same exact production/evaluation pair and surface; it does not flatten their provenance or claim ceilings. The active road then reuses the exact input and mutation witness retained by the no-mutation qualification and admits evidence only after the selected damage reports a positive firing count.
//!
//! The caller-supplied production, evaluation, and check function pointers retain their ordinary Rust effect and unwind ceilings. This module records their returned facts and delegates wall measurement to the `TestPak` clock owner; measurement posture never enters parity or mutation classification.

use super::types::{
    ActivationEvidence, ActiveSelection, AdmittedAlternative, CompiledPressureWitness,
    EvaluationControl, EvaluationSurface, FamilyAttribution, InterpretedExecutionRefusal,
    InterpretedMutationEvidence, InterpretedTrust, InterpreterAvailability, MUTERPRATER_NAMESPACE,
    MappingPosture, MissingTrustEvidence, MutationIdentity, MutationPoint, MutationSite,
    MutationTarget, MutationWitness, NO_MUTATION_PAIRING, NoMutationObservationRefusal,
    NoMutationParityQualification, NoMutationParityReading, NoMutationParityStanding,
    NoMutationReports, NoMutationResults, PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE,
    ParityQualificationRefusal, ParityRefusal, PlanRefusal, PlannedDamage, PlannedRun,
    PressureLane, ProofPlan, RejectedNoMutationParity, ScopedInvocation, SelectionRefusal,
};
use crate::descriptor::{MutationPointRef, NamespacedName};
use crate::properties::{SharedSubstrate, SubstrateRef, SubstrateRoster, agreement};
use crate::report::{FindingCause, HostTrialRecord, RunAttempt, TrialConclusion};
use crate::runner::{Invocation, lens_verdict, record_one, trial_identity};
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

/// Find and validate the exact alternative one active selection names.
fn selected_alternative(
    surface: &EvaluationSurface,
    selection: ActiveSelection,
) -> Result<(&MutationPoint, &AdmittedAlternative), SelectionRefusal> {
    if selection.surface() != surface.identity() {
        return Err(SelectionRefusal::SelectionFromAnotherSurface {
            expected: surface.identity(),
            found: selection.surface(),
        });
    }
    let Some(point) = surface.point(selection.point()) else {
        return Err(SelectionRefusal::NoSuchPoint(selection.point()));
    };
    let Some(alternative) = point
        .admitted_alternatives()
        .iter()
        .find(|alternative| alternative.identity() == selection.alternative())
    else {
        return Err(SelectionRefusal::NoSuchAlternative {
            point: selection.point(),
            alternative: selection.alternative(),
        });
    };
    Ok((point, alternative))
}

/// What one point reads as under one evaluation control.
///
/// No mutation returns every point's original bytes. An active control changes exactly the selected point to the canonical bytes behind its stable alternative identity; every other point remains original.
///
/// # Errors
///
/// Refuses a selection issued by another surface, a point absent from this surface, or an alternative absent from that point.
pub fn point_reading(
    surface: &EvaluationSurface,
    point: MutationPointRef,
    control: EvaluationControl,
) -> Result<&[u8], SelectionRefusal> {
    let Some(found) = surface.point(point) else {
        return Err(SelectionRefusal::NoSuchPoint(point));
    };
    let EvaluationControl::Active(selection) = control else {
        return Ok(found.original_operation());
    };
    if selection.surface() != surface.identity() {
        return Err(SelectionRefusal::SelectionFromAnotherSurface {
            expected: surface.identity(),
            found: selection.surface(),
        });
    }
    if selection.point() != point {
        return Ok(found.original_operation());
    }
    let (_, alternative) = selected_alternative(surface, selection)?;
    Ok(alternative.operation())
}

/// Plan one interpreted pass over every admitted alternative on a surface.
///
/// A point-free surface yields [`PlanRefusal::NoRunPlanned`]; it remains a lawful parity surface and does not impersonate a mutation campaign.
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
                crate::runner::Selection::ByClaim(claims.clone()),
                budget,
            ));
        }
    }
    ProofPlan::planned(scope, runs)
}

/// Execute production and the no-mutation evaluation copy over one exact input and retain both meanings and both ordinary trial reports.
///
/// The same [`MutationWitness`] judges both meanings and both host observations join its exact [`crate::runner::TrialBinding`]. The returned reading is evidence input, not qualification; [`qualify_no_mutation`] decides whether it earned that standing.
///
/// # Errors
///
/// Refuses the shared-substrate declaration, then a production or evaluation observation that cannot join the witness binding.
pub fn observe_no_mutation<'pair, 'input, Input, Meaning>(
    pair: &'pair super::EvaluationPair<Input, Meaning>,
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
        .evaluate(input, EvaluationControl::NoMutation);
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
/// Qualification requires both ordinary trial reports to conclude lawfully, zero activation under the no-mutation control, and agreement under the owner-declared equivalence. Rejection retains the entire reading and the exact failed stage.
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

/// Read whether interpreted evidence is available for one exact surface and production/evaluation pair.
///
/// The gate consumes the compiled-pressure witness and no-mutation qualification themselves. It checks both against the surface and pair rather than accepting parallel labels. A lawful point-free surface can open this trust boundary but still has no [`ActiveSelection`] to execute.
#[must_use]
pub fn availability<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>(
    surface: Option<&'surface EvaluationSurface>,
    compiled: Option<&'compiled CompiledPressureWitness>,
    parity: Option<&'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>>,
) -> InterpreterAvailability<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning> {
    let Some(surface) = surface else {
        return InterpreterAvailability::NoConformingSurface;
    };
    let Some(compiled) = compiled else {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledPressure,
        };
    };
    if compiled.pair().family() != surface.family()
        || compiled.pair().surface() != surface.identity()
    {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledPressureForAnotherPair,
        };
    }
    let Some(parity) = parity else {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::NoMutationParity,
        };
    };
    let standing = parity.reading().pair().standing();
    if standing.family() != surface.family() || standing.surface() != surface.identity() {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::ParityForAnotherSurface,
        };
    }
    if compiled.pair() != standing {
        return InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledPressureForAnotherPair,
        };
    }
    InterpreterAvailability::Available(InterpretedTrust::opened(surface, compiled, parity))
}

/// Execute one active selection under opened trust and admit it through the ordinary trial and mutation report spines.
///
/// The input is the exact input retained by the no-mutation qualification. The evaluation callable is the exact callable retained by that qualification's pair. A positive firing count becomes an activation observation bound to the exact selection and witness; zero returns a [`super::DudPlant`] and no mutation evidence exists. The callback supplies the count, and this road does not independently instrument it.
///
/// # Errors
///
/// Refuses a selection outside the opened surface, a parity witness trial owned by another claim, a callback report of zero firings for the selected damage, or a host observation that could not join that exact witness binding. Selection and claim membership are validated before any caller evaluation or clock callable runs.
#[expect(
    clippy::result_large_err,
    reason = "the rare dud refusal retains the exact surface-issued selection and trial; indirection would add allocation without changing this harness-only control path"
)]
pub fn execute_active<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>(
    trust: &InterpretedTrust<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>,
    selection: ActiveSelection,
    invocation: &Invocation,
) -> Result<
    InterpretedMutationEvidence<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>,
    InterpretedExecutionRefusal,
> {
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
    let trial = trial_identity(witness.binding().row());
    let measurement = invocation.clock().begin();
    let observed = pair
        .evaluation()
        .evaluate(input, EvaluationControl::Active(selection));
    let (meaning, firings) = observed.into_parts();
    let Some(activation) = ActivationEvidence::observed(selection, trial, firings) else {
        return Err(InterpretedExecutionRefusal::DudPlant(
            super::DudPlant::unfired(selection, trial),
        ));
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
    let mutation = super::MutationReport::interpreted(target, activation, &report);
    Ok(InterpretedMutationEvidence::admitted(
        trust.duplicate(),
        selection,
        meaning,
        report,
        mutation,
    ))
}

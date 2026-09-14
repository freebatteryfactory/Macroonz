//! Independent witness judgments over retained mutation results.

use crate::muterprater::{
    ActivationEvidence, EquivalenceAxis, FamilyAttribution, MappingPosture, MutationAssessment,
    MutationIdentity, MutationReport, MutationSite, MutationTarget, MutationWitness,
    MutationWitnessObservationRefusal, MutationWitnessQualificationRefusal, MutationWitnessReading,
    QualifiedMutation, RejectedMutationWitness,
};
use crate::properties::Agreement;
use crate::report::{HostTrialRecord, RunAttempt, TrialReport};
use crate::runner::{Invocation, lens_verdict, record_one, trial_identity};

/// Run a declared witness against each retained result without rerunning the subject.
///
/// # Errors
///
/// Refuses a foreign context or claim before invoking the witness, then any report join failure.
pub fn observe_witness<'scope, Input, Meaning>(
    qualification: &'scope QualifiedMutation<'scope, Input, Meaning>,
    witness: MutationWitness<Meaning>,
    invocation: &Invocation,
) -> Result<MutationWitnessReading<'scope, Input, Meaning>, MutationWitnessObservationRefusal> {
    let observed = qualification.compiled().observation();
    if observed.context().profile() != invocation.profile()
        || observed.context().target() != invocation.target()
    {
        return Err(MutationWitnessObservationRefusal::Context);
    }
    let expected = observed.point().owner_claim();
    let found = witness.binding().row().claim();
    if expected != found {
        return Err(MutationWitnessObservationRefusal::Claim { expected, found });
    }
    let attempts = [
        judged(&witness, invocation, observed.production()),
        judged(&witness, invocation, qualification.baseline()),
        judged(&witness, invocation, qualification.compiled_baseline()),
        judged(&witness, invocation, qualification.compiled_selected()),
        judged(&witness, invocation, qualification.selected()),
    ];
    let [
        production,
        baseline,
        compiled_baseline,
        compiled_selected,
        selected,
    ] = attempts;
    let reports = [
        production?,
        baseline?,
        compiled_baseline?,
        compiled_selected?,
        selected?,
    ];
    Ok(MutationWitnessReading::observed(
        qualification,
        witness,
        reports,
    ))
}

fn judged<Meaning>(
    witness: &MutationWitness<Meaning>,
    invocation: &Invocation,
    meaning: &Meaning,
) -> Result<TrialReport, MutationWitnessObservationRefusal> {
    let measurement = invocation.clock().begin();
    let conclusion = witness.conclude(meaning);
    record_one(
        witness.binding(),
        invocation,
        HostTrialRecord::recorded_with_attribution(
            trial_identity(witness.binding().row()),
            RunAttempt::Executed(conclusion),
            measurement.finish(),
            invocation.clock().attribution(),
        ),
    )
    .map_err(MutationWitnessObservationRefusal::Report)
}

/// Qualify a witness's passing baseline and agreement across selected execution roads.
///
/// # Errors
///
/// Retains all judgments beside the first failing baseline or selected-conclusion obligation.
pub fn qualify_witness<Input, Meaning>(
    reading: MutationWitnessReading<'_, Input, Meaning>,
) -> Result<MutationAssessment<'_, Input, Meaning>, RejectedMutationWitness<'_, Input, Meaning>> {
    if let Some(cause) = witness_refusal(&reading) {
        return Err(RejectedMutationWitness::rejected(reading, cause));
    }
    let qualification = reading.qualification();
    let observed = qualification.compiled().observation();
    let selection = observed.selection();
    let target = MutationTarget::pressed(
        MutationIdentity::Interpreted {
            point: observed.point().identity(),
            alternative: observed.alternative().identity(),
        },
        FamilyAttribution::Declared(observed.alternative().family()),
        MutationSite::Declared(observed.point().activation_site()),
        MappingPosture::Mapped(observed.point().owner_claim()),
    );
    let activation = ActivationEvidence::reported(
        selection,
        reading.selected_report().trial(),
        qualification.firings(),
    );
    let equivalence = match qualification.difference() {
        Agreement::Differs => EquivalenceAxis::Refuted,
        Agreement::Agrees => EquivalenceAxis::NotAssessed,
    };
    let mutation =
        MutationReport::assessed(target, activation, reading.selected_report(), equivalence);
    Ok(MutationAssessment::qualified(reading, mutation))
}

fn witness_refusal<Input, Meaning>(
    reading: &MutationWitnessReading<'_, Input, Meaning>,
) -> Option<MutationWitnessQualificationRefusal> {
    if lens_verdict(reading.production_report()).is_err() {
        Some(MutationWitnessQualificationRefusal::ProductionDidNotPass)
    } else if lens_verdict(reading.baseline_report()).is_err() {
        Some(MutationWitnessQualificationRefusal::BaselineDidNotPass)
    } else if lens_verdict(reading.compiled_baseline_report()).is_err() {
        Some(MutationWitnessQualificationRefusal::CompiledBaselineDidNotPass)
    } else if reading.selected_report().attempt() != reading.compiled_selected_report().attempt() {
        Some(MutationWitnessQualificationRefusal::SelectedReportsDisagreed)
    } else {
        None
    }
}

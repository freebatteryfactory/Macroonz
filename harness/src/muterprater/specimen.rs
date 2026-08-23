//! Exact compiled pressure over one separately materialized selected projection.
//!
//! The materializer is a capture-free function pointer that returns exact source bytes for the unchanged production shape or for one `TestPak`-resolved selection baked into that shape. `TestPak` derives bytes-only identities, hands immutable requests to a caller-owned host adapter, and judges the returned meanings through the retained ordinary trial binding. The library performs no filesystem or process operation; the permanent outside-consumer lane observes that its concrete host writes the exact bytes, invokes the declared compiler and target, and executes the artifact.
//!
//! Generic cargo-mutants pressure remains a separate evidence book. It demonstrates that a qualified external suite bit somewhere under its adapter profile, but it carries no exact pair or selection authority and cannot substitute for this operation.

use super::interpret::selected_alternative;
use super::types::{
    ActiveSelection, AdmittedAlternative, ArtifactContent, CompiledProjectionPressure,
    CompiledProjectionRefusal, CompiledSpecimenHost, CompiledSpecimenHostRefusal,
    CompiledSpecimenObservationMismatch, CompiledSpecimenRequest, CompiledSpecimenRole,
    CompiledSpecimenStanding, EvaluationDirective, EvaluationSurface, FamilyAttribution,
    MappingPosture, MutationIdentity, MutationPoint, MutationSite, MutationTarget, MutationWitness,
    NoMutationParityQualification, SpecimenMaterializerBinding,
};
use crate::descriptor::CheckRef;
use crate::report::{ExecutionKey, HostTrialRecord, RunAttempt, TrialReport};
use crate::runner::{
    Invocation, ReportRecordingRefusal, execution_key, lens_verdict, record_one, trial_identity,
};

/// The two immutable source artifacts rendered before any host effect.
struct MaterializedSpecimens {
    baseline: ArtifactContent,
    selected: ArtifactContent,
}

/// Why one private host observation did not become an ordinary report.
enum ObservationRefusal {
    Host(CompiledSpecimenHostRefusal),
    Foreign(CompiledSpecimenObservationMismatch),
    Report(ReportRecordingRefusal),
}

/// The already-validated execution facts shared by baseline and selected observations.
struct ObservationSeat<'standing, 'input, Input, Meaning> {
    input: &'input Input,
    execution: &'standing ExecutionKey,
    check: CheckRef,
    witness: &'standing MutationWitness<Meaning>,
    invocation: &'standing Invocation,
    host: CompiledSpecimenHost<Input, Meaning>,
}

impl<Input, Meaning> ObservationSeat<'_, '_, Input, Meaning> {
    /// Run one immutable request through the caller host and ordinary report owner.
    fn observe<'content>(
        &self,
        content: &'content ArtifactContent,
        role: CompiledSpecimenRole,
        operation: &'content [u8],
    ) -> Result<TrialReport, ObservationRefusal> {
        let measurement = self.invocation.clock().begin();
        let request = CompiledSpecimenRequest::requested(
            content,
            role,
            operation,
            self.input,
            self.execution,
            self.check,
        );
        let expected_content = request.content().identity();
        let expected_role = request.role();
        let expected_execution = request.execution().clone();
        let expected_check = request.check();
        let observation = (self.host)(request).map_err(ObservationRefusal::Host)?;
        if let Some(mismatch) = observation.mismatch(
            expected_content,
            expected_role,
            &expected_execution,
            expected_check,
        ) {
            return Err(ObservationRefusal::Foreign(mismatch));
        }
        let meaning = observation.into_meaning();
        record_one(
            self.witness.binding(),
            self.invocation,
            HostTrialRecord::recorded(
                trial_identity(self.witness.binding().row()),
                RunAttempt::Executed(self.witness.conclude(&meaning)),
                measurement.finish(),
            ),
        )
        .map_err(ObservationRefusal::Report)
    }
}

/// Render both artifact roles before host effects and require different exact bytes.
fn materialize_specimens(
    materializer: &SpecimenMaterializerBinding,
    selection: ActiveSelection,
    point: &MutationPoint,
    alternative: &AdmittedAlternative,
) -> Result<MaterializedSpecimens, CompiledProjectionRefusal> {
    let render = materializer.call();
    let baseline = ArtifactContent::recorded(
        render(EvaluationDirective::no_mutation())
            .map_err(CompiledProjectionRefusal::BaselineMaterialization)?,
    );
    let selected = ArtifactContent::recorded(
        render(EvaluationDirective::active(selection, point, alternative))
            .map_err(CompiledProjectionRefusal::SelectedMaterialization)?,
    );
    if baseline.identity() == selected.identity() {
        return Err(CompiledProjectionRefusal::ArtifactDidNotChange(
            baseline.identity(),
        ));
    }
    Ok(MaterializedSpecimens { baseline, selected })
}

/// Materialize, compile, execute, and judge one exact selected mutation projection.
///
/// Every structural join is checked before either caller-owned renderer or host callback runs. Both unchanged and selected source are rendered before any host effect; their bytes must differ. Each host observation is admitted through the exact retained [`crate::runner::TrialBinding`], the unchanged report must pass, and the selected report must refuse before exact pressure exists.
///
/// # Authority
///
/// Function pointers remain caller statements. This operation binds their returned bytes and meanings to exact requests and ordinary reports; it does not independently prove a compiler process ran. The outside-consumer lane owns that behavioral claim for the admitted concrete host adapter.
///
/// # Errors
///
/// Refuses, before caller code, a parity qualification for another surface, a materializer for another pair, a foreign selection, an unrelated witness claim, or an invocation that does not reproduce the qualified execution. It then refuses baseline or selected materialization, byte-identical artifacts, host failures, report joins, a baseline that does not pass, or a selected artifact the exact witness does not reject.
pub fn demonstrate_compiled_projection<'parity, 'pair, 'input, Input, Meaning>(
    surface: &EvaluationSurface,
    parity: &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>,
    materializer: &SpecimenMaterializerBinding,
    selection: ActiveSelection,
    invocation: &Invocation,
    host: CompiledSpecimenHost<Input, Meaning>,
) -> Result<
    CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
    CompiledProjectionRefusal,
> {
    let pair = parity.reading().pair().standing();
    if pair.surface() != surface.identity() {
        return Err(CompiledProjectionRefusal::ParityForAnotherSurface {
            expected: surface.identity(),
            found: pair.surface(),
        });
    }
    if let Some(mismatch) = pair.mismatch(materializer.pair()) {
        return Err(CompiledProjectionRefusal::MaterializerForAnotherPair(
            mismatch,
        ));
    }
    let (point, alternative) =
        selected_alternative(surface, selection).map_err(CompiledProjectionRefusal::Selection)?;
    let witness = parity.reading().witness();
    let witness_claim = witness.binding().row().claim();
    if witness_claim != point.owner_claim() {
        return Err(CompiledProjectionRefusal::WitnessForAnotherClaim {
            expected: point.owner_claim(),
            found: witness_claim,
        });
    }
    let execution = execution_key(witness.binding(), invocation);
    if parity.reading().production_report().standing().key() != &execution {
        return Err(CompiledProjectionRefusal::InvocationForAnotherExecution);
    }

    let specimens = materialize_specimens(materializer, selection, point, alternative)?;
    let check = witness.check_ref();
    let observer = ObservationSeat {
        input: parity.reading().input(),
        execution: &execution,
        check,
        witness,
        invocation,
        host,
    };
    let baseline_report = observer
        .observe(
            &specimens.baseline,
            CompiledSpecimenRole::Baseline,
            point.original_operation(),
        )
        .map_err(|refusal| match refusal {
            ObservationRefusal::Host(cause) => CompiledProjectionRefusal::BaselineHost(cause),
            ObservationRefusal::Foreign(cause) => {
                CompiledProjectionRefusal::BaselineObservation(cause)
            }
            ObservationRefusal::Report(cause) => CompiledProjectionRefusal::BaselineReport(cause),
        })?;
    if lens_verdict(&baseline_report).is_err() {
        return Err(CompiledProjectionRefusal::BaselineDidNotQualify);
    }

    let selected_report = observer
        .observe(
            &specimens.selected,
            CompiledSpecimenRole::Selected(selection),
            alternative.operation(),
        )
        .map_err(|refusal| match refusal {
            ObservationRefusal::Host(cause) => CompiledProjectionRefusal::SelectedHost(cause),
            ObservationRefusal::Foreign(cause) => {
                CompiledProjectionRefusal::SelectedObservation(cause)
            }
            ObservationRefusal::Report(cause) => CompiledProjectionRefusal::SelectedReport(cause),
        })?;

    let target = MutationTarget::pressed(
        MutationIdentity::CompiledProjection {
            point: point.identity(),
            alternative: alternative.identity(),
        },
        FamilyAttribution::Declared(alternative.family()),
        MutationSite::Declared(point.activation_site()),
        MappingPosture::Mapped(point.owner_claim()),
    );
    let Some(mutation) = super::MutationReport::compiled_projection(target, &selected_report)
    else {
        return Err(CompiledProjectionRefusal::ProjectionDidNotReject);
    };
    let standing = CompiledSpecimenStanding::recorded(
        specimens.selected.identity(),
        pair,
        selection,
        execution,
        check,
    );
    Ok(CompiledProjectionPressure::demonstrated(
        parity,
        specimens.baseline.identity(),
        standing,
        baseline_report,
        selected_report,
        mutation,
    ))
}

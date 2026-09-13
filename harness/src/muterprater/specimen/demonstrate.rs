//! Exact compiled pressure over one separately materialized selected projection.
//!
//! The materializer is a capture-free function pointer returning source bytes for the unchanged production shape, or for one resolved selection baked into that shape.
//! The harness derives bytes-only identities, hands immutable requests to a caller-owned host adapter, and judges the returned meanings through the retained trial binding.
//! It performs no filesystem or process operation of its own: that the concrete host writes those exact bytes, invokes the declared compiler and target, and executes the artifact is the caller's lane to observe.
//!
//! Generic pressure from a wrapped backend is a separate evidence book.
//! It shows that a qualified external suite bit somewhere under its adapter profile, and it carries no pair or selection authority, so it cannot substitute for this road.

use super::observe::observe_specimen;
use super::types::{
    ArtifactContent, CompiledProjectionPressure, CompiledProjectionRefusal,
    CompiledSpecimenContext, CompiledSpecimenHost, CompiledSpecimenHostRefusal,
    CompiledSpecimenObservationMismatch, CompiledSpecimenRole, CompiledSpecimenStanding,
    SpecimenMaterializerBinding, SpecimenObservationRefusal,
};
use crate::muterprater::{
    ActiveSelection, AdmittedAlternative, EvaluationDirective, EvaluationPairStanding,
    EvaluationSurface, FamilyAttribution, MappingPosture, MutationIdentity, MutationPoint,
    MutationReport, MutationSite, MutationTarget, MutationWitness, NoMutationParityQualification,
};
use crate::report::{ExecutionKey, HostTrialRecord, RunAttempt, TrialReport};
use crate::runner::{
    Invocation, ReportRecordingRefusal, execution_key, lens_verdict, record_one, trial_identity,
};

/// Why one host observation did not become a report.
enum ObservationRefusal {
    Host(CompiledSpecimenHostRefusal),
    Foreign(CompiledSpecimenObservationMismatch),
    Report(ReportRecordingRefusal),
}

/// The already-validated execution facts the baseline and selected observations share.
struct ObservationSeat<'standing, 'input, Input, Meaning> {
    input: &'input Input,
    context: &'standing CompiledSpecimenContext,
    witness: &'standing MutationWitness<Meaning>,
    invocation: &'standing Invocation,
    host: CompiledSpecimenHost<Input, Meaning>,
}

impl<Input, Meaning> ObservationSeat<'_, '_, Input, Meaning> {
    /// Run one immutable request through the caller's host, then through the report owner.
    fn observe<'content>(
        &self,
        content: &'content ArtifactContent,
        role: CompiledSpecimenRole,
        operation: &'content [u8],
    ) -> Result<TrialReport, ObservationRefusal> {
        let measurement = self.invocation.clock().begin();
        let meaning = observe_specimen(
            content,
            role,
            operation,
            self.input,
            self.context,
            self.host,
        )
        .map_err(|cause| match cause {
            SpecimenObservationRefusal::Host(cause) => ObservationRefusal::Host(cause),
            SpecimenObservationRefusal::Foreign(cause) => ObservationRefusal::Foreign(cause),
        })?;
        record_one(
            self.witness.binding(),
            self.invocation,
            HostTrialRecord::recorded_with_attribution(
                trial_identity(self.witness.binding().row()),
                RunAttempt::Executed(self.witness.conclude(&meaning)),
                measurement.finish(),
                self.invocation.clock().attribution(),
            ),
        )
        .map_err(ObservationRefusal::Report)
    }
}

/// Render both artifact roles before any host effect, and require different exact bytes.
pub(super) fn materialize_specimens(
    materializer: &SpecimenMaterializerBinding,
    selection: ActiveSelection,
    point: &MutationPoint,
    alternative: &AdmittedAlternative,
) -> Result<(ArtifactContent, ArtifactContent), CompiledProjectionRefusal> {
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
    Ok((baseline, selected))
}

/// What the structural joins established, before any caller callback runs.
struct Joined<'surface> {
    point: &'surface MutationPoint,
    alternative: &'surface AdmittedAlternative,
    pair: EvaluationPairStanding,
    execution: ExecutionKey,
}

/// Check every structural join this road stands on, before either caller callback runs.
fn joined<'surface, Input, Meaning>(
    surface: &'surface EvaluationSurface,
    parity: &NoMutationParityQualification<'_, '_, Input, Meaning>,
    materializer: &SpecimenMaterializerBinding,
    selection: ActiveSelection,
    invocation: &Invocation,
) -> Result<Joined<'surface>, CompiledProjectionRefusal> {
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
    let (point, alternative) = surface
        .selected_alternative(selection)
        .map_err(CompiledProjectionRefusal::Selection)?;
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
    Ok(Joined {
        point,
        alternative,
        pair,
        execution,
    })
}

/// Materialize, compile, execute, and judge one exact selected mutation projection.
///
/// Both unchanged and selected source are rendered before any host effect, and their bytes must differ.
/// Each host observation is admitted through the retained [`crate::runner::TrialBinding`]; the unchanged report must pass, and the selected report must refuse, before exact pressure exists.
///
/// Function pointers remain caller statements: this operation binds their returned bytes and meanings to exact requests and reports, and does not prove that a compiler process ran.
///
/// # Errors
///
/// Refuses, before any caller code runs, a parity qualification for another surface, a materializer for another pair, a foreign selection, an unrelated witness claim, or an invocation that does not reproduce the qualified execution.
/// It then refuses baseline or selected materialization, byte-identical artifacts, host failures, report joins, a baseline that does not pass, or a selected artifact the exact witness does not reject.
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
    let Joined {
        point,
        alternative,
        pair,
        execution,
    } = joined(surface, parity, materializer, selection, invocation)?;
    let witness = parity.reading().witness();
    let check = witness.check_ref();

    let (baseline_content, selected_content) =
        materialize_specimens(materializer, selection, point, alternative)?;
    let context = CompiledSpecimenContext::recorded(pair, invocation);
    let observer = ObservationSeat {
        input: parity.reading().input(),
        context: &context,
        witness,
        invocation,
        host,
    };

    let baseline_report = observer
        .observe(
            &baseline_content,
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
            &selected_content,
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
    let Some(mutation) = MutationReport::compiled_projection(target, &selected_report) else {
        return Err(CompiledProjectionRefusal::ProjectionDidNotReject);
    };
    let standing = CompiledSpecimenStanding::recorded(
        selected_content.identity(),
        pair,
        selection,
        execution,
        check,
    );
    Ok(CompiledProjectionPressure::demonstrated(
        parity,
        baseline_content,
        selected_content,
        standing,
        baseline_report,
        selected_report,
        mutation,
    ))
}

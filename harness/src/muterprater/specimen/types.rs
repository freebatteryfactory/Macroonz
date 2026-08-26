//! Exact compiled selected-projection content, host requests, standings, and pressure.

use crate::descriptor::{CheckRef, ClaimRef};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::{
    ActiveSelection, EvaluationDirective, EvaluationPairStanding, EvaluationPairStandingMismatch,
    EvaluationSurfaceId, MutationReport, NoMutationParityQualification, SelectionRefusal,
};
use crate::report::{ExecutionKey, ForeignText, TrialReport};
use crate::runner::ReportRecordingRefusal;
#[path = "type_guard.rs"]
mod guard;

/// The domain tag of the exact source bytes one compiled specimen host consumes.
pub const ARTIFACT_CONTENT_TAG: DomainTag = DomainTag::declared(
    "compiled-artifact-content",
    IdentityProfileVersion::declared(1),
);

// ---------------------------------------------------------------------------
// Exact compiled selected-projection pressure.
// ---------------------------------------------------------------------------

/// The bytes handed unchanged to one compiled-specimen host, under their bytes-only content identity.
///
/// The identity commits to no pair, selection, target, toolchain, or caller label; those relationships live in [`CompiledSpecimenStanding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactContent {
    identity: ArtifactContentId,
    bytes: Vec<u8>,
}

/// The bytes-only content identity of one compiler-source artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactContentId(ContentAddress);

/// Whether one compiled specimen is the unchanged baseline or one exact selected mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompiledSpecimenRole {
    /// The production-shaped artifact under no mutation.
    Baseline,
    /// The production-shaped artifact with this surface-issued selection baked in.
    Selected(ActiveSelection),
}

/// Why one specimen source could not be rendered.
#[must_use = "a refusal is the reason one specimen source was not rendered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecimenMaterializerRefusal {
    /// The materializer contains no unchanged-production branch.
    NoMutationNotImplemented,
    /// The surface admitted a selection the materializer has no branch for.
    ActiveSelectionNotImplemented(ActiveSelection),
}

/// A capture-free source materializer over a surface-bound directive.
pub type SpecimenMaterializerCall =
    for<'surface> fn(EvaluationDirective<'surface>) -> Result<Vec<u8>, SpecimenMaterializerRefusal>;

/// One materializer bound, before execution, to the exact pair whose source it renders.
///
/// The compiled-specimen road validates that pair before calling it, resolves the active directive itself, and derives content identity from the bytes returned; nothing here inspects the callable's implementation.
pub struct SpecimenMaterializerBinding {
    pair: EvaluationPairStanding,
    call: SpecimenMaterializerCall,
}

/// One immutable request handed to a compiled-specimen host.
///
/// The request binds the exact content, operation, parity-qualified input, semantic role, execution key, and check identity before any caller code runs.
pub struct CompiledSpecimenRequest<'content, 'input, Input> {
    content: &'content ArtifactContent,
    role: CompiledSpecimenRole,
    operation: &'content [u8],
    input: &'input Input,
    execution: &'content ExecutionKey,
    check: CheckRef,
}

/// A host's typed report that it compiled and executed one exact request and recovered this meaning.
///
/// The constructor copies the content, role, execution, and check facts off the request, so a host cannot supply sibling identity labels.
/// This is still caller output: it records what the host reported and does not prove that a compiler process ran or that the host used those inputs faithfully.
pub struct CompiledSpecimenObservation<Meaning> {
    content: ArtifactContentId,
    role: CompiledSpecimenRole,
    execution: ExecutionKey,
    check: CheckRef,
    meaning: Meaning,
}

/// Which request member made one host observation foreign to the request being judged.
#[must_use = "a mismatch is the exact request member a host observation did not reproduce"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledSpecimenObservationMismatch {
    /// The observation names compiler-source content other than the requested content.
    Content {
        /// The requested compiler-source content identity.
        expected: ArtifactContentId,
        /// The compiler-source content identity retained by the observation.
        found: ArtifactContentId,
    },
    /// The observation names another baseline or selected role.
    Role,
    /// The observation retains another execution key.
    Execution,
    /// The observation names another check contract.
    Check,
}

/// Why a compiled-specimen host produced no execution observation.
#[must_use = "a refusal is the reason one compiled specimen produced no observation"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledSpecimenHostRefusal {
    /// The host reported that its compiler produced no executable artifact.
    Compilation(ForeignText),
    /// The host reported that the compiled artifact did not complete execution.
    Execution(ForeignText),
    /// The host recovered no meaning tied to the requested operation.
    Meaning(ForeignText),
}

/// A capture-free host adapter that compiles and executes one exact specimen request.
pub type CompiledSpecimenHost<Input, Meaning> = for<'content, 'input> fn(
    CompiledSpecimenRequest<'content, 'input, Input>,
) -> Result<
    CompiledSpecimenObservation<Meaning>,
    CompiledSpecimenHostRefusal,
>;

/// What one selected projection's compiled specimen stands on: its artifact, pair, selection, execution, and check.
///
/// The artifact identity names compiler-source bytes alone, and this standing carries their relationship to the host-reported execution without rehashing caller labels into that identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSpecimenStanding {
    artifact: ArtifactContentId,
    pair: EvaluationPairStanding,
    selection: ActiveSelection,
    execution: ExecutionKey,
    check: CheckRef,
}

/// Exact compiled pressure for one selected projection and one surface-issued selection.
///
/// Building one takes the retained no-mutation qualification, separately rendered baseline and selected artifacts, host-reported executions of those exact bytes, a passing baseline report, and a rejecting selected report.
/// It cannot be minted from an external backend's output or from labels attached after execution.
pub struct CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning> {
    parity: &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>,
    baseline_artifact: ArtifactContentId,
    standing: CompiledSpecimenStanding,
    baseline_report: TrialReport,
    selected_report: TrialReport,
    mutation: MutationReport,
}

/// Why exact compiled selected-projection pressure could not be established.
#[must_use = "a refusal is the reason exact compiled projection pressure was not established"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledProjectionRefusal {
    /// The no-mutation qualification belongs to another evaluation surface.
    ParityForAnotherSurface {
        /// The surface offered for compiled projection pressure.
        expected: EvaluationSurfaceId,
        /// The surface retained by the qualified pair.
        found: EvaluationSurfaceId,
    },
    /// The source materializer is bound to another production and evaluation pair.
    MaterializerForAnotherPair(EvaluationPairStandingMismatch),
    /// The active selection does not belong to the qualified surface.
    Selection(SelectionRefusal),
    /// The retained witness trial belongs to another owner claim.
    WitnessForAnotherClaim {
        /// The claim that owns the selected point.
        expected: ClaimRef,
        /// The claim carried by the retained witness binding.
        found: ClaimRef,
    },
    /// The supplied invocation does not reproduce the qualification's execution key.
    InvocationForAnotherExecution,
    /// The materializer refused the unchanged source.
    BaselineMaterialization(SpecimenMaterializerRefusal),
    /// The materializer refused the exact selected source.
    SelectedMaterialization(SpecimenMaterializerRefusal),
    /// The selected rendering has the same bytes as the unchanged rendering.
    ArtifactDidNotChange(ArtifactContentId),
    /// The host refused compilation or execution of the unchanged artifact.
    BaselineHost(CompiledSpecimenHostRefusal),
    /// The unchanged host observation belongs to another request.
    BaselineObservation(CompiledSpecimenObservationMismatch),
    /// The unchanged host observation could not join the retained trial binding.
    BaselineReport(ReportRecordingRefusal),
    /// The separately compiled unchanged artifact did not pass its witness.
    BaselineDidNotQualify,
    /// The host refused compilation or execution of the selected artifact.
    SelectedHost(CompiledSpecimenHostRefusal),
    /// The selected host observation belongs to another request.
    SelectedObservation(CompiledSpecimenObservationMismatch),
    /// The selected host observation could not join the retained trial binding.
    SelectedReport(ReportRecordingRefusal),
    /// The exact witness did not reject the selected compiled artifact.
    ProjectionDidNotReject,
}

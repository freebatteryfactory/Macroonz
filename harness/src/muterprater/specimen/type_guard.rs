//! The invariant nucleus of exact compiled selected-projection pressure.

use super::{
    ARTIFACT_CONTENT_TAG, ActiveSelection, ArtifactContent, ArtifactContentId, CheckRef,
    CompiledProjectionPressure, CompiledSpecimenContext, CompiledSpecimenObservation,
    CompiledSpecimenObservationMismatch, CompiledSpecimenRequest, CompiledSpecimenRole,
    CompiledSpecimenStanding, EvaluationPairStanding, ExecutionKey, MutationReport,
    NoMutationParityQualification, SpecimenMaterializerBinding, SpecimenMaterializerCall,
    TrialReport,
};
use super::{CompiledMutationObservation, MutationObservation, SpecimenObservationRefusal};
use crate::identity::ContentAddress;
use crate::muterprater::EvaluationPair;
use crate::report::{InvocationProfile, TargetBinding};
use crate::runner::Invocation;

impl<'scope, Input, Meaning> CompiledMutationObservation<'scope, Input, Meaning> {
    pub(in crate::muterprater) fn observed(
        observation: &'scope MutationObservation<'scope, Input, Meaning>,
        contents: (ArtifactContent, ArtifactContent),
        results: (
            Result<Meaning, SpecimenObservationRefusal>,
            Result<Meaning, SpecimenObservationRefusal>,
        ),
    ) -> Self {
        let (baseline_content, selected_content) = contents;
        let (baseline, selected) = results;
        Self {
            observation,
            baseline_content,
            selected_content,
            baseline,
            selected,
        }
    }

    /// The in-process observation this compiled execution compared.
    #[must_use]
    pub const fn observation(&self) -> &'scope MutationObservation<'scope, Input, Meaning> {
        self.observation
    }

    /// The exact unchanged source handed to the host.
    #[must_use]
    pub const fn baseline_content(&self) -> &ArtifactContent {
        &self.baseline_content
    }

    /// The exact selected source handed to the host.
    #[must_use]
    pub const fn selected_content(&self) -> &ArtifactContent {
        &self.selected_content
    }

    /// The unchanged compiled meaning or observed failure.
    pub const fn baseline(&self) -> &Result<Meaning, SpecimenObservationRefusal> {
        &self.baseline
    }

    /// The selected compiled meaning or observed failure.
    pub const fn selected(&self) -> &Result<Meaning, SpecimenObservationRefusal> {
        &self.selected
    }
}
impl ArtifactContentId {
    /// Derive the identity of exact compiler-source bytes.
    pub(in crate::muterprater) fn derived(bytes: &[u8]) -> Self {
        Self(ContentAddress::derived(ARTIFACT_CONTENT_TAG, bytes))
    }
}

crate::identity::content_address_reference! {
    /// The underlying content address.
    value ArtifactContentId;
}

impl ArtifactContent {
    /// Retain exact compiler-source bytes under their bytes-only identity.
    pub(in crate::muterprater) fn recorded(bytes: Vec<u8>) -> Self {
        let identity = ArtifactContentId::derived(&bytes);
        Self { identity, bytes }
    }

    /// The bytes-only identity of this exact content.
    #[must_use]
    pub const fn identity(&self) -> ArtifactContentId {
        self.identity
    }

    /// The exact bytes the host must hand unchanged to its compiler.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl SpecimenMaterializerBinding {
    /// Bind one capture-free source materializer to the exact pair it renders from.
    #[must_use]
    pub fn bound<Input, Meaning>(
        pair: &EvaluationPair<Input, Meaning>,
        call: SpecimenMaterializerCall,
    ) -> Self {
        Self {
            pair: pair.standing(),
            call,
        }
    }

    /// The exact pair this source renderer is declared over.
    #[must_use]
    pub const fn pair(&self) -> EvaluationPairStanding {
        self.pair
    }

    /// The capture-free source materializer.
    #[must_use]
    pub const fn call(&self) -> SpecimenMaterializerCall {
        self.call
    }
}

impl<'content, 'input, Input> CompiledSpecimenRequest<'content, 'input, Input> {
    /// Bind one exact artifact and semantic role to its execution standing.
    pub(in crate::muterprater) const fn requested(
        content: &'content ArtifactContent,
        role: CompiledSpecimenRole,
        operation: &'content [u8],
        input: &'input Input,
        standing: &'content CompiledSpecimenContext,
    ) -> Self {
        Self {
            content,
            role,
            operation,
            input,
            context: standing,
        }
    }

    /// The exact compiler-source content.
    #[must_use]
    pub const fn content(&self) -> &'content ArtifactContent {
        self.content
    }

    /// Whether this request is the unchanged baseline or one selected mutation.
    #[must_use]
    pub const fn role(&self) -> CompiledSpecimenRole {
        self.role
    }

    /// The exact operation the host must find baked into this artifact.
    #[must_use]
    pub const fn operation(&self) -> &'content [u8] {
        self.operation
    }

    /// The supplied input the host must exercise.
    #[must_use]
    pub const fn input(&self) -> &'input Input {
        self.input
    }

    /// The declared execution context, independent of any later judgment.
    #[must_use]
    pub const fn context(&self) -> &'content CompiledSpecimenContext {
        self.context
    }

    /// The target the host must execute for.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        self.context.target()
    }
}

impl CompiledSpecimenContext {
    pub(in crate::muterprater) fn recorded(
        pair: EvaluationPairStanding,
        invocation: &Invocation,
    ) -> Self {
        Self {
            pair,
            profile: invocation.profile(),
            target: invocation.target().clone(),
        }
    }

    /// The declared production and evaluation pair.
    #[must_use]
    pub const fn pair(&self) -> EvaluationPairStanding {
        self.pair
    }

    /// The declared invocation profile.
    #[must_use]
    pub const fn profile(&self) -> InvocationProfile {
        self.profile
    }

    /// The declared execution target.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }
}

impl<Meaning> CompiledSpecimenObservation<Meaning> {
    /// Report successful compilation and execution of the exact supplied request.
    ///
    /// Every binding fact is copied from `request`, and the host supplies only the recovered meaning.
    #[must_use]
    pub fn executed<Input>(
        request: &CompiledSpecimenRequest<'_, '_, Input>,
        meaning: Meaning,
    ) -> Self {
        Self {
            content: request.content().identity(),
            role: request.role(),
            context: request.context().clone(),
            meaning,
        }
    }

    /// The exact compiler-source content the host says it compiled and executed.
    #[must_use]
    pub const fn content(&self) -> ArtifactContentId {
        self.content
    }

    /// The semantic role of the executed artifact.
    #[must_use]
    pub const fn role(&self) -> CompiledSpecimenRole {
        self.role
    }

    /// The execution context retained from the request.
    #[must_use]
    pub const fn context(&self) -> &CompiledSpecimenContext {
        &self.context
    }

    /// Compare the copied request standing before this observation supplies a meaning.
    pub(in crate::muterprater) fn mismatch(
        &self,
        content: ArtifactContentId,
        role: CompiledSpecimenRole,
        standing: &CompiledSpecimenContext,
    ) -> Option<CompiledSpecimenObservationMismatch> {
        if self.content != content {
            return Some(CompiledSpecimenObservationMismatch::Content {
                expected: content,
                found: self.content,
            });
        }
        if self.role != role {
            return Some(CompiledSpecimenObservationMismatch::Role);
        }
        if &self.context != standing {
            return Some(CompiledSpecimenObservationMismatch::Context);
        }
        None
    }

    /// The meaning the host recovered from the compiled specimen.
    #[must_use]
    pub const fn meaning(&self) -> &Meaning {
        &self.meaning
    }

    /// Consume the host observation into its recovered meaning.
    pub(in crate::muterprater) fn into_meaning(self) -> Meaning {
        self.meaning
    }
}

impl CompiledSpecimenStanding {
    /// Bind exact compiler-source bytes to the pair, selection, and execution that pressed them.
    pub(in crate::muterprater) fn recorded(
        artifact: ArtifactContentId,
        pair: EvaluationPairStanding,
        selection: ActiveSelection,
        execution: ExecutionKey,
        check: CheckRef,
    ) -> Self {
        Self {
            artifact,
            pair,
            selection,
            execution,
            check,
        }
    }

    /// The exact selected compiler-source content identity.
    #[must_use]
    pub const fn artifact(&self) -> ArtifactContentId {
        self.artifact
    }

    /// The exact pair the materializer was bound to.
    #[must_use]
    pub const fn pair(&self) -> EvaluationPairStanding {
        self.pair
    }

    /// The exact surface-issued selection baked into the artifact.
    #[must_use]
    pub const fn selection(&self) -> ActiveSelection {
        self.selection
    }

    /// The execution key the compiled meaning was judged under.
    #[must_use]
    pub const fn execution(&self) -> &ExecutionKey {
        &self.execution
    }

    /// The declared check identity that rejected the compiled meaning.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.check
    }
}

impl<'parity, 'pair, 'input, Input, Meaning>
    CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>
{
    /// Retain one exact selected compiled rejection and the unchanged baseline it stood over.
    pub(in crate::muterprater) fn demonstrated(
        parity: &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>,
        baseline_artifact: ArtifactContent,
        selected_artifact: ArtifactContent,
        standing: CompiledSpecimenStanding,
        baseline_report: TrialReport,
        selected_report: TrialReport,
        mutation: MutationReport,
    ) -> Self {
        Self {
            parity,
            baseline_artifact,
            selected_artifact,
            standing,
            baseline_report,
            selected_report,
            mutation,
        }
    }

    /// The no-mutation qualification whose pair, input, and witness this pressure reuses.
    #[must_use]
    pub const fn parity(
        &self,
    ) -> &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning> {
        self.parity
    }

    /// The separately compiled unchanged compiler-source content identity.
    #[must_use]
    pub const fn baseline_artifact(&self) -> ArtifactContentId {
        self.baseline_artifact.identity()
    }

    /// The exact unchanged source retained after its host request completed.
    #[must_use]
    pub const fn baseline_content(&self) -> &ArtifactContent {
        &self.baseline_artifact
    }

    /// The exact selected source retained after its host request completed.
    #[must_use]
    pub const fn selected_content(&self) -> &ArtifactContent {
        &self.selected_artifact
    }

    /// The selected compiled specimen's exact standing.
    #[must_use]
    pub const fn standing(&self) -> &CompiledSpecimenStanding {
        &self.standing
    }

    /// The passing report from the separately compiled unchanged artifact.
    #[must_use]
    pub const fn baseline_report(&self) -> &TrialReport {
        &self.baseline_report
    }

    /// The rejecting report from the separately compiled selected artifact.
    #[must_use]
    pub const fn selected_report(&self) -> &TrialReport {
        &self.selected_report
    }

    /// The mutation report derived from the selected artifact's report.
    #[must_use]
    pub const fn mutation(&self) -> &MutationReport {
        &self.mutation
    }
}

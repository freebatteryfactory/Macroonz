//! The invariant nucleus of exact compiled selected-projection pressure.

use super::{
    ARTIFACT_CONTENT_TAG, ActiveSelection, ArtifactContent, ArtifactContentId, CheckRef,
    CompiledProjectionPressure, CompiledSpecimenObservation, CompiledSpecimenObservationMismatch,
    CompiledSpecimenRequest, CompiledSpecimenRole, CompiledSpecimenStanding, ContentAddress,
    EvaluationPairStanding, ExecutionKey, MutationReport, NoMutationParityQualification,
    SpecimenMaterializerBinding, SpecimenMaterializerCall, TrialReport,
};
use crate::muterprater::EvaluationPair;
impl ArtifactContentId {
    /// Derive the identity of exact compiler-source bytes.
    pub(in crate::muterprater) fn derived(bytes: &[u8]) -> Self {
        Self(ContentAddress::derived(ARTIFACT_CONTENT_TAG, bytes))
    }

    /// The underlying content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
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
        execution: &'content ExecutionKey,
        check: CheckRef,
    ) -> Self {
        Self {
            content,
            role,
            operation,
            input,
            execution,
            check,
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

    /// The exact parity-qualified input the host must exercise.
    #[must_use]
    pub const fn input(&self) -> &'input Input {
        self.input
    }

    /// The execution key the recovered meaning will be judged under.
    #[must_use]
    pub const fn execution(&self) -> &'content ExecutionKey {
        self.execution
    }

    /// The declared check identity that will judge the recovered meaning.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.check
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
            execution: request.execution().clone(),
            check: request.check(),
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

    /// The execution key retained from the request.
    #[must_use]
    pub const fn execution(&self) -> &ExecutionKey {
        &self.execution
    }

    /// The declared check identity retained from the request.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.check
    }

    /// Compare the copied request standing before this observation supplies a meaning.
    pub(in crate::muterprater) fn mismatch(
        &self,
        content: ArtifactContentId,
        role: CompiledSpecimenRole,
        execution: &ExecutionKey,
        check: CheckRef,
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
        if &self.execution != execution {
            return Some(CompiledSpecimenObservationMismatch::Execution);
        }
        if self.check != check {
            return Some(CompiledSpecimenObservationMismatch::Check);
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
        baseline_artifact: ArtifactContentId,
        standing: CompiledSpecimenStanding,
        baseline_report: TrialReport,
        selected_report: TrialReport,
        mutation: MutationReport,
    ) -> Self {
        Self {
            parity,
            baseline_artifact,
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
        self.baseline_artifact
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

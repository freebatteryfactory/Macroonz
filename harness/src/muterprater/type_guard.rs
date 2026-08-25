//! The invariant nucleus: every road that builds one of this home's values, and every reader that hands its seats back.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's claims structural rather than remembered.
//! A kill is minted here, so a kill standing on an unqualified baseline is not a value that exists.
//! A survivor is minted here, so a mutant with no observed firing can never earn the word.
//! An adapter qualification is minted here, and only over a grammar somebody checked against the very backend version the reading names.
//! A duplicate is refused here, so "not a duplicate" is a comparison rather than a paragraph.
//! And a proposal is married to its ground here, so evidence that does not fit the ground is not a proposal anybody can offer.

use super::{
    ARTIFACT_CONTENT_TAG, ActivationDisposition, ActivationEvidence, ActivationSite,
    ActiveSelection, AdapterProfile, AdapterQualification, AdmittedAlternative,
    AlternativeDeclaration, AlternativeId, AnnouncedRoster, ArtifactContent, ArtifactContentId,
    ArtifactCustodyRefusal, BACKEND_OUTPUT_TAG, BackendCommand, BackendCommandRefusal,
    BackendOutputId, BackendVersion, BackendVersionPosture, BackendVersionRefusal, BaselineAxis,
    BaselinePrecondition, BaselineQualification, BudgetRefusal, CandidateSketch, CheckGap,
    ClaimCeiling, ClaimPinnedGround, ClaimPinnedProposal, CompiledProjectionPressure,
    CompiledSpecimenObservation, CompiledSpecimenObservationMismatch, CompiledSpecimenRequest,
    CompiledSpecimenRole, CompiledSpecimenStanding, CompiledSuiteArtifactCustody,
    CompiledSuiteArtifactManifest, CompiledSuiteArtifactStanding, CompiledSuitePressure,
    CoordinateRefusal, DemonstratedRejection, Demonstration, DiffPath, DiffPathRefusal,
    DischargeAdmissionReceipt, DischargeEvidence, DiscoveredMutationSite, DiscoveryDisposition,
    DiscoveryEntry, DiscoveryRefusal, DudPlant, DuplicateRefusal, EVALUATION_SURFACE_TAG,
    EquivalenceAxis, EvaluationBinding, EvaluationCall, EvaluationCallRefusal, EvaluationDirective,
    EvaluationFamilyRef, EvaluationObservation, EvaluationPair, EvaluationPairRefusal,
    EvaluationPairStanding, EvaluationPairStandingMismatch, EvaluationSurface, EvaluationSurfaceId,
    ExecutionAxis, ExplanationRefusal, FailureComparison, FamilyAttribution, GrammarStanding,
    GrammarVersion, InconclusiveCause, InferredObligation, IntendedRejection,
    InterpretedMutationEvidence, InterpretedTrust, KillRefusal, MUTATION_ALTERNATIVE_TAG,
    MUTATION_DISCOVERY_TAG, MUTATION_POLICY_TAG, MUTATION_SOURCE_REVISION_TAG, MUTATION_TARGET_TAG,
    MappingPosture, MaterializationAxis, MeaningCheck, MutantId, MutantKilledGround,
    MutantKilledProposal, MutationBackendInvocation, MutationCensus, MutationDiscoveryId,
    MutationDiscoveryReading, MutationIdentity, MutationOutcome, MutationPermission, MutationPoint,
    MutationPolicy, MutationPolicyId, MutationReport, MutationRun, MutationSite,
    MutationSourceRevision, MutationSourceRevisionId, MutationSurfaceLowering, MutationTarget,
    MutationVerdict, MutationWitness, MutationWitnessRefusal, NoComparison, NoComparisonReason,
    NoMutationParityQualification, NoMutationParityReading, NoMutationParityStanding,
    NoMutationReports, NoMutationResults, ObligationComparison, ObligationDischargedGround,
    ObligationDischargedProposal, ObligationLane, OperatorFamilyRef, OracleClass, OwedClaim,
    OwedClaimRefusal, OwedDeclaration, OwnerClaimMapping, PROPOSAL_TAG, ParityQualificationRefusal,
    PermissionRefusal, PlanRefusal, PlannedDamage, PlannedRun, PointCatalogPosture,
    PolicyMembership, PolicyRefusal, PressureBudget, PressureLane, ProductionBinding,
    ProductionCall, ProofDelta, ProofDeltaRefusal, ProofPlan, ProofRefusal, ProofShape,
    ProposalDestination, ProposalDocument, ProposalRefusal, QualificationRefusal, ReadingSource,
    RejectedNoMutationParity, RejectionIdentity, ReplayAdmissionReceipt, ReplayBearingProposal,
    ResolvedMutation, RewriteCandidate, RewriteDescriptor, RewriteRefusal, RewriteRoster,
    RewriteTrust, RosterRefusal, ScopeShape, ScopedInvocation, SelectionRefusal, SinkRefusal,
    SourceCoordinate, SpecimenMaterializerBinding, SpecimenMaterializerCall, StoredProposalRef,
    SuitePressureRefusal, SurvivorExplanation, UnparsedLine, WrapReading, WrapRefusal,
    WrappedBackend,
};
use crate::depot::capsules::{ReplayCapsuleEntry, StoredReplayEntryRef};
use crate::depot::operator_families::OPERATOR_FAMILIES;
use crate::depot::types::OperatorFamily;
use crate::descriptor::{
    AdmissionGround, CheckRef, ClaimRef, Classification, ExecutionSuite, MutationPointRef,
    NameRefusal, Namespace, NamespacedName, Origin, PopulationRef, ProposalId, ReplayBearingGround,
    RevisionBinding, Row, SubjectRoute, SynthesisFacts, TablePosture,
};
use crate::identity::ContentAddress;
use crate::muterprater::encode;
use crate::properties::{Equivalence, SharedSubstrate};
use crate::report::{
    ClaimExercise, ExecutionKey, Fingerprint, ForeignText, InvocationProfile, ReplayCapsule,
    RunAttempt, RunReport, TrialConclusion, TrialFinding, TrialId, TrialReport, encode_bytes,
};
use crate::runner::{Selection, TrialBinding};
use std::collections::BTreeMap;

/// The version of the external-mutant identity encoding.
///
/// It rides the preimage, so changing how the bytes are cut renames every mutant derived under the old cut rather than letting two encodings be mistaken for one another.
const MUTANT_ENCODING_VERSION: u32 = 1;

/// The version of the proposal identity encoding.
const PROPOSAL_ENCODING_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// The verdict chain.
// ---------------------------------------------------------------------------

impl ClaimCeiling {
    /// The strongest verdict this ceiling grants.
    #[must_use]
    pub const fn strongest(self) -> MutationVerdict {
        match self {
            Self::WitnessRejection => MutationVerdict::Killed,
        }
    }

    /// Whether one verdict stands inside this ceiling.
    ///
    /// A kill and an inconclusive both stand inside witness rejection; survived stands outside it, because earning that word takes an activation the source offers no channel to observe.
    #[must_use]
    pub const fn admits(self, verdict: MutationVerdict) -> bool {
        match (self, verdict) {
            (Self::WitnessRejection, MutationVerdict::Killed | MutationVerdict::Inconclusive) => {
                true
            }
            (Self::WitnessRejection, MutationVerdict::Survived) => false,
        }
    }
}

/// Whether the baseline every lawful outcome stands on qualified.
fn baseline_qualified(baseline: BaselineAxis) -> bool {
    match baseline {
        BaselineAxis::Qualified => true,
        BaselineAxis::Failed | BaselineAxis::NotRun => false,
    }
}

/// Whether the damage became a thing that could be executed.
fn materialized(materialization: MaterializationAxis) -> bool {
    match materialization {
        MaterializationAxis::Built => true,
        MaterializationAxis::Unviable | MaterializationAxis::ToolFailed => false,
    }
}

/// Whether the witness execution reached a conclusion.
fn witness_completed(execution: ExecutionAxis) -> bool {
    match execution {
        ExecutionAxis::Completed => true,
        ExecutionAxis::NotExecuted
        | ExecutionAxis::TimedOut
        | ExecutionAxis::Crashed
        | ExecutionAxis::InfrastructureFailed => false,
    }
}

// ---------------------------------------------------------------------------
// The mutation target.
// ---------------------------------------------------------------------------

impl SourceCoordinate {
    /// The coordinate an external backend reported.
    ///
    /// # Errors
    ///
    /// Refuses an empty file spelling, because a coordinate that names no file places nothing.
    pub fn reported(file: &str, line: u32, column: u32) -> Result<Self, CoordinateRefusal> {
        if file.is_empty() {
            return Err(CoordinateRefusal::EmptyFile);
        }
        Ok(Self {
            file: file.to_owned(),
            line,
            column,
        })
    }

    /// The file the backend named.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The line the backend named.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The column the backend named.
    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

impl MutantId {
    /// Derive one external mutant's identity from what the backend reported.
    ///
    /// # The specification
    ///
    /// Two primitives: `u32be(n)` — the integer in four big-endian bytes — and `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`, which is the record vocabulary's framing law ([`crate::report::encode_bytes`]).
    ///
    /// The members, in exactly this order, with no separators and no padding:
    ///
    /// | # | member | encoding |
    /// | - | ------ | -------- |
    /// | 1 | encoding version | `u32be` |
    /// | 2 | file | `bytes(utf8)` |
    /// | 3 | line | `u32be` |
    /// | 4 | column | `u32be` |
    /// | 5 | damage | `bytes(…)` — the backend's own damage text, at full length |
    #[must_use]
    pub fn over(coordinate: &SourceCoordinate, damage: &[u8]) -> Self {
        let mut preimage = Vec::new();
        preimage.extend_from_slice(&MUTANT_ENCODING_VERSION.to_be_bytes());
        encode_bytes(coordinate.file().as_bytes(), &mut preimage);
        preimage.extend_from_slice(&coordinate.line().to_be_bytes());
        preimage.extend_from_slice(&coordinate.column().to_be_bytes());
        encode_bytes(damage, &mut preimage);
        Self(ContentAddress::derived(MUTATION_TARGET_TAG, &preimage))
    }

    /// The identity's address, for comparison and for rendering.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl MutationIdentity {
    /// The mutation point this identity names, where it names one.
    ///
    /// An external identity names a coordinate rather than a point, so it answers nothing here.
    #[must_use]
    pub const fn point(self) -> Option<MutationPointRef> {
        match self {
            Self::External(_) => None,
            Self::Interpreted {
                point,
                alternative: _,
            }
            | Self::CompiledProjection {
                point,
                alternative: _,
            } => Some(point),
        }
    }

    /// The admitted alternative this identity names, where it names one.
    #[must_use]
    pub const fn alternative(self) -> Option<AlternativeId> {
        match self {
            Self::External(_) => None,
            Self::Interpreted {
                point: _,
                alternative,
            }
            | Self::CompiledProjection {
                point: _,
                alternative,
            } => Some(alternative),
        }
    }
}

impl OperatorFamilyRef {
    /// The reference the bank declares under this slug, where the bank declares one.
    #[must_use]
    pub fn of_slug(slug: &str) -> Option<Self> {
        OPERATOR_FAMILIES
            .iter()
            .copied()
            .find(|family| family.slug() == slug)
            .map(Self)
    }

    /// The bank row this reference carries.
    #[must_use]
    pub const fn family(self) -> OperatorFamily {
        self.0
    }

    /// The family's stable slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        self.0.slug()
    }
}

impl MutationTarget {
    /// One damaged thing this lane pressed.
    #[must_use]
    pub(in crate::muterprater) fn pressed(
        identity: MutationIdentity,
        family: FamilyAttribution,
        site: MutationSite,
        owner: MappingPosture,
    ) -> Self {
        Self {
            identity,
            family,
            site,
            owner,
        }
    }

    /// How the damaged thing is identified.
    #[must_use]
    pub const fn identity(&self) -> MutationIdentity {
        self.identity
    }

    /// Which operator family the damage realizes, where the bank names one.
    #[must_use]
    pub const fn family(&self) -> FamilyAttribution {
        self.family
    }

    /// Where the damage lives.
    #[must_use]
    pub const fn site(&self) -> &MutationSite {
        &self.site
    }

    /// Whether the origin reading named the claim that owns the site.
    #[must_use]
    pub const fn owner(&self) -> MappingPosture {
        self.owner
    }

    /// The claim that owns the site, where the reading named one.
    #[must_use]
    pub const fn owning_claim(&self) -> Option<ClaimRef> {
        match self.owner {
            MappingPosture::Mapped(claim) => Some(claim),
            MappingPosture::OwnerUnmapped => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Activation evidence.
// ---------------------------------------------------------------------------

impl ActivationEvidence {
    /// The positive firing reading one evaluation callback reported for a planted damage.
    ///
    /// A zero count returns absence, so the receiver can return the exact [`DudPlant`] finding instead.
    pub(in crate::muterprater) fn observed(
        selection: ActiveSelection,
        witness: TrialId,
        firings: u32,
    ) -> Option<Self> {
        if firings == 0_u32 {
            return None;
        }
        Some(Self {
            selection,
            witness,
            firings,
        })
    }

    /// The point whose selected damage received the positive-count report.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.selection.point()
    }

    /// The exact active selection bound to the positive-count report.
    #[must_use]
    pub const fn selection(self) -> ActiveSelection {
        self.selection
    }

    /// The trial bound to the positive-count report.
    #[must_use]
    pub const fn witness(self) -> TrialId {
        self.witness
    }

    /// The positive firing count reported by the evaluation callback or backend.
    #[must_use]
    pub const fn firings(self) -> u32 {
        self.firings
    }
}

impl DudPlant {
    /// The exact selection and trial for which the evaluation callback reported zero firings.
    pub(in crate::muterprater) const fn unfired(
        selection: ActiveSelection,
        witness: TrialId,
    ) -> Self {
        Self { selection, witness }
    }

    /// The point whose selected damage received a reported count of zero.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.selection.point()
    }

    /// The exact active selection that received a reported count of zero.
    #[must_use]
    pub const fn selection(self) -> ActiveSelection {
        self.selection
    }

    /// The trial bound to the callback's zero-count report.
    #[must_use]
    pub const fn witness(self) -> TrialId {
        self.witness
    }
}

impl ActivationDisposition {
    /// The evidence, where the disposition carries any.
    #[must_use]
    pub const fn evidence(self) -> Option<ActivationEvidence> {
        match self {
            Self::Observed(evidence) => Some(evidence),
            Self::NotObserved | Self::UnobservableUnderBackend => None,
        }
    }
}

// ---------------------------------------------------------------------------
// The rejection a kill stands on.
// ---------------------------------------------------------------------------

impl DemonstratedRejection {
    /// The rejection this harness's own engine demonstrated.
    #[must_use]
    pub(in crate::muterprater) fn demonstrated(trial: TrialId, finding: TrialFinding) -> Self {
        Self { trial, finding }
    }

    /// The trial that refused.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The finding it refused with.
    #[must_use]
    pub const fn finding(&self) -> &TrialFinding {
        &self.finding
    }

    /// The failure identity, derived from the trial and the finding.
    #[must_use]
    pub fn fingerprint(&self) -> Fingerprint {
        Fingerprint::of(self.trial, &self.finding)
    }
}

impl IntendedRejection {
    /// The failure identity this rejection carries, where it carries one.
    #[must_use]
    pub fn identity(&self) -> RejectionIdentity {
        match self {
            Self::Demonstrated(rejection) => {
                RejectionIdentity::Fingerprinted(rejection.fingerprint())
            }
            Self::ReportedByBackend { stated: _ } => RejectionIdentity::Unfingerprinted,
        }
    }
}

// ---------------------------------------------------------------------------
// The per-mutant record.
// ---------------------------------------------------------------------------

impl MutationReport {
    /// The record of one mutant this lane killed lawfully.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a baseline that is not a qualified unchanged pass, a damage that did not materialize, an activation not observed under a backend that can observe firing, and a witness that did not complete.
    /// The unobservable-under-backend arm is admitted at its stated ceiling, where a kill asserts witness rejection and never observed activation.
    pub(in crate::muterprater) fn killed(
        target: MutationTarget,
        baseline: BaselineAxis,
        materialization: MaterializationAxis,
        activation: ActivationDisposition,
        execution: ExecutionAxis,
        rejection: IntendedRejection,
        equivalence: EquivalenceAxis,
    ) -> Result<Self, KillRefusal> {
        if !baseline_qualified(baseline) {
            return Err(KillRefusal::BaselineNotQualified(baseline));
        }
        if !materialized(materialization) {
            return Err(KillRefusal::NotMaterialized(materialization));
        }
        match activation {
            ActivationDisposition::Observed(_)
            | ActivationDisposition::UnobservableUnderBackend => {}
            ActivationDisposition::NotObserved => {
                return Err(KillRefusal::ActivationNotObserved);
            }
        }
        if !witness_completed(execution) {
            return Err(KillRefusal::WitnessDidNotComplete(execution));
        }
        Ok(Self {
            target,
            baseline,
            materialization,
            activation,
            execution,
            outcome: MutationOutcome::Killed(rejection),
            equivalence,
        })
    }

    /// The record of one mutant that established nothing about the suite.
    ///
    /// Total, and deliberately so: any chain can fail to establish anything, and the cause names which link did not hold.
    #[must_use]
    pub(in crate::muterprater) fn inconclusive(
        target: MutationTarget,
        baseline: BaselineAxis,
        materialization: MaterializationAxis,
        activation: ActivationDisposition,
        execution: ExecutionAxis,
        cause: InconclusiveCause,
        equivalence: EquivalenceAxis,
    ) -> Self {
        Self {
            target,
            baseline,
            materialization,
            activation,
            execution,
            outcome: MutationOutcome::Inconclusive(cause),
            equivalence,
        }
    }

    /// Derive one interpreted report from an activated execution under a qualified no-mutation baseline.
    #[must_use]
    pub(in crate::muterprater) fn interpreted(
        target: MutationTarget,
        activation: ActivationEvidence,
        report: &TrialReport,
    ) -> Self {
        let (execution, outcome) = match report.attempt() {
            RunAttempt::Executed(TrialConclusion::Passed) => {
                (ExecutionAxis::Completed, MutationOutcome::Survived)
            }
            RunAttempt::Executed(TrialConclusion::Refused(finding)) => (
                ExecutionAxis::Completed,
                MutationOutcome::Killed(IntendedRejection::Demonstrated(DemonstratedRejection {
                    trial: report.trial(),
                    finding: finding.clone(),
                })),
            ),
            RunAttempt::SkippedWithReason(_) => (
                ExecutionAxis::NotExecuted,
                MutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ),
            RunAttempt::TimedOut => (
                ExecutionAxis::TimedOut,
                MutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ),
            RunAttempt::InfrastructureFailed(_) => (
                ExecutionAxis::InfrastructureFailed,
                MutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ),
        };
        Self {
            target,
            baseline: BaselineAxis::Qualified,
            materialization: MaterializationAxis::Built,
            activation: ActivationDisposition::Observed(activation),
            execution,
            outcome,
            equivalence: EquivalenceAxis::NotAssessed,
        }
    }

    /// Derive one compiled-projection kill from the report that rejected it.
    ///
    /// Returns no value unless the report completed with a typed refusal.
    /// Compiled projection pressure has no activation channel, so its axis states that ceiling while retaining the exact trial and finding the runner admitted.
    pub(in crate::muterprater) fn compiled_projection(
        target: MutationTarget,
        report: &TrialReport,
    ) -> Option<Self> {
        let RunAttempt::Executed(TrialConclusion::Refused(finding)) = report.attempt() else {
            return None;
        };
        Some(Self {
            target,
            baseline: BaselineAxis::Qualified,
            materialization: MaterializationAxis::Built,
            activation: ActivationDisposition::UnobservableUnderBackend,
            execution: ExecutionAxis::Completed,
            outcome: MutationOutcome::Killed(IntendedRejection::Demonstrated(
                DemonstratedRejection {
                    trial: report.trial(),
                    finding: finding.clone(),
                },
            )),
            equivalence: EquivalenceAxis::NotAssessed,
        })
    }

    /// What was damaged.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// What the unchanged subject's suite did.
    #[must_use]
    pub const fn baseline(&self) -> BaselineAxis {
        self.baseline
    }

    /// Whether the damage became executable.
    #[must_use]
    pub const fn materialization(&self) -> MaterializationAxis {
        self.materialization
    }

    /// What was established about the damage firing.
    #[must_use]
    pub const fn activation(&self) -> ActivationDisposition {
        self.activation
    }

    /// What became of the witness execution.
    #[must_use]
    pub const fn execution(&self) -> ExecutionAxis {
        self.execution
    }

    /// The outcome with its evidence.
    #[must_use]
    pub const fn outcome(&self) -> &MutationOutcome {
        &self.outcome
    }

    /// The outcome at axis width.
    #[must_use]
    pub fn verdict(&self) -> MutationVerdict {
        MutationVerdict::from(&self.outcome)
    }

    /// What was established about equivalence.
    #[must_use]
    pub const fn equivalence(&self) -> EquivalenceAxis {
        self.equivalence
    }
}

// ---------------------------------------------------------------------------
// The run, and the accounting over it.
// ---------------------------------------------------------------------------

impl MutationCensus {
    /// The accounting over one run's mutants, counted from the records themselves.
    #[must_use]
    pub fn over(reports: &[MutationReport]) -> Self {
        let mut killed: u32 = 0;
        let mut survived: u32 = 0;
        let mut inconclusive: u32 = 0;
        for report in reports {
            match report.verdict() {
                MutationVerdict::Killed => killed = killed.saturating_add(1),
                MutationVerdict::Survived => survived = survived.saturating_add(1),
                MutationVerdict::Inconclusive => inconclusive = inconclusive.saturating_add(1),
            }
        }
        Self {
            killed,
            survived,
            inconclusive,
        }
    }

    /// How many mutants the suite rejected.
    #[must_use]
    pub const fn killed(self) -> u32 {
        self.killed
    }

    /// How many mutants the suite accepted after a positive firing observation.
    #[must_use]
    pub const fn survived(self) -> u32 {
        self.survived
    }

    /// How many mutants established nothing.
    #[must_use]
    pub const fn inconclusive(self) -> u32 {
        self.inconclusive
    }

    /// How many mutants the run pressed, as the sum of its parts.
    #[must_use]
    pub const fn pressed(self) -> u32 {
        self.killed
            .saturating_add(self.survived)
            .saturating_add(self.inconclusive)
    }
}

impl BaselineQualification {
    /// The precondition, read from the baseline axis one run recorded.
    ///
    /// # Errors
    ///
    /// Refuses a baseline that ran and did not pass, then one that was not run at all.
    pub(in crate::muterprater) const fn read(
        axis: BaselineAxis,
    ) -> Result<Self, BaselinePrecondition> {
        match axis {
            BaselineAxis::Qualified => Ok(Self { axis }),
            BaselineAxis::Failed => Err(BaselinePrecondition::BaselineFailed),
            BaselineAxis::NotRun => Err(BaselinePrecondition::BaselineNotRun),
        }
    }

    /// The axis reading this qualification was read from.
    #[must_use]
    pub const fn axis(self) -> BaselineAxis {
        self.axis
    }
}

impl MutationRun {
    /// One pressure run's record, with the census counted from the reports.
    #[must_use]
    pub(in crate::muterprater) fn recorded(
        baseline: BaselineQualification,
        reports: Vec<MutationReport>,
    ) -> Self {
        let census = MutationCensus::over(&reports);
        Self {
            baseline,
            reports,
            census,
        }
    }

    /// The qualified baseline the run stood on.
    #[must_use]
    pub const fn baseline(&self) -> BaselineQualification {
        self.baseline
    }

    /// Every mutant's record, in the order the run pressed them.
    #[must_use]
    pub fn reports(&self) -> &[MutationReport] {
        &self.reports
    }

    /// The accounting over them.
    #[must_use]
    pub const fn census(&self) -> MutationCensus {
        self.census
    }

    /// Every mutant this run killed, in the order the run pressed them.
    ///
    /// The roster a trust-opening fact is read out of: a run with no kill has shown no property biting.
    pub fn kills(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() == MutationVerdict::Killed)
    }

    /// Every mutant this run did not kill, whatever the reason.
    ///
    /// The roster a reader means by "what got through", which under a backend with no activation channel is inconclusive rather than survived.
    pub fn non_kills(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() != MutationVerdict::Killed)
    }

    /// Every mutant this run classified as survived under its activation ceiling.
    pub fn survivors(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() == MutationVerdict::Survived)
    }
}

// ---------------------------------------------------------------------------
// What a wrapped backend's output is read into.
// ---------------------------------------------------------------------------

impl BackendCommand {
    /// Retain one backend command as an executable followed by its exact argument tokens.
    ///
    /// # Errors
    ///
    /// Refuses an empty executable, which states no program to invoke.
    pub fn declared(executable: &str, arguments: &[&str]) -> Result<Self, BackendCommandRefusal> {
        if executable.is_empty() {
            return Err(BackendCommandRefusal::EmptyExecutable);
        }
        Ok(Self {
            executable: executable.to_owned(),
            arguments: arguments
                .iter()
                .map(|argument| (*argument).to_owned())
                .collect(),
        })
    }

    /// The executable token.
    #[must_use]
    pub fn executable(&self) -> &str {
        &self.executable
    }

    /// The argument tokens, in invocation order.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

impl MutationBackendInvocation {
    /// State the exact backend execution context one imported artifact records.
    #[must_use]
    pub fn declared(
        backend: WrappedBackend,
        version: BackendVersion,
        command: BackendCommand,
        target: crate::report::TargetBinding,
    ) -> Self {
        Self {
            backend,
            version,
            command,
            target,
        }
    }

    /// The backend the command invokes.
    #[must_use]
    pub const fn backend(&self) -> WrappedBackend {
        self.backend
    }

    /// The backend version the artifact states produced its output.
    #[must_use]
    pub const fn version(&self) -> &BackendVersion {
        &self.version
    }

    /// The exact command tokens the artifact states were invoked.
    #[must_use]
    pub const fn command(&self) -> &BackendCommand {
        &self.command
    }

    /// The target and toolchain the artifact states it ran under.
    #[must_use]
    pub const fn target(&self) -> &crate::report::TargetBinding {
        &self.target
    }
}

impl BackendVersion {
    /// The version the party that ran the backend states.
    ///
    /// # Errors
    ///
    /// Refuses an empty spelling, which states no version.
    pub fn stated(spelling: &str) -> Result<Self, BackendVersionRefusal> {
        if spelling.is_empty() {
            return Err(BackendVersionRefusal::EmptySpelling);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The spelling that party stated.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl BackendOutputId {
    /// Derive the content identity of exact imported backend-output bytes.
    pub(in crate::muterprater) fn derived(bytes: &[u8]) -> Self {
        Self(ContentAddress::derived(BACKEND_OUTPUT_TAG, bytes))
    }

    /// The underlying content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl MutationSourceRevisionId {
    /// Derive one exact mutation-source revision from its bytes.
    fn derived(bytes: &[u8]) -> Self {
        Self(ContentAddress::derived(MUTATION_SOURCE_REVISION_TAG, bytes))
    }

    /// The underlying content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl MutationSourceRevision {
    /// Bind one reported source path to the exact source bytes an artifact or current comparison stood over.
    ///
    /// # Errors
    ///
    /// Refuses an empty file spelling, which identifies no source seat.
    pub fn from_content(file: &str, bytes: &[u8]) -> Result<Self, CoordinateRefusal> {
        if file.is_empty() {
            return Err(CoordinateRefusal::EmptyFile);
        }
        Ok(Self {
            file: file.to_owned(),
            revision: MutationSourceRevisionId::derived(bytes),
        })
    }

    /// The reported source-file spelling.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The exact content revision of that source file.
    #[must_use]
    pub const fn revision(&self) -> MutationSourceRevisionId {
        self.revision
    }
}

impl CompiledSuiteArtifactManifest {
    /// Retain one parser-produced reading under its exact backend invocation, output identity, and source revisions.
    pub(in crate::muterprater) fn recorded(
        invocation: MutationBackendInvocation,
        output: BackendOutputId,
        sources: Vec<MutationSourceRevision>,
        reading: WrapReading,
    ) -> Self {
        Self {
            invocation,
            output,
            sources,
            reading,
        }
    }

    /// The backend execution context the artifact states.
    #[must_use]
    pub const fn invocation(&self) -> &MutationBackendInvocation {
        &self.invocation
    }

    /// The exact imported backend-output content identity.
    #[must_use]
    pub const fn output(&self) -> BackendOutputId {
        self.output
    }

    /// The exact source revisions, ordered by reported file spelling.
    #[must_use]
    pub fn sources(&self) -> &[MutationSourceRevision] {
        &self.sources
    }

    /// The parser-produced reading retained by this manifest.
    #[must_use]
    pub const fn reading(&self) -> &WrapReading {
        &self.reading
    }
}

impl CompiledSuiteArtifactCustody {
    /// Join an imported artifact manifest to the exact current source revisions a caller supplies.
    ///
    /// The comparison is over the complete manifest roster by file and revision, so a missing, added, duplicated, or moved source refuses instead of silently narrowing currency.
    ///
    /// # Errors
    ///
    /// Refuses duplicate current files first, then a manifest file missing from the current roster, then an unexpected current file, then the first source revision that moved in file order.
    pub fn current(
        manifest: CompiledSuiteArtifactManifest,
        current_sources: Vec<MutationSourceRevision>,
    ) -> Result<Self, ArtifactCustodyRefusal> {
        let mut current = BTreeMap::new();
        for source in current_sources {
            let file = source.file().to_owned();
            if current.insert(file.clone(), source).is_some() {
                return Err(ArtifactCustodyRefusal::DuplicateCurrentSource(file));
            }
        }
        let expected: BTreeMap<&str, MutationSourceRevisionId> = manifest
            .sources()
            .iter()
            .map(|source| (source.file(), source.revision()))
            .collect();
        for file in expected.keys().copied() {
            if !current.contains_key(file) {
                return Err(ArtifactCustodyRefusal::CurrentSourceMissing(
                    file.to_owned(),
                ));
            }
        }
        for file in current.keys() {
            if !expected.contains_key(file.as_str()) {
                return Err(ArtifactCustodyRefusal::CurrentSourceUnexpected(
                    file.to_owned(),
                ));
            }
        }
        for (file, expected_revision) in expected {
            match current.get(file) {
                Some(found) if expected_revision != found.revision() => {
                    return Err(ArtifactCustodyRefusal::CurrentSourceMoved {
                        file: file.to_owned(),
                        expected: expected_revision,
                        found: found.revision(),
                    });
                }
                Some(_) => {}
                None => {
                    return Err(ArtifactCustodyRefusal::CurrentSourceMissing(
                        file.to_owned(),
                    ));
                }
            }
        }
        Ok(Self { manifest })
    }

    /// The complete imported artifact manifest this current-source join stands over.
    #[must_use]
    pub const fn manifest(&self) -> &CompiledSuiteArtifactManifest {
        &self.manifest
    }
}

impl GrammarVersion {
    /// The version an adapter's own page states for its line grammar.
    #[must_use]
    pub const fn adapter(version: u32) -> Self {
        Self(version)
    }

    /// The number the adapter states.
    #[must_use]
    pub const fn number(self) -> u32 {
        self.0
    }
}

impl AdapterProfile {
    /// What one reading is stated under.
    #[must_use]
    pub fn stated(
        backend: WrappedBackend,
        version: BackendVersionPosture,
        source: ReadingSource,
        grammar: GrammarVersion,
    ) -> Self {
        Self {
            backend,
            version,
            source,
            grammar,
        }
    }

    /// The backend the reading was taken from.
    #[must_use]
    pub const fn backend(&self) -> WrappedBackend {
        self.backend
    }

    /// Whether the party that ran that backend stated its version.
    #[must_use]
    pub const fn version(&self) -> &BackendVersionPosture {
        &self.version
    }

    /// Which of the backend's outputs the reading was taken from.
    #[must_use]
    pub const fn source(&self) -> ReadingSource {
        self.source
    }

    /// The adapter grammar version the reading was taken under.
    #[must_use]
    pub const fn grammar(&self) -> GrammarVersion {
        self.grammar
    }

    /// The most a reading under this profile can establish.
    ///
    /// Read from the source rather than stored, so a profile can never grant more than the output it was taken from affords.
    #[must_use]
    pub fn ceiling(&self) -> ClaimCeiling {
        ClaimCeiling::from(self.source)
    }
}

impl UnparsedLine {
    /// One line of a backend's output this parser could not read.
    ///
    /// The material is admitted through the record vocabulary's bounded foreign text, so a long line is cut at the bound with the cut marked.
    #[must_use]
    pub fn unread(ordinal: usize, material: &[u8]) -> Self {
        Self {
            ordinal,
            text: ForeignText::admitted(material),
        }
    }

    /// Which line of the output it was, counting from zero.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// The line itself.
    #[must_use]
    pub const fn text(&self) -> &ForeignText {
        &self.text
    }
}

impl WrapReading {
    /// What one reading recovered, stated under the profile it was read through.
    ///
    /// # Errors
    ///
    /// Refuses a run carrying a record whose verdict the profile's ceiling does not admit, naming the record, its verdict, and the ceiling.
    pub(in crate::muterprater) fn read(
        profile: AdapterProfile,
        run: MutationRun,
        announced: AnnouncedRoster,
        unparsed: Vec<UnparsedLine>,
    ) -> Result<Self, WrapRefusal> {
        let ceiling = profile.ceiling();
        for (at, report) in run.reports().iter().enumerate() {
            let verdict = report.verdict();
            if !ceiling.admits(verdict) {
                return Err(WrapRefusal::VerdictPastCeiling {
                    at,
                    verdict,
                    ceiling,
                });
            }
        }
        Ok(Self {
            profile,
            run,
            announced,
            unparsed,
        })
    }

    /// What the reading is stated under.
    #[must_use]
    pub const fn profile(&self) -> &AdapterProfile {
        &self.profile
    }

    /// The run the reading recovered.
    #[must_use]
    pub const fn run(&self) -> &MutationRun {
        &self.run
    }

    /// What the backend announced about its own roster.
    #[must_use]
    pub const fn announced(&self) -> AnnouncedRoster {
        self.announced
    }

    /// Every line the parser could not read, in output order.
    #[must_use]
    pub fn unparsed(&self) -> &[UnparsedLine] {
        &self.unparsed
    }
}

// ---------------------------------------------------------------------------
// Qualification, and the generic suite bite.
// ---------------------------------------------------------------------------

impl AdapterQualification {
    /// The qualification one exact adapter profile stands under.
    ///
    /// The profile is taken from the reading rather than stated beside it, and what the caller states is the grammar standing.
    /// One pairing qualifies: the reading's profile states backend version `v`, and the standing is [`GrammarStanding::Checked`] over `v`.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a standing under which nobody has checked anything, a reading whose profile states no backend version, then a check made against a version other than the one the reading names.
    pub fn of(
        reading: &WrapReading,
        standing: GrammarStanding,
    ) -> Result<Self, QualificationRefusal> {
        let GrammarStanding::Checked(checked) = &standing else {
            return Err(QualificationRefusal::GrammarUnchecked);
        };
        let BackendVersionPosture::Stated(stated) = reading.profile().version() else {
            return Err(QualificationRefusal::BackendVersionUnstated);
        };
        if stated != checked {
            return Err(QualificationRefusal::CheckedAgainstAnotherVersion {
                stated: stated.clone(),
                checked: checked.clone(),
            });
        }
        Ok(Self {
            profile: reading.profile().clone(),
            standing,
        })
    }

    /// The profile the reading was taken under.
    #[must_use]
    pub const fn profile(&self) -> &AdapterProfile {
        &self.profile
    }

    /// Whether that adapter's grammar has been checked against real output.
    #[must_use]
    pub const fn standing(&self) -> &GrammarStanding {
        &self.standing
    }

    /// The most a reading under this qualification can establish.
    ///
    /// The profile's own ceiling, read through rather than restated: qualifying an adapter never widens what its source affords.
    #[must_use]
    pub fn ceiling(&self) -> ClaimCeiling {
        self.profile.ceiling()
    }
}

impl CompiledSuitePressure {
    /// The generic suite pressure one current-source-qualified artifact demonstrated, where it demonstrated one.
    ///
    /// The qualification arrives from [`AdapterQualification::of`] rather than being minted here, so this road weighs a standing somebody already earned against the reading in hand.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a standing that has not reported, a qualification naming a profile other than this artifact's reading, then a reading whose run demonstrated no lawful kill.
    pub fn demonstrated(
        artifact: CompiledSuiteArtifactStanding<'_>,
        qualification: &AdapterQualification,
    ) -> Result<Self, SuitePressureRefusal> {
        let CompiledSuiteArtifactStanding::Reported(custody) = artifact else {
            return Err(SuitePressureRefusal::ArtifactNotReported);
        };
        let reading = custody.manifest().reading();
        if qualification.profile() != reading.profile() {
            return Err(SuitePressureRefusal::QualificationUnderAnotherProfile);
        }
        let Some(kill) = reading
            .run()
            .reports()
            .iter()
            .find(|report| report.verdict() == MutationVerdict::Killed)
        else {
            return Err(SuitePressureRefusal::NoKillDemonstrated);
        };
        Ok(Self {
            qualification: qualification.clone(),
            custody: custody.clone(),
            kill: kill.clone(),
        })
    }

    /// The qualification the witness was demonstrated under.
    #[must_use]
    pub const fn qualification(&self) -> &AdapterQualification {
        &self.qualification
    }

    /// The exact backend invocation, output, parser reading, and current-source custody behind this pressure.
    #[must_use]
    pub const fn custody(&self) -> &CompiledSuiteArtifactCustody {
        &self.custody
    }

    /// The kill it was demonstrated by.
    #[must_use]
    pub const fn kill(&self) -> &MutationReport {
        &self.kill
    }
}

// ---------------------------------------------------------------------------
// Owner policy.
// ---------------------------------------------------------------------------

impl EvaluationFamilyRef {
    /// The evaluation family, parsed from its owner and spelling.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        NamespacedName::named(namespace, stem).map(Self)
    }

    /// The evaluation family over an already-parsed name.
    #[must_use]
    pub const fn over(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The namespaced name this family carries.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl MutationPolicyId {
    /// The policy's derived content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl MutationPermission {
    /// One owner claim's nonempty roster of admitted operator families.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then a family stated twice.
    pub fn declared(
        owner_claim: ClaimRef,
        mut admitted_families: Vec<OperatorFamilyRef>,
    ) -> Result<Self, PermissionRefusal> {
        if admitted_families.is_empty() {
            return Err(PermissionRefusal::NoOperatorFamily);
        }
        admitted_families.sort_by_key(|family| family.slug());
        for pair in admitted_families.windows(2) {
            let [left, right] = pair else {
                continue;
            };
            if left == right {
                return Err(PermissionRefusal::DuplicateOperatorFamily(*right));
            }
        }
        Ok(Self {
            owner_claim,
            admitted_families,
        })
    }

    /// The owner claim this permission is scoped to.
    #[must_use]
    pub const fn owner_claim(&self) -> ClaimRef {
        self.owner_claim
    }

    /// The operator families the owner admits for this claim, in canonical slug order.
    #[must_use]
    pub fn admitted_families(&self) -> &[OperatorFamilyRef] {
        &self.admitted_families
    }

    /// Whether this permission admits one operator family.
    #[must_use]
    pub fn admits(&self, family: OperatorFamilyRef) -> bool {
        self.admitted_families.contains(&family)
    }
}

impl MutationPolicy {
    /// One evaluation family's owner-authored mutation policy.
    ///
    /// An empty permission roster is lawful and admits a point-free evaluation surface; it earns no parity or mutation evidence by existing.
    ///
    /// # Errors
    ///
    /// Refuses two permission rows naming one claim.
    pub fn declared(
        family: EvaluationFamilyRef,
        mut permissions: Vec<MutationPermission>,
    ) -> Result<Self, PolicyRefusal> {
        permissions.sort_by_key(MutationPermission::owner_claim);
        for pair in permissions.windows(2) {
            let [left, right] = pair else {
                continue;
            };
            if left.owner_claim() == right.owner_claim() {
                return Err(PolicyRefusal::DuplicateClaim(right.owner_claim()));
            }
        }
        let preimage = encode::policy_preimage(family, &permissions);
        let identity = MutationPolicyId(encode::address(MUTATION_POLICY_TAG, &preimage));
        Ok(Self {
            family,
            permissions,
            identity,
        })
    }

    /// The evaluation family this policy belongs to.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The policy's derived identity.
    #[must_use]
    pub const fn identity(&self) -> MutationPolicyId {
        self.identity
    }

    /// The policy's permissions, in canonical claim order.
    #[must_use]
    pub fn permissions(&self) -> &[MutationPermission] {
        &self.permissions
    }

    /// The permission row for one owner claim, where this policy carries one.
    #[must_use]
    pub fn permission(&self, claim: ClaimRef) -> Option<&MutationPermission> {
        self.permissions
            .iter()
            .find(|permission| permission.owner_claim() == claim)
    }
}

impl PolicyMembership {
    /// The policy that issued this membership.
    #[must_use]
    pub const fn policy(self) -> MutationPolicyId {
        self.policy
    }

    /// The owner claim this membership is scoped to.
    #[must_use]
    pub const fn owner_claim(self) -> ClaimRef {
        self.owner_claim
    }
}

// ---------------------------------------------------------------------------
// Producer discovery.
// ---------------------------------------------------------------------------

impl ActivationSite {
    /// The site, parsed from the owner that declares it and the spelling it carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        NamespacedName::named(namespace, stem).map(Self)
    }

    /// The site, over a name already parsed.
    #[must_use]
    pub const fn over(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The namespaced name this site carries.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl AlternativeDeclaration {
    /// One discovered operator family and canonical mutation meaning, before policy admission.
    #[must_use]
    pub fn stated(family: OperatorFamilyRef, operation: Vec<u8>) -> Self {
        Self { family, operation }
    }

    /// The operator family the producer attributes this meaning to.
    #[must_use]
    pub const fn family(&self) -> OperatorFamilyRef {
        self.family
    }

    /// The canonical mutation meaning supplied for admission.
    #[must_use]
    pub fn operation(&self) -> &[u8] {
        &self.operation
    }
}

impl DiscoveredMutationSite {
    /// Read one complete discovered site, before owner-policy admission.
    ///
    /// # Errors
    ///
    /// Refuses an empty unchanged operation, an empty alternative roster, then each alternative whose bytes are empty, equal the unchanged operation, or duplicate an earlier family and meaning.
    pub fn discovered(
        identity: MutationPointRef,
        mapping: OwnerClaimMapping,
        original_operation: Vec<u8>,
        alternatives: Vec<AlternativeDeclaration>,
        activation_site: ActivationSite,
    ) -> Result<Self, DiscoveryRefusal> {
        if original_operation.is_empty() {
            return Err(DiscoveryRefusal::EmptyOriginalOperation);
        }
        if alternatives.is_empty() {
            return Err(DiscoveryRefusal::NoAlternative);
        }
        for (at, alternative) in alternatives.iter().enumerate() {
            if alternative.operation().is_empty() {
                return Err(DiscoveryRefusal::EmptyAlternative { at });
            }
            if alternative.operation() == original_operation {
                return Err(DiscoveryRefusal::AlternativeIsOriginal { at });
            }
            if alternatives.iter().take(at).any(|earlier| {
                earlier.family() == alternative.family()
                    && earlier.operation() == alternative.operation()
            }) {
                return Err(DiscoveryRefusal::DuplicateAlternativeMeaning { at });
            }
        }
        Ok(Self {
            identity,
            mapping,
            original_operation,
            alternatives,
            activation_site,
        })
    }

    /// The stable point identity the producer discovered.
    #[must_use]
    pub const fn identity(&self) -> MutationPointRef {
        self.identity
    }

    /// The origin reading's owner-claim mapping posture.
    #[must_use]
    pub const fn mapping(&self) -> OwnerClaimMapping {
        self.mapping
    }

    /// The unchanged operation at this site.
    #[must_use]
    pub fn original_operation(&self) -> &[u8] {
        &self.original_operation
    }

    /// Every discovered alternative, in producer order.
    #[must_use]
    pub fn alternatives(&self) -> &[AlternativeDeclaration] {
        &self.alternatives
    }

    /// The named activation site the producer discovered.
    #[must_use]
    pub const fn activation_site(&self) -> ActivationSite {
        self.activation_site
    }
}

impl DiscoveryEntry {
    /// Retain one discovered site and its derived owner-policy disposition.
    pub(in crate::muterprater) fn recorded(
        site: DiscoveredMutationSite,
        disposition: DiscoveryDisposition,
    ) -> Self {
        Self { site, disposition }
    }

    /// The complete discovered site.
    #[must_use]
    pub const fn site(&self) -> &DiscoveredMutationSite {
        &self.site
    }

    /// Whether and why this site entered the executable surface.
    #[must_use]
    pub const fn disposition(&self) -> DiscoveryDisposition {
        self.disposition
    }
}

impl MutationDiscoveryId {
    /// The discovery reading's content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl MutationDiscoveryReading {
    /// Retain one complete discovery denominator and derive its content identity.
    pub(in crate::muterprater) fn recorded(
        policy: &MutationPolicy,
        entries: Vec<DiscoveryEntry>,
    ) -> Self {
        let preimage = encode::discovery_preimage(policy.family(), policy.identity(), &entries);
        Self {
            family: policy.family(),
            policy: policy.identity(),
            identity: MutationDiscoveryId(encode::address(MUTATION_DISCOVERY_TAG, &preimage)),
            entries,
        }
    }

    /// The evaluation family whose discovery was read.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The owner policy the discovered sites were admitted against.
    #[must_use]
    pub const fn policy(&self) -> MutationPolicyId {
        self.policy
    }

    /// The content identity of the complete discovery denominator.
    #[must_use]
    pub const fn identity(&self) -> MutationDiscoveryId {
        self.identity
    }

    /// Every discovered site and disposition, in producer order.
    #[must_use]
    pub fn entries(&self) -> &[DiscoveryEntry] {
        &self.entries
    }
}

impl MutationSurfaceLowering {
    /// Bind one complete discovery reading to the executable surface derived from it.
    pub(in crate::muterprater) fn lowered(
        discovery: MutationDiscoveryReading,
        surface: EvaluationSurface,
    ) -> Self {
        Self { discovery, surface }
    }

    /// The complete discovery denominator.
    #[must_use]
    pub const fn discovery(&self) -> &MutationDiscoveryReading {
        &self.discovery
    }

    /// The executable subset admitted by owner policy.
    #[must_use]
    pub const fn surface(&self) -> &EvaluationSurface {
        &self.surface
    }

    /// Consume the closed lowering into its reading and its executable surface.
    #[must_use]
    pub fn into_parts(self) -> (MutationDiscoveryReading, EvaluationSurface) {
        (self.discovery, self.surface)
    }
}

// ---------------------------------------------------------------------------
// The evaluation surface.
// ---------------------------------------------------------------------------

impl AlternativeId {
    /// The alternative's derived content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl AdmittedAlternative {
    /// The alternative's stable identity.
    #[must_use]
    pub const fn identity(&self) -> AlternativeId {
        self.identity
    }

    /// The owner-permitted operator family this alternative realizes.
    #[must_use]
    pub const fn family(&self) -> OperatorFamilyRef {
        self.family
    }

    /// The canonical mutation meaning selected at runtime.
    #[must_use]
    pub fn operation(&self) -> &[u8] {
        &self.operation
    }
}

impl MutationPoint {
    /// Admit one structurally read, mapped, and policy-permitted discovery.
    pub(in crate::muterprater) fn admitted(
        policy: &MutationPolicy,
        owner_claim: ClaimRef,
        discovered: DiscoveredMutationSite,
    ) -> Self {
        let identity = discovered.identity;
        let mut admitted = Vec::new();
        for alternative in discovered.alternatives {
            let preimage = encode::alternative_preimage(
                identity,
                alternative.family(),
                alternative.operation(),
            );
            admitted.push(AdmittedAlternative {
                identity: AlternativeId(encode::address(MUTATION_ALTERNATIVE_TAG, &preimage)),
                family: alternative.family(),
                operation: alternative.operation,
            });
        }
        admitted.sort_by_key(AdmittedAlternative::identity);
        Self {
            identity,
            membership: PolicyMembership {
                policy: policy.identity(),
                owner_claim,
            },
            original_operation: discovered.original_operation,
            admitted_alternatives: admitted,
            activation_site: discovered.activation_site,
        }
    }

    /// The reference this point is known by.
    #[must_use]
    pub const fn identity(&self) -> MutationPointRef {
        self.identity
    }

    /// The policy-issued membership this point carries.
    #[must_use]
    pub const fn membership(&self) -> PolicyMembership {
        self.membership
    }

    /// The claim that owns the behaviour at this point.
    #[must_use]
    pub const fn owner_claim(&self) -> ClaimRef {
        self.membership.owner_claim()
    }

    /// What the point reads as under no mutation.
    #[must_use]
    pub fn original_operation(&self) -> &[u8] {
        &self.original_operation
    }

    /// The damages this point may be selected into, in canonical alternative-identity order.
    #[must_use]
    pub fn admitted_alternatives(&self) -> &[AdmittedAlternative] {
        &self.admitted_alternatives
    }

    /// Where a selected alternative fires.
    #[must_use]
    pub const fn activation_site(&self) -> ActivationSite {
        self.activation_site
    }
}

impl EvaluationSurfaceId {
    /// The surface's derived content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl EvaluationSurface {
    /// Assemble an already policy-issued, identity-distinct point roster.
    pub(in crate::muterprater) fn admitted(
        policy: &MutationPolicy,
        mut points: Vec<MutationPoint>,
    ) -> Self {
        points.sort_by_key(MutationPoint::identity);
        let preimage = encode::surface_preimage(policy.family(), policy.identity(), &points);
        let identity = EvaluationSurfaceId(encode::address(EVALUATION_SURFACE_TAG, &preimage));
        Self {
            family: policy.family(),
            policy: policy.identity(),
            identity,
            points,
        }
    }

    /// The evaluation family this surface belongs to.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The owner policy this surface was admitted under.
    #[must_use]
    pub const fn policy(&self) -> MutationPolicyId {
        self.policy
    }

    /// The exact surface identity.
    #[must_use]
    pub const fn identity(&self) -> EvaluationSurfaceId {
        self.identity
    }

    /// Whether this surface admits an active directive.
    #[must_use]
    pub const fn catalog_posture(&self) -> PointCatalogPosture {
        if self.points.is_empty() {
            PointCatalogPosture::NoAdmittedPoints
        } else {
            PointCatalogPosture::Mutable
        }
    }

    /// Every point the table carries, in canonical point-identity order.
    #[must_use]
    pub fn points(&self) -> &[MutationPoint] {
        &self.points
    }

    /// The point this reference names, where the table carries one.
    #[must_use]
    pub fn point(&self, identity: MutationPointRef) -> Option<&MutationPoint> {
        self.points
            .iter()
            .find(|point| point.identity() == identity)
    }

    /// Select one point into one admitted mutation meaning.
    ///
    /// Runtime is selection among admitted alternatives, never interpretation of arbitrary source, and alternative identity is independent of roster order.
    ///
    /// # Errors
    ///
    /// Refuses a point the table does not carry, then an alternative that point does not admit.
    pub fn select(
        &self,
        point: MutationPointRef,
        alternative: AlternativeId,
    ) -> Result<ActiveSelection, SelectionRefusal> {
        let Some(found) = self.point(point) else {
            return Err(SelectionRefusal::NoSuchPoint(point));
        };
        if !found
            .admitted_alternatives()
            .iter()
            .any(|admitted| admitted.identity() == alternative)
        {
            return Err(SelectionRefusal::NoSuchAlternative { point, alternative });
        }
        Ok(ActiveSelection {
            surface: self.identity,
            point,
            alternative,
        })
    }

    /// Every active selection this surface admits, in canonical point and alternative order.
    #[must_use]
    pub fn selections(&self) -> Vec<ActiveSelection> {
        self.points
            .iter()
            .flat_map(|point| {
                point
                    .admitted_alternatives()
                    .iter()
                    .map(|alternative| ActiveSelection {
                        surface: self.identity,
                        point: point.identity(),
                        alternative: alternative.identity(),
                    })
            })
            .collect()
    }
}

impl ActiveSelection {
    /// The evaluation surface that issued this selection.
    #[must_use]
    pub const fn surface(self) -> EvaluationSurfaceId {
        self.surface
    }

    /// The point that is damaged.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.point
    }

    /// Which of its admitted alternatives is active.
    #[must_use]
    pub const fn alternative(self) -> AlternativeId {
        self.alternative
    }
}

impl<'surface> ResolvedMutation<'surface> {
    /// Bind one surface-issued selection to the exact point and alternative it resolved to.
    pub(in crate::muterprater) const fn resolved(
        selection: ActiveSelection,
        point: &'surface MutationPoint,
        alternative: &'surface AdmittedAlternative,
    ) -> Self {
        Self {
            selection,
            point,
            alternative,
        }
    }

    /// The exact surface-issued selection that was resolved.
    #[must_use]
    pub const fn selection(self) -> ActiveSelection {
        self.selection
    }

    /// The admitted point selected for this call.
    #[must_use]
    pub const fn point(self) -> &'surface MutationPoint {
        self.point
    }

    /// The admitted alternative selected for this call.
    #[must_use]
    pub const fn alternative(self) -> &'surface AdmittedAlternative {
        self.alternative
    }
}

impl<'surface> EvaluationDirective<'surface> {
    /// The directly representable no-mutation posture.
    #[must_use]
    pub const fn no_mutation() -> Self {
        Self { resolved: None }
    }

    /// One active directive, after its selection was resolved against the exact surface.
    pub(in crate::muterprater) const fn active(
        selection: ActiveSelection,
        point: &'surface MutationPoint,
        alternative: &'surface AdmittedAlternative,
    ) -> Self {
        Self {
            resolved: Some(ResolvedMutation::resolved(selection, point, alternative)),
        }
    }

    /// The exact resolved mutation, where this directive is active.
    #[must_use]
    pub const fn resolved(self) -> Option<ResolvedMutation<'surface>> {
        self.resolved
    }
}

// ---------------------------------------------------------------------------
// The pair, and its bindings.
// ---------------------------------------------------------------------------

impl<Meaning> EvaluationObservation<Meaning> {
    /// Raw output from one evaluation call.
    #[must_use]
    pub const fn observed(meaning: Meaning, firings: u32) -> Self {
        Self { meaning, firings }
    }

    /// The evaluation meaning.
    #[must_use]
    pub const fn meaning(&self) -> &Meaning {
        &self.meaning
    }

    /// How many activation firings the evaluation callable reports.
    #[must_use]
    pub const fn firings(&self) -> u32 {
        self.firings
    }

    /// Split the raw output for receiver validation.
    pub(in crate::muterprater) fn into_parts(self) -> (Meaning, u32) {
        (self.meaning, self.firings)
    }
}

impl<Input, Meaning> ProductionBinding<Input, Meaning> {
    /// The production callable and revision declared for one evaluation family.
    #[must_use]
    pub const fn declared(
        family: EvaluationFamilyRef,
        revision: RevisionBinding,
        call: ProductionCall<Input, Meaning>,
    ) -> Self {
        Self {
            family,
            revision,
            call,
        }
    }

    /// The declared evaluation family.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The production revision binding.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }

    /// Run the production callable.
    #[must_use]
    pub fn evaluate(&self, input: &Input) -> Meaning {
        (self.call)(input)
    }
}

impl<Input, Meaning> EvaluationBinding<Input, Meaning> {
    /// Bind the evaluation callable and revision to one exact surface.
    ///
    /// The family and surface identity are derived from `surface`, so a caller keeps no parallel labels coherent.
    #[must_use]
    pub const fn declared(
        surface: &EvaluationSurface,
        revision: RevisionBinding,
        call: EvaluationCall<Input, Meaning>,
    ) -> Self {
        Self {
            family: surface.family(),
            revision,
            surface: surface.identity(),
            call,
        }
    }

    /// The declared evaluation family.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The evaluation revision binding.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }

    /// The exact evaluation surface this callable executes.
    #[must_use]
    pub const fn surface(&self) -> EvaluationSurfaceId {
        self.surface
    }

    /// Run the evaluation callable under one surface-bound directive.
    ///
    /// # Errors
    ///
    /// Returns the callable's typed refusal when it does not implement the offered directive.
    pub fn evaluate(
        &self,
        input: &Input,
        directive: EvaluationDirective<'_>,
    ) -> Result<EvaluationObservation<Meaning>, EvaluationCallRefusal> {
        (self.call)(input, directive)
    }
}

impl<Input, Meaning> EvaluationPair<Input, Meaning> {
    /// Join production and evaluation bindings under one declared family and equivalence.
    ///
    /// # Errors
    ///
    /// Refuses bindings naming different evaluation families.
    pub fn paired(
        production: ProductionBinding<Input, Meaning>,
        evaluation: EvaluationBinding<Input, Meaning>,
        same: Equivalence<Meaning>,
    ) -> Result<Self, EvaluationPairRefusal> {
        if production.family() != evaluation.family() {
            return Err(EvaluationPairRefusal::FamilyMismatch {
                production: production.family(),
                evaluation: evaluation.family(),
            });
        }
        Ok(Self {
            production,
            evaluation,
            same,
        })
    }

    /// The production binding.
    #[must_use]
    pub const fn production(&self) -> &ProductionBinding<Input, Meaning> {
        &self.production
    }

    /// The evaluation binding.
    #[must_use]
    pub const fn evaluation(&self) -> &EvaluationBinding<Input, Meaning> {
        &self.evaluation
    }

    /// The owner-declared equivalence over the two meanings.
    #[must_use]
    pub const fn equivalence(&self) -> Equivalence<Meaning> {
        self.same
    }

    /// The identity and revision facts this pair retains in evidence.
    #[must_use]
    pub const fn standing(&self) -> EvaluationPairStanding {
        EvaluationPairStanding {
            family: self.production.family(),
            production_revision: self.production.revision(),
            evaluation_revision: self.evaluation.revision(),
            surface: self.evaluation.surface(),
        }
    }
}

impl EvaluationPairStanding {
    /// Project one exact standing disagreement, without weakening whole-standing admission.
    pub(in crate::muterprater) fn mismatch(
        self,
        found: Self,
    ) -> Option<EvaluationPairStandingMismatch> {
        if self == found {
            return None;
        }
        if self.family != found.family {
            return Some(EvaluationPairStandingMismatch::Family {
                expected: self.family,
                found: found.family,
            });
        }
        if self.production_revision != found.production_revision {
            return Some(EvaluationPairStandingMismatch::ProductionRevision {
                expected: self.production_revision,
                found: found.production_revision,
            });
        }
        if self.evaluation_revision != found.evaluation_revision {
            return Some(EvaluationPairStandingMismatch::EvaluationRevision {
                expected: self.evaluation_revision,
                found: found.evaluation_revision,
            });
        }
        if self.surface != found.surface {
            return Some(EvaluationPairStandingMismatch::Surface {
                expected: self.surface,
                found: found.surface,
            });
        }
        Some(EvaluationPairStandingMismatch::StandingChanged)
    }

    /// The evaluation family shared by both bindings.
    #[must_use]
    pub const fn family(self) -> EvaluationFamilyRef {
        self.family
    }

    /// The production revision.
    #[must_use]
    pub const fn production_revision(self) -> RevisionBinding {
        self.production_revision
    }

    /// The evaluation revision.
    #[must_use]
    pub const fn evaluation_revision(self) -> RevisionBinding {
        self.evaluation_revision
    }

    /// The exact evaluation surface.
    #[must_use]
    pub const fn surface(self) -> EvaluationSurfaceId {
        self.surface
    }
}

impl<Meaning> MutationWitness<Meaning> {
    /// Join one trial binding to the identity and callable of the check its executions report through.
    ///
    /// # Errors
    ///
    /// Refuses a check identity other than the one the trial row retains.
    /// The function-pointer shape excludes captured state and cannot establish that the callable's behavior matches its declared identity; the execution lane observes that.
    pub fn bound(
        binding: TrialBinding,
        check_ref: CheckRef,
        check: MeaningCheck<Meaning>,
    ) -> Result<Self, MutationWitnessRefusal> {
        let expected = binding.row().check();
        if check_ref != expected {
            return Err(MutationWitnessRefusal::CheckMismatch {
                expected,
                found: check_ref,
            });
        }
        Ok(Self { binding, check })
    }

    /// The exact trial binding the receiver reports through.
    #[must_use]
    pub const fn binding(&self) -> &TrialBinding {
        &self.binding
    }

    /// The check identity bound to the callable.
    #[must_use]
    pub const fn check_ref(&self) -> CheckRef {
        self.binding.row().check()
    }

    /// Judge one produced meaning under the declared check callable.
    #[must_use]
    pub fn conclude(&self, meaning: &Meaning) -> TrialConclusion {
        (self.check)(meaning)
    }
}

// ---------------------------------------------------------------------------
// The no-mutation parity.
// ---------------------------------------------------------------------------

impl<Meaning> NoMutationResults<Meaning> {
    /// The production meaning, the no-mutation evaluation meaning, and the evaluation firing count.
    pub(in crate::muterprater) const fn observed(
        production: Meaning,
        evaluation: Meaning,
        evaluation_firings: u32,
    ) -> Self {
        Self {
            production,
            evaluation,
            evaluation_firings,
        }
    }

    /// The production meaning.
    #[must_use]
    pub const fn production(&self) -> &Meaning {
        &self.production
    }

    /// The evaluation meaning under no mutation.
    #[must_use]
    pub const fn evaluation(&self) -> &Meaning {
        &self.evaluation
    }

    /// How many activation firings the no-mutation call reported.
    #[must_use]
    pub const fn evaluation_firings(&self) -> u32 {
        self.evaluation_firings
    }
}

impl NoMutationReports {
    /// Retain the production and evaluation reports in their semantic roles.
    pub(in crate::muterprater) fn recorded(
        production: TrialReport,
        evaluation: TrialReport,
    ) -> Self {
        Self {
            production,
            evaluation,
        }
    }

    /// The production report.
    const fn production(&self) -> &TrialReport {
        &self.production
    }

    /// The evaluation report.
    const fn evaluation(&self) -> &TrialReport {
        &self.evaluation
    }
}

impl<'pair, 'input, Input, Meaning> NoMutationParityReading<'pair, 'input, Input, Meaning> {
    /// Record one complete no-mutation comparison, after both observations joined the same trial binding.
    pub(in crate::muterprater) fn recorded(
        pair: &'pair EvaluationPair<Input, Meaning>,
        witness: MutationWitness<Meaning>,
        input: &'input Input,
        results: NoMutationResults<Meaning>,
        substrate: SharedSubstrate,
        conclusion: TrialConclusion,
        reports: NoMutationReports,
    ) -> Self {
        Self {
            pair,
            witness,
            input,
            results,
            substrate,
            conclusion,
            reports,
        }
    }

    /// The exact pair that ran.
    #[must_use]
    pub const fn pair(&self) -> &'pair EvaluationPair<Input, Meaning> {
        self.pair
    }

    /// The exact trial binding, check identity, and check callable both roads used.
    #[must_use]
    pub const fn witness(&self) -> &MutationWitness<Meaning> {
        &self.witness
    }

    /// The exact input both roads received.
    #[must_use]
    pub const fn input(&self) -> &'input Input {
        self.input
    }

    /// The production meaning.
    #[must_use]
    pub const fn production(&self) -> &Meaning {
        self.results.production()
    }

    /// The evaluation meaning under no mutation.
    #[must_use]
    pub const fn evaluation(&self) -> &Meaning {
        self.results.evaluation()
    }

    /// How many activation firings the no-mutation call reported.
    #[must_use]
    pub const fn evaluation_firings(&self) -> u32 {
        self.results.evaluation_firings()
    }

    /// The foundations both roads share.
    #[must_use]
    pub const fn substrate(&self) -> &SharedSubstrate {
        &self.substrate
    }

    /// The owner-declared equivalence's conclusion.
    #[must_use]
    pub const fn conclusion(&self) -> &TrialConclusion {
        &self.conclusion
    }

    /// The production execution report.
    #[must_use]
    pub const fn production_report(&self) -> &TrialReport {
        self.reports.production()
    }

    /// The no-mutation evaluation execution report.
    #[must_use]
    pub const fn evaluation_report(&self) -> &TrialReport {
        self.reports.evaluation()
    }
}

impl<'pair, 'input, Input, Meaning> NoMutationParityQualification<'pair, 'input, Input, Meaning> {
    /// A no-mutation reading that both reports, zero activation, and semantic agreement qualified.
    pub(in crate::muterprater) fn qualified(
        reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
    ) -> Self {
        Self { reading }
    }

    /// The complete reading this qualification stands on.
    #[must_use]
    pub const fn reading(&self) -> &NoMutationParityReading<'pair, 'input, Input, Meaning> {
        &self.reading
    }
}

impl<'pair, 'input, Input, Meaning> RejectedNoMutationParity<'pair, 'input, Input, Meaning> {
    /// A complete no-mutation reading that did not qualify.
    pub(in crate::muterprater) fn rejected(
        cause: ParityQualificationRefusal,
        reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
    ) -> Self {
        Self { cause, reading }
    }

    /// Why the reading did not qualify.
    pub const fn cause(&self) -> ParityQualificationRefusal {
        self.cause
    }

    /// The complete reading that did not qualify.
    #[must_use]
    pub const fn reading(&self) -> &NoMutationParityReading<'pair, 'input, Input, Meaning> {
        &self.reading
    }
}

impl<'pair, 'input, Input, Meaning> NoMutationParityStanding<'pair, 'input, Input, Meaning> {
    /// The qualification, where this reading earned one.
    #[must_use]
    pub const fn qualification(
        &self,
    ) -> Option<&NoMutationParityQualification<'pair, 'input, Input, Meaning>> {
        match self {
            Self::Qualified(qualification) => Some(qualification),
            Self::Rejected(_) => None,
        }
    }

    /// The rejected reading, where qualification was refused.
    #[must_use]
    pub const fn rejection(
        &self,
    ) -> Option<&RejectedNoMutationParity<'pair, 'input, Input, Meaning>> {
        match self {
            Self::Qualified(_) => None,
            Self::Rejected(rejection) => Some(rejection),
        }
    }
}

// ---------------------------------------------------------------------------
// Compiled specimens.
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// The interpreted lane's trust boundary.
// ---------------------------------------------------------------------------

impl<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
    InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
{
    /// Open interpreted execution over one surface, generic suite bite, and exact selection pressure.
    pub(in crate::muterprater) fn opened(
        surface: &'surface EvaluationSurface,
        suite: &'suite CompiledSuitePressure,
        projection: &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
    ) -> Self {
        Self {
            surface,
            suite,
            projection,
        }
    }

    /// The exact evaluation surface interpreted selection runs over.
    #[must_use]
    pub const fn surface(&self) -> &'surface EvaluationSurface {
        self.surface
    }

    /// The generic compiled suite bite, retained without evaluation-pair authority.
    #[must_use]
    pub const fn suite(&self) -> &'suite CompiledSuitePressure {
        self.suite
    }

    /// The exact compiled selected-projection pressure for this selection.
    #[must_use]
    pub const fn projection(
        &self,
    ) -> &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning> {
        self.projection
    }

    /// The no-mutation qualification the exact projection pressure retains.
    #[must_use]
    pub const fn parity(
        &self,
    ) -> &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning> {
        self.projection.parity()
    }

    /// The only surface-issued selection this trust authorizes.
    #[must_use]
    pub const fn selection(&self) -> ActiveSelection {
        self.projection.standing().selection()
    }

    /// Duplicate this borrowed trust statement for one admitted evidence record.
    pub(in crate::muterprater) fn duplicate(&self) -> Self {
        Self {
            surface: self.surface,
            suite: self.suite,
            projection: self.projection,
        }
    }
}

impl<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
    InterpretedMutationEvidence<
        'surface,
        'suite,
        'projection,
        'parity,
        'pair,
        'input,
        Input,
        Meaning,
    >
{
    /// One active execution, admitted under the trust boundary that made it evidence.
    pub(in crate::muterprater) fn admitted(
        trust: InterpretedTrust<
            'surface,
            'suite,
            'projection,
            'parity,
            'pair,
            'input,
            Input,
            Meaning,
        >,
        meaning: Meaning,
        report: TrialReport,
        mutation: MutationReport,
    ) -> Self {
        Self {
            trust,
            meaning,
            report,
            mutation,
        }
    }

    /// The trust evidence this interpreted result was admitted under.
    #[must_use]
    pub const fn trust(
        &self,
    ) -> &InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
    {
        &self.trust
    }

    /// The exact active selection that ran.
    #[must_use]
    pub const fn selection(&self) -> ActiveSelection {
        self.trust.selection()
    }

    /// The meaning the active evaluation callable returned.
    #[must_use]
    pub const fn meaning(&self) -> &Meaning {
        &self.meaning
    }

    /// The trial report admitted through the report spine.
    #[must_use]
    pub const fn report(&self) -> &TrialReport {
        &self.report
    }

    /// The mutation report derived from the active execution.
    #[must_use]
    pub const fn mutation(&self) -> &MutationReport {
        &self.mutation
    }
}

// ---------------------------------------------------------------------------
// The rewrite lane.
// ---------------------------------------------------------------------------

impl RewriteDescriptor {
    /// One rewrite-mutation descriptor, as its author states it.
    ///
    /// # Errors
    ///
    /// Refuses an empty pattern, then an empty rewrite, then a pair whose two sides are one shape.
    pub fn declared(
        family: OperatorFamilyRef,
        pattern: &'static str,
        rewrite: &'static str,
    ) -> Result<Self, RewriteRefusal> {
        if pattern.is_empty() {
            return Err(RewriteRefusal::EmptyPattern);
        }
        if rewrite.is_empty() {
            return Err(RewriteRefusal::EmptyRewrite);
        }
        if pattern == rewrite {
            return Err(RewriteRefusal::RewriteIsPattern);
        }
        Ok(Self {
            family,
            pattern,
            rewrite,
        })
    }

    /// The operator family this pair realizes.
    #[must_use]
    pub const fn family(self) -> OperatorFamilyRef {
        self.family
    }

    /// The shape a damage matches.
    #[must_use]
    pub const fn pattern(self) -> &'static str {
        self.pattern
    }

    /// The shape it rewrites to.
    #[must_use]
    pub const fn rewrite(self) -> &'static str {
        self.rewrite
    }
}

impl RewriteRoster {
    /// The lane's declared descriptors.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then two entries stating one pattern-and-rewrite pair — refused rather than folded away, because collapsing a duplicate silently would normalize an authoring defect out of sight.
    pub fn declared(descriptors: Vec<RewriteDescriptor>) -> Result<Self, RosterRefusal> {
        if descriptors.is_empty() {
            return Err(RosterRefusal::EmptyRoster);
        }
        for (at, descriptor) in descriptors.iter().enumerate() {
            if descriptors.iter().take(at).any(|earlier| {
                earlier.pattern() == descriptor.pattern()
                    && earlier.rewrite() == descriptor.rewrite()
            }) {
                return Err(RosterRefusal::DuplicateDescriptor { at });
            }
        }
        Ok(Self { descriptors })
    }

    /// Every descriptor the roster carries, in declared order.
    #[must_use]
    pub fn descriptors(&self) -> &[RewriteDescriptor] {
        &self.descriptors
    }
}

impl RewriteCandidate {
    /// One descriptor planned for the harness's audit.
    #[must_use]
    pub fn planned(descriptor: RewriteDescriptor, scope: ScopeShape) -> Self {
        Self {
            descriptor,
            scope,
            trust: RewriteTrust::AuditPending,
        }
    }

    /// The descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> RewriteDescriptor {
        self.descriptor
    }

    /// The scope its application was planned under.
    #[must_use]
    pub const fn scope(&self) -> &ScopeShape {
        &self.scope
    }

    /// The trust posture it stands under.
    #[must_use]
    pub const fn trust(&self) -> RewriteTrust {
        self.trust
    }
}

// ---------------------------------------------------------------------------
// Survivor explanation, and the check gap.
// ---------------------------------------------------------------------------

impl SurvivorExplanation {
    /// The explanation one survivor's record hands into synthesis.
    ///
    /// # Errors
    ///
    /// Refuses a record whose verdict is not survived, then a target whose owning claim is unmapped.
    pub fn of(
        report: &MutationReport,
        missing: OracleClass,
        closing: CheckRef,
    ) -> Result<Self, ExplanationRefusal> {
        let verdict = report.verdict();
        match verdict {
            MutationVerdict::Survived => {}
            MutationVerdict::Killed | MutationVerdict::Inconclusive => {
                return Err(ExplanationRefusal::NotASurvivor(verdict));
            }
        }
        let Some(claim) = report.target().owning_claim() else {
            return Err(ExplanationRefusal::OwnerUnmapped);
        };
        Ok(Self {
            target: report.target().clone(),
            claim,
            missing,
            closing,
        })
    }

    /// The target that survived.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// The claim that owns it.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The oracle class no check of that claim supplies.
    #[must_use]
    pub const fn missing(&self) -> OracleClass {
        self.missing
    }

    /// The check reference that would close the opening.
    #[must_use]
    pub const fn closing(&self) -> CheckRef {
        self.closing
    }
}

impl CheckGap {
    /// The finding a synthesis raises where the closing check has no attachment.
    pub const fn found(claim: ClaimRef, check: CheckRef, missing: OracleClass) -> Self {
        Self {
            claim,
            check,
            missing,
        }
    }

    /// The claim the opening belongs to.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The check reference nobody has written an attachment for.
    #[must_use]
    pub const fn check(self) -> CheckRef {
        self.check
    }

    /// The oracle class the missing check would supply.
    #[must_use]
    pub const fn missing(self) -> OracleClass {
        self.missing
    }
}

impl CandidateSketch {
    /// The row coordinates the caller states beside an explanation.
    #[must_use]
    pub fn stated(
        suite: ExecutionSuite,
        classification: Classification,
        subject: SubjectRoute,
        population: PopulationRef,
    ) -> Self {
        Self {
            suite,
            classification,
            subject,
            population,
        }
    }

    /// The aggregate seat the candidate would run under.
    #[must_use]
    pub const fn suite(&self) -> ExecutionSuite {
        self.suite
    }

    /// How the candidate would be classified.
    #[must_use]
    pub const fn classification(&self) -> &Classification {
        &self.classification
    }

    /// What the candidate would exercise.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.subject
    }

    /// The population that would supply its inputs.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }
}

// ---------------------------------------------------------------------------
// Scope, budget, and the proof plan.
// ---------------------------------------------------------------------------

impl DiffPath {
    /// One path a diff touched.
    ///
    /// # Errors
    ///
    /// Refuses an empty spelling, which names nothing.
    pub fn reported(spelling: &str) -> Result<Self, DiffPathRefusal> {
        if spelling.is_empty() {
            return Err(DiffPathRefusal::EmptyPath);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The spelling the caller read.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl PressureBudget {
    /// What one scoped run may spend.
    ///
    /// # Errors
    ///
    /// Refuses a budget admitting no mutant, because the run it bounds would press nothing.
    pub const fn declared(
        mutants: u32,
        invocation: InvocationProfile,
    ) -> Result<Self, BudgetRefusal> {
        if mutants == 0_u32 {
            return Err(BudgetRefusal::ZeroMutants);
        }
        Ok(Self {
            mutants,
            invocation,
        })
    }

    /// The greatest number of mutants the run may press.
    #[must_use]
    pub const fn mutants(self) -> u32 {
        self.mutants
    }

    /// The per-trial budgets every witness run stands under.
    #[must_use]
    pub const fn invocation(self) -> InvocationProfile {
        self.invocation
    }
}

impl ScopedInvocation {
    /// One scope shape with the budget its run may spend.
    #[must_use]
    pub fn scoped(scope: ScopeShape, budget: PressureBudget) -> Self {
        Self { scope, budget }
    }

    /// What the run is scoped to.
    #[must_use]
    pub const fn scope(&self) -> &ScopeShape {
        &self.scope
    }

    /// What it may spend.
    #[must_use]
    pub const fn budget(&self) -> PressureBudget {
        self.budget
    }
}

impl PlannedRun {
    /// One intended run.
    #[must_use]
    pub fn intended(
        lane: PressureLane,
        target: MutationIdentity,
        damage: PlannedDamage,
        selection: Selection,
        budget: PressureBudget,
    ) -> Self {
        Self {
            lane,
            target,
            damage,
            selection,
            budget,
        }
    }

    /// Which lane it belongs to.
    #[must_use]
    pub const fn lane(&self) -> PressureLane {
        self.lane
    }

    /// What it presses.
    #[must_use]
    pub const fn target(&self) -> MutationIdentity {
        self.target
    }

    /// Which damage of that target it presses.
    #[must_use]
    pub const fn damage(&self) -> PlannedDamage {
        self.damage
    }

    /// What it selects from the complete world.
    #[must_use]
    pub const fn selection(&self) -> &Selection {
        &self.selection
    }

    /// What it may spend.
    #[must_use]
    pub const fn budget(&self) -> PressureBudget {
        self.budget
    }
}

impl ProofPlan {
    /// The complete statement of an intended pressure pass.
    ///
    /// # Errors
    ///
    /// Refuses a plan stating no run, then a plan stating more runs than the scope's mutant budget admits — so a budget is weighed before it is spent rather than discovered spent.
    pub fn planned(scope: ScopedInvocation, runs: Vec<PlannedRun>) -> Result<Self, PlanRefusal> {
        if runs.is_empty() {
            return Err(PlanRefusal::NoRunPlanned);
        }
        let admitted = scope.budget().mutants();
        let planned = runs.len();
        let within = u32::try_from(planned).is_ok_and(|count| count <= admitted);
        if !within {
            return Err(PlanRefusal::BudgetOverspent { admitted, planned });
        }
        Ok(Self { scope, runs })
    }

    /// The scope and budget the pass runs under.
    #[must_use]
    pub const fn scope(&self) -> &ScopedInvocation {
        &self.scope
    }

    /// Every intended run, in planned order.
    #[must_use]
    pub fn runs(&self) -> &[PlannedRun] {
        &self.runs
    }
}

// ---------------------------------------------------------------------------
// The obligation road.
// ---------------------------------------------------------------------------

impl OwedClaim {
    /// The owed posture a claim's declaration states.
    ///
    /// # Errors
    ///
    /// Refuses a posture naming no opening condition, because an obligation that never comes due is one nobody can discharge.
    pub const fn declared(
        claim: ClaimRef,
        opening_condition: &'static str,
    ) -> Result<Self, OwedClaimRefusal> {
        if opening_condition.is_empty() {
            return Err(OwedClaimRefusal::NoOpeningCondition);
        }
        Ok(Self {
            claim,
            opening_condition,
        })
    }

    /// The claim declared owed.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The condition its declaration named as the opening.
    #[must_use]
    pub const fn opening_condition(self) -> &'static str {
        self.opening_condition
    }
}

impl OwedDeclaration {
    /// One owed claim and the shape of proof its opening asks for.
    #[must_use]
    pub const fn stated(owed: OwedClaim, shape: ProofShape) -> Self {
        Self { owed, shape }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The shape of proof it asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl InferredObligation {
    /// One opening a coverage reading stated.
    #[must_use]
    pub const fn inferred(owed: OwedClaim, exercise: ClaimExercise, shape: ProofShape) -> Self {
        Self {
            owed,
            exercise,
            shape,
        }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The counts the coverage reading recorded for it.
    #[must_use]
    pub const fn exercise(self) -> ClaimExercise {
        self.exercise
    }

    /// The shape of proof the opening asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl DischargeEvidence {
    /// What discharged one owed claim.
    #[must_use]
    pub fn recorded(lane: ObligationLane, trial: TrialId, key: ExecutionKey) -> Self {
        Self { lane, trial, key }
    }

    /// The lane the obligation was routed to.
    #[must_use]
    pub const fn lane(&self) -> ObligationLane {
        self.lane
    }

    /// The trial that discharged it.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The key that trial ran under.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }
}

// ---------------------------------------------------------------------------
// Demonstration, and the duplicate comparisons.
// ---------------------------------------------------------------------------

impl Demonstration {
    /// Read a demonstrated kill out of the report a staged run wrote.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a report standing over the authored world rather than a staged view, a census that does not carry the candidate, a candidate the selection passed over, a candidate that did not execute, and a candidate that executed and did not refuse.
    pub fn read(report: RunReport, candidate: TrialId) -> Result<Self, ProofRefusal> {
        match report.posture() {
            TablePosture::Staged { parent: _ } => {}
            TablePosture::Authored => return Err(ProofRefusal::NotStaged),
        }
        let (trial_report, rejection) = {
            let Some(entry) = report
                .census()
                .iter()
                .find(|accounting| accounting.trial() == candidate)
            else {
                return Err(ProofRefusal::CandidateNotInCensus);
            };
            let Some(executed) = entry.disposition().report() else {
                return Err(ProofRefusal::CandidateNotSelected);
            };
            match executed.attempt() {
                RunAttempt::Executed(TrialConclusion::Refused(finding)) => (
                    executed.clone(),
                    DemonstratedRejection::demonstrated(candidate, finding.clone()),
                ),
                RunAttempt::Executed(TrialConclusion::Passed) => {
                    return Err(ProofRefusal::CandidateDidNotRefuse);
                }
                RunAttempt::SkippedWithReason(_)
                | RunAttempt::TimedOut
                | RunAttempt::InfrastructureFailed(_) => {
                    return Err(ProofRefusal::CandidateDidNotExecute);
                }
            }
        };
        Ok(Self {
            report,
            trial_report,
            rejection,
        })
    }

    /// The report the staged run wrote.
    #[must_use]
    pub const fn report(&self) -> &RunReport {
        &self.report
    }

    /// The candidate trial report the rejection was read from.
    #[must_use]
    pub const fn trial_report(&self) -> &TrialReport {
        &self.trial_report
    }

    /// The rejection read out of it.
    #[must_use]
    pub const fn rejection(&self) -> &DemonstratedRejection {
        &self.rejection
    }
}

impl ProofDelta {
    /// How much proof one candidate added to the claim it pins.
    ///
    /// # Errors
    ///
    /// Refuses a pair that does not move, because a candidate that leaves the exercised count where it was pins nothing.
    pub const fn between(before: usize, after: usize) -> Result<Self, ProofDeltaRefusal> {
        if after <= before {
            return Err(ProofDeltaRefusal::NoProofAdded { before, after });
        }
        Ok(Self { before, after })
    }

    /// The exercised count before the candidate ran.
    #[must_use]
    pub const fn before(self) -> usize {
        self.before
    }

    /// The exercised count after it ran.
    #[must_use]
    pub const fn after(self) -> usize {
        self.after
    }
}

impl FailureComparison {
    /// The comparison a failure-bearing ground offers.
    ///
    /// # Errors
    ///
    /// Refuses a candidate whose fingerprint the known roster already carries.
    pub fn compared(
        candidate: Fingerprint,
        known: Vec<Fingerprint>,
    ) -> Result<Self, DuplicateRefusal> {
        if known.contains(&candidate) {
            return Err(DuplicateRefusal::FingerprintAlreadyKnown(candidate));
        }
        Ok(Self { candidate, known })
    }

    /// The fingerprint this candidate carries.
    #[must_use]
    pub const fn candidate(&self) -> Fingerprint {
        self.candidate
    }

    /// The fingerprints already known, in the order they were compared.
    pub fn known(&self) -> impl Iterator<Item = &Fingerprint> {
        self.known.iter()
    }
}

impl ObligationComparison {
    /// The comparison a discharge ground offers.
    ///
    /// # Errors
    ///
    /// Refuses an owed claim that already carries a discharge, naming the first one recorded for it.
    pub fn compared(owed: ClaimRef, discharges: &[TrialId]) -> Result<Self, DuplicateRefusal> {
        if let Some(first) = discharges.first() {
            return Err(DuplicateRefusal::ObligationAlreadyDischarged(*first));
        }
        Ok(Self { owed })
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(&self) -> ClaimRef {
        self.owed
    }
}

impl NoComparison {
    /// The statement a ground with no comparable subject makes.
    #[must_use]
    pub const fn stated(reason: NoComparisonReason) -> Self {
        Self { reason }
    }

    /// Why nothing was compared.
    #[must_use]
    pub const fn reason(self) -> NoComparisonReason {
        self.reason
    }
}

// ---------------------------------------------------------------------------
// The three proposal grounds.
// ---------------------------------------------------------------------------

impl MutantKilledGround {
    /// The ground a demonstrated kill stands on.
    #[must_use]
    pub(in crate::muterprater) const fn shown(
        target: MutationTarget,
        activation: ActivationDisposition,
        capsule: ReplayCapsule,
        demonstration: Demonstration,
    ) -> Self {
        Self {
            target,
            activation,
            capsule,
            demonstration,
        }
    }

    /// What was damaged.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// What the damage's activation was.
    #[must_use]
    pub const fn activation(&self) -> ActivationDisposition {
        self.activation
    }

    /// The reproduction account of the demonstrating run.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }

    /// The demonstrated kill.
    #[must_use]
    pub const fn demonstration(&self) -> &Demonstration {
        &self.demonstration
    }
}

impl ClaimPinnedGround {
    /// The ground a pin stands on.
    #[must_use]
    pub const fn moved(claim: ClaimRef, capsule: ReplayCapsule, delta: ProofDelta) -> Self {
        Self {
            claim,
            capsule,
            delta,
        }
    }

    /// The claim pinned.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The reproduction account of the pinning run.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }

    /// What the pin added to the claim's proof.
    #[must_use]
    pub const fn delta(&self) -> ProofDelta {
        self.delta
    }
}

impl ObligationDischargedGround {
    /// The ground a discharge stands on.
    #[must_use]
    pub const fn discharged(owed: OwedClaim, discharge: DischargeEvidence) -> Self {
        Self { owed, discharge }
    }

    /// The owed claim's identity.
    #[must_use]
    pub const fn owed(&self) -> &OwedClaim {
        &self.owed
    }

    /// What discharged it.
    #[must_use]
    pub const fn discharge(&self) -> &DischargeEvidence {
        &self.discharge
    }
}

// ---------------------------------------------------------------------------
// The proposals, and their one identity road.
// ---------------------------------------------------------------------------

impl ProposalDestination {
    /// Where an admitted row would land.
    #[must_use]
    pub const fn naming(suite: ExecutionSuite) -> Self {
        Self { suite }
    }

    /// The aggregate seat it would land in.
    #[must_use]
    pub const fn suite(self) -> ExecutionSuite {
        self.suite
    }

    /// The semantic owner: the suite's own namespace.
    #[must_use]
    pub const fn owner(self) -> Namespace {
        self.suite.name().namespace()
    }
}

impl MutantKilledProposal {
    /// One proposal on the mutant-killed ground, offered.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a row that does not carry the candidate origin arm, and a survivor synthesis fact naming a different point than the ground's target names.
    pub(in crate::muterprater) fn offered(
        candidate: Row,
        ground: MutantKilledGround,
        duplicate: FailureComparison,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        let facts = candidate_facts(&candidate)?;
        survivor_point_agrees(facts, ground.target())?;
        Ok(Self {
            candidate,
            ground,
            duplicate,
            destination,
        })
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &MutantKilledGround {
        &self.ground
    }

    /// The evidence it is not a duplicate.
    #[must_use]
    pub const fn duplicate(&self) -> &FailureComparison {
        &self.duplicate
    }
}

impl ClaimPinnedProposal {
    /// One proposal on the claim-pinned ground, offered.
    ///
    /// The comparison seat states its own vacancy: a pin carries no failure to fingerprint, so [`NoComparisonReason::GroundCarriesNoFailure`] is the whole of what there is to compare.
    ///
    /// # Errors
    ///
    /// Refuses a row that does not carry the candidate origin arm.
    pub(in crate::muterprater) fn offered(
        candidate: Row,
        ground: ClaimPinnedGround,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        candidate_facts(&candidate)?;
        Ok(Self {
            candidate,
            ground,
            duplicate: NoComparison::stated(NoComparisonReason::GroundCarriesNoFailure),
            destination,
        })
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &ClaimPinnedGround {
        &self.ground
    }

    /// The stated reason nothing was compared.
    #[must_use]
    pub const fn duplicate(&self) -> NoComparison {
        self.duplicate
    }
}

impl ObligationDischargedProposal {
    /// One proposal on the obligation-discharged ground, offered.
    ///
    /// # Errors
    ///
    /// Refuses a row that does not carry the candidate origin arm.
    pub(in crate::muterprater) fn offered(
        candidate: Row,
        ground: ObligationDischargedGround,
        duplicate: ObligationComparison,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        candidate_facts(&candidate)?;
        Ok(Self {
            candidate,
            ground,
            duplicate,
            destination,
        })
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &ObligationDischargedGround {
        &self.ground
    }

    /// The evidence it is not a duplicate.
    #[must_use]
    pub const fn duplicate(&self) -> &ObligationComparison {
        &self.duplicate
    }
}

impl ProposalDocument for MutantKilledProposal {
    fn candidate(&self) -> &Row {
        &self.candidate
    }

    fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    fn destination(&self) -> ProposalDestination {
        self.destination
    }

    fn identity(&self) -> ProposalId {
        proposal_identity(&self.candidate, self.ground_summary(), self.destination)
    }
}

impl ProposalDocument for ClaimPinnedProposal {
    fn candidate(&self) -> &Row {
        &self.candidate
    }

    fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    fn destination(&self) -> ProposalDestination {
        self.destination
    }

    fn identity(&self) -> ProposalId {
        proposal_identity(&self.candidate, self.ground_summary(), self.destination)
    }
}

impl ProposalDocument for ObligationDischargedProposal {
    fn candidate(&self) -> &Row {
        &self.candidate
    }

    fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    fn destination(&self) -> ProposalDestination {
        self.destination
    }

    fn identity(&self) -> ProposalId {
        proposal_identity(&self.candidate, self.ground_summary(), self.destination)
    }
}

impl ReplayBearingProposal for MutantKilledProposal {
    fn replay_capsule(&self) -> &ReplayCapsule {
        self.ground.capsule()
    }

    fn replay_ground(&self) -> ReplayBearingGround {
        ReplayBearingGround::MutantKilled
    }
}

impl ReplayBearingProposal for ClaimPinnedProposal {
    fn replay_capsule(&self) -> &ReplayCapsule {
        self.ground.capsule()
    }

    fn replay_ground(&self) -> ReplayBearingGround {
        ReplayBearingGround::ClaimPinned
    }
}

/// The one road every proposal's identity is derived by, over the three readings the three of them share.
///
/// The candidate row's canonical bytes were written where that row was born, so this reads them rather than encoding a row a second time.
/// Written once rather than per proposal: three copies of one preimage agree until one is edited, and the specification is stated on [`ProposalDocument::identity`].
fn proposal_identity(
    candidate: &Row,
    ground: AdmissionGround,
    destination: ProposalDestination,
) -> ProposalId {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(&PROPOSAL_ENCODING_VERSION.to_be_bytes());
    encode_bytes(candidate.canonical_bytes().as_bytes(), &mut preimage);
    preimage.push(ground.slot());
    destination.suite().name().encode_into(&mut preimage);
    ProposalId::over(ContentAddress::derived(PROPOSAL_TAG, &preimage))
}

/// The synthesis facts a candidate row carries, or the refusal it earns.
fn candidate_facts(candidate: &Row) -> Result<SynthesisFacts, ProposalRefusal> {
    match candidate.origin() {
        Origin::Candidate(facts) => Ok(facts),
        Origin::HandWritten
        | Origin::Generated(_)
        | Origin::AdmittedReplay(_)
        | Origin::AdmittedDischarge(_) => Err(ProposalRefusal::NotACandidate),
    }
}

/// Whether the row's survivor point and the ground's target name one point.
///
/// The check is possible only where both name a point: an external target names a coordinate, and a proof-gap synthesis names no point at all.
/// It takes the target rather than a ground, because the target is the whole of what it reads and only one ground has one.
fn survivor_point_agrees(
    facts: SynthesisFacts,
    target: &MutationTarget,
) -> Result<(), ProposalRefusal> {
    let SynthesisFacts::Survivor(synthesis) = facts else {
        return Ok(());
    };
    let Some(point) = target.identity().point() else {
        return Ok(());
    };
    if synthesis == point {
        return Ok(());
    }
    Err(ProposalRefusal::SurvivorPointMismatch {
        synthesis,
        target: point,
    })
}

// ---------------------------------------------------------------------------
// Custody, and the admission receipts.
// ---------------------------------------------------------------------------

impl StoredProposalRef {
    /// Bind a sink's storage location to the proposal it stored.
    ///
    /// # Errors
    ///
    /// Refuses an empty token, which names nowhere.
    pub fn at(proposal: ProposalId, token: &str) -> Result<Self, SinkRefusal> {
        if token.is_empty() {
            return Err(SinkRefusal::EmptyLocation);
        }
        Ok(Self {
            proposal,
            token: token.to_owned(),
        })
    }

    /// The content identity of the proposal stored at this location.
    #[must_use]
    pub const fn proposal(&self) -> ProposalId {
        self.proposal
    }

    /// The token, for a sink to read its own location back.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl ReplayAdmissionReceipt {
    /// Retain the exact outputs of one completed replay-bearing human admission.
    #[must_use]
    pub(in crate::muterprater) fn completed(
        row: Row,
        entry: ReplayCapsuleEntry,
        proposal_custody: StoredProposalRef,
        replay_custody: StoredReplayEntryRef,
    ) -> Self {
        Self {
            row,
            entry,
            proposal_custody,
            replay_custody,
        }
    }

    /// The row whose candidate origin became human-admitted provenance.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The exact capsule entry the human admission stored.
    #[must_use]
    pub const fn entry(&self) -> &ReplayCapsuleEntry {
        &self.entry
    }

    /// The caller's review-durable custody of the proposal.
    #[must_use]
    pub const fn proposal_custody(&self) -> &StoredProposalRef {
        &self.proposal_custody
    }

    /// The caller's storage location for the replay entry.
    #[must_use]
    pub const fn replay_custody(&self) -> &StoredReplayEntryRef {
        &self.replay_custody
    }
}

impl DischargeAdmissionReceipt {
    /// Retain the outputs of one completed obligation-discharge human admission.
    #[must_use]
    pub(in crate::muterprater) fn completed(row: Row, proposal_custody: StoredProposalRef) -> Self {
        Self {
            row,
            proposal_custody,
        }
    }

    /// The row whose candidate origin became human-admitted provenance.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The caller's review-durable custody of the proposal.
    #[must_use]
    pub const fn proposal_custody(&self) -> &StoredProposalRef {
        &self.proposal_custody
    }
}

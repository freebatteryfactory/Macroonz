//! Shared outside-test construction for mutation claim modules.

#[path = "specimen_support.rs"]
mod specimen_support;

pub(super) use specimen_support::{
    CACHED_SIBLING_OBSERVATION_HOST, COMPILED_SPECIMEN_HOST, SPECIMEN_HOST_CALLS,
    SPECIMEN_MATERIALIZER, SPECIMEN_MATERIALIZER_CALLS, UNCHANGED_SPECIMEN_MATERIALIZER,
    WRONG_SELECTED_SPECIMEN, clear_cached_sibling_observation, lock_specimen_tests,
    omitted_baseline_branch, omitted_specimen_branch, specimen_source,
};

use macroonz_harness::clock::{HarnessClock, MeasurementReading};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite,
    MutationPointRef, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute,
    Tag, TrialTableRefusal,
};
use macroonz_harness::generate::{ReductionPlanRefusal, ReductionProbeRefusal, ReductionRefusal};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::interpret::{
    availability, execute_active, observe_no_mutation, qualify_no_mutation,
};
use macroonz_harness::muterprater::rewrite::admission;
use macroonz_harness::muterprater::specimen::demonstrate_compiled_projection;
use macroonz_harness::muterprater::wrap::{read_artifact, read_output};
use macroonz_harness::muterprater::{
    ARTIFACT_CONTENT_TAG, ActivationEvidence, ActivationSite, ActiveSelection,
    AdapterQualification, AdmittedAlternative, AlternativeDeclaration, AnnouncedRoster,
    ArtifactCustodyRefusal, ArtifactManifestRefusal, BackendCommand, BackendVersion,
    BackendVersionPosture, CompiledProjectionPressure, CompiledProjectionRefusal,
    CompiledSuiteArtifactCustody, CompiledSuiteArtifactManifest, CompiledSuiteArtifactStanding,
    CompiledSuitePressure, DischargeProposalRefusal, DiscoveredMutationSite,
    DiscoveryLoweringRefusal, DiscoveryRefusal, EvaluationBinding, EvaluationCall,
    EvaluationCallRefusal, EvaluationDirective, EvaluationFamilyRef, EvaluationObservation,
    EvaluationPair, EvaluationPairRefusal, EvaluationSurface, FamilyAttribution, GrammarStanding,
    HumanAdmissionRefusal, IntendedRejection, InterpretedExecutionRefusal, InterpretedTrust,
    InterpreterAvailability, KillProposalRefusal, MissingTrustEvidence, MutationBackendInvocation,
    MutationIdentity, MutationOutcome, MutationPermission, MutationPolicy, MutationReport,
    MutationSite, MutationSourceRevision, MutationVerdict, MutationWitness, MutationWitnessRefusal,
    NoMutationObservationRefusal, NoMutationParityQualification, NoMutationParityReading,
    NoMutationParityStanding, OperatorFamilyRef, OwedClaimRefusal, OwnerClaimMapping,
    ParityQualificationRefusal, PermissionRefusal, PlanRefusal, PolicyRefusal, ProductionBinding,
    ProofDeltaRefusal, ProofRefusal, ProposalRefusal, QualificationRefusal, RewriteAdmission,
    SinkRefusal, SourceCoordinate, SpecimenMaterializerBinding, SuitePressureRefusal, WrapReading,
    WrapRefusal, WrappedBackend,
};
use macroonz_harness::properties::{Agreement, agreement};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, CoverageRefusal, FindingCause, InvocationProfile, RunAttempt,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialId,
    TrialReport, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, lens_verdict};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
pub(super) const OWNER: &str = "harness.mutation.receiver";
pub(super) const BACKEND_CONSOLE: &str =
    include_str!("current-compiled-pressure-artifact/cargo-mutants-27.0.0-console.txt");
pub(super) const CURRENT_BACKEND_SOURCE: &[u8] =
    include_bytes!("../../src/muterprater/backend/wrap/parse.rs");
/// The harness-derived revision identity of the wrapped-backend source the current campaign ran against.
///
/// The `0.2.0` release receipt under `.durafx` records that source's Git blob, hash, and reconstruction road.
pub(super) const CAMPAIGN_BACKEND_REVISION: [u8; 32] = [
    65, 174, 110, 41, 237, 73, 162, 0, 130, 114, 221, 57, 38, 66, 223, 87, 93, 4, 182, 8, 198, 250,
    248, 216, 89, 93, 186, 52, 112, 98, 166, 170,
];
pub(super) const HISTORICAL_BACKEND_CONSOLE: &str =
    include_str!("compiled-pressure-artifact/cargo-mutants-27.0.0-console.txt");
pub(super) const BACKEND_NO_KILL: &str = "Found 1 mutant to test\n\
    ok Unmutated baseline in 3.1s\n\
    missed src/subject/lane.rs:41:9: replace is_qualified -> bool with true in 4.0s";
pub(super) const BACKEND_VERSION: &str = "27.0.0";
pub(super) const BACKEND_TARGET: &str = "x86_64-pc-windows-msvc";
pub(super) const BACKEND_TOOLCHAIN: &str = "rustc 1.98.0 (88d9e12ae 2026-08-18)";
pub(super) const BACKEND_COMMAND: &[&str] = &[
    "mutants",
    "--package",
    "macroonz-harness",
    "--file",
    "harness/src/muterprater/backend/wrap/parse.rs",
    "--re",
    "replace != with == in roster_count",
    "--test-tool",
    "nextest",
    "--no-shuffle",
    "--jobs",
    "1",
    "--caught",
    "--no-times",
];
pub(super) const HISTORICAL_BACKEND_COMMAND: &[&str] = &[
    "mutants",
    "--package",
    "macroonz-harness",
    "--file",
    "harness/src/muterprater/wrap.rs",
    "--re",
    "replace != with == in roster_count",
    "--test-tool",
    "nextest",
    "--no-shuffle",
    "--jobs",
    "1",
    "--caught",
    "--no-times",
];
pub(super) const COMPILED_MUTANT_FILE: &str = "harness/src/muterprater/backend/wrap/parse.rs";
pub(super) const HISTORICAL_COMPILED_MUTANT_FILE: &str = "harness/src/muterprater/wrap.rs";
pub(super) const COMPILED_MUTANT_DAMAGE: &[u8] = b"replace != with == in roster_count";
pub(super) const ORIGINAL_OPERATION: &[u8] = b"input != 0";
pub(super) const SELECTED_OPERATION: &[u8] = b"input == 0";
pub(super) const MEANING_DISAGREEMENT: FindingCause =
    FindingCause::named(OWNER, "meaning-disagreement");
pub(super) const REVISION_TAG: DomainTag = DomainTag::declared(
    "mutation-receiver-revision",
    IdentityProfileVersion::declared(1),
);
pub(super) const REPLAY_SCHEMA_TAG: DomainTag = DomainTag::declared(
    "mutation-receiver-replay-schema",
    IdentityProfileVersion::declared(1),
);
pub(super) static CLAIM_MISMATCH_EVALUATION_CALLS: AtomicU32 = AtomicU32::new(0);
pub(super) static INTERPRETED_CLOCK_CALLS: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, PartialEq, Eq)]
pub(super) enum MutationRoadFailure {
    Name,
    Permission(PermissionRefusal),
    Policy(PolicyRefusal),
    Discovery(DiscoveryRefusal),
    DiscoveryLowering(DiscoveryLoweringRefusal),
    Pair(EvaluationPairRefusal),
    Table(TrialTableRefusal),
    Wrap(WrapRefusal),
    ArtifactManifest(ArtifactManifestRefusal),
    ArtifactCustody(ArtifactCustodyRefusal),
    Qualification(QualificationRefusal),
    Pressure(SuitePressureRefusal),
    Projection(CompiledProjectionRefusal),
    Witness(MutationWitnessRefusal),
    Observation(NoMutationObservationRefusal),
    Plan(PlanRefusal),
    Interpreted(InterpretedFailureStage),
    MissingFamily,
    NativeToolchain,
    MissingAlternative,
    MissingActiveSelection,
    MissingQualification(ParityQualificationRefusal),
    MissingTrust(MissingTrustEvidence),
    CampaignSourceMoved,
    Proof(ProofRefusal),
    ReductionPlan(ReductionPlanRefusal),
    ReductionProbe(ReductionProbeRefusal),
    Reduction(ReductionRefusal),
    Coverage(CoverageRefusal),
    Proposal(KillProposalRefusal),
    PinProposal(ProposalRefusal),
    DischargeProposal(DischargeProposalRefusal),
    Delta(ProofDeltaRefusal),
    Owed(OwedClaimRefusal),
    ProposalSink(SinkRefusal),
    Admission(HumanAdmissionRefusal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InterpretedFailureStage {
    Invocation,
    Selection,
    WitnessClaim,
    EvaluationCall,
    DudPlant,
    Report,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CompiledRosterMeaning {
    Stated(u32),
    Unstated,
    SetupRefused,
    ReadingRefused(WrapRefusal),
}

impl From<PermissionRefusal> for MutationRoadFailure {
    fn from(refusal: PermissionRefusal) -> Self {
        Self::Permission(refusal)
    }
}

impl From<PolicyRefusal> for MutationRoadFailure {
    fn from(refusal: PolicyRefusal) -> Self {
        Self::Policy(refusal)
    }
}

impl From<DiscoveryRefusal> for MutationRoadFailure {
    fn from(refusal: DiscoveryRefusal) -> Self {
        Self::Discovery(refusal)
    }
}

impl From<DiscoveryLoweringRefusal> for MutationRoadFailure {
    fn from(refusal: DiscoveryLoweringRefusal) -> Self {
        Self::DiscoveryLowering(refusal)
    }
}

impl From<EvaluationPairRefusal> for MutationRoadFailure {
    fn from(refusal: EvaluationPairRefusal) -> Self {
        Self::Pair(refusal)
    }
}

impl From<TrialTableRefusal> for MutationRoadFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

impl From<WrapRefusal> for MutationRoadFailure {
    fn from(refusal: WrapRefusal) -> Self {
        Self::Wrap(refusal)
    }
}

impl From<ArtifactManifestRefusal> for MutationRoadFailure {
    fn from(refusal: ArtifactManifestRefusal) -> Self {
        Self::ArtifactManifest(refusal)
    }
}

impl From<ArtifactCustodyRefusal> for MutationRoadFailure {
    fn from(refusal: ArtifactCustodyRefusal) -> Self {
        Self::ArtifactCustody(refusal)
    }
}

impl From<QualificationRefusal> for MutationRoadFailure {
    fn from(refusal: QualificationRefusal) -> Self {
        Self::Qualification(refusal)
    }
}

impl From<SuitePressureRefusal> for MutationRoadFailure {
    fn from(refusal: SuitePressureRefusal) -> Self {
        Self::Pressure(refusal)
    }
}

impl From<CompiledProjectionRefusal> for MutationRoadFailure {
    fn from(refusal: CompiledProjectionRefusal) -> Self {
        Self::Projection(refusal)
    }
}

impl From<MutationWitnessRefusal> for MutationRoadFailure {
    fn from(refusal: MutationWitnessRefusal) -> Self {
        Self::Witness(refusal)
    }
}

impl From<NoMutationObservationRefusal> for MutationRoadFailure {
    fn from(refusal: NoMutationObservationRefusal) -> Self {
        Self::Observation(refusal)
    }
}

impl From<PlanRefusal> for MutationRoadFailure {
    fn from(refusal: PlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

impl From<InterpretedExecutionRefusal> for MutationRoadFailure {
    fn from(refusal: InterpretedExecutionRefusal) -> Self {
        let stage = match refusal {
            InterpretedExecutionRefusal::InvocationForAnotherExecution => {
                InterpretedFailureStage::Invocation
            }
            InterpretedExecutionRefusal::Selection(_) => InterpretedFailureStage::Selection,
            InterpretedExecutionRefusal::WitnessForAnotherClaim { .. } => {
                InterpretedFailureStage::WitnessClaim
            }
            InterpretedExecutionRefusal::EvaluationCall(_) => {
                InterpretedFailureStage::EvaluationCall
            }
            InterpretedExecutionRefusal::DudPlant(_) => InterpretedFailureStage::DudPlant,
            InterpretedExecutionRefusal::Report(_) => InterpretedFailureStage::Report,
        };
        Self::Interpreted(stage)
    }
}

impl From<ProofRefusal> for MutationRoadFailure {
    fn from(refusal: ProofRefusal) -> Self {
        Self::Proof(refusal)
    }
}

impl From<ReductionPlanRefusal> for MutationRoadFailure {
    fn from(refusal: ReductionPlanRefusal) -> Self {
        Self::ReductionPlan(refusal)
    }
}

impl From<ReductionProbeRefusal> for MutationRoadFailure {
    fn from(refusal: ReductionProbeRefusal) -> Self {
        Self::ReductionProbe(refusal)
    }
}

impl From<ReductionRefusal> for MutationRoadFailure {
    fn from(refusal: ReductionRefusal) -> Self {
        Self::Reduction(refusal)
    }
}

impl From<CoverageRefusal> for MutationRoadFailure {
    fn from(refusal: CoverageRefusal) -> Self {
        Self::Coverage(refusal)
    }
}

impl From<KillProposalRefusal> for MutationRoadFailure {
    fn from(refusal: KillProposalRefusal) -> Self {
        Self::Proposal(refusal)
    }
}

impl From<HumanAdmissionRefusal> for MutationRoadFailure {
    fn from(refusal: HumanAdmissionRefusal) -> Self {
        Self::Admission(refusal)
    }
}

pub(super) fn family(stem: &'static str) -> Result<EvaluationFamilyRef, MutationRoadFailure> {
    EvaluationFamilyRef::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)
}

pub(super) fn claim() -> Result<ClaimRef, MutationRoadFailure> {
    ClaimRef::named(OWNER, "comparison-behaviour").map_err(|_| MutationRoadFailure::Name)
}

pub(super) fn operator() -> Result<OperatorFamilyRef, MutationRoadFailure> {
    OperatorFamilyRef::of_slug("comparison-boundaries").ok_or(MutationRoadFailure::MissingFamily)
}

pub(super) fn policy(family: EvaluationFamilyRef) -> Result<MutationPolicy, MutationRoadFailure> {
    Ok(MutationPolicy::declared(
        family,
        vec![MutationPermission::declared(claim()?, vec![operator()?])?],
    )?)
}

pub(super) fn discovered_point(
    stem: &'static str,
    mapping: OwnerClaimMapping,
    alternatives: Vec<&'static [u8]>,
) -> Result<DiscoveredMutationSite, MutationRoadFailure> {
    let admitted_family = operator()?;
    let declarations = alternatives
        .into_iter()
        .map(|operation| AlternativeDeclaration::stated(admitted_family, operation.to_vec()))
        .collect();
    Ok(DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)?,
        mapping,
        ORIGINAL_OPERATION.to_vec(),
        declarations,
        ActivationSite::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)?,
    )?)
}

pub(super) fn surface_with(
    family: EvaluationFamilyRef,
    alternatives: Vec<&'static [u8]>,
) -> Result<EvaluationSurface, MutationRoadFailure> {
    let policy = policy(family)?;
    let discovered = discovered_point(
        "comparison-edge",
        OwnerClaimMapping::Mapped(claim()?),
        alternatives,
    )?;
    Ok(lower_discoveries(&policy, vec![discovered])?.into_parts().1)
}

pub(super) fn production(_input: &[u32; 3]) -> CompiledRosterMeaning {
    match compiled_reading() {
        Ok(reading) => match reading.announced() {
            AnnouncedRoster::Stated(count) => CompiledRosterMeaning::Stated(count),
            AnnouncedRoster::Unstated => CompiledRosterMeaning::Unstated,
        },
        Err(MutationRoadFailure::Wrap(refusal)) => CompiledRosterMeaning::ReadingRefused(refusal),
        Err(_) => CompiledRosterMeaning::SetupRefused,
    }
}

/// The shape every evaluation callable below inhabits: the contract's own, whose refusing side belongs to the fixtures that refuse.
///
/// The lawful and hostile fixtures that never refuse are `const` closures over this shape rather than `fn` items, so their always-passing bodies carry no fallibility of their own.
pub(super) type EvaluationFn =
    fn(
        &[u32; 3],
        EvaluationDirective<'_>,
    ) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal>;

/// This capture-free fixture's lawful branches both return observations.
pub(super) const EVALUATION: EvaluationFn = |input, directive| {
    Ok(if directive.resolved().is_some() {
        EvaluationObservation::observed(CompiledRosterMeaning::Unstated, 1)
    } else {
        EvaluationObservation::observed(production(input), 0)
    })
};

pub(super) fn evaluation_reads_resolved_payload(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    let Some(resolved) = directive.resolved() else {
        return Ok(EvaluationObservation::observed(production(input), 0));
    };
    if resolved.point().original_operation() != ORIGINAL_OPERATION
        || resolved.alternative().operation() != SELECTED_OPERATION
    {
        return Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        ));
    }
    Ok(EvaluationObservation::observed(
        CompiledRosterMeaning::Unstated,
        1,
    ))
}

pub(super) fn evaluation_reads_resolved_payload_counted(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    CLAIM_MISMATCH_EVALUATION_CALLS.fetch_add(1, Ordering::SeqCst);
    evaluation_reads_resolved_payload(input, directive)
}

pub(super) fn evaluation_counted(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    CLAIM_MISMATCH_EVALUATION_CALLS.fetch_add(1, Ordering::SeqCst);
    EVALUATION(input, directive)
}

pub(super) fn same(left: &CompiledRosterMeaning, right: &CompiledRosterMeaning) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

pub(super) fn check(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &CompiledRosterMeaning::Stated(1),
        MEANING_DISAGREEMENT,
    )
}

pub(super) fn unused_trial_call(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

pub(super) fn trial_binding_for(
    claim_stem: &'static str,
) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, "comparison-subject")?;
    let check_ref = CheckRef::named(OWNER, "comparison-check")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, claim_stem)?,
        ExecutionSuite::named(OWNER, "mutation-receiver")?,
        Classification::authored(
            vec![Role::named(OWNER, "mutation")?],
            vec![Tag::named(OWNER, "outside-consumer")?],
        )?,
        subject,
        check_ref,
        PopulationRef::named(OWNER, "one-input")?,
        Origin::HandWritten,
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check_ref, revision, revision, unused_trial_call),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

pub(super) fn trial_binding() -> Result<TrialBinding, TrialTableRefusal> {
    trial_binding_for("comparison-behaviour")
}

pub(super) fn check_ref() -> Result<CheckRef, MutationRoadFailure> {
    CheckRef::named(OWNER, "comparison-check").map_err(|_| MutationRoadFailure::Name)
}

pub(super) fn invocation() -> Result<Invocation, MutationRoadFailure> {
    let declared_toolchain = "1.98.1";
    let version = Command::new("rustup")
        .arg("run")
        .arg(declared_toolchain)
        .arg("rustc")
        .arg("-vV")
        .output()
        .map_err(|_| MutationRoadFailure::NativeToolchain)?;
    if !version.status.success() {
        return Err(MutationRoadFailure::NativeToolchain);
    }
    let output =
        std::str::from_utf8(&version.stdout).map_err(|_| MutationRoadFailure::NativeToolchain)?;
    let native_target = output
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or(MutationRoadFailure::NativeToolchain)?;
    Ok(Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared(native_target),
            ToolchainIdentity::declared(declared_toolchain),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "mutation-receiver"),
        HarnessClock::unavailable(),
    ))
}

pub(super) fn foreign_invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("wasm32-unknown-unknown"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(
            module_path!(),
            file!(),
            line!(),
            "foreign-mutation-receiver",
        ),
        HarnessClock::unavailable(),
    )
}

pub(super) fn counted_tick() -> u64 {
    u64::from(INTERPRETED_CLOCK_CALLS.fetch_add(1, Ordering::SeqCst))
}

pub(super) fn foreign_measured_invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("wasm32-unknown-unknown"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(
            module_path!(),
            file!(),
            line!(),
            "foreign-measured-mutation-receiver",
        ),
        HarnessClock::reading(counted_tick),
    )
}

pub(super) fn pair(
    family: EvaluationFamilyRef,
    surface: &EvaluationSurface,
    evaluated: EvaluationCall<[u32; 3], CompiledRosterMeaning>,
) -> Result<EvaluationPair<[u32; 3], CompiledRosterMeaning>, MutationRoadFailure> {
    pair_with_evaluation_revision(family, surface, evaluated, b"evaluation")
}

pub(super) fn pair_with_evaluation_revision(
    family: EvaluationFamilyRef,
    surface: &EvaluationSurface,
    evaluated: EvaluationCall<[u32; 3], CompiledRosterMeaning>,
    evaluation_revision_bytes: &[u8],
) -> Result<EvaluationPair<[u32; 3], CompiledRosterMeaning>, MutationRoadFailure> {
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        evaluation_revision_bytes,
    ));
    Ok(EvaluationPair::paired(
        ProductionBinding::declared(family, production_revision, production),
        EvaluationBinding::declared(surface, evaluation_revision, evaluated),
        same,
    )?)
}

pub(super) fn compiled_owner(coordinate: &SourceCoordinate) -> Option<ClaimRef> {
    (coordinate.file() == COMPILED_MUTANT_FILE)
        .then(claim)
        .and_then(Result::ok)
}

pub(super) fn historical_compiled_owner(coordinate: &SourceCoordinate) -> Option<ClaimRef> {
    (coordinate.file() == HISTORICAL_COMPILED_MUTANT_FILE)
        .then(claim)
        .and_then(Result::ok)
}

pub(super) fn compiled_family(
    coordinate: &SourceCoordinate,
    damage: &[u8],
) -> Option<OperatorFamilyRef> {
    (coordinate.file() == COMPILED_MUTANT_FILE && damage == COMPILED_MUTANT_DAMAGE)
        .then(operator)
        .and_then(Result::ok)
}

pub(super) fn historical_compiled_family(
    coordinate: &SourceCoordinate,
    damage: &[u8],
) -> Option<OperatorFamilyRef> {
    (coordinate.file() == HISTORICAL_COMPILED_MUTANT_FILE && damage == COMPILED_MUTANT_DAMAGE)
        .then(operator)
        .and_then(Result::ok)
}

pub(super) fn compiled_reading() -> Result<WrapReading, MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    Ok(read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(version),
        compiled_owner,
        compiled_family,
    )?)
}

pub(super) fn backend_invocation(
    version: BackendVersion,
) -> Result<MutationBackendInvocation, MutationRoadFailure> {
    let command = BackendCommand::declared("cargo", BACKEND_COMMAND)
        .map_err(|_| MutationRoadFailure::Name)?;
    Ok(MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        version,
        command,
        TargetBinding::bound(
            TargetTriple::declared(BACKEND_TARGET),
            ToolchainIdentity::declared(BACKEND_TOOLCHAIN),
        ),
    ))
}

pub(super) fn historical_backend_invocation(
    version: BackendVersion,
) -> Result<MutationBackendInvocation, MutationRoadFailure> {
    let command = BackendCommand::declared("cargo", HISTORICAL_BACKEND_COMMAND)
        .map_err(|_| MutationRoadFailure::Name)?;
    Ok(MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        version,
        command,
        TargetBinding::bound(
            TargetTriple::declared(BACKEND_TARGET),
            ToolchainIdentity::declared(BACKEND_TOOLCHAIN),
        ),
    ))
}

pub(super) fn source_revision(bytes: &[u8]) -> Result<MutationSourceRevision, MutationRoadFailure> {
    MutationSourceRevision::from_content(COMPILED_MUTANT_FILE, bytes)
        .map_err(|_| MutationRoadFailure::Name)
}

pub(super) fn historical_source_revision(
    bytes: &[u8],
) -> Result<MutationSourceRevision, MutationRoadFailure> {
    MutationSourceRevision::from_content(HISTORICAL_COMPILED_MUTANT_FILE, bytes)
        .map_err(|_| MutationRoadFailure::Name)
}

pub(super) fn compiled_artifact(
    console: &str,
    version: BackendVersion,
    artifact_source: &[u8],
) -> Result<CompiledSuiteArtifactManifest, MutationRoadFailure> {
    Ok(read_artifact(
        console,
        backend_invocation(version)?,
        vec![source_revision(artifact_source)?],
        compiled_owner,
        compiled_family,
    )?)
}

pub(super) fn historical_compiled_artifact(
    console: &str,
    version: BackendVersion,
    artifact_source: &[u8],
) -> Result<CompiledSuiteArtifactManifest, MutationRoadFailure> {
    Ok(read_artifact(
        console,
        historical_backend_invocation(version)?,
        vec![historical_source_revision(artifact_source)?],
        historical_compiled_owner,
        historical_compiled_family,
    )?)
}

pub(super) fn current_custody(
    manifest: CompiledSuiteArtifactManifest,
    current_source: &[u8],
) -> Result<CompiledSuiteArtifactCustody, MutationRoadFailure> {
    Ok(CompiledSuiteArtifactCustody::current(
        manifest,
        vec![source_revision(current_source)?],
    )?)
}

/// The current wrapped-backend source, admitted only while it is the exact source the retained campaign ran against.
pub(super) fn campaign_source() -> Result<MutationSourceRevision, MutationRoadFailure> {
    let source = source_revision(CURRENT_BACKEND_SOURCE)?;
    if source.revision().address().as_bytes() != &CAMPAIGN_BACKEND_REVISION {
        return Err(MutationRoadFailure::CampaignSourceMoved);
    }
    Ok(source)
}

pub(super) fn compiled_suite_pressure() -> Result<CompiledSuitePressure, MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    campaign_source()?;
    let manifest = compiled_artifact(BACKEND_CONSOLE, version.clone(), CURRENT_BACKEND_SOURCE)?;
    let qualification =
        AdapterQualification::of(manifest.reading(), GrammarStanding::Checked(version))?;
    let custody = current_custody(manifest, CURRENT_BACKEND_SOURCE)?;
    Ok(CompiledSuitePressure::demonstrated(
        CompiledSuiteArtifactStanding::Reported(&custody),
        &qualification,
    )?)
}

pub(super) fn active_selection(
    surface: &EvaluationSurface,
) -> Result<ActiveSelection, MutationRoadFailure> {
    let point = surface
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let alternative = point
        .admitted_alternatives()
        .first()
        .map(AdmittedAlternative::identity)
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    surface
        .select(point.identity(), alternative)
        .map_err(|_| MutationRoadFailure::MissingActiveSelection)
}

pub(super) fn selection_for_operation(
    surface: &EvaluationSurface,
    operation: &[u8],
) -> Result<ActiveSelection, MutationRoadFailure> {
    let point = surface
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let alternative = point
        .admitted_alternatives()
        .iter()
        .find(|alternative| alternative.operation() == operation)
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    surface
        .select(point.identity(), alternative.identity())
        .map_err(|_| MutationRoadFailure::MissingActiveSelection)
}

/// The no-mutation parity standing one pair earned under one witness for one input.
pub(super) type ParityStanding<'pair, 'input> =
    NoMutationParityStanding<'pair, 'input, [u32; 3], CompiledRosterMeaning>;

/// The qualification a parity standing earned, borrowed for as long as that standing lives.
pub(super) type ParityQualification<'pair, 'input> =
    NoMutationParityQualification<'pair, 'input, [u32; 3], CompiledRosterMeaning>;

/// The exact compiled projection pressure one selection demonstrated over one qualification.
pub(super) type Projection<'parity, 'pair, 'input> =
    CompiledProjectionPressure<'parity, 'pair, 'input, [u32; 3], CompiledRosterMeaning>;

/// The standard mutation witness: the comparison trial row bound to its declared check.
pub(super) fn witness() -> Result<MutationWitness<CompiledRosterMeaning>, MutationRoadFailure> {
    Ok(MutationWitness::bound(
        trial_binding()?,
        check_ref()?,
        check,
    )?)
}

/// Observe the no-mutation pass of one pair under one witness and qualify its parity reading.
pub(super) fn qualified_no_mutation<'pair, 'input>(
    pair: &'pair EvaluationPair<[u32; 3], CompiledRosterMeaning>,
    witness: MutationWitness<CompiledRosterMeaning>,
    input: &'input [u32; 3],
) -> Result<ParityStanding<'pair, 'input>, MutationRoadFailure> {
    Ok(qualify_no_mutation(observe_no_mutation(
        pair,
        witness,
        input,
        &invocation()?,
    )?))
}

/// The qualification one standing earned, or the disagreement refusal the claim modules name for its absence.
pub(super) fn qualification_of<'standing, 'pair, 'input>(
    standing: &'standing ParityStanding<'pair, 'input>,
) -> Result<&'standing ParityQualification<'pair, 'input>, MutationRoadFailure> {
    standing
        .qualification()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::MeaningsDisagreed,
        ))
}

/// Demonstrate one selected alternative through the standard materializer and the pinned compiled specimen host.
pub(super) fn standard_projection<'parity, 'pair, 'input>(
    surface: &EvaluationSurface,
    qualification: &'parity ParityQualification<'pair, 'input>,
    pair: &EvaluationPair<[u32; 3], CompiledRosterMeaning>,
    selection: ActiveSelection,
) -> Result<Projection<'parity, 'pair, 'input>, MutationRoadFailure> {
    let materializer = SpecimenMaterializerBinding::bound(pair, SPECIMEN_MATERIALIZER);
    Ok(demonstrate_compiled_projection(
        surface,
        qualification,
        &materializer,
        selection,
        &invocation()?,
        COMPILED_SPECIMEN_HOST,
    )?)
}

/// The interpreted trust one availability reading opened, or the missing evidence it names.
pub(super) fn opened_trust<'surface, 'suite, 'projection, 'parity, 'pair, 'input>(
    availability: InterpreterAvailability<
        'surface,
        'suite,
        'projection,
        'parity,
        'pair,
        'input,
        [u32; 3],
        CompiledRosterMeaning,
    >,
) -> Result<
    InterpretedTrust<
        'surface,
        'suite,
        'projection,
        'parity,
        'pair,
        'input,
        [u32; 3],
        CompiledRosterMeaning,
    >,
    MutationRoadFailure,
> {
    match availability {
        InterpreterAvailability::Available(trust) => Ok(trust),
        InterpreterAvailability::NoConformingSurface => Err(MutationRoadFailure::MissingTrust(
            MissingTrustEvidence::CompiledProjectionPressure,
        )),
        InterpreterAvailability::TrustNotOpened { missing } => {
            Err(MutationRoadFailure::MissingTrust(missing))
        }
    }
}

pub(super) fn assert_compiled_projection_custody(
    projection: &CompiledProjectionPressure<'_, '_, '_, [u32; 3], CompiledRosterMeaning>,
    pair: &EvaluationPair<[u32; 3], CompiledRosterMeaning>,
    selection: ActiveSelection,
) {
    assert_ne!(
        projection.baseline_artifact(),
        projection.standing().artifact()
    );
    assert_eq!(
        projection.baseline_artifact().address(),
        ContentAddress::derived(ARTIFACT_CONTENT_TAG, &specimen_source(ORIGINAL_OPERATION))
    );
    assert_eq!(
        projection.standing().artifact().address(),
        ContentAddress::derived(ARTIFACT_CONTENT_TAG, &specimen_source(SELECTED_OPERATION))
    );
    assert_eq!(projection.standing().pair(), pair.standing());
    assert_eq!(projection.standing().selection(), selection);
    assert!(lens_verdict(projection.baseline_report()).is_ok());
    assert!(lens_verdict(projection.selected_report()).is_err());
    assert_eq!(projection.mutation().verdict(), MutationVerdict::Killed);
    assert!(matches!(
        projection.mutation().target().identity(),
        MutationIdentity::CompiledProjection { point: _, alternative }
            if alternative == selection.alternative()
    ));
}

pub(super) fn assert_no_mutation_reading(
    reading: &NoMutationParityReading<'_, '_, [u32; 3], CompiledRosterMeaning>,
) {
    assert_eq!(reading.production(), &CompiledRosterMeaning::Stated(1));
    assert_eq!(reading.evaluation(), &CompiledRosterMeaning::Stated(1));
    assert_eq!(reading.evaluation_firings(), 0u32);
    assert_eq!(
        reading.production_report().trial(),
        reading.evaluation_report().trial()
    );
    assert_eq!(
        reading.production_report().measurement(),
        MeasurementReading::Unavailable
    );
    assert_eq!(
        reading.evaluation_report().measurement(),
        MeasurementReading::Unavailable
    );
}

pub(super) fn assert_interpreted_evidence_custody(
    report: &TrialReport,
    mutation: &MutationReport,
    expected_trial: TrialId,
    selection: ActiveSelection,
) {
    assert_eq!(report.trial(), expected_trial);
    assert_eq!(mutation.verdict(), MutationVerdict::Killed);
    assert!(matches!(
        mutation.activation().evidence(),
        Some(activation) if activation.witness() == report.trial()
    ));
    assert!(matches!(
        (report.attempt(), mutation.outcome()),
        (
            RunAttempt::Executed(TrialConclusion::Refused(report_finding)),
            MutationOutcome::Killed(IntendedRejection::Demonstrated(rejection)),
        ) if rejection.trial() == report.trial() && rejection.finding() == report_finding
    ));
    assert_eq!(
        mutation
            .activation()
            .evidence()
            .map(ActivationEvidence::selection),
        Some(selection)
    );
    assert!(matches!(
        mutation.target().identity(),
        MutationIdentity::Interpreted { point: _, alternative }
            if alternative == selection.alternative()
    ));
}

pub(super) fn interpreted_kill() -> Result<MutationReport, MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("comparison-family")?;
    let surface = surface_with(family, vec![b"input > 0", SELECTED_OPERATION])?;
    let pair = pair(family, &surface, evaluation_reads_resolved_payload_counted)?;
    let input = [1u32, 0, 0];
    let reading = observe_no_mutation(&pair, witness()?, &input, &invocation()?)?;
    assert_no_mutation_reading(&reading);
    let standing = qualify_no_mutation(reading);
    let qualification = qualification_of(&standing)?;
    let suite = compiled_suite_pressure()?;
    assert_eq!(suite.kill().target().owning_claim(), Some(claim()?));
    assert_eq!(
        suite.kill().target().family(),
        FamilyAttribution::Declared(operator()?)
    );
    assert!(matches!(
        suite.kill().target().site(),
        MutationSite::Reported(coordinate)
            if coordinate.file() == COMPILED_MUTANT_FILE
                && coordinate.line() == 68
                && coordinate.column() == 13
    ));
    let selection = selection_for_operation(&surface, SELECTED_OPERATION)?;
    let sibling = selection_for_operation(&surface, b"input > 0")?;
    let [point] = surface.points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(point.admitted_alternatives().len(), 2usize);
    assert_ne!(selection.alternative(), sibling.alternative());
    let projection = standard_projection(&surface, qualification, &pair, selection)?;
    assert_compiled_projection_custody(&projection, &pair, selection);
    let availability = availability(Some(&surface), Some(&suite), Some(&projection));
    assert_eq!(admission(&availability), RewriteAdmission::Admitted);
    let trust = opened_trust(availability)?;
    assert_eq!(trust.selection(), selection);
    CLAIM_MISMATCH_EVALUATION_CALLS.store(0, Ordering::SeqCst);
    INTERPRETED_CLOCK_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        execute_active(&trust, &foreign_measured_invocation()),
        Err(InterpretedExecutionRefusal::InvocationForAnotherExecution)
    ));
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(INTERPRETED_CLOCK_CALLS.load(Ordering::SeqCst), 0);
    let evidence = execute_active(&trust, &invocation()?)?;
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(evidence.selection(), selection);
    assert_eq!(evidence.meaning(), &CompiledRosterMeaning::Unstated);
    assert_interpreted_evidence_custody(
        evidence.report(),
        evidence.mutation(),
        qualification.reading().production_report().trial(),
        selection,
    );

    Ok(evidence.mutation().clone())
}

//! The public mutation receiver from owner policy through compiled pressure, exact no-mutation parity, active execution, and ordinary report evidence.

mod discovery;
mod interpretation;
mod proposal;
mod specimen;
mod structural_rewrite_planning;
mod support;

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
    HumanAdmissionRefusal, IntendedRejection, InterpretedExecutionRefusal, InterpreterAvailability,
    KillProposalRefusal, MissingTrustEvidence, MutationBackendInvocation, MutationIdentity,
    MutationOutcome, MutationPermission, MutationPolicy, MutationReport, MutationSite,
    MutationSourceRevision, MutationVerdict, MutationWitness, MutationWitnessRefusal,
    NoMutationObservationRefusal, NoMutationParityReading, OperatorFamilyRef, OwedClaimRefusal,
    OwnerClaimMapping, ParityQualificationRefusal, PermissionRefusal, PlanRefusal, PolicyRefusal,
    ProductionBinding, ProofDeltaRefusal, ProofRefusal, ProposalRefusal, QualificationRefusal,
    ReadingSource, RewriteAdmission, RewriteWithheld, SelectionRefusal, SinkRefusal,
    SourceCoordinate, SpecimenMaterializerBinding, SuitePressureRefusal, WrapReading, WrapRefusal,
    WrappedBackend,
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
use support::{
    COMPILED_SPECIMEN_HOST, SPECIMEN_MATERIALIZER, lock_specimen_tests, specimen_source,
};

const OWNER: &str = "harness.mutation.receiver";
const BACKEND_CONSOLE: &str =
    include_str!("compiled-pressure-artifact/cargo-mutants-27.0.0-console.txt");
const BACKEND_SOURCE: &[u8] = include_bytes!("compiled-pressure-artifact/wrap.rs");
const CURRENT_BACKEND_SOURCE: &[u8] = include_bytes!("../../src/muterprater/wrap.rs");
const BACKEND_NO_KILL: &str = "Found 1 mutant to test\n\
    ok Unmutated baseline in 3.1s\n\
    missed src/subject/lane.rs:41:9: replace is_qualified -> bool with true in 4.0s";
const BACKEND_VERSION: &str = "27.0.0";
const BACKEND_TARGET: &str = "x86_64-pc-windows-msvc";
const BACKEND_TOOLCHAIN: &str = "rustc 1.98.0 (88d9e12ae 2026-08-18)";
const BACKEND_COMMAND: &[&str] = &[
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
const COMPILED_MUTANT_FILE: &str = "harness/src/muterprater/wrap.rs";
const COMPILED_MUTANT_DAMAGE: &[u8] = b"replace != with == in roster_count";
const ORIGINAL_OPERATION: &[u8] = b"input != 0";
const SELECTED_OPERATION: &[u8] = b"input == 0";
const MEANING_DISAGREEMENT: FindingCause = FindingCause::named(OWNER, "meaning-disagreement");
const REVISION_TAG: DomainTag = DomainTag::declared(
    "mutation-receiver-revision",
    IdentityProfileVersion::declared(1),
);
const REPLAY_SCHEMA_TAG: DomainTag = DomainTag::declared(
    "mutation-receiver-replay-schema",
    IdentityProfileVersion::declared(1),
);
static CLAIM_MISMATCH_EVALUATION_CALLS: AtomicU32 = AtomicU32::new(0);
static INTERPRETED_CLOCK_CALLS: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, PartialEq, Eq)]
enum MutationRoadFailure {
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
enum InterpretedFailureStage {
    Invocation,
    Selection,
    WitnessClaim,
    EvaluationCall,
    DudPlant,
    Report,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompiledRosterMeaning {
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

fn family(stem: &'static str) -> Result<EvaluationFamilyRef, MutationRoadFailure> {
    EvaluationFamilyRef::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)
}

fn claim() -> Result<ClaimRef, MutationRoadFailure> {
    ClaimRef::named(OWNER, "comparison-behaviour").map_err(|_| MutationRoadFailure::Name)
}

fn operator() -> Result<OperatorFamilyRef, MutationRoadFailure> {
    OperatorFamilyRef::of_slug("comparison-boundaries").ok_or(MutationRoadFailure::MissingFamily)
}

fn policy(family: EvaluationFamilyRef) -> Result<MutationPolicy, MutationRoadFailure> {
    Ok(MutationPolicy::declared(
        family,
        vec![MutationPermission::declared(claim()?, vec![operator()?])?],
    )?)
}

fn discovered_point(
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

fn surface_with(
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

fn production(_input: &[u32; 3]) -> CompiledRosterMeaning {
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
type EvaluationFn =
    fn(
        &[u32; 3],
        EvaluationDirective<'_>,
    ) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal>;

/// This capture-free fixture's lawful branches both return observations.
const EVALUATION: EvaluationFn = |input, directive| {
    Ok(if directive.resolved().is_some() {
        EvaluationObservation::observed(CompiledRosterMeaning::Unstated, 1)
    } else {
        EvaluationObservation::observed(production(input), 0)
    })
};

fn evaluation_reads_resolved_payload(
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

fn evaluation_reads_resolved_payload_counted(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    CLAIM_MISMATCH_EVALUATION_CALLS.fetch_add(1, Ordering::SeqCst);
    evaluation_reads_resolved_payload(input, directive)
}

fn evaluation_counted(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    CLAIM_MISMATCH_EVALUATION_CALLS.fetch_add(1, Ordering::SeqCst);
    EVALUATION(input, directive)
}

fn same(left: &CompiledRosterMeaning, right: &CompiledRosterMeaning) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

fn check(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &CompiledRosterMeaning::Stated(1),
        MEANING_DISAGREEMENT,
    )
}

fn unused_trial_call(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn trial_binding_for(claim_stem: &'static str) -> Result<TrialBinding, TrialTableRefusal> {
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

fn trial_binding() -> Result<TrialBinding, TrialTableRefusal> {
    trial_binding_for("comparison-behaviour")
}

fn check_ref() -> Result<CheckRef, MutationRoadFailure> {
    CheckRef::named(OWNER, "comparison-check").map_err(|_| MutationRoadFailure::Name)
}

fn invocation() -> Result<Invocation, MutationRoadFailure> {
    let declared_toolchain = "1.98.0";
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

fn foreign_invocation() -> Invocation {
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

fn counted_tick() -> u64 {
    u64::from(INTERPRETED_CLOCK_CALLS.fetch_add(1, Ordering::SeqCst))
}

fn foreign_measured_invocation() -> Invocation {
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

fn pair(
    family: EvaluationFamilyRef,
    surface: &EvaluationSurface,
    evaluated: EvaluationCall<[u32; 3], CompiledRosterMeaning>,
) -> Result<EvaluationPair<[u32; 3], CompiledRosterMeaning>, MutationRoadFailure> {
    pair_with_evaluation_revision(family, surface, evaluated, b"evaluation")
}

fn pair_with_evaluation_revision(
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

fn compiled_owner(coordinate: &SourceCoordinate) -> Option<ClaimRef> {
    (coordinate.file() == COMPILED_MUTANT_FILE)
        .then(claim)
        .and_then(Result::ok)
}

fn compiled_family(coordinate: &SourceCoordinate, damage: &[u8]) -> Option<OperatorFamilyRef> {
    (coordinate.file() == COMPILED_MUTANT_FILE && damage == COMPILED_MUTANT_DAMAGE)
        .then(operator)
        .and_then(Result::ok)
}

fn compiled_reading() -> Result<WrapReading, MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    Ok(read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(version),
        compiled_owner,
        compiled_family,
    )?)
}

fn backend_invocation(
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

fn source_revision(bytes: &[u8]) -> Result<MutationSourceRevision, MutationRoadFailure> {
    MutationSourceRevision::from_content(COMPILED_MUTANT_FILE, bytes)
        .map_err(|_| MutationRoadFailure::Name)
}

fn compiled_artifact(
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

fn current_custody(
    manifest: CompiledSuiteArtifactManifest,
    current_source: &[u8],
) -> Result<CompiledSuiteArtifactCustody, MutationRoadFailure> {
    Ok(CompiledSuiteArtifactCustody::current(
        manifest,
        vec![source_revision(current_source)?],
    )?)
}

fn compiled_suite_pressure() -> Result<CompiledSuitePressure, MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let manifest = compiled_artifact(BACKEND_CONSOLE, version.clone(), BACKEND_SOURCE)?;
    let qualification =
        AdapterQualification::of(manifest.reading(), GrammarStanding::Checked(version))?;
    let custody = current_custody(manifest, CURRENT_BACKEND_SOURCE)?;
    Ok(CompiledSuitePressure::demonstrated(
        CompiledSuiteArtifactStanding::Reported(&custody),
        &qualification,
    )?)
}

fn active_selection(surface: &EvaluationSurface) -> Result<ActiveSelection, MutationRoadFailure> {
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

fn selection_for_operation(
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

/// Claim: a surface refuses absent points and alternatives borrowed from another point.
///
/// Subject: the public discovery lowering and surface selection roads.
/// Population: one duplicate input and one two-point admitted surface.
/// Hostile control: the selection attempts an absent point and a sibling point's alternative.
/// Denominator: every selection refusal coordinate exposed by the two-point fixture.
/// Evidence ceiling: this establishes discovery-owned selection boundaries for one outside fixture and says nothing about interpretation execution.
/// Retained regression: this discovery composition claim remains in the original integration target.
#[test]
fn surface_selection_refuses_absent_points_and_crossed_alternatives()
-> Result<(), MutationRoadFailure> {
    let first_family = family("constructor-family")?;
    let first_policy = policy(first_family)?;
    let duplicate = discovered_point(
        "duplicate-selection-point",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a <= b"],
    )?;
    let duplicate_ref = duplicate.identity();
    assert!(matches!(
        lower_discoveries(&first_policy, vec![duplicate.clone(), duplicate]),
        Err(DiscoveryLoweringRefusal::DuplicateSite { at: 1, point }) if point == duplicate_ref
    ));
    let two = lower_discoveries(
        &first_policy,
        vec![
            discovered_point(
                "selection-first",
                OwnerClaimMapping::Mapped(claim()?),
                vec![b"a <= b"],
            )?,
            discovered_point(
                "selection-second",
                OwnerClaimMapping::Mapped(claim()?),
                vec![b"a >= b"],
            )?,
        ],
    )?;
    let [first_point, second_point] = two.surface().points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let first_point_ref = first_point.identity();
    let second_alternative = second_point
        .admitted_alternatives()
        .first()
        .map(AdmittedAlternative::identity)
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let absent_point =
        MutationPointRef::named(OWNER, "absent-point").map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        two.surface().select(absent_point, second_alternative,),
        Err(SelectionRefusal::NoSuchPoint(absent_point))
    );
    assert_eq!(
        two.surface().select(first_point_ref, second_alternative),
        Err(SelectionRefusal::NoSuchAlternative {
            point: first_point_ref,
            alternative: second_alternative,
        })
    );

    Ok(())
}

/// Claim: point-free parity cannot cross the specimen or rewrite joins into active mutation authority.
///
/// Subject: the interpretation parity, specimen pressure, and rewrite admission composition.
/// Population: one lawfully qualified point-free surface.
/// Hostile control: generic suite pressure is present while selection-scoped projection pressure is absent.
/// Denominator: the point-free qualification, interpretation availability, and rewrite admission readings.
/// Evidence ceiling: this establishes one typed cross-owner composition and does not annex any participating claim.
/// Retained regression: the composition claim remains in the original integration target.
#[test]
fn point_free_trust_does_not_admit_mutation_execution() -> Result<(), MutationRoadFailure> {
    let family = family("point-free-family")?;
    let policy = policy(family)?;
    let surface = lower_discoveries(&policy, Vec::new())?.into_parts().1;
    let pair = pair(family, &surface, EVALUATION)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing =
        qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation()?)?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let suite = compiled_suite_pressure()?;
    let availability =
        availability::<[u32; 3], CompiledRosterMeaning>(Some(&surface), Some(&suite), None);
    assert!(matches!(
        &availability,
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledProjectionPressure,
        }
    ));
    assert_eq!(
        admission(&availability),
        RewriteAdmission::Withheld(RewriteWithheld::TrustNotOpened(
            MissingTrustEvidence::CompiledProjectionPressure,
        ))
    );
    assert_eq!(qualification.reading().pair().standing(), pair.standing());
    Ok(())
}

fn assert_compiled_projection_custody(
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

fn assert_no_mutation_reading(
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

fn assert_interpreted_evidence_custody(
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

fn interpreted_kill() -> Result<MutationReport, MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("comparison-family")?;
    let surface = surface_with(family, vec![b"input > 0", SELECTED_OPERATION])?;
    let pair = pair(family, &surface, evaluation_reads_resolved_payload_counted)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let reading = observe_no_mutation(&pair, witness, &input, &invocation()?)?;
    assert_no_mutation_reading(&reading);
    let standing = qualify_no_mutation(reading);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
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
                && coordinate.line() == 348
                && coordinate.column() == 13
    ));
    let selection = selection_for_operation(&surface, SELECTED_OPERATION)?;
    let sibling = selection_for_operation(&surface, b"input > 0")?;
    let [point] = surface.points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(point.admitted_alternatives().len(), 2usize);
    assert_ne!(selection.alternative(), sibling.alternative());
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let projection = demonstrate_compiled_projection(
        &surface,
        qualification,
        &materializer,
        selection,
        &invocation()?,
        COMPILED_SPECIMEN_HOST,
    )?;
    assert_compiled_projection_custody(&projection, &pair, selection);
    let availability = availability(Some(&surface), Some(&suite), Some(&projection));
    assert_eq!(admission(&availability), RewriteAdmission::Admitted);
    let trust = match availability {
        InterpreterAvailability::Available(trust) => trust,
        InterpreterAvailability::NoConformingSurface => {
            return Err(MutationRoadFailure::MissingTrust(
                MissingTrustEvidence::CompiledProjectionPressure,
            ));
        }
        InterpreterAvailability::TrustNotOpened { missing } => {
            return Err(MutationRoadFailure::MissingTrust(missing));
        }
    };
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

/// Claim: generic suite pressure and exact projection pressure join without flattening their evidence into interpreted execution.
///
/// Subject: the compiled-suite, specimen-projection, and interpretation execution composition.
/// Population: one two-alternative surface and one selected compiled projection.
/// Hostile control: a foreign invocation is refused before evaluation or clock effects.
/// Denominator: every evidence book and custody join traversed by the selected execution.
/// Evidence ceiling: this establishes one complete outside composition and preserves each owner's narrower evidence ceiling.
/// Retained regression: the cross-owner claim remains in the original integration target.
#[test]
fn compiled_and_interpreted_evidence_join_without_flattening() -> Result<(), MutationRoadFailure> {
    let mutation = interpreted_kill()?;
    assert_eq!(mutation.verdict(), MutationVerdict::Killed);
    Ok(())
}

/// Claim: exact projection pressure cannot open interpreted trust for another surface.
///
/// Subject: the public specimen demonstration and interpretation availability roads.
/// Population: two surfaces under one family with distinct admitted selections.
/// Hostile control: a lawful projection from the second surface is offered to the first.
/// Denominator: one complete projection and the one crossed surface join.
/// Evidence ceiling: this establishes the typed cross-owner join for one outside fixture and does not widen either evidence book.
/// Retained regression: this composition claim remains in the original integration target.
#[test]
fn compiled_pressure_cannot_open_trust_for_another_surface() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("same-family-pair-scope")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let evaluation_pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let another_surface = surface_with(family, vec![SELECTED_OPERATION])?;
    let another_pair = pair(family, &another_surface, EVALUATION)?;
    assert_ne!(another_pair.standing(), evaluation_pair.standing());
    let another_standing = qualify_no_mutation(observe_no_mutation(
        &another_pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let another_qualification =
        another_standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let another_selection = active_selection(&another_surface)?;
    let another_materializer =
        SpecimenMaterializerBinding::bound(&another_pair, SPECIMEN_MATERIALIZER);
    let another_projection = demonstrate_compiled_projection(
        &another_surface,
        another_qualification,
        &another_materializer,
        another_selection,
        &invocation()?,
        COMPILED_SPECIMEN_HOST,
    )?;
    let suite = compiled_suite_pressure()?;
    assert!(matches!(
        availability(Some(&surface), Some(&suite), Some(&another_projection)),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::ProjectionPressureForAnotherSurface,
        }
    ));
    Ok(())
}

/// Adapter qualification remains bound to the exact backend profile whose reading earned it.
#[test]
fn a_compiled_witness_refuses_another_profile() -> Result<(), MutationRoadFailure> {
    let here_version =
        BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let here = compiled_artifact(BACKEND_CONSOLE, here_version, BACKEND_SOURCE)?;
    let other_version = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    let elsewhere = read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(other_version.clone()),
        compiled_owner,
        compiled_family,
    )?;
    let borrowed = AdapterQualification::of(&elsewhere, GrammarStanding::Checked(other_version))?;
    let custody = current_custody(here, CURRENT_BACKEND_SOURCE)?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(
            CompiledSuiteArtifactStanding::Reported(&custody),
            &borrowed,
        ),
        Err(SuitePressureRefusal::QualificationUnderAnotherProfile)
    );
    Ok(())
}

/// Imported suite pressure retains backend, version, command, target, output, parser, and exact current source revision without turning any of them into pair authority.
#[test]
fn compiled_suite_artifact_custody_is_complete_and_current() -> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let manifest = compiled_artifact(BACKEND_CONSOLE, version.clone(), BACKEND_SOURCE)?;
    assert_eq!(
        manifest.invocation().backend(),
        WrappedBackend::CargoMutants
    );
    assert_eq!(manifest.invocation().version(), &version);
    assert_eq!(manifest.invocation().command().executable(), "cargo");
    assert_eq!(manifest.invocation().command().arguments(), BACKEND_COMMAND);
    assert_eq!(
        manifest.invocation().target().target().spelling(),
        BACKEND_TARGET
    );
    assert_eq!(
        manifest.invocation().target().toolchain().spelling(),
        BACKEND_TOOLCHAIN
    );
    assert_eq!(
        manifest.reading().profile().backend(),
        manifest.invocation().backend()
    );
    assert_eq!(
        manifest.reading().profile().version(),
        &BackendVersionPosture::Stated(version)
    );
    assert_eq!(
        manifest.reading().profile().source(),
        ReadingSource::ConsoleStream
    );
    assert_eq!(manifest.reading().profile().grammar().number(), 1u32);
    let [source] = manifest.sources() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(source.file(), COMPILED_MUTANT_FILE);
    assert_eq!(source, &source_revision(BACKEND_SOURCE)?);

    let same = compiled_artifact(
        BACKEND_CONSOLE,
        manifest.invocation().version().clone(),
        BACKEND_SOURCE,
    )?;
    assert_eq!(manifest.output(), same.output());
    let changed_console = format!("{BACKEND_CONSOLE}artifact-note\n");
    let changed = compiled_artifact(
        &changed_console,
        manifest.invocation().version().clone(),
        BACKEND_SOURCE,
    )?;
    assert_ne!(manifest.output(), changed.output());

    let custody = current_custody(manifest.clone(), CURRENT_BACKEND_SOURCE)?;
    assert_eq!(custody.manifest(), &manifest);
    let moved = source_revision(b"moved-source")?;
    assert!(matches!(
        CompiledSuiteArtifactCustody::current(manifest, vec![moved]),
        Err(ArtifactCustodyRefusal::CurrentSourceMoved { file, expected, found })
            if file == COMPILED_MUTANT_FILE && expected != found
    ));
    Ok(())
}

/// Source custody closes both the artifact-time and current-source rosters instead of accepting a convenient subset.
#[test]
fn compiled_suite_source_rosters_refuse_missing_extra_and_duplicate_files()
-> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let invocation = || backend_invocation(version.clone());
    let source = source_revision(BACKEND_SOURCE)?;
    assert_eq!(
        read_artifact(
            BACKEND_CONSOLE,
            invocation()?,
            Vec::new(),
            compiled_owner,
            compiled_family,
        ),
        Err(ArtifactManifestRefusal::ReportedSourceMissing(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );
    let extra = MutationSourceRevision::from_content("elsewhere.rs", b"elsewhere")
        .map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        read_artifact(
            BACKEND_CONSOLE,
            invocation()?,
            vec![source.clone(), extra.clone()],
            compiled_owner,
            compiled_family,
        ),
        Err(ArtifactManifestRefusal::SourceNotReported(
            "elsewhere.rs".to_owned(),
        ))
    );
    assert_eq!(
        read_artifact(
            BACKEND_CONSOLE,
            invocation()?,
            vec![source.clone(), source.clone()],
            compiled_owner,
            compiled_family,
        ),
        Err(ArtifactManifestRefusal::DuplicateSource(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );

    let manifest = compiled_artifact(BACKEND_CONSOLE, version, BACKEND_SOURCE)?;
    assert_eq!(
        CompiledSuiteArtifactCustody::current(manifest.clone(), Vec::new()),
        Err(ArtifactCustodyRefusal::CurrentSourceMissing(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );
    assert_eq!(
        CompiledSuiteArtifactCustody::current(manifest.clone(), vec![source.clone(), extra],),
        Err(ArtifactCustodyRefusal::CurrentSourceUnexpected(
            "elsewhere.rs".to_owned(),
        ))
    );
    assert_eq!(
        CompiledSuiteArtifactCustody::current(manifest, vec![source.clone(), source]),
        Err(ArtifactCustodyRefusal::DuplicateCurrentSource(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );
    Ok(())
}

/// Adapter qualification preserves its complete refusal order over unchecked, unstated, and differently versioned profiles.
#[test]
fn adapter_qualification_requires_one_checked_profile_version() -> Result<(), MutationRoadFailure> {
    let stated = compiled_reading()?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Unchecked),
        Err(QualificationRefusal::GrammarUnchecked)
    );

    let checked = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let unstated = read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Unstated,
        compiled_owner,
        compiled_family,
    )?;
    assert_eq!(
        AdapterQualification::of(&unstated, GrammarStanding::Checked(checked.clone())),
        Err(QualificationRefusal::BackendVersionUnstated)
    );

    let another = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Checked(another.clone())),
        Err(QualificationRefusal::CheckedAgainstAnotherVersion {
            stated: checked,
            checked: another,
        })
    );
    Ok(())
}

/// Generic compiled suite pressure requires both a reported reading and a lawful backend-reported kill from that reading.
#[test]
fn generic_suite_pressure_requires_a_reported_kill() -> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let killed = compiled_artifact(BACKEND_CONSOLE, version.clone(), BACKEND_SOURCE)?;
    assert_eq!(killed.reading().announced(), AnnouncedRoster::Stated(1));
    assert!(matches!(
        killed.reading().unparsed(),
        [summary]
            if summary.ordinal() == 3
                && summary.text().bytes() == b"1 mutant tested: 1 caught"
    ));
    let killed_qualification =
        AdapterQualification::of(killed.reading(), GrammarStanding::Checked(version.clone()))?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(
            CompiledSuiteArtifactStanding::NotReported,
            &killed_qualification,
        ),
        Err(SuitePressureRefusal::ArtifactNotReported)
    );

    let missed_source = MutationSourceRevision::from_content("src/subject/lane.rs", b"missed")
        .map_err(|_| MutationRoadFailure::Name)?;
    let missed = read_artifact(
        BACKEND_NO_KILL,
        backend_invocation(version.clone())?,
        vec![missed_source.clone()],
        compiled_owner,
        compiled_family,
    )?;
    let missed_qualification =
        AdapterQualification::of(missed.reading(), GrammarStanding::Checked(version))?;
    let missed_custody = CompiledSuiteArtifactCustody::current(missed, vec![missed_source])?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(
            CompiledSuiteArtifactStanding::Reported(&missed_custody),
            &missed_qualification,
        ),
        Err(SuitePressureRefusal::NoKillDemonstrated)
    );
    Ok(())
}

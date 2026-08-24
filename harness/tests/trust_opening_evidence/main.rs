//! The public mutation receiver from owner policy through compiled pressure, exact no-mutation parity, active execution, and ordinary report evidence.

use macroonz_harness::clock::{HarnessClock, MeasurementReading};
use macroonz_harness::depot::capsules::{
    ReplayCapsuleEntry, ReplayDepotRefusal, ReplayDepotSink, StoredReplayEntryRef,
};
use macroonz_harness::descriptor::{
    AuthoredTableName, AuthoredTableRefusal, Binding, CheckRef, ClaimRef, Classification,
    ExecutableAttachment, ExecutionSuite, GeneratedSupportSchemaId, MutationPointRef,
    NamespacedName, Origin, PopulationRef, ProposalId, Provenance, RevisionBinding, Role, Row,
    SubjectRoute, SynthesisFacts, Tag, TrialTableRefusal,
};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionPlanRefusal, ReductionProbeBinding, ReductionProbeRefusal, ReductionRefusal,
    capture_replay, reduce,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::interpret::{
    availability, execute_active, observe_no_mutation, qualify_no_mutation,
};
use macroonz_harness::muterprater::propose::{
    human_admit_discharge, human_admit_replay, offer_claim_pin, offer_mutant_kill,
    offer_obligation_discharge, prove_candidate,
};
use macroonz_harness::muterprater::rewrite::admission;
use macroonz_harness::muterprater::specimen::demonstrate_compiled_projection;
use macroonz_harness::muterprater::wrap::read_output;
use macroonz_harness::muterprater::{
    ARTIFACT_CONTENT_TAG, ActivationEvidence, ActivationSite, ActiveSelection,
    AdapterQualification, AdmittedAlternative, AlternativeDeclaration, AlternativeId,
    AnnouncedRoster, BackendVersion, BackendVersionPosture, CompiledProjectionPressure,
    CompiledProjectionRefusal, CompiledSpecimenHostRefusal, CompiledSpecimenObservation,
    CompiledSpecimenObservationMismatch, CompiledSpecimenRequest, CompiledSpecimenRole,
    CompiledSuitePressure, Demonstration, DischargeEvidence, DischargeProposalRefusal,
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryLoweringRefusal, DiscoveryRefusal,
    DuplicateRefusal, EvaluationBinding, EvaluationCall, EvaluationCallRefusal,
    EvaluationDirective, EvaluationFamilyRef, EvaluationObservation, EvaluationPair,
    EvaluationPairRefusal, EvaluationPairStandingMismatch, EvaluationSurface, FamilyAttribution,
    GrammarStanding, HumanAdmissionRefusal, IntendedRejection, InterpretedExecutionRefusal,
    InterpreterAvailability, KillProposalRefusal, MappedUnpermittedCause, MissingTrustEvidence,
    MutationDiscoveryReading, MutationIdentity, MutationOutcome, MutationPermission, MutationPoint,
    MutationPolicy, MutationReport, MutationSite, MutationVerdict, MutationWitness,
    MutationWitnessRefusal, NoComparisonReason, NoMutationObservationRefusal,
    NoMutationParityReading, ObligationLane, OperatorFamilyRef, OwedClaim, OwedClaimRefusal,
    OwnerClaimMapping, ParityQualificationRefusal, PermissionRefusal, PointCatalogPosture,
    PolicyRefusal, ProductionBinding, ProofDelta, ProofDeltaRefusal, ProofRefusal,
    ProposalDestination, ProposalDocument, ProposalRefusal, ProposalSink, QualificationRefusal,
    ReplayBearingProposal, RewriteAdmission, RewriteWithheld, SelectionRefusal, SinkRefusal,
    SourceCoordinate, SpecimenMaterializerBinding, SpecimenMaterializerRefusal, StoredProposalRef,
    SuitePressureRefusal, WrapReading, WrapRefusal, WrapStanding,
};
use macroonz_harness::properties::{Agreement, agreement};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FindingCause, Fingerprint, ForeignText, GenerationProfile,
    InvocationProfile, MinimizationProfile, ReplayCapsule, RunAttempt, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialId, TrialReport, TrialSite, encode_bytes,
    encode_length,
};
use macroonz_harness::runner::{
    Invocation, TrialBinding, TrialTable, lens_verdict, trial_identity,
};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

const OWNER: &str = "harness.mutation.receiver";
const BACKEND_CONSOLE: &str =
    include_str!("compiled-pressure-artifact/cargo-mutants-27.0.0-console.txt");
const BACKEND_NO_KILL: &str = "Found 1 mutant to test\n\
    ok Unmutated baseline in 3.1s\n\
    missed src/subject/lane.rs:41:9: replace is_qualified -> bool with true in 4.0s";
const BACKEND_VERSION: &str = "27.0.0";
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
const POLICY_READING_TAG: DomainTag =
    DomainTag::declared("mutation-policy", IdentityProfileVersion::declared(1));
const ALTERNATIVE_READING_TAG: DomainTag =
    DomainTag::declared("mutation-alternative", IdentityProfileVersion::declared(1));
const SURFACE_READING_TAG: DomainTag =
    DomainTag::declared("evaluation-surface", IdentityProfileVersion::declared(1));
const DISCOVERY_READING_TAG: DomainTag =
    DomainTag::declared("mutation-discovery", IdentityProfileVersion::declared(1));
static CLAIM_MISMATCH_EVALUATION_CALLS: AtomicU32 = AtomicU32::new(0);
static NO_MUTATION_CALL_ORDER: AtomicU32 = AtomicU32::new(0);
static SPECIMEN_ORDINAL: AtomicU32 = AtomicU32::new(0);
static SPECIMEN_MATERIALIZER_CALLS: AtomicU32 = AtomicU32::new(0);
static SPECIMEN_HOST_CALLS: AtomicU32 = AtomicU32::new(0);
static INTERPRETED_CLOCK_CALLS: AtomicU32 = AtomicU32::new(0);
static SPECIMEN_TEST_LOCK: Mutex<()> = Mutex::new(());
static CACHED_SIBLING_OBSERVATION: Mutex<
    Option<CompiledSpecimenObservation<CompiledRosterMeaning>>,
> = Mutex::new(None);

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
    Qualification(QualificationRefusal),
    Pressure(SuitePressureRefusal),
    Projection(CompiledProjectionRefusal),
    Witness(MutationWitnessRefusal),
    Observation(NoMutationObservationRefusal),
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

fn lock_specimen_tests() -> Result<std::sync::MutexGuard<'static, ()>, MutationRoadFailure> {
    SPECIMEN_TEST_LOCK
        .lock()
        .map_err(|_| MutationRoadFailure::NativeToolchain)
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

#[derive(Default)]
struct ReviewSink {
    proposals: Vec<ProposalId>,
}

impl ProposalSink for ReviewSink {
    fn store<Document: ProposalDocument>(
        &mut self,
        proposal: &Document,
    ) -> Result<StoredProposalRef, SinkRefusal> {
        let identity = proposal.identity();
        self.proposals.push(identity);
        StoredProposalRef::at(identity, "review://mutation-proposal")
    }
}

#[derive(Default)]
struct ReplayDepot {
    entries: Vec<ReplayCapsuleEntry>,
}

impl ReplayDepotSink for ReplayDepot {
    fn store(
        &mut self,
        entry: &ReplayCapsuleEntry,
    ) -> Result<StoredReplayEntryRef, ReplayDepotRefusal> {
        self.entries.push(entry.clone());
        StoredReplayEntryRef::at(entry.replay(), "depot://mutation-replay")
    }
}

fn family(stem: &'static str) -> Result<EvaluationFamilyRef, MutationRoadFailure> {
    EvaluationFamilyRef::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)
}

fn push_name(into: &mut Vec<u8>, name: NamespacedName) {
    encode_bytes(name.namespace().written().as_bytes(), into);
    encode_bytes(name.stem().written().as_bytes(), into);
}

fn independently_frame_discovery(reading: &MutationDiscoveryReading) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_name(&mut bytes, reading.family().name());
    encode_bytes(reading.policy().address().as_bytes(), &mut bytes);
    encode_length(reading.entries().len(), &mut bytes);
    for entry in reading.entries() {
        let site = entry.site();
        push_name(&mut bytes, site.identity().name());
        match site.mapping() {
            OwnerClaimMapping::Mapped(owner_claim) => {
                bytes.push(1);
                push_name(&mut bytes, owner_claim.name());
            }
            OwnerClaimMapping::OwnerUnmapped => bytes.push(0),
        }
        encode_bytes(site.original_operation(), &mut bytes);
        encode_length(site.alternatives().len(), &mut bytes);
        for alternative in site.alternatives() {
            encode_bytes(alternative.family().slug().as_bytes(), &mut bytes);
            encode_bytes(alternative.operation(), &mut bytes);
        }
        push_name(&mut bytes, site.activation_site().name());
    }
    bytes
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

fn point(
    policy: &MutationPolicy,
    stem: &'static str,
    alternatives: Vec<&'static [u8]>,
) -> Result<MutationPoint, MutationRoadFailure> {
    let discovered = discovered_point(stem, OwnerClaimMapping::Mapped(claim()?), alternatives)?;
    let lowered = lower_discoveries(policy, vec![discovered])?;
    lowered
        .surface()
        .points()
        .first()
        .cloned()
        .ok_or(MutationRoadFailure::MissingAlternative)
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

fn production_ordered(input: &[u32; 3]) -> CompiledRosterMeaning {
    if NO_MUTATION_CALL_ORDER
        .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        NO_MUTATION_CALL_ORDER.store(u32::MAX, Ordering::SeqCst);
    }
    production(input)
}

/// The shape every evaluation callable below inhabits: the contract's own, whose refusing side belongs to the fixtures that refuse.
///
/// The lawful and hostile fixtures that never refuse are `const` closures over this shape rather than `fn` items, so their always-passing bodies carry no fallibility of their own.
type EvaluationFn =
    fn(
        &[u32; 3],
        EvaluationDirective<'_>,
    ) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal>;

/// The materializer-callable shape, on the same terms.
type MaterializerFn = fn(EvaluationDirective<'_>) -> Result<Vec<u8>, SpecimenMaterializerRefusal>;

/// The compiled-specimen host shape: by value, because the contract passes custody of each request.
type SpecimenHostFn =
    fn(
        CompiledSpecimenRequest<'_, '_, [u32; 3]>,
    )
        -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal>;

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

/// This capture-free hostile fixture returns a semantic disagreement rather than a call refusal.
const PARITY_BROKEN: EvaluationFn = |_input, directive| {
    Ok(EvaluationObservation::observed(
        CompiledRosterMeaning::Unstated,
        u32::from(directive.resolved().is_some()),
    ))
};

fn no_mutation_branch_omitted(
    _input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    match directive.resolved() {
        None => {
            if NO_MUTATION_CALL_ORDER
                .compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst)
                .is_err()
            {
                NO_MUTATION_CALL_ORDER.store(u32::MAX, Ordering::SeqCst);
            }
            Err(EvaluationCallRefusal::NoMutationNotImplemented)
        }
        Some(resolved) => Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        )),
    }
}

fn active_branch_omitted(
    input: &[u32; 3],
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<CompiledRosterMeaning>, EvaluationCallRefusal> {
    match directive.resolved() {
        None => Ok(EvaluationObservation::observed(production(input), 0)),
        Some(resolved) => Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        )),
    }
}

/// This capture-free hostile fixture reports zero firing as a successful raw observation.
const ACTIVATION_MISSING: EvaluationFn = |input, directive| {
    Ok(if directive.resolved().is_some() {
        EvaluationObservation::observed(CompiledRosterMeaning::Unstated, 0)
    } else {
        EvaluationObservation::observed(production(input), 0)
    })
};

/// This capture-free hostile fixture reports an invalid positive no-mutation count.
const NO_MUTATION_ACTIVATES: EvaluationFn =
    |input, _directive| Ok(EvaluationObservation::observed(production(input), 1));

/// This capture-free fixture's active observation remains semantically lawful.
const ACTIVATION_SURVIVES: EvaluationFn = |input, directive| {
    Ok(EvaluationObservation::observed(
        production(input),
        u32::from(directive.resolved().is_some()),
    ))
};

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

fn check_passes(_meaning: &CompiledRosterMeaning) -> TrialConclusion {
    TrialConclusion::Passed
}

fn check_evaluation_meaning(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &CompiledRosterMeaning::Unstated,
        MEANING_DISAGREEMENT,
    )
}

fn check_refuses(meaning: &CompiledRosterMeaning) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &CompiledRosterMeaning::SetupRefused,
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

fn candidate_trial_call(_invocation: &Invocation) -> TrialConclusion {
    check(&CompiledRosterMeaning::Unstated)
}

fn candidate_binding(point: MutationPointRef) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, "comparison-subject")?;
    let check_ref = CheckRef::named(OWNER, "comparison-check")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, "comparison-behaviour")?,
        ExecutionSuite::named(OWNER, "mutation-receiver")?,
        Classification::authored(
            vec![Role::named(OWNER, "mutation")?],
            vec![Tag::named(OWNER, "outside-consumer")?],
        )?,
        subject,
        check_ref,
        PopulationRef::named(OWNER, "one-input")?,
        Origin::Candidate(SynthesisFacts::Survivor(point)),
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check_ref,
            revision,
            revision,
            candidate_trial_call,
        ),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn authored_parent() -> Result<TrialTable, TrialTableRefusal> {
    TrialTable::authored(
        AuthoredTableName::named(OWNER, "mutation-parent")?,
        Provenance::Unproduced,
        vec![trial_binding_for("parent-behaviour")?],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)
}

fn replay_probe(_input: &[u8]) -> ProbeOutcome {
    let Ok(point) = MutationPointRef::named(OWNER, "comparison-edge") else {
        return ProbeOutcome::NoFailure;
    };
    let Ok(binding) = candidate_binding(point) else {
        return ProbeOutcome::NoFailure;
    };
    let Ok(invocation) = invocation() else {
        return ProbeOutcome::NoFailure;
    };
    let TrialConclusion::Refused(finding) = candidate_trial_call(&invocation) else {
        return ProbeOutcome::NoFailure;
    };
    ProbeOutcome::Reproduced(Fingerprint::of(trial_identity(binding.row()), &finding))
}

fn demonstrate_mutation(
    mutation: &MutationReport,
) -> Result<(Row, Demonstration), MutationRoadFailure> {
    let Some(point) = mutation.target().identity().point() else {
        return Err(MutationRoadFailure::MissingActiveSelection);
    };
    let candidate = candidate_binding(point)?;
    let candidate_key = candidate.trial_key();
    let candidate_row = candidate.row().clone();
    assert!(matches!(
        TrialTable::authored(
            AuthoredTableName::named(OWNER, "candidate-cannot-enter")
                .map_err(|_| MutationRoadFailure::Name)?,
            Provenance::Unproduced,
            vec![candidate.clone()],
        ),
        Err(AuthoredTableRefusal::CandidateOrigin(found)) if found == candidate_key
    ));
    let demonstration = prove_candidate(
        &authored_parent()?,
        candidate,
        mutation.target(),
        &invocation()?,
    )?;
    let mutation_fingerprint = match mutation.outcome() {
        MutationOutcome::Killed(IntendedRejection::Demonstrated(rejection)) => {
            rejection.fingerprint()
        }
        MutationOutcome::Killed(IntendedRejection::ReportedByBackend { stated: _ })
        | MutationOutcome::Survived
        | MutationOutcome::Inconclusive(_) => {
            return Err(MutationRoadFailure::MissingActiveSelection);
        }
    };
    assert_eq!(
        demonstration.rejection().fingerprint(),
        mutation_fingerprint
    );
    Ok((candidate_row, demonstration))
}

fn capture_demonstration(
    demonstration: &Demonstration,
) -> Result<ReplayCapsule, MutationRoadFailure> {
    let reduction_binding = ReductionProbeBinding::bound(
        demonstration.trial_report(),
        GenerationProfile::declared("mutation-replay", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(
            REPLAY_SCHEMA_TAG,
            b"mutation-replay-schema",
        )),
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"replay-probe")),
        replay_probe,
    )?;
    let reduction_plan = ReductionPlan::declared(
        MinimizationProfile::declared("mutation-replay", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(1),
    )?;
    let reduction = reduce(&reduction_plan, &[1u8, 2u8], &reduction_binding)?;
    let capsule = capture_replay(&reduction);
    assert_eq!(capsule.key(), demonstration.trial_report().standing().key());
    assert_eq!(
        capsule.fingerprint(),
        demonstration.rejection().fingerprint()
    );
    Ok(capsule)
}

fn reject_capsule_for_another_execution(
    candidate: &Row,
    mutation: &MutationReport,
    demonstration: &Demonstration,
    destination: ProposalDestination,
) -> Result<(), MutationRoadFailure> {
    let Some(point) = mutation.target().identity().point() else {
        return Err(MutationRoadFailure::MissingActiveSelection);
    };
    let foreign_demonstration = prove_candidate(
        &authored_parent()?,
        candidate_binding(point)?,
        mutation.target(),
        &foreign_invocation(),
    )?;
    let foreign_capsule = capture_demonstration(&foreign_demonstration)?;
    let replay = foreign_capsule.key().address();
    let expected = demonstration.trial_report().standing().key().address();
    assert!(matches!(
        offer_mutant_kill(
            candidate.clone(),
            mutation,
            foreign_capsule,
            demonstration.clone(),
            Vec::new(),
            destination,
        ),
        Err(KillProposalRefusal::ReplayExecutionMismatch {
            replay: found,
            demonstration: required,
        }) if found == replay && required == expected
    ));
    Ok(())
}

fn admit_mutation(mutation: &MutationReport) -> Result<(), MutationRoadFailure> {
    let (candidate, demonstration) = demonstrate_mutation(mutation)?;
    let capsule = capture_demonstration(&demonstration)?;
    let destination = ProposalDestination::naming(
        ExecutionSuite::named(OWNER, "mutation-receiver").map_err(|_| MutationRoadFailure::Name)?,
    );
    reject_capsule_for_another_execution(&candidate, mutation, &demonstration, destination)?;
    let proposal = offer_mutant_kill(
        candidate,
        mutation,
        capsule.clone(),
        demonstration,
        Vec::new(),
        destination,
    )?;
    let mut review = ReviewSink::default();
    let proposal_custody = review
        .store(&proposal)
        .map_err(MutationRoadFailure::ProposalSink)?;
    assert_eq!(proposal_custody.proposal(), proposal.identity());
    assert_eq!(review.proposals.as_slice(), &[proposal.identity()]);
    reject_foreign_proposal_custody(&proposal)?;
    let mut depot = ReplayDepot::default();
    let receipt = human_admit_replay(&proposal, proposal_custody, &mut depot)?;
    assert_eq!(receipt.entry().proposal(), proposal.identity());
    assert_eq!(receipt.entry().capsule(), &capsule);
    assert_eq!(receipt.replay_custody().replay(), receipt.entry().replay());
    assert_eq!(depot.entries.as_slice(), &[receipt.entry().clone()]);
    assert!(matches!(receipt.row().origin(), Origin::AdmittedReplay(_)));
    admit_row(receipt.row().clone())
}

/// The pin and the discharge walk the same offer → store → human road the kill walks, each on its own ground.
///
/// The duplicate gate is observed on both sides of it: a pin's comparison states its own vacancy, and a discharge over an already-recorded roster is refused at the offer rather than reviewed.
fn admit_pin_and_discharge(mutation: &MutationReport) -> Result<(), MutationRoadFailure> {
    let (candidate, demonstration) = demonstrate_mutation(mutation)?;
    let capsule = capture_demonstration(&demonstration)?;
    let destination = ProposalDestination::naming(
        ExecutionSuite::named(OWNER, "mutation-receiver").map_err(|_| MutationRoadFailure::Name)?,
    );
    let claim = candidate.claim();

    let delta = ProofDelta::between(0, 1).map_err(MutationRoadFailure::Delta)?;
    let pin = offer_claim_pin(
        candidate.clone(),
        claim,
        capsule.clone(),
        delta,
        destination,
    )
    .map_err(MutationRoadFailure::PinProposal)?;
    assert_eq!(
        pin.duplicate().reason(),
        NoComparisonReason::GroundCarriesNoFailure
    );
    let mut review = ReviewSink::default();
    let pin_custody = review
        .store(&pin)
        .map_err(MutationRoadFailure::ProposalSink)?;
    let mut depot = ReplayDepot::default();
    let pin_receipt = human_admit_replay(&pin, pin_custody, &mut depot)
        .map_err(MutationRoadFailure::Admission)?;
    assert!(matches!(
        pin_receipt.row().origin(),
        Origin::AdmittedReplay(_)
    ));
    assert_eq!(pin_receipt.entry().capsule(), &capsule);

    let trial = trial_identity(&candidate);
    let owed = OwedClaim::declared(claim, "uncovered-claim").map_err(MutationRoadFailure::Owed)?;
    let evidence = DischargeEvidence::recorded(
        ObligationLane::TestRow,
        trial,
        demonstration.trial_report().standing().key().clone(),
    );
    assert!(matches!(
        offer_obligation_discharge(
            candidate.clone(),
            owed,
            evidence.clone(),
            &[trial],
            destination,
        ),
        Err(DischargeProposalRefusal::Duplicate(
            DuplicateRefusal::ObligationAlreadyDischarged(recorded)
        )) if recorded == trial
    ));
    let discharge = offer_obligation_discharge(candidate, owed, evidence, &[], destination)
        .map_err(MutationRoadFailure::DischargeProposal)?;
    assert_eq!(discharge.duplicate().owed(), claim);
    let discharge_custody = review
        .store(&discharge)
        .map_err(MutationRoadFailure::ProposalSink)?;
    let receipt = human_admit_discharge(&discharge, discharge_custody)
        .map_err(MutationRoadFailure::Admission)?;
    assert!(matches!(
        receipt.row().origin(),
        Origin::AdmittedDischarge(_)
    ));
    admit_row(receipt.row().clone())
}

fn reject_foreign_proposal_custody(
    proposal: &impl ReplayBearingProposal,
) -> Result<(), MutationRoadFailure> {
    let foreign = ProposalId::over(ContentAddress::derived(
        REPLAY_SCHEMA_TAG,
        b"foreign-proposal",
    ));
    let custody = StoredProposalRef::at(foreign, "review://foreign")
        .map_err(MutationRoadFailure::ProposalSink)?;
    let mut depot = ReplayDepot::default();
    assert!(matches!(
        human_admit_replay(proposal, custody, &mut depot),
        Err(HumanAdmissionRefusal::ProposalCustodyMismatch { expected, found })
            if expected == proposal.identity() && found == foreign
    ));
    assert!(depot.entries.is_empty());
    Ok(())
}

fn admit_row(admitted: Row) -> Result<(), MutationRoadFailure> {
    let subject = admitted.subject();
    let check = admitted.check();
    let revision = RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"trial"));
    let binding = Binding::bound(
        admitted,
        ExecutableAttachment::attached(subject, check, revision, revision, candidate_trial_call),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)?;
    let world = TrialTable::authored(
        AuthoredTableName::named(OWNER, "admitted-world").map_err(|_| MutationRoadFailure::Name)?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)?;
    assert_eq!(world.bindings().len(), 1);
    Ok(())
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

fn compiled_suite_pressure() -> Result<CompiledSuitePressure, MutationRoadFailure> {
    let reading = compiled_reading()?;
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let qualification = AdapterQualification::of(&reading, GrammarStanding::Checked(version))?;
    Ok(CompiledSuitePressure::demonstrated(
        WrapStanding::Reported(&reading),
        &qualification,
    )?)
}

fn specimen_source(operation: &[u8]) -> Vec<u8> {
    let mut source = b"fn main() { let input: u32 = std::env::args().nth(1).expect(\"input\").parse().expect(\"u32\"); let a = 1u32; let b = 0u32; if ".to_vec();
    source.extend_from_slice(operation);
    source.extend_from_slice(b" { print!(\"1\"); } else { print!(\"0\"); } }\n");
    source
}

/// This admitted materializer implements both directive postures.
const SPECIMEN_MATERIALIZER: MaterializerFn = |directive| {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    let payload = directive.resolved().map_or(ORIGINAL_OPERATION, |resolved| {
        resolved.alternative().operation()
    });
    Ok(specimen_source(payload))
};

fn omitted_specimen_branch(
    directive: EvaluationDirective<'_>,
) -> Result<Vec<u8>, SpecimenMaterializerRefusal> {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    match directive.resolved() {
        Some(resolved) => Err(SpecimenMaterializerRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        )),
        None => Ok(specimen_source(ORIGINAL_OPERATION)),
    }
}

fn omitted_baseline_branch(
    directive: EvaluationDirective<'_>,
) -> Result<Vec<u8>, SpecimenMaterializerRefusal> {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    match directive.resolved() {
        None => Err(SpecimenMaterializerRefusal::NoMutationNotImplemented),
        Some(resolved) => Ok(specimen_source(resolved.alternative().operation())),
    }
}

/// This hostile materializer returns wrong but syntactically valid selected bytes.
const WRONG_SELECTED_SPECIMEN: MaterializerFn = |directive| {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    Ok(match directive.resolved() {
        None => specimen_source(ORIGINAL_OPERATION),
        Some(_) => specimen_source(b"input > 0"),
    })
};

/// This hostile materializer returns byte-identical baseline and selected source.
const UNCHANGED_SPECIMEN_MATERIALIZER: MaterializerFn = |_directive| {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    Ok(specimen_source(ORIGINAL_OPERATION))
};

fn specimen_path(extension: &str) -> PathBuf {
    let ordinal = SPECIMEN_ORDINAL.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "macroonz_harness_specimen_{}_{ordinal}{extension}",
        std::process::id()
    ))
}

fn host_failure(error: &[u8]) -> CompiledSpecimenHostRefusal {
    CompiledSpecimenHostRefusal::Execution(ForeignText::admitted(error))
}

fn compilation_failure(error: &[u8]) -> CompiledSpecimenHostRefusal {
    CompiledSpecimenHostRefusal::Compilation(ForeignText::admitted(error))
}

/// The real host, as a `const` closure: the contract consumes each private-minted request so one call cannot reuse request custody, and the borrowing body below is the part with something to say.
const COMPILED_SPECIMEN_HOST: SpecimenHostFn = |request| specimen_hosted(&request);

/// Compile one specimen through the pinned toolchain, execute it, and read the meaning off its output.
fn specimen_hosted(
    request: &CompiledSpecimenRequest<'_, '_, [u32; 3]>,
) -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal> {
    SPECIMEN_HOST_CALLS.fetch_add(1, Ordering::SeqCst);
    let source = specimen_path(".rs");
    let executable = specimen_path(std::env::consts::EXE_SUFFIX);
    std::fs::write(&source, request.content().bytes())
        .map_err(|error| compilation_failure(error.to_string().as_bytes()))?;
    let target = request.execution().target();
    let compiled = Command::new("rustup")
        .arg("run")
        .arg(target.toolchain().spelling())
        .arg("rustc")
        .arg(&source)
        .arg("--edition=2024")
        .arg("--target")
        .arg(target.target().spelling())
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| compilation_failure(error.to_string().as_bytes()))?;
    drop(std::fs::remove_file(&source));
    if !compiled.status.success() {
        return Err(compilation_failure(&compiled.stderr));
    }
    let executed = Command::new(&executable)
        .arg(request.input()[0].to_string())
        .output()
        .map_err(|error| host_failure(error.to_string().as_bytes()))?;
    drop(std::fs::remove_file(&executable));
    if !executed.status.success() {
        return Err(host_failure(&executed.stderr));
    }
    if !request
        .content()
        .bytes()
        .windows(request.operation().len())
        .any(|window| window == request.operation())
    {
        return Err(CompiledSpecimenHostRefusal::Meaning(ForeignText::admitted(
            request.operation(),
        )));
    }
    let meaning = match executed.stdout.as_slice() {
        b"1" => CompiledRosterMeaning::Stated(1),
        b"0" => CompiledRosterMeaning::Unstated,
        other => {
            return Err(CompiledSpecimenHostRefusal::Meaning(ForeignText::admitted(
                other,
            )));
        }
    };
    Ok(CompiledSpecimenObservation::executed(request, meaning))
}

/// This hostile host retains only a prior observation, answering the selected role from its cache.
const CACHED_SIBLING_OBSERVATION_HOST: SpecimenHostFn = |request| sibling_cached(&request);

/// The cached-sibling body: the baseline call plants an observation, and the selected call answers with it.
fn sibling_cached(
    request: &CompiledSpecimenRequest<'_, '_, [u32; 3]>,
) -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal> {
    SPECIMEN_HOST_CALLS.fetch_add(1, Ordering::SeqCst);
    let mut cached = CACHED_SIBLING_OBSERVATION
        .lock()
        .map_err(|error| host_failure(error.to_string().as_bytes()))?;
    match request.role() {
        CompiledSpecimenRole::Baseline => {
            *cached = Some(CompiledSpecimenObservation::executed(
                request,
                CompiledRosterMeaning::Unstated,
            ));
            Ok(CompiledSpecimenObservation::executed(
                request,
                CompiledRosterMeaning::Stated(1),
            ))
        }
        CompiledSpecimenRole::Selected(_) => cached
            .take()
            .ok_or_else(|| host_failure(b"cached sibling observation absent")),
    }
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

/// Owner policy, point admission, and stable identities are independent of caller roster order.
#[test]
fn policy_admission_and_identity_are_structural() -> Result<(), MutationRoadFailure> {
    let evaluation_family = family("comparison-family")?;
    let first = surface_with(evaluation_family, vec![b"a <= b", b"a > b"])?;
    let reordered = surface_with(evaluation_family, vec![b"a > b", b"a <= b"])?;
    assert_eq!(first.identity(), reordered.identity());
    let first_point = first
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let reordered_point = reordered
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let first_ids = first_point
        .admitted_alternatives()
        .iter()
        .map(AdmittedAlternative::identity)
        .collect::<Vec<AlternativeId>>();
    let reordered_ids = reordered_point
        .admitted_alternatives()
        .iter()
        .map(AdmittedAlternative::identity)
        .collect::<Vec<AlternativeId>>();
    assert_eq!(first_ids, reordered_ids);

    let policy = policy(evaluation_family)?;
    let point_free = lower_discoveries(&policy, Vec::new())?.into_parts().1;
    assert_eq!(
        point_free.catalog_posture(),
        PointCatalogPosture::NoAdmittedPoints
    );
    assert!(point_free.selections().is_empty());
    assert_eq!(
        discovered_point(
            "empty-point",
            OwnerClaimMapping::Mapped(claim()?),
            Vec::new(),
        ),
        Err(MutationRoadFailure::Discovery(
            DiscoveryRefusal::NoAlternative
        ))
    );

    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let two_family_policy = MutationPolicy::declared(
        family("two-operator-family")?,
        vec![MutationPermission::declared(
            claim()?,
            vec![operator()?, boolean_family],
        )?],
    )?;
    let discovered = DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, "operator-identity")
            .map_err(|_| MutationRoadFailure::Name)?,
        OwnerClaimMapping::Mapped(claim()?),
        b"a < b".to_vec(),
        vec![
            AlternativeDeclaration::stated(operator()?, b"a > b".to_vec()),
            AlternativeDeclaration::stated(boolean_family, b"a > b".to_vec()),
        ],
        ActivationSite::named(OWNER, "operator-identity").map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let lowering = lower_discoveries(&two_family_policy, vec![discovered])?;
    let same_bytes_under_two_operators = lowering
        .surface()
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let first_alternative = same_bytes_under_two_operators
        .admitted_alternatives()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let last_alternative = same_bytes_under_two_operators
        .admitted_alternatives()
        .last()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    assert_ne!(first_alternative.identity(), last_alternative.identity());
    Ok(())
}

/// Complete discovery retains unmapped and unpermitted sites while admitting only the exact mapped subset.
#[test]
fn mutation_constructor_and_selection_boundaries_refuse_crossed_joins()
-> Result<(), MutationRoadFailure> {
    let first_family = family("constructor-family")?;
    let first_policy = policy(first_family)?;
    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let mapped = discovered_point(
        "first-point",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a <= b"],
    )?;
    let owner_unmapped = discovered_point(
        "owner-unmapped-point",
        OwnerClaimMapping::OwnerUnmapped,
        vec![b"a >= b"],
    )?;
    let unpermitted_family = DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, "foreign-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
        OwnerClaimMapping::Mapped(claim()?),
        b"a < b".to_vec(),
        vec![AlternativeDeclaration::stated(
            boolean_family,
            b"true".to_vec(),
        )],
        ActivationSite::named(OWNER, "foreign-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let another_claim =
        ClaimRef::named(OWNER, "unpermitted-claim").map_err(|_| MutationRoadFailure::Name)?;
    let unpermitted_claim = discovered_point(
        "foreign-claim-point",
        OwnerClaimMapping::Mapped(another_claim),
        vec![b"a == b"],
    )?;
    let mapped_ref = mapped.identity();
    let unmapped_ref = owner_unmapped.identity();
    let family_ref = unpermitted_family.identity();
    let claim_ref = unpermitted_claim.identity();
    let lowering = lower_discoveries(
        &first_policy,
        vec![
            mapped.clone(),
            owner_unmapped,
            unpermitted_family,
            unpermitted_claim,
        ],
    )?;
    let entries = lowering.discovery().entries();
    let [mapped_entry, unmapped_entry, family_entry, claim_entry] = entries else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(
        mapped_entry.disposition(),
        DiscoveryDisposition::Mapped { point: mapped_ref }
    );
    assert_eq!(
        unmapped_entry.disposition(),
        DiscoveryDisposition::OwnerUnmapped
    );
    assert_eq!(
        family_entry.disposition(),
        DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Family {
                at: 0,
                family: boolean_family,
            },
        }
    );
    assert_eq!(
        claim_entry.disposition(),
        DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Claim(another_claim),
        }
    );
    let first_surface = lowering.surface();
    let [admitted_point] = first_surface.points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let [admitted] = admitted_point.admitted_alternatives() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let admitted_alternative = admitted.identity();
    assert!(matches!(
        first_surface.select(unmapped_ref, admitted_alternative),
        Err(SelectionRefusal::NoSuchPoint(found))
            if found == unmapped_ref
    ));
    assert!(matches!(
        first_surface.select(family_ref, admitted_alternative),
        Err(SelectionRefusal::NoSuchPoint(found))
            if found == family_ref
    ));
    assert!(matches!(
        first_surface.select(claim_ref, admitted_alternative),
        Err(SelectionRefusal::NoSuchPoint(found))
            if found == claim_ref
    ));
    Ok(())
}

/// One unpermitted candidate withholds a discovered site's entire mixed alternative roster.
#[test]
fn mixed_discovery_rosters_are_admitted_all_or_nothing() -> Result<(), MutationRoadFailure> {
    let policy = policy(family("mixed-roster-family")?)?;
    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let site = DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, "mixed-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
        OwnerClaimMapping::Mapped(claim()?),
        b"a != b".to_vec(),
        vec![
            AlternativeDeclaration::stated(operator()?, b"a == b".to_vec()),
            AlternativeDeclaration::stated(boolean_family, b"true".to_vec()),
        ],
        ActivationSite::named(OWNER, "mixed-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let point = site.identity();
    let lowering = lower_discoveries(&policy, vec![site])?;
    assert!(matches!(
        lowering.discovery().entries(),
        [entry] if entry.disposition() == DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Family {
                at: 1,
                family: boolean_family,
            },
        }
    ));
    assert!(lowering.surface().points().is_empty());
    assert!(
        lowering
            .surface()
            .points()
            .iter()
            .all(|found| found.identity() != point)
    );
    Ok(())
}

/// Surface selection and pair construction refuse absent points, crossed alternatives, and foreign families.
#[test]
fn selection_and_pair_boundaries_refuse_crossed_joins() -> Result<(), MutationRoadFailure> {
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

    let other_family = family("another-constructor-family")?;
    let other_surface = surface_with(other_family, vec![b"a <= b"])?;
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"evaluation"));
    assert!(matches!(
        EvaluationPair::paired(
            ProductionBinding::declared(first_family, production_revision, production),
            EvaluationBinding::declared(&other_surface, evaluation_revision, EVALUATION),
            same,
        ),
        Err(EvaluationPairRefusal::FamilyMismatch {
            production,
            evaluation,
        }) if production == first_family && evaluation == other_family
    ));
    Ok(())
}

/// A missing materializer branch and a byte-identical selected rendering refuse before any compiler host runs.
#[test]
fn exact_projection_requires_one_real_selected_artifact() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("compiled-artifact-boundary")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, evaluation_reads_resolved_payload)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;

    let baseline_omitted = SpecimenMaterializerBinding::bound(&pair, omitted_baseline_branch);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &baseline_omitted,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::BaselineMaterialization(
            SpecimenMaterializerRefusal::NoMutationNotImplemented,
        ))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let omitted = SpecimenMaterializerBinding::bound(&pair, omitted_specimen_branch);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &omitted,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::SelectedMaterialization(
            SpecimenMaterializerRefusal::ActiveSelectionNotImplemented(found),
        )) if found == selection
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let unchanged = SpecimenMaterializerBinding::bound(&pair, UNCHANGED_SPECIMEN_MATERIALIZER);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &unchanged,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::ArtifactDidNotChange(_))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let wrong_selected = SpecimenMaterializerBinding::bound(&pair, WRONG_SELECTED_SPECIMEN);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &wrong_selected,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::SelectedHost(
            CompiledSpecimenHostRefusal::Meaning(_),
        ))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);

    Ok(())
}

/// A lawful observation cached from the baseline request cannot impersonate the selected request.
#[test]
fn host_observations_must_join_the_current_specimen_request() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("compiled-observation-boundary")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    *CACHED_SIBLING_OBSERVATION
        .lock()
        .map_err(|_| MutationRoadFailure::NativeToolchain)? = None;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            CACHED_SIBLING_OBSERVATION_HOST,
        ),
        Err(CompiledProjectionRefusal::SelectedObservation(
            CompiledSpecimenObservationMismatch::Content { expected, found },
        )) if expected.address()
                == ContentAddress::derived(ARTIFACT_CONTENT_TAG, &specimen_source(b"a <= b"))
            && found.address()
                == ContentAddress::derived(ARTIFACT_CONTENT_TAG, &specimen_source(ORIGINAL_OPERATION))
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    Ok(())
}

/// Exact projection pressure requires the qualified execution, a passing compiled baseline, and rejection of the selected compiled behavior.
#[test]
fn exact_projection_requires_both_compiled_witness_outcomes() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let baseline_family = family("compiled-baseline-outcome")?;
    let baseline_surface = surface_with(baseline_family, vec![SELECTED_OPERATION])?;
    let baseline_pair = pair(baseline_family, &baseline_surface, EVALUATION)?;
    let baseline_input = [0u32, 0, 0];
    let baseline_standing = qualify_no_mutation(observe_no_mutation(
        &baseline_pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &baseline_input,
        &invocation()?,
    )?);
    let baseline_qualification =
        baseline_standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let baseline_selection = active_selection(&baseline_surface)?;
    let baseline_materializer =
        SpecimenMaterializerBinding::bound(&baseline_pair, SPECIMEN_MATERIALIZER);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &baseline_surface,
            baseline_qualification,
            &baseline_materializer,
            baseline_selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::BaselineDidNotQualify)
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 1);

    let surviving_family = family("compiled-selected-survives")?;
    let surviving_surface = surface_with(surviving_family, vec![b"input > 0"])?;
    let surviving_pair = pair(surviving_family, &surviving_surface, EVALUATION)?;
    let surviving_input = [1u32, 0, 0];
    let surviving_standing = qualify_no_mutation(observe_no_mutation(
        &surviving_pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &surviving_input,
        &invocation()?,
    )?);
    let surviving_qualification =
        surviving_standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let surviving_selection = active_selection(&surviving_surface)?;
    let surviving_materializer =
        SpecimenMaterializerBinding::bound(&surviving_pair, SPECIMEN_MATERIALIZER);

    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surviving_surface,
            surviving_qualification,
            &surviving_materializer,
            surviving_selection,
            &foreign_invocation(),
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::InvocationForAnotherExecution)
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surviving_surface,
            surviving_qualification,
            &surviving_materializer,
            surviving_selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::ProjectionDidNotReject)
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    let suite = compiled_suite_pressure()?;
    assert!(matches!(
        availability::<[u32; 3], CompiledRosterMeaning>(
            Some(&surviving_surface),
            Some(&suite),
            None,
        ),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledProjectionPressure,
        }
    ));
    Ok(())
}

/// A point-free surface may earn parity qualification but cannot mint selection-scoped compiled pressure or active trust.
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

/// Policy, alternative, and surface identities match independently framed owner facts.
#[test]
fn mutation_identity_preimages_are_independently_read() -> Result<(), MutationRoadFailure> {
    let family = family("identity-family")?;
    let policy = policy(family)?;
    let point = point(&policy, "identity-point", vec![b"a <= b", b"a > b"])?;

    let mut policy_preimage = Vec::new();
    push_name(&mut policy_preimage, family.name());
    encode_length(policy.permissions().len(), &mut policy_preimage);
    for permission in policy.permissions() {
        push_name(&mut policy_preimage, permission.owner_claim().name());
        encode_length(permission.admitted_families().len(), &mut policy_preimage);
        for admitted in permission.admitted_families() {
            encode_bytes(admitted.slug().as_bytes(), &mut policy_preimage);
        }
    }
    assert_eq!(
        policy.identity().address(),
        ContentAddress::derived(POLICY_READING_TAG, &policy_preimage)
    );

    for alternative in point.admitted_alternatives() {
        let mut alternative_preimage = Vec::new();
        push_name(&mut alternative_preimage, point.identity().name());
        encode_bytes(
            alternative.family().slug().as_bytes(),
            &mut alternative_preimage,
        );
        encode_bytes(alternative.operation(), &mut alternative_preimage);
        assert_eq!(
            alternative.identity().address(),
            ContentAddress::derived(ALTERNATIVE_READING_TAG, &alternative_preimage)
        );
    }

    let surface = lower_discoveries(
        &policy,
        vec![discovered_point(
            "identity-point",
            OwnerClaimMapping::Mapped(claim()?),
            vec![b"a <= b", b"a > b"],
        )?],
    )?
    .into_parts()
    .1;
    let mut surface_preimage = Vec::new();
    push_name(&mut surface_preimage, family.name());
    encode_bytes(
        policy.identity().address().as_bytes(),
        &mut surface_preimage,
    );
    encode_length(surface.points().len(), &mut surface_preimage);
    for surface_point in surface.points() {
        push_name(&mut surface_preimage, surface_point.identity().name());
        push_name(&mut surface_preimage, surface_point.owner_claim().name());
        encode_bytes(surface_point.original_operation(), &mut surface_preimage);
        push_name(
            &mut surface_preimage,
            surface_point.activation_site().name(),
        );
        encode_length(
            surface_point.admitted_alternatives().len(),
            &mut surface_preimage,
        );
        for alternative in surface_point.admitted_alternatives() {
            encode_bytes(
                alternative.identity().address().as_bytes(),
                &mut surface_preimage,
            );
            encode_bytes(
                alternative.family().slug().as_bytes(),
                &mut surface_preimage,
            );
            encode_bytes(alternative.operation(), &mut surface_preimage);
        }
    }
    assert_eq!(
        surface.identity().address(),
        ContentAddress::derived(SURFACE_READING_TAG, &surface_preimage)
    );

    Ok(())
}

/// Discovery identity preserves producer order while the admitted surface retains canonical point order.
#[test]
fn discovery_identity_and_surface_identity_keep_their_own_ordering()
-> Result<(), MutationRoadFailure> {
    let policy = policy(family("discovery-identity-family")?)?;
    let first = discovered_point(
        "producer-order-first",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a <= b"],
    )?;
    let second = discovered_point(
        "producer-order-second",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a >= b"],
    )?;
    let forward = lower_discoveries(&policy, vec![first.clone(), second.clone()])?;
    let reversed = lower_discoveries(&policy, vec![second, first])?;
    assert_eq!(
        forward.discovery().identity().address(),
        ContentAddress::derived(
            DISCOVERY_READING_TAG,
            &independently_frame_discovery(forward.discovery()),
        )
    );
    assert_ne!(
        forward.discovery().identity(),
        reversed.discovery().identity()
    );
    assert_eq!(forward.surface().identity(), reversed.surface().identity());
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

/// Generic cargo-mutants suite bite, an exact separately compiled projection, and parity open one selection-scoped interpreted execution.
#[test]
fn compiled_and_interpreted_evidence_join_without_flattening() -> Result<(), MutationRoadFailure> {
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
                && coordinate.line() == 360
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
    let trust = match availability(Some(&surface), Some(&suite), Some(&projection)) {
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

    admit_mutation(evidence.mutation())?;
    admit_pin_and_discharge(evidence.mutation())?;
    Ok(())
}

/// The same admitted report authority also preserves a surviving active execution instead of hard-coding every firing as a kill.
#[test]
fn active_classification_is_derived_from_the_admitted_report() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("surviving-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, ACTIVATION_SURVIVES)?;
    let input = [1u32, 0, 0];
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let standing =
        qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation()?)?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let suite = compiled_suite_pressure()?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let projection = demonstrate_compiled_projection(
        &surface,
        qualification,
        &materializer,
        selection,
        &invocation()?,
        COMPILED_SPECIMEN_HOST,
    )?;
    let trust = match availability(Some(&surface), Some(&suite), Some(&projection)) {
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
    let evidence = execute_active(&trust, &invocation()?)?;
    assert!(matches!(
        (evidence.report().attempt(), evidence.mutation().outcome()),
        (
            RunAttempt::Executed(TrialConclusion::Passed),
            MutationOutcome::Survived,
        )
    ));
    Ok(())
}

/// A passing trial conclusion cannot launder a no-mutation disagreement into parity qualification.
#[test]
fn no_mutation_agreement_must_be_earned() -> Result<(), MutationRoadFailure> {
    let family = family("parity-hostile-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, PARITY_BROKEN)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check_passes)?;
    let input = [1u32, 0, 0];
    let standing =
        qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation()?)?);
    let rejection = standing
        .rejection()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::MeaningsDisagreed,
        ))?;
    assert_eq!(
        rejection.cause(),
        ParityQualificationRefusal::MeaningsDisagreed
    );
    assert!(matches!(
        rejection.reading().conclusion(),
        TrialConclusion::Refused(_)
    ));
    Ok(())
}

/// Production and evaluation reports retain their roles, and production refusal has declared priority when both roles refuse.
#[test]
fn no_mutation_report_roles_and_refusal_priority_are_observed() -> Result<(), MutationRoadFailure> {
    let family = family("report-role-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, PARITY_BROKEN)?;
    let input = [1u32, 0, 0];

    let evaluation_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    assert!(matches!(
        evaluation_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::EvaluationDidNotQualify
    ));

    let production_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_evaluation_meaning)?,
        &input,
        &invocation()?,
    )?);
    assert!(matches!(
        production_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::ProductionDidNotQualify
    ));

    let both_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_refuses)?,
        &input,
        &invocation()?,
    )?);
    assert!(matches!(
        both_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::ProductionDidNotQualify
    ));
    Ok(())
}

/// No-mutation semantic agreement cannot qualify when the evaluation callable reports any activation.
#[test]
fn no_mutation_requires_zero_firings() -> Result<(), MutationRoadFailure> {
    let family = family("no-mutation-firing-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, NO_MUTATION_ACTIVATES)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing =
        qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation()?)?);
    let rejection = standing
        .rejection()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::NoMutationActivated { firings: 1 },
        ))?;
    assert_eq!(
        rejection.cause(),
        ParityQualificationRefusal::NoMutationActivated { firings: 1 }
    );
    assert_eq!(rejection.reading().evaluation_firings(), 1);
    Ok(())
}

/// Evaluation call refusals name whether the absent branch was no-mutation or one exact active selection.
#[test]
fn evaluation_call_refusals_preserve_directive_posture() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let no_mutation_family = family("no-mutation-branch-omitted")?;
    let no_mutation_surface = surface_with(no_mutation_family, vec![SELECTED_OPERATION])?;
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"evaluation"));
    let no_mutation_pair = EvaluationPair::paired(
        ProductionBinding::declared(no_mutation_family, production_revision, production_ordered),
        EvaluationBinding::declared(
            &no_mutation_surface,
            evaluation_revision,
            no_mutation_branch_omitted,
        ),
        same,
    )?;
    let input = [1u32, 0, 0];
    NO_MUTATION_CALL_ORDER.store(0, Ordering::SeqCst);
    assert!(matches!(
        observe_no_mutation(
            &no_mutation_pair,
            MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
            &input,
            &invocation()?,
        ),
        Err(NoMutationObservationRefusal::EvaluationCall(
            EvaluationCallRefusal::NoMutationNotImplemented,
        ))
    ));
    assert_eq!(NO_MUTATION_CALL_ORDER.load(Ordering::SeqCst), 2);

    let active_family = family("active-branch-omitted")?;
    let active_surface = surface_with(active_family, vec![SELECTED_OPERATION])?;
    let active_pair = pair(active_family, &active_surface, active_branch_omitted)?;
    let standing = qualify_no_mutation(observe_no_mutation(
        &active_pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&active_surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&active_pair, SPECIMEN_MATERIALIZER);
    let projection = demonstrate_compiled_projection(
        &active_surface,
        qualification,
        &materializer,
        selection,
        &invocation()?,
        COMPILED_SPECIMEN_HOST,
    )?;
    let suite = compiled_suite_pressure()?;
    let trust = match availability(Some(&active_surface), Some(&suite), Some(&projection)) {
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
    assert!(matches!(
        execute_active(&trust, &invocation()?),
        Err(InterpretedExecutionRefusal::EvaluationCall(
            EvaluationCallRefusal::ActiveSelectionNotImplemented(found),
        )) if found == selection
    ));
    Ok(())
}

/// A selected alternative that reports zero firings yields the exact dud and no admitted evidence.
#[test]
fn an_unfired_selection_is_not_mutation_evidence() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("dud-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, ACTIVATION_MISSING)?;
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
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let projection = demonstrate_compiled_projection(
        &surface,
        qualification,
        &materializer,
        selection,
        &invocation()?,
        COMPILED_SPECIMEN_HOST,
    )?;
    let trust = match availability(Some(&surface), Some(&suite), Some(&projection)) {
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
    assert!(matches!(
        execute_active(&trust, &invocation()?),
        Err(InterpretedExecutionRefusal::DudPlant(dud)) if dud.selection() == selection
    ));
    Ok(())
}

/// Exact compiled pressure cannot cross its issuing surface or owner claim, and either invalid join reaches no materializer or host code.
#[test]
fn active_execution_keeps_surface_claim_and_witness_together() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("claim-bound-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, evaluation_counted)?;
    let foreign_witness =
        MutationWitness::bound(trial_binding_for("another-behaviour")?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        foreign_witness,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let selection = active_selection(&surface)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
    let expected_claim = claim()?;
    let foreign_claim =
        ClaimRef::named(OWNER, "another-behaviour").map_err(|_| MutationRoadFailure::Name)?;
    CLAIM_MISMATCH_EVALUATION_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::WitnessForAnotherClaim { expected, found })
            if expected == expected_claim && found == foreign_claim
    ));
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);

    let local_standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation()?,
    )?);
    let local_qualification =
        local_standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let foreign_surface = surface_with(family, vec![b"a >= b"])?;
    let foreign_selection = active_selection(&foreign_surface)?;
    let expected_surface = surface.identity();
    let found_surface = foreign_surface.identity();
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            local_qualification,
            &materializer,
            foreign_selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::Selection(
            SelectionRefusal::SelectionFromAnotherSurface { expected, found },
        )) if expected == expected_surface && found == found_surface
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);
    Ok(())
}

/// A meaning-check callable cannot be placed under a different check identity than the exact trial row declares.
#[test]
fn a_mutation_witness_keeps_its_check_identity_and_callable_together()
-> Result<(), MutationRoadFailure> {
    let expected = check_ref()?;
    let found = CheckRef::named(OWNER, "another-check").map_err(|_| MutationRoadFailure::Name)?;
    assert!(matches!(
        MutationWitness::bound(trial_binding()?, found, check),
        Err(MutationWitnessRefusal::CheckMismatch {
            expected: refusal_expected,
            found: refusal_found,
        }) if refusal_expected == expected && refusal_found == found
    ));
    Ok(())
}

/// Generic cargo-mutants suite pressure carries no evaluation-family or pair authority and cannot open exact trust alone.
#[test]
fn generic_suite_pressure_cannot_open_exact_pair_trust() -> Result<(), MutationRoadFailure> {
    let surface = surface_with(family("local-family")?, vec![b"a <= b"])?;
    let suite = compiled_suite_pressure()?;
    assert!(matches!(
        availability::<[u32; 3], CompiledRosterMeaning>(Some(&surface), Some(&suite), None),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledProjectionPressure,
        }
    ));
    Ok(())
}

/// Exact projection pressure cannot cross a surface, and a materializer bound to another revision cannot be attached before execution.
#[test]
fn compiled_pressure_is_exact_pair_scoped() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("same-family-pair-scope")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let evaluation_pair = pair(family, &surface, EVALUATION)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &evaluation_pair,
        witness,
        &input,
        &invocation()?,
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
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

    let revision_pair = pair_with_evaluation_revision(
        family,
        &surface,
        EVALUATION,
        b"another-evaluation-revision",
    )?;
    assert_ne!(revision_pair.standing(), evaluation_pair.standing());
    let revision_materializer =
        SpecimenMaterializerBinding::bound(&revision_pair, SPECIMEN_MATERIALIZER);
    let selection = active_selection(&surface)?;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &revision_materializer,
            selection,
            &invocation()?,
            COMPILED_SPECIMEN_HOST,
        ),
        Err(CompiledProjectionRefusal::MaterializerForAnotherPair(
            EvaluationPairStandingMismatch::EvaluationRevision { expected, found },
        )) if expected == evaluation_pair.standing().evaluation_revision()
            && found == revision_pair.standing().evaluation_revision()
    ));
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 0);
    Ok(())
}

/// Adapter qualification remains bound to the exact backend profile whose reading earned it.
#[test]
fn a_compiled_witness_refuses_another_profile() -> Result<(), MutationRoadFailure> {
    let here = compiled_reading()?;
    let other_version = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    let elsewhere = read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(other_version.clone()),
        compiled_owner,
        compiled_family,
    )?;
    let borrowed = AdapterQualification::of(&elsewhere, GrammarStanding::Checked(other_version))?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(WrapStanding::Reported(&here), &borrowed),
        Err(SuitePressureRefusal::QualificationUnderAnotherProfile)
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
    let killed = compiled_reading()?;
    assert_eq!(killed.announced(), AnnouncedRoster::Stated(1));
    assert!(matches!(
        killed.unparsed(),
        [summary]
            if summary.ordinal() == 3
                && summary.text().bytes() == b"1 mutant tested: 1 caught"
    ));
    let killed_qualification =
        AdapterQualification::of(&killed, GrammarStanding::Checked(version.clone()))?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(WrapStanding::NotReported, &killed_qualification),
        Err(SuitePressureRefusal::WrapNotReported)
    );

    let missed = read_output(
        BACKEND_NO_KILL,
        BackendVersionPosture::Stated(version.clone()),
        compiled_owner,
        compiled_family,
    )?;
    let missed_qualification =
        AdapterQualification::of(&missed, GrammarStanding::Checked(version))?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(WrapStanding::Reported(&missed), &missed_qualification,),
        Err(SuitePressureRefusal::NoKillDemonstrated)
    );
    Ok(())
}

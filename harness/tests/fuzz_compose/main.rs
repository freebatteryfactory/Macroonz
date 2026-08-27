//! The fuzz home admits interesting bytes into Macroonz reduction and replay without owning the coverage engine.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, GeneratedSupportSchemaId, NamespacedName, Origin, PopulationRef, Provenance,
    RevisionBinding, Role, Row, SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz_harness::fuzz::{
    BackendSelection, BackendSelectionRefusal, ComposeRefusal, FRIDA_GUM_CRATE_PIN,
    FRIDA_GUM_WINDOWS_X86_64_DEVKIT, HostDisposition, InterestingBytes, InterestingBytesRefusal,
    LIBAFL_PIN, NamedCeiling, PreflightCapability, PreflightFact, PreflightIncomplete,
    PreflightStatus, SelectedBackend, compose_reduce_replay, preflight_ready,
};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionPlanRefusal, ReductionProbeBinding, ReductionRefusal,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, ReplayPosture, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialId,
    TrialProfile, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};
use std::fmt;

const PRESERVED_CAUSE: FindingCause = FindingCause::named("harness", "fuzz-compose-preserved");
const SCHEMA_TAG: DomainTag =
    DomainTag::declared("fuzz-compose-schema", IdentityProfileVersion::declared(1));

enum FuzzRoadFailure {
    Plan(ReductionPlanRefusal),
    Compose(ComposeRefusal),
    Selection(BackendSelectionRefusal),
    Interesting(InterestingBytesRefusal),
    Preflight(PreflightIncomplete),
    Fixture,
}

impl fmt::Debug for FuzzRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
            Self::Compose(refusal) => formatter.debug_tuple("Compose").field(refusal).finish(),
            Self::Selection(refusal) => formatter.debug_tuple("Selection").field(refusal).finish(),
            Self::Interesting(refusal) => {
                formatter.debug_tuple("Interesting").field(refusal).finish()
            }
            Self::Preflight(refusal) => formatter.debug_tuple("Preflight").field(refusal).finish(),
            Self::Fixture => formatter.write_str("Fixture"),
        }
    }
}

impl From<ReductionPlanRefusal> for FuzzRoadFailure {
    fn from(refusal: ReductionPlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

impl From<ComposeRefusal> for FuzzRoadFailure {
    fn from(refusal: ComposeRefusal) -> Self {
        Self::Compose(refusal)
    }
}

impl From<BackendSelectionRefusal> for FuzzRoadFailure {
    fn from(refusal: BackendSelectionRefusal) -> Self {
        Self::Selection(refusal)
    }
}

impl From<InterestingBytesRefusal> for FuzzRoadFailure {
    fn from(refusal: InterestingBytesRefusal) -> Self {
        Self::Interesting(refusal)
    }
}

impl From<PreflightIncomplete> for FuzzRoadFailure {
    fn from(refusal: PreflightIncomplete) -> Self {
        Self::Preflight(refusal)
    }
}

fn trial_fingerprint() -> Option<Fingerprint> {
    let coordinates = TrialCoordinates::over(
        ClaimRef::named("harness", "fuzz-compose").ok()?,
        SubjectRoute::named("harness", "byte-input").ok()?,
        CheckRef::named("harness", "fingerprint-preserved").ok()?,
        PopulationRef::named("harness", "fuzz-interesting").ok()?,
    );
    let key = TrialKey::over(coordinates);
    let trial = TrialId::of_key(key, TrialProfile::Unprofiled);
    Some(Fingerprint::over(
        trial,
        PRESERVED_CAUSE,
        FailureClass::PropertyDisagreement,
    ))
}

fn probe(input: &[u8]) -> ProbeOutcome {
    let Some(preserved) = trial_fingerprint() else {
        return ProbeOutcome::NoFailure;
    };
    match input {
        [1u8, 2u8, 3u8] | [1u8, 2u8] | [1u8] => ProbeOutcome::Reproduced(preserved),
        _ => ProbeOutcome::NoFailure,
    }
}

fn refused_trial(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        PRESERVED_CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

fn trial_binding() -> Option<TrialBinding> {
    let subject = SubjectRoute::named("harness", "byte-input").ok()?;
    let check = CheckRef::named("harness", "fingerprint-preserved").ok()?;
    let row = Row::declared(
        ClaimRef::named("harness", "fuzz-compose").ok()?,
        ExecutionSuite::named("harness", "fuzz").ok()?,
        Classification::authored(
            vec![Role::named("harness", "fuzz").ok()?],
            vec![Tag::named("harness", "compose").ok()?],
        )
        .ok()?,
        subject,
        check,
        PopulationRef::named("harness", "fuzz-interesting").ok()?,
        Origin::HandWritten,
    )
    .ok()?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(b"fuzz-compose-trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, refused_trial),
        Provenance::Unproduced,
    )
    .ok()
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("x86_64-pc-windows-msvc"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "fuzz-compose"),
        HarnessClock::unavailable(),
    )
}

fn probe_binding() -> Option<ReductionProbeBinding> {
    let trial = trial_binding()?;
    let report = run_one(&trial, &invocation());
    ReductionProbeBinding::bound(
        &report,
        GenerationProfile::declared("fuzz-interesting", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        RevisionBinding::derived(DerivedRevision::from_material(b"fuzz-compose-probe")),
        probe,
    )
    .ok()
}

fn all_available_facts() -> Vec<PreflightFact> {
    [
        PreflightCapability::VsWhere,
        PreflightCapability::VcVarsAll,
        PreflightCapability::ComposedMsvcSdkEnv,
        PreflightCapability::RustcMsrv,
        PreflightCapability::RustcHostTuple,
        PreflightCapability::RustcSysroot,
        PreflightCapability::RustcTargetLibdir,
        PreflightCapability::RustStdDll,
        PreflightCapability::LlvmReported,
        PreflightCapability::FridaGumLib,
        PreflightCapability::FridaGumHeader,
        PreflightCapability::FridaDevkitHash,
    ]
    .into_iter()
    .map(|capability| PreflightFact::declared(capability, PreflightStatus::Available))
    .collect()
}

#[test]
fn selection_pins_and_ceilings_match_f0_accept() -> Result<(), FuzzRoadFailure> {
    assert_eq!(LIBAFL_PIN, "0.16.1");
    assert_eq!(FRIDA_GUM_CRATE_PIN, "0.17.2");
    assert_eq!(FRIDA_GUM_WINDOWS_X86_64_DEVKIT, "17.9.5");
    let Some(name) = NamespacedName::named("harness", "f0-frida").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let ceilings = vec![
        NamedCeiling::Lnk4098Coexistence,
        NamedCeiling::LibAppendMsvcSdk,
        NamedCeiling::RustStdDllOnPath,
        NamedCeiling::LinuxMacOsUnexecutedUntilWaveF,
    ];
    let hosts = vec![
        HostDisposition::ObservedWindows,
        HostDisposition::CredibleUnexecutedLinux,
        HostDisposition::CredibleUnexecutedMacOs,
    ];
    let selection = BackendSelection::libafl_frida(name, ceilings.clone(), hosts.clone())?;
    assert_eq!(selection.backend(), SelectedBackend::LibAflFrida);
    assert_eq!(selection.ceilings(), ceilings.as_slice());
    assert_eq!(selection.hosts(), hosts.as_slice());
    Ok(())
}

#[test]
fn preflight_ready_requires_every_frida_windows_capability() -> Result<(), FuzzRoadFailure> {
    let ready = preflight_ready(SelectedBackend::LibAflFrida, &all_available_facts())?;
    assert_eq!(ready.backend(), SelectedBackend::LibAflFrida);
    let mut incomplete = all_available_facts();
    incomplete.pop();
    assert_eq!(
        preflight_ready(SelectedBackend::LibAflFrida, &incomplete),
        Err(PreflightIncomplete::Missing(
            PreflightCapability::FridaDevkitHash
        ))
    );
    Ok(())
}

#[test]
fn preflight_ready_rejects_duplicate_and_contradictory_facts() {
    let mut duplicate = all_available_facts();
    duplicate.push(PreflightFact::declared(
        PreflightCapability::VsWhere,
        PreflightStatus::Available,
    ));
    assert_eq!(
        preflight_ready(SelectedBackend::LibAflFrida, &duplicate),
        Err(PreflightIncomplete::Duplicate(
            PreflightCapability::VsWhere
        ))
    );
    let mut contradictory = all_available_facts();
    contradictory.push(PreflightFact::declared(
        PreflightCapability::VsWhere,
        PreflightStatus::Unavailable,
    ));
    assert_eq!(
        preflight_ready(SelectedBackend::LibAflFrida, &contradictory),
        Err(PreflightIncomplete::Contradictory(
            PreflightCapability::VsWhere
        ))
    );
}

#[test]
fn hostile_surface_refuses_malformed_fuzz_road() -> Result<(), FuzzRoadFailure> {
    let Some(name) = NamespacedName::named("harness", "f0-frida-hostile").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        BackendSelection::libafl_frida(name, Vec::new(), vec![HostDisposition::ObservedWindows]),
        Err(BackendSelectionRefusal::NoCeiling)
    );
    assert_eq!(
        BackendSelection::libafl_frida(name, vec![NamedCeiling::Lnk4098Coexistence], Vec::new()),
        Err(BackendSelectionRefusal::NoHostDisposition)
    );
    assert_eq!(
        BackendSelection::libafl_frida(
            name,
            vec![
                NamedCeiling::LibAppendMsvcSdk,
                NamedCeiling::RustStdDllOnPath,
                NamedCeiling::LinuxMacOsUnexecutedUntilWaveF,
            ],
            vec![
                HostDisposition::ObservedWindows,
                HostDisposition::CredibleUnexecutedLinux,
                HostDisposition::CredibleUnexecutedMacOs,
            ],
        ),
        Err(BackendSelectionRefusal::MissingRequiredCeiling(
            NamedCeiling::Lnk4098Coexistence
        ))
    );
    assert_eq!(
        BackendSelection::libafl_frida(
            name,
            vec![
                NamedCeiling::Lnk4098Coexistence,
                NamedCeiling::LibAppendMsvcSdk,
                NamedCeiling::RustStdDllOnPath,
                NamedCeiling::LinuxMacOsUnexecutedUntilWaveF,
            ],
            vec![
                HostDisposition::ObservedWindows,
                HostDisposition::CredibleUnexecutedMacOs,
            ],
        ),
        Err(BackendSelectionRefusal::MissingRequiredHost(
            HostDisposition::CredibleUnexecutedLinux
        ))
    );
    assert_eq!(
        BackendSelection::libafl_frida(
            name,
            vec![
                NamedCeiling::Lnk4098Coexistence,
                NamedCeiling::LibAppendMsvcSdk,
                NamedCeiling::LinuxMacOsUnexecutedUntilWaveF,
            ],
            vec![
                HostDisposition::ObservedWindows,
                HostDisposition::CredibleUnexecutedLinux,
                HostDisposition::CredibleUnexecutedMacOs,
            ],
        ),
        Err(BackendSelectionRefusal::MissingRequiredCeiling(
            NamedCeiling::RustStdDllOnPath
        ))
    );

    let mut unavailable = all_available_facts();
    let Some(first) = unavailable.get_mut(0) else {
        return Err(FuzzRoadFailure::Fixture);
    };
    *first = PreflightFact::declared(
        PreflightCapability::VsWhere,
        PreflightStatus::Unavailable,
    );
    assert_eq!(
        preflight_ready(SelectedBackend::LibAflFrida, &unavailable),
        Err(PreflightIncomplete::Unavailable(PreflightCapability::VsWhere))
    );

    assert_eq!(
        InterestingBytes::admitted(Vec::new()),
        Err(InterestingBytesRefusal::Empty)
    );

    let interesting = InterestingBytes::admitted(vec![9u8])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose-hostile", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(4),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    match compose_reduce_replay(&interesting, &plan, &binding) {
        Err(ComposeRefusal::Reduction(ReductionRefusal::BaselineDidNotFail)) => Ok(()),
        Err(refusal) => Err(FuzzRoadFailure::Compose(refusal)),
        Ok(_) => Err(FuzzRoadFailure::Fixture),
    }
}

#[test]
fn interesting_bytes_compose_into_exact_derived_replay() -> Result<(), FuzzRoadFailure> {
    assert_eq!(
        InterestingBytes::admitted(Vec::new()),
        Err(InterestingBytesRefusal::Empty)
    );
    let interesting = InterestingBytes::admitted(vec![1u8, 2u8, 3u8])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(16),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let capsule = compose_reduce_replay(&interesting, &plan, &binding)?;
    assert_eq!(capsule.input(), &[1u8]);
    assert_eq!(capsule.posture(), ReplayPosture::ExactDerived);
    assert_eq!(
        probe(capsule.input()),
        ProbeOutcome::Reproduced(capsule.fingerprint())
    );
    Ok(())
}

#[test]
fn compose_refuses_when_seed_does_not_fail() -> Result<(), FuzzRoadFailure> {
    let interesting = InterestingBytes::admitted(vec![9u8])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(4),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    match compose_reduce_replay(&interesting, &plan, &binding) {
        Err(ComposeRefusal::Reduction(ReductionRefusal::BaselineDidNotFail)) => Ok(()),
        Err(refusal) => Err(FuzzRoadFailure::Compose(refusal)),
        Ok(_) => Err(FuzzRoadFailure::Fixture),
    }
}

//! The fuzz home admits interesting bytes into Macroonz reduction and replay without owning the coverage engine.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::corpus::{SeedInput, pack, warm_start};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, GeneratedSupportSchemaId, NamespacedName, Origin, PopulationRef, Provenance,
    RevisionBinding, Role, Row, SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz_harness::fuzz::{
    BackendSelection, BackendSelectionRefusal, ComposeRefusal, CoverageAdmission,
    CoverageAdmissionRefusal, CoverageCorpus, CoveragePoint, CoverageReadRefusal, FuzzExecution,
    HostDisposition, InstrumentedTarget, InterestingBytes, InterestingBytesRefusal, MutationKind,
    MutationPlan, NamedCeiling, PreflightCapability, PreflightFact, PreflightIncomplete,
    PreflightStatus, RUSTC_COVERAGE_TOOLCHAIN, RustcCoverageTools, RustcProfileRefusal,
    RustcProfileRequest, SelectedBackend, compose_reduce_replay, neighboring_inputs,
    observe_rustc_profile, preflight_ready, read_lcov,
};
use macroonz_harness::generate::{
    ByteReducerId, ByteSource, CaseWidth, FingerprintPreservation, GenerationPlan, InputOrigin,
    PreconditionVerdict, ProbeOutcome, ReductionBudget, ReductionPlan, ReductionPlanRefusal,
    ReductionProbeBinding, ReductionRefusal, RejectionAllowance, SizeProgression, drive,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, ReplayPosture, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialId,
    TrialProfile, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};
use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

const PRESERVED_CAUSE: FindingCause = FindingCause::named("harness", "fuzz-compose-preserved");
const SCHEMA_TAG: DomainTag =
    DomainTag::declared("fuzz-compose-schema", IdentityProfileVersion::declared(1));

enum FuzzRoadFailure {
    Plan(ReductionPlanRefusal),
    Compose(ComposeRefusal),
    Selection(BackendSelectionRefusal),
    Interesting(InterestingBytesRefusal),
    Preflight(PreflightIncomplete),
    CoverageAdmission(CoverageAdmissionRefusal),
    CoverageRead(CoverageReadRefusal),
    Profile(RustcProfileRefusal),
    External(String),
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
            Self::CoverageAdmission(refusal) => formatter
                .debug_tuple("CoverageAdmission")
                .field(refusal)
                .finish(),
            Self::CoverageRead(refusal) => formatter
                .debug_tuple("CoverageRead")
                .field(refusal)
                .finish(),
            Self::Profile(refusal) => formatter.debug_tuple("Profile").field(refusal).finish(),
            Self::External(refusal) => formatter.debug_tuple("External").field(refusal).finish(),
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

impl From<CoverageAdmissionRefusal> for FuzzRoadFailure {
    fn from(refusal: CoverageAdmissionRefusal) -> Self {
        Self::CoverageAdmission(refusal)
    }
}

impl From<CoverageReadRefusal> for FuzzRoadFailure {
    fn from(refusal: CoverageReadRefusal) -> Self {
        Self::CoverageRead(refusal)
    }
}

impl From<RustcProfileRefusal> for FuzzRoadFailure {
    fn from(refusal: RustcProfileRefusal) -> Self {
        Self::Profile(refusal)
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
        PreflightCapability::RustcMsrv,
        PreflightCapability::RustcHostTuple,
        PreflightCapability::RustcSysroot,
        PreflightCapability::LlvmReported,
        PreflightCapability::LlvmToolsPreview,
        PreflightCapability::LlvmProfdata,
        PreflightCapability::LlvmCov,
        PreflightCapability::InstrumentCoverage,
    ]
    .into_iter()
    .map(|capability| PreflightFact::declared(capability, PreflightStatus::Available))
    .collect()
}

#[test]
fn selection_retains_rustc_ceiling_and_host_truth() -> Result<(), FuzzRoadFailure> {
    assert_eq!(RUSTC_COVERAGE_TOOLCHAIN, "1.98.0");
    let Some(name) = NamespacedName::named("harness", "rustc-coverage").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let ceilings = vec![
        NamedCeiling::FreshProcessPerCandidate,
        NamedCeiling::InstrumentedSourceTargetRequired,
        NamedCeiling::LlvmCoverageToolsRequired,
        NamedCeiling::CallerSuppliesProcessSupervisor,
    ];
    let hosts = vec![
        HostDisposition::ObservedWindows,
        HostDisposition::UnexecutedLinux,
        HostDisposition::UnexecutedMacOs,
    ];
    let selection = BackendSelection::rustc_coverage(name, ceilings.clone(), hosts.clone())?;
    assert_eq!(
        selection.backend(),
        SelectedBackend::RustcInstrumentCoverage
    );
    assert_eq!(selection.ceilings(), ceilings.as_slice());
    assert_eq!(selection.hosts(), hosts.as_slice());
    Ok(())
}

#[test]
fn preflight_ready_requires_every_rustc_coverage_capability() -> Result<(), FuzzRoadFailure> {
    let ready = preflight_ready(
        SelectedBackend::RustcInstrumentCoverage,
        &all_available_facts(),
    )?;
    assert_eq!(ready.backend(), SelectedBackend::RustcInstrumentCoverage);
    let mut incomplete = all_available_facts();
    incomplete.pop();
    assert_eq!(
        preflight_ready(SelectedBackend::RustcInstrumentCoverage, &incomplete),
        Err(PreflightIncomplete::Missing(
            PreflightCapability::InstrumentCoverage
        ))
    );
    Ok(())
}

#[test]
fn preflight_ready_rejects_duplicate_and_contradictory_facts() {
    let mut duplicate = all_available_facts();
    duplicate.push(PreflightFact::declared(
        PreflightCapability::RustcMsrv,
        PreflightStatus::Available,
    ));
    assert_eq!(
        preflight_ready(SelectedBackend::RustcInstrumentCoverage, &duplicate),
        Err(PreflightIncomplete::Duplicate(
            PreflightCapability::RustcMsrv
        ))
    );
    let mut contradictory = all_available_facts();
    contradictory.push(PreflightFact::declared(
        PreflightCapability::RustcMsrv,
        PreflightStatus::Unavailable,
    ));
    assert_eq!(
        preflight_ready(SelectedBackend::RustcInstrumentCoverage, &contradictory),
        Err(PreflightIncomplete::Contradictory(
            PreflightCapability::RustcMsrv
        ))
    );
}

#[test]
fn hostile_surface_refuses_malformed_fuzz_road() -> Result<(), FuzzRoadFailure> {
    let Some(name) = NamespacedName::named("harness", "rustc-coverage-hostile").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        BackendSelection::rustc_coverage(name, Vec::new(), vec![HostDisposition::ObservedWindows]),
        Err(BackendSelectionRefusal::NoCeiling)
    );
    assert_eq!(
        BackendSelection::rustc_coverage(
            name,
            vec![NamedCeiling::FreshProcessPerCandidate],
            Vec::new()
        ),
        Err(BackendSelectionRefusal::NoHostDisposition)
    );
    assert_eq!(
        BackendSelection::rustc_coverage(
            name,
            vec![
                NamedCeiling::InstrumentedSourceTargetRequired,
                NamedCeiling::LlvmCoverageToolsRequired,
                NamedCeiling::CallerSuppliesProcessSupervisor,
            ],
            vec![
                HostDisposition::ObservedWindows,
                HostDisposition::UnexecutedLinux,
                HostDisposition::UnexecutedMacOs,
            ],
        ),
        Err(BackendSelectionRefusal::MissingRequiredCeiling(
            NamedCeiling::FreshProcessPerCandidate
        ))
    );
    assert_eq!(
        BackendSelection::rustc_coverage(
            name,
            vec![
                NamedCeiling::FreshProcessPerCandidate,
                NamedCeiling::InstrumentedSourceTargetRequired,
                NamedCeiling::LlvmCoverageToolsRequired,
                NamedCeiling::CallerSuppliesProcessSupervisor,
            ],
            vec![
                HostDisposition::ObservedWindows,
                HostDisposition::UnexecutedMacOs,
            ],
        ),
        Err(BackendSelectionRefusal::MissingRequiredHost(
            HostDisposition::UnexecutedLinux
        ))
    );

    let mut unavailable = all_available_facts();
    let Some(first) = unavailable.get_mut(0) else {
        return Err(FuzzRoadFailure::Fixture);
    };
    *first = PreflightFact::declared(PreflightCapability::RustcMsrv, PreflightStatus::Unavailable);
    assert_eq!(
        preflight_ready(SelectedBackend::RustcInstrumentCoverage, &unavailable),
        Err(PreflightIncomplete::Unavailable(
            PreflightCapability::RustcMsrv
        ))
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
fn lcov_points_are_canonical_and_frontier_admission_is_deterministic() -> Result<(), FuzzRoadFailure>
{
    let alpha = read_lcov(
        b"TN:\nSF:src/subject.rs\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n",
    )?;
    assert_eq!(
        alpha.points(),
        &[
            CoveragePoint::Line {
                source: "src/subject.rs".to_owned(),
                line: 10,
            },
            CoveragePoint::Branch {
                source: "src/subject.rs".to_owned(),
                line: 12,
                block: 0,
                branch: 0,
            },
        ]
    );
    let beta = read_lcov(b"TN:\nSF:src/subject.rs\nDA:10,1\nDA:20,1\nend_of_record\n")?;
    let mut corpus = CoverageCorpus::opening();
    assert_eq!(
        corpus.admit(b"alpha".to_vec(), &alpha)?,
        CoverageAdmission::Interesting(InterestingBytes::admitted(b"alpha".to_vec())?)
    );
    assert_eq!(
        corpus.admit(b"alpha-repeat".to_vec(), &alpha)?,
        CoverageAdmission::Known
    );
    assert_eq!(
        corpus.admit(b"beta".to_vec(), &beta)?,
        CoverageAdmission::Interesting(InterestingBytes::admitted(b"beta".to_vec())?)
    );
    assert_eq!(corpus.observed().len(), 3);
    assert_eq!(corpus.interesting().len(), 2);
    Ok(())
}

#[test]
fn neighboring_frontier_is_bounded_unique_and_repeatable() -> Result<(), FuzzRoadFailure> {
    let plan = MutationPlan::declared(128, 16, vec![b"token".to_vec()]).map_err(external)?;
    let first = neighboring_inputs(&[11, 200], Some(b"peer"), &plan).map_err(external)?;
    let second = neighboring_inputs(&[11, 200], Some(b"peer"), &plan).map_err(external)?;
    assert_eq!(first, second);
    assert!(!first.is_empty());
    assert!(first.len() <= 128);
    assert!(
        first
            .iter()
            .all(|candidate| { !candidate.bytes().is_empty() && candidate.bytes().len() <= 16 })
    );
    let unique = first
        .iter()
        .map(|candidate| candidate.bytes().to_vec())
        .collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), first.len());
    for kind in [
        MutationKind::BitFlip,
        MutationKind::BoundarySubstitution,
        MutationKind::Increment,
        MutationKind::Decrement,
        MutationKind::Delete,
        MutationKind::InsertBoundary,
        MutationKind::Duplicate,
        MutationKind::Splice,
        MutationKind::DictionaryInsert,
    ] {
        assert!(first.iter().any(|candidate| candidate.kind() == kind));
    }
    Ok(())
}

#[test]
fn stable_rustc_profiles_cross_generation_novelty_and_corpus() -> Result<(), FuzzRoadFailure> {
    let (request, run) = rustc_profile_request("feedback")?;
    let Some(population) = PopulationRef::named("harness", "rustc-profile-seeds").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let seeds = vec![SeedInput::declared(vec![0]).map_err(external)?];
    let supplied = pack(population, seeds).map_err(external)?;
    let mut coverage = CoverageCorpus::opening();

    for origin in warm_start(&supplied) {
        let InputOrigin::Supplied(material) = origin else {
            return Err(FuzzRoadFailure::Fixture);
        };
        let width = CaseWidth::declared(material.len()).map_err(external)?;
        let bytes = u64::try_from(material.len()).map_err(external)?;
        let plan = GenerationPlan::declared(
            population,
            GenerationProfile::declared("rustc-profile-candidate", 1),
            InputOrigin::Supplied(material.clone()),
            CaseBudget::declared(1),
            ByteBudget::declared(bytes),
            RejectionAllowance::NoRejections,
            SizeProgression::Constant { width },
        )
        .map_err(external)?;
        let source = ByteSource::of_plan(&plan);
        let generated = drive::<u8>(
            &plan,
            &source,
            macroonz_harness::generate::decode_arbitrary::<u8>,
            admit_byte_sequences,
        );
        let [candidate] = generated.sequences() else {
            return Err(FuzzRoadFailure::Fixture);
        };
        assert_eq!(candidate.input(), material.as_slice());
        let result = observe_rustc_profile(&request, candidate.input(), 0, wait_for_exit)?;
        assert_eq!(result.execution(), FuzzExecution::Success);
        match coverage.admit(candidate.input().to_vec(), result.observation())? {
            CoverageAdmission::Interesting(_) => {}
            CoverageAdmission::Known => return Err(FuzzRoadFailure::Fixture),
        }
    }

    let mutation = MutationPlan::declared(8, 4, Vec::new()).map_err(external)?;
    let neighbors = neighboring_inputs(&[0], None, &mutation).map_err(external)?;
    let mut known = 0usize;
    for (ordinal, candidate) in neighbors.iter().enumerate() {
        let case = u64::try_from(ordinal.saturating_add(1)).map_err(external)?;
        let result = observe_rustc_profile(&request, candidate.bytes(), case, wait_for_exit)?;
        assert_eq!(result.execution(), FuzzExecution::Success);
        match coverage.admit(candidate.bytes().to_vec(), result.observation())? {
            CoverageAdmission::Interesting(_) => {}
            CoverageAdmission::Known => known = known.saturating_add(1),
        }
    }

    assert_eq!(neighbors.len(), 8);
    assert!(known > 0);
    assert_eq!(coverage.interesting().len(), 4);
    let evolved = coverage
        .interesting()
        .iter()
        .map(|interesting| SeedInput::declared(interesting.as_bytes().to_vec()).map_err(external))
        .collect::<Result<Vec<_>, _>>()?;
    let retained = pack(population, evolved).map_err(external)?;
    assert_eq!(retained.seeds().len(), 4);
    assert_eq!(
        retained
            .seeds()
            .iter()
            .map(SeedInput::bytes)
            .collect::<Vec<_>>(),
        vec![&[0][..], &[1][..], &[2][..], &[0x80][..]]
    );
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn declared_supervisor_transports_crash_timeout_and_resource_classes() -> Result<(), FuzzRoadFailure>
{
    let (request, run) = rustc_profile_request("classifications")?;
    let crash = observe_rustc_profile(&request, &[0xff], 100, wait_for_crash)?;
    assert!(matches!(crash.execution(), FuzzExecution::Crash(_)));
    assert!(crash.observation().points().is_empty());
    let timeout = observe_rustc_profile(&request, &[0xfe], 101, |child| {
        stop_as(child, FuzzExecution::Timeout)
    })?;
    assert_eq!(timeout.execution(), FuzzExecution::Timeout);
    assert!(timeout.observation().points().is_empty());
    let resource = observe_rustc_profile(&request, &[0xfe], 102, |child| {
        stop_as(child, FuzzExecution::ResourceExhaustion)
    })?;
    assert_eq!(resource.execution(), FuzzExecution::ResourceExhaustion);
    assert!(resource.observation().points().is_empty());
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

fn admit_byte_sequences(_commands: &[u8]) -> PreconditionVerdict {
    PreconditionVerdict::Admitted
}

fn rustc_profile_request(stem: &str) -> Result<(RustcProfileRequest, PathBuf), FuzzRoadFailure> {
    let version = successful_output(Command::new("rustc").arg("--version"), "rustc version")?;
    let version = String::from_utf8(version).map_err(external)?;
    if !version.starts_with("rustc 1.98.0 ") {
        return Err(FuzzRoadFailure::External(format!(
            "rustc profile crossing requires 1.98.0, found {version:?}"
        )));
    }
    let sysroot = successful_output(
        Command::new("rustc").args(["--print", "sysroot"]),
        "rustc sysroot",
    )?;
    let sysroot = PathBuf::from(String::from_utf8(sysroot).map_err(external)?.trim());
    let verbose = successful_output(Command::new("rustc").arg("-vV"), "rustc verbose version")?;
    let verbose = String::from_utf8(verbose).map_err(external)?;
    let host = verbose
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or_else(|| FuzzRoadFailure::External("rustc did not report its host".to_owned()))?;
    let tool_directory = sysroot.join("lib").join("rustlib").join(host).join("bin");
    let profdata = tool_directory.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX));
    let cov = tool_directory.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX));
    if !profdata.is_file() || !cov.is_file() {
        return Err(FuzzRoadFailure::External(
            "matching llvm-tools component is not installed".to_owned(),
        ));
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = manifest
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?;
    let run = repository
        .join("target")
        .join("qualification")
        .join(format!(
            "fuzz-rustc-profile-test-{}-{stem}",
            std::process::id()
        ));
    std::fs::create_dir_all(&run).map_err(external)?;
    let subject = run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ));
    let source = manifest
        .join("tests")
        .join("fuzz_compose")
        .join("rustc_coverage_subject.rs");
    let status = Command::new("rustc")
        .args([
            "--edition=2024",
            "-C",
            "instrument-coverage",
            "-C",
            "opt-level=0",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&subject)
        .status()
        .map_err(external)?;
    if !status.success() {
        return Err(FuzzRoadFailure::External(format!(
            "rustc coverage subject compilation failed with {status}"
        )));
    }
    let target = InstrumentedTarget::declared(subject, Vec::new()).map_err(external)?;
    let tools = RustcCoverageTools::declared(profdata, cov).map_err(external)?;
    let request =
        RustcProfileRequest::declared(target, tools, run.join("cases")).map_err(external)?;
    Ok((request, run))
}

fn wait_for_exit(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(FuzzExecution::Success)
    } else {
        Ok(FuzzExecution::NonzeroExit(status.code()))
    }
}

fn wait_for_crash(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Err("crash control exited successfully".to_owned())
    } else {
        Ok(FuzzExecution::Crash(status.code()))
    }
}

fn stop_as(
    child: &mut std::process::Child,
    execution: FuzzExecution,
) -> Result<FuzzExecution, String> {
    child.kill().map_err(|error| error.to_string())?;
    child.wait().map_err(|error| error.to_string())?;
    Ok(execution)
}

fn successful_output(command: &mut Command, role: &str) -> Result<Vec<u8>, FuzzRoadFailure> {
    let output = command.output().map_err(external)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(FuzzRoadFailure::External(format!(
            "{role} failed with {}",
            output.status
        )))
    }
}

fn external(error: impl fmt::Debug) -> FuzzRoadFailure {
    FuzzRoadFailure::External(format!("{error:?}"))
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

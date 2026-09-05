//! What every claim of the fuzz composition lane shares: the lane failure roster, the trial fixture roads, the campaign and request constructors, the compiled subject, the supervisors, and the interesting-bytes road.

use super::trial_fixture::TrialFixture;

use macroonz_harness::descriptor::{
    DerivedRevision, GeneratedSupportSchemaId, NamespacedName, PopulationRef, RevisionBinding,
};
use macroonz_harness::fuzz::{
    ComposeRefusal, CoverageAdmission, CoverageAdmissionRefusal, CoverageBudgets, CoverageCampaign,
    CoverageCorpus, CoverageProfile, CoverageReadRefusal, CoverageSourceRoot, FuzzExecution,
    InstrumentedTarget, InterestingBytes, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN,
    ReadyPreflight, RustcProfileRefusal, RustcProfileRequest, observe_rustc_profile,
    preflight_ready,
};
use macroonz_harness::generate::{
    PreconditionVerdict, ProbeOutcome, ReductionPlanRefusal, ReductionProbeBinding,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, TrialConclusion, TrialFinding, TrialSite,
};
use macroonz_harness::runner::Invocation;
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub(super) const PRESERVED_CAUSE: FindingCause =
    FindingCause::named("harness", "fuzz-compose-preserved");
pub(super) const SCHEMA_TAG: DomainTag =
    DomainTag::declared("fuzz-compose-schema", IdentityProfileVersion::declared(1));
pub(super) const SUPERVISED_MATERIALIZED_INPUT_BYTES: usize = 16_777_216;

pub(super) enum FuzzRoadFailure {
    Plan(ReductionPlanRefusal),
    Compose(ComposeRefusal),
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

pub(super) fn trial_fingerprint() -> Option<Fingerprint> {
    Some(trial_fixture()?.fingerprint(PRESERVED_CAUSE))
}

pub(super) fn probe(input: &[u8]) -> ProbeOutcome {
    let Some(preserved) = trial_fingerprint() else {
        return ProbeOutcome::NoFailure;
    };
    match input {
        [1u8, 2u8, 3u8] | [1u8, 2u8] | [1u8] => ProbeOutcome::Reproduced(preserved),
        _ => ProbeOutcome::NoFailure,
    }
}

pub(super) fn refused_trial(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        PRESERVED_CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

pub(super) fn trial_fixture() -> Option<TrialFixture> {
    TrialFixture::named(
        "fuzz-compose",
        "fuzz",
        "fuzz",
        "compose",
        "fuzz-interesting",
        TrialSite::located(module_path!(), file!(), line!(), "fuzz-compose"),
        super::trial_fixture::synthetic_target(),
    )
}

pub(super) fn probe_binding() -> Option<ReductionProbeBinding> {
    trial_fixture()?.probe_binding(
        refused_trial,
        RevisionBinding::derived(DerivedRevision::from_material(b"fuzz-compose-trial")),
        GenerationProfile::declared("fuzz-interesting", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        RevisionBinding::derived(DerivedRevision::from_material(b"fuzz-compose-probe")),
        probe,
    )
}

pub(super) fn admit_byte_sequences(_commands: &[u8]) -> PreconditionVerdict {
    PreconditionVerdict::Admitted
}

pub(super) fn coverage_campaign() -> Result<CoverageCampaign, FuzzRoadFailure> {
    let budgets = coverage_budgets(512, 33_554_432, 33_554_432, 1_000_000, 128, 1_048_576)?;
    coverage_campaign_with_budgets(budgets)
}

pub(super) fn coverage_budgets(
    executions: u32,
    input_bytes: u64,
    export_bytes: u64,
    points: u64,
    retained_cases: u32,
    retained_bytes: u64,
) -> Result<CoverageBudgets, FuzzRoadFailure> {
    CoverageBudgets::declared(
        CaseBudget::declared(executions),
        ByteBudget::declared(input_bytes),
        export_bytes,
        points,
        CaseBudget::declared(retained_cases),
        ByteBudget::declared(retained_bytes),
    )
    .map_err(external)
}

pub(super) fn coverage_campaign_with_budgets(
    budgets: CoverageBudgets,
) -> Result<CoverageCampaign, FuzzRoadFailure> {
    let Some(population) = PopulationRef::named("harness", "rustc-profile-candidates").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let Some(profile) = NamespacedName::named("harness", "rustc-region-coverage").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let revision = RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "rustc_coverage_subject.rs"
    )));
    Ok(CoverageCampaign::declared(
        population,
        revision,
        CoverageProfile::declared(profile, 1),
        budgets,
    ))
}

pub(super) fn alternate_coverage_campaign() -> Result<CoverageCampaign, FuzzRoadFailure> {
    let campaign = coverage_campaign()?;
    let Some(population) = PopulationRef::named("harness", "another-rustc-profile-campaign").ok()
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    Ok(CoverageCampaign::declared(
        population,
        campaign.revision(),
        campaign.profile(),
        campaign.budgets(),
    ))
}

pub(super) fn rustc_profile_request(
    stem: &str,
) -> Result<(ReadyPreflight, RunScratch), FuzzRoadFailure> {
    rustc_profile_request_with_arguments(stem, Vec::new())
}

pub(super) fn rustc_profile_request_with_arguments(
    stem: &str,
    arguments: Vec<String>,
) -> Result<(ReadyPreflight, RunScratch), FuzzRoadFailure> {
    rustc_profile_request_with_campaign(stem, arguments, coverage_campaign()?)
}

pub(super) fn rustc_profile_request_with_campaign(
    stem: &str,
    arguments: Vec<String>,
    campaign: CoverageCampaign,
) -> Result<(ReadyPreflight, RunScratch), FuzzRoadFailure> {
    let rustc = rustc_path()?;
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
    let run = RunScratch::created(run)?;
    let subject = run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ));
    let source = manifest
        .join("tests")
        .join("fuzz_compose")
        .join("rustc_coverage_subject.rs");
    compile_instrumented_subject(&rustc, &source, &subject)?;
    let target = InstrumentedTarget::declared(subject, arguments).map_err(external)?;
    let Some(logical) = NamespacedName::named("harness", "rustc-profile-subject").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let source_root =
        CoverageSourceRoot::declared(logical, repository.to_path_buf()).map_err(external)?;
    let request =
        RustcProfileRequest::declared(rustc, target, source_root, run.join("cases"), campaign)
            .map_err(external)?;
    let ready = preflight_ready(request)?;
    Ok((ready, run))
}

pub(super) fn compile_instrumented_subject(
    rustc: &std::path::Path,
    source: &std::path::Path,
    subject: &std::path::Path,
) -> Result<(), FuzzRoadFailure> {
    let status = Command::new(rustc)
        .args([
            "--edition=2024",
            "-C",
            "instrument-coverage",
            "-C",
            "opt-level=0",
        ])
        .arg(source)
        .arg("-o")
        .arg(subject)
        .status()
        .map_err(external)?;
    if !status.success() {
        return Err(FuzzRoadFailure::External(format!(
            "rustc coverage subject compilation failed with {status}"
        )));
    }
    Ok(())
}

pub(super) fn ready_for_compiled_root(
    rustc: PathBuf,
    subject: PathBuf,
    source_root: &std::path::Path,
    scratch: PathBuf,
    campaign: CoverageCampaign,
) -> Result<ReadyPreflight, FuzzRoadFailure> {
    let target = InstrumentedTarget::declared(subject, Vec::new()).map_err(external)?;
    let Some(logical) = NamespacedName::named("harness", "rustc-profile-subject").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let root =
        CoverageSourceRoot::declared(logical, source_root.to_path_buf()).map_err(external)?;
    let request =
        RustcProfileRequest::declared(rustc, target, root, scratch, campaign).map_err(external)?;
    preflight_ready(request).map_err(FuzzRoadFailure::Preflight)
}

pub(super) fn rebound_ready(
    _ready: &ReadyPreflight,
    run: &std::path::Path,
    scratch: &str,
    campaign: CoverageCampaign,
) -> Result<ReadyPreflight, FuzzRoadFailure> {
    let target =
        InstrumentedTarget::declared(profile_subject(run), Vec::new()).map_err(external)?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?
        .to_path_buf();
    let Some(logical) = NamespacedName::named("harness", "rustc-profile-subject").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let root = CoverageSourceRoot::declared(logical, repository).map_err(external)?;
    let request =
        RustcProfileRequest::declared(rustc_path()?, target, root, run.join(scratch), campaign)
            .map_err(external)?;
    preflight_ready(request).map_err(FuzzRoadFailure::Preflight)
}

pub(super) fn coverage_export_size(
    ready: &ReadyPreflight,
    run: &std::path::Path,
    candidate: &[u8],
) -> Result<u64, FuzzRoadFailure> {
    let probe = run.join("export-size-probe");
    std::fs::create_dir(&probe).map_err(external)?;
    let input_path = probe.join("candidate.bin");
    let raw = probe.join("coverage.profraw");
    let merged = probe.join("coverage.profdata");
    std::fs::write(&input_path, candidate).map_err(external)?;
    let input = File::open(&input_path).map_err(external)?;
    let subject = profile_subject(run);
    let status = Command::new(&subject)
        .env("LLVM_PROFILE_FILE", &raw)
        .stdin(Stdio::from(input))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(external)?;
    if !status.success() || !raw.is_file() {
        return Err(FuzzRoadFailure::External(
            "coverage-size probe did not produce a successful raw profile".to_owned(),
        ));
    }
    let tools = ready
        .sysroot()
        .join("lib")
        .join("rustlib")
        .join(ready.host())
        .join("bin");
    let profdata = tools.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX));
    let cov = tools.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX));
    let merge = Command::new(profdata)
        .arg("merge")
        .arg("-sparse")
        .arg(&raw)
        .arg("-o")
        .arg(&merged)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(external)?;
    if !merge.success() {
        return Err(FuzzRoadFailure::External(
            "coverage-size profile merge failed".to_owned(),
        ));
    }
    let mut profile = OsString::from("-instr-profile=");
    profile.push(merged.as_os_str());
    let output = Command::new(cov)
        .arg("export")
        .arg("-format=lcov")
        .arg(profile)
        .arg(subject)
        .output()
        .map_err(external)?;
    if !output.status.success() {
        return Err(FuzzRoadFailure::External(
            "coverage-size export failed".to_owned(),
        ));
    }
    let bytes = u64::try_from(output.stdout.len()).map_err(external)?;
    std::fs::remove_dir_all(probe).map_err(external)?;
    Ok(bytes)
}

pub(super) fn profile_subject(run: &std::path::Path) -> PathBuf {
    run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ))
}

pub(super) fn rustc_path() -> Result<PathBuf, FuzzRoadFailure> {
    let output = successful_output(
        Command::new("rustup").args(["which", "--toolchain", RUSTC_COVERAGE_TOOLCHAIN, "rustc"]),
        "rustup rustc path",
    )?;
    let path = PathBuf::from(String::from_utf8(output).map_err(external)?.trim());
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(FuzzRoadFailure::External(
            "rustup returned a relative rustc path".to_owned(),
        ))
    }
}

pub(super) fn rustc_field(rustc: &PathBuf, prefix: &str) -> Result<String, FuzzRoadFailure> {
    let output = successful_output(Command::new(rustc).arg("-vV"), "rustc identity")?;
    let text = String::from_utf8(output).map_err(external)?;
    text.lines()
        .find_map(|line| line.strip_prefix(prefix))
        .filter(|field| !field.is_empty())
        .map(str::to_owned)
        .ok_or(FuzzRoadFailure::Fixture)
}

pub(super) fn preflight_double(
    rustc: &PathBuf,
    manifest: &std::path::Path,
    directory: &std::path::Path,
    host: &str,
    versions: [&str; 4],
) -> Result<PathBuf, FuzzRoadFailure> {
    let [release, rustc_llvm, profdata_llvm, cov_llvm] = versions;
    std::fs::create_dir_all(directory).map_err(external)?;
    let executable = directory.join(format!("fake-rustc{}", std::env::consts::EXE_SUFFIX));
    let source = manifest
        .join("tests")
        .join("fuzz_compose")
        .join("rustc_preflight_subject.rs");
    let status = Command::new(rustc)
        .arg("--edition=2024")
        .arg(source)
        .arg("-o")
        .arg(&executable)
        .status()
        .map_err(external)?;
    if !status.success() {
        return Err(FuzzRoadFailure::External(format!(
            "preflight double compilation failed with {status}"
        )));
    }
    let sysroot = directory.join("sysroot");
    let tool_directory = sysroot.join("lib").join("rustlib").join(host).join("bin");
    std::fs::create_dir_all(&tool_directory).map_err(external)?;
    std::fs::write(directory.join("release.txt"), release).map_err(external)?;
    std::fs::write(directory.join("host.txt"), host).map_err(external)?;
    std::fs::write(directory.join("rustc-llvm.txt"), rustc_llvm).map_err(external)?;
    std::fs::write(
        directory.join("sysroot.txt"),
        sysroot.to_string_lossy().as_bytes(),
    )
    .map_err(external)?;
    std::fs::write(tool_directory.join("profdata-version.txt"), profdata_llvm).map_err(external)?;
    std::fs::write(tool_directory.join("cov-version.txt"), cov_llvm).map_err(external)?;
    std::fs::copy(
        &executable,
        tool_directory.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX)),
    )
    .map_err(external)?;
    std::fs::copy(
        &executable,
        tool_directory.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX)),
    )
    .map_err(external)?;
    Ok(executable)
}

pub(super) fn preflight_double_request(
    rustc: PathBuf,
    repository: &std::path::Path,
    run: &std::path::Path,
    stem: &str,
) -> Result<RustcProfileRequest, FuzzRoadFailure> {
    let target = InstrumentedTarget::declared(rustc.clone(), Vec::new()).map_err(external)?;
    let Some(logical) = NamespacedName::named("harness", "preflight-double").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let source_root =
        CoverageSourceRoot::declared(logical, repository.to_path_buf()).map_err(external)?;
    RustcProfileRequest::declared(
        rustc,
        target,
        source_root,
        run.join(stem).join("cases"),
        coverage_campaign()?,
    )
    .map_err(external)
}

pub(super) fn wait_for_exit(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(FuzzExecution::Success)
    } else {
        Ok(FuzzExecution::NonzeroExit(status.code()))
    }
}

pub(super) fn wait_for_crash(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Err("crash control exited successfully".to_owned())
    } else {
        Ok(FuzzExecution::Crash(status.code()))
    }
}

pub(super) fn stop_as(
    child: &mut std::process::Child,
    execution: FuzzExecution,
) -> Result<FuzzExecution, String> {
    child.kill().map_err(|error| error.to_string())?;
    child.wait().map_err(|error| error.to_string())?;
    Ok(execution)
}

#[cfg(windows)]
pub(super) fn process_is_running(process: u32) -> Result<bool, FuzzRoadFailure> {
    let filter = format!("PID eq {process}");
    let output = successful_output(
        Command::new("tasklist").args(["/FI", &filter, "/FO", "CSV", "/NH"]),
        "tasklist process observation",
    )?;
    let text = String::from_utf8(output).map_err(external)?;
    Ok(text.contains(&format!("\"{process}\"")))
}

#[cfg(unix)]
pub(super) fn process_is_running(process: u32) -> Result<bool, FuzzRoadFailure> {
    let output = Command::new("ps")
        .args(["-p", &process.to_string(), "-o", "pid="])
        .output()
        .map_err(external)?;
    let text = String::from_utf8(output.stdout).map_err(external)?;
    Ok(output.status.success() && text.trim() == process.to_string())
}

#[cfg(not(any(windows, unix)))]
pub(super) fn process_is_running(_process: u32) -> Result<bool, FuzzRoadFailure> {
    Err(FuzzRoadFailure::External(
        "this target has no external process observer".to_owned(),
    ))
}

pub(super) fn successful_output(
    command: &mut Command,
    role: &str,
) -> Result<Vec<u8>, FuzzRoadFailure> {
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

pub(super) fn external(error: impl fmt::Debug) -> FuzzRoadFailure {
    FuzzRoadFailure::External(format!("{error:?}"))
}

pub(super) fn interesting_bytes(
    stem: &str,
    candidate: &[u8],
) -> Result<InterestingBytes, FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request(stem)?;
    let mut coverage = CoverageCorpus::opening(&ready);
    let result = observe_rustc_profile(&ready, &mut coverage, candidate, wait_for_exit)?;
    let admission = coverage.admit(result);
    run.removed()?;
    match admission? {
        CoverageAdmission::Interesting(interesting) => Ok(interesting),
        CoverageAdmission::Known => Err(FuzzRoadFailure::Fixture),
    }
}
/// One run directory beneath the task qualification root, removed when the claim that opened it ends, however it ends.
///
/// A claim ends with [`RunScratch::removed`], which takes the path out of the custody before it drops, so a removal that fails is reported; a claim that refuses earlier drops the value, and the drop removes the directory without a report.
pub(super) struct RunScratch {
    path: PathBuf,
}

impl RunScratch {
    /// Create the run directory and take custody of it.
    pub(super) fn created(path: PathBuf) -> Result<Self, FuzzRoadFailure> {
        std::fs::create_dir_all(&path).map_err(external)?;
        Ok(Self { path })
    }

    /// The run directory.
    pub(super) fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// One seat beneath the run directory.
    pub(super) fn join(&self, seat: impl AsRef<std::path::Path>) -> PathBuf {
        self.path.join(seat)
    }

    /// Remove the run directory now, reporting a removal that failed.
    pub(super) fn removed(self) -> Result<(), FuzzRoadFailure> {
        let mut held = core::mem::ManuallyDrop::new(self);
        let path = core::mem::take(&mut held.path);
        std::fs::remove_dir_all(path).map_err(external)
    }
}

impl Drop for RunScratch {
    fn drop(&mut self) {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path).ok();
        }
    }
}

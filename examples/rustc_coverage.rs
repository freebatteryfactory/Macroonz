//! Runnable stable-rustc coverage composition through the Macroonz facade.

use macroonz::harness::clock::HarnessClock;
use macroonz::harness::corpus::{SeedInput, pack};
use macroonz::harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, GeneratedSupportSchemaId, NamespacedName, Origin, PopulationRef, Provenance,
    RevisionBinding, Role, Row, SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz::harness::fuzz::{
    CoverageAdmission, CoverageCorpus, CoverageSourceRoot, FuzzExecution, InstrumentedTarget,
    InterestingBytes, RUSTC_COVERAGE_TOOLCHAIN, RustcProfileRequest, compose_reduce_replay,
    observe_rustc_profile, preflight_ready,
};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding,
};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialId, TrialProfile, TrialSite,
};
use macroonz::harness::runner::{Invocation, TrialBinding, run_one};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

const PRESERVED_CAUSE: FindingCause = FindingCause::named("macroonz.example", "coverage-road");
const SCHEMA_TAG: DomainTag = DomainTag::declared(
    "rustc-coverage-example",
    IdentityProfileVersion::declared(1),
);

struct ExampleFailure(String);

impl fmt::Debug for ExampleFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn main() -> Result<(), ExampleFailure> {
    let rustc = declared_rustc()?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let run = run_directory();
    let Some(parent) = run.parent() else {
        return Err(ExampleFailure(
            "qualification run had no parent directory".to_owned(),
        ));
    };
    std::fs::create_dir_all(parent).map_err(failure)?;
    std::fs::create_dir(&run).map_err(failure)?;
    let subject = compile_subject(&rustc, &manifest, &run)?;
    let target = InstrumentedTarget::declared(subject, Vec::new()).map_err(failure)?;
    let logical = NamespacedName::named("macroonz.example", "rustc-coverage").map_err(failure)?;
    let source_root = CoverageSourceRoot::declared(logical, manifest).map_err(failure)?;
    let request = RustcProfileRequest::declared(rustc, target, source_root, run.join("cases"))
        .map_err(failure)?;
    let ready = preflight_ready(request).map_err(failure)?;

    let mut frontier = CoverageCorpus::opening();
    let baseline = observe_rustc_profile(&ready, &[0], 0, wait_for_exit).map_err(failure)?;
    if baseline.execution() != FuzzExecution::Success {
        return Err(ExampleFailure("baseline target did not succeed".to_owned()));
    }
    let first = frontier
        .admit(vec![0], baseline.observation())
        .map_err(failure)?;
    let CoverageAdmission::Interesting(first) = first else {
        return Err(ExampleFailure(
            "the opening candidate added no coverage".to_owned(),
        ));
    };

    let candidate = vec![1, 2, 3];
    let expanded = observe_rustc_profile(&ready, &candidate, 1, wait_for_exit).map_err(failure)?;
    let second = frontier
        .admit(candidate, expanded.observation())
        .map_err(failure)?;
    let CoverageAdmission::Interesting(second) = second else {
        return Err(ExampleFailure(
            "the second candidate added no coverage".to_owned(),
        ));
    };

    let repeated = observe_rustc_profile(&ready, &[0], 2, wait_for_exit).map_err(failure)?;
    if frontier
        .admit(vec![0], repeated.observation())
        .map_err(failure)?
        != CoverageAdmission::Known
    {
        return Err(ExampleFailure(
            "repeated coverage was admitted as novel".to_owned(),
        ));
    }

    retain_seed_pack(&first, &second)?;
    reduce_and_replay(&second)?;
    Ok(())
}

fn declared_rustc() -> Result<PathBuf, ExampleFailure> {
    let path = rustup_rustc()?;
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(ExampleFailure("rustc path was not absolute".to_owned()))
    }
}

fn rustup_rustc() -> Result<PathBuf, ExampleFailure> {
    let output = Command::new("rustup")
        .args(["which", "--toolchain", RUSTC_COVERAGE_TOOLCHAIN, "rustc"])
        .output()
        .map_err(failure)?;
    if !output.status.success() {
        return Err(ExampleFailure(format!(
            "rustup could not resolve stable Rust {RUSTC_COVERAGE_TOOLCHAIN}: {}",
            output.status
        )));
    }
    let text = String::from_utf8(output.stdout).map_err(failure)?;
    let path = text.trim();
    if path.is_empty() {
        Err(ExampleFailure("rustup returned no rustc path".to_owned()))
    } else {
        Ok(PathBuf::from(path))
    }
}

fn run_directory() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("target")
        .join("qualification")
        .join(format!("rustc-coverage-example-{}", std::process::id()))
}

fn compile_subject(rustc: &Path, manifest: &Path, run: &Path) -> Result<PathBuf, ExampleFailure> {
    let source = manifest
        .join("examples")
        .join("support")
        .join("rustc_coverage_subject.rs");
    let target = run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new(rustc)
        .args([
            "--edition=2024",
            "-C",
            "instrument-coverage",
            "-C",
            "opt-level=0",
        ])
        .arg(source)
        .arg("-o")
        .arg(&target)
        .output()
        .map_err(failure)?;
    if output.status.success() {
        Ok(target)
    } else {
        Err(ExampleFailure(format!(
            "instrumented subject compilation failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn wait_for_exit(child: &mut Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(FuzzExecution::Success)
    } else {
        Ok(FuzzExecution::NonzeroExit(status.code()))
    }
}

fn retain_seed_pack(
    first: &InterestingBytes,
    second: &InterestingBytes,
) -> Result<(), ExampleFailure> {
    let population = PopulationRef::named("macroonz.example", "coverage-seeds").map_err(failure)?;
    let seeds = vec![
        SeedInput::declared(first.as_bytes().to_vec()).map_err(failure)?,
        SeedInput::declared(second.as_bytes().to_vec()).map_err(failure)?,
    ];
    let retained = pack(population, seeds).map_err(failure)?;
    if retained.seeds().len() == 2 {
        Ok(())
    } else {
        Err(ExampleFailure(
            "coverage corpus did not retain both novel seeds".to_owned(),
        ))
    }
}

fn reduce_and_replay(interesting: &InterestingBytes) -> Result<(), ExampleFailure> {
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("rustc-coverage-example", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(16),
    )
    .map_err(failure)?;
    let binding = probe_binding()?;
    let capsule = compose_reduce_replay(interesting, &plan, &binding).map_err(failure)?;
    if capsule.input() == [1] {
        Ok(())
    } else {
        Err(ExampleFailure(
            "reduction did not reach the expected replay input".to_owned(),
        ))
    }
}

fn trial_fingerprint() -> Option<Fingerprint> {
    let coordinates = TrialCoordinates::over(
        ClaimRef::named("macroonz.example", "coverage-road").ok()?,
        SubjectRoute::named("macroonz.example", "byte-input").ok()?,
        CheckRef::named("macroonz.example", "fingerprint-preserved").ok()?,
        PopulationRef::named("macroonz.example", "coverage-seeds").ok()?,
    );
    let trial = TrialId::of_key(TrialKey::over(coordinates), TrialProfile::Unprofiled);
    Some(Fingerprint::over(
        trial,
        PRESERVED_CAUSE,
        FailureClass::PropertyDisagreement,
    ))
}

fn probe(input: &[u8]) -> ProbeOutcome {
    let Some(fingerprint) = trial_fingerprint() else {
        return ProbeOutcome::NoFailure;
    };
    match input {
        [1, 2, 3] | [1, 2] | [1] => ProbeOutcome::Reproduced(fingerprint),
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

fn probe_binding() -> Result<ReductionProbeBinding, ExampleFailure> {
    let trial = trial_binding()?;
    let report = run_one(&trial, &invocation());
    ReductionProbeBinding::bound(
        &report,
        GenerationProfile::declared("rustc-coverage-example", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        RevisionBinding::derived(DerivedRevision::from_material(b"coverage-example-probe")),
        probe,
    )
    .map_err(failure)
}

fn trial_binding() -> Result<TrialBinding, ExampleFailure> {
    let subject = SubjectRoute::named("macroonz.example", "byte-input").map_err(failure)?;
    let check = CheckRef::named("macroonz.example", "fingerprint-preserved").map_err(failure)?;
    let row = Row::declared(
        ClaimRef::named("macroonz.example", "coverage-road").map_err(failure)?,
        ExecutionSuite::named("macroonz.example", "fuzz").map_err(failure)?,
        Classification::authored(
            vec![Role::named("macroonz.example", "fuzz").map_err(failure)?],
            vec![Tag::named("macroonz.example", "coverage").map_err(failure)?],
        )
        .map_err(failure)?,
        subject,
        check,
        PopulationRef::named("macroonz.example", "coverage-seeds").map_err(failure)?,
        Origin::HandWritten,
    )
    .map_err(failure)?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(b"coverage-example"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, refused_trial),
        Provenance::Unproduced,
    )
    .map_err(failure)
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("rustc-coverage-example"),
            ToolchainIdentity::declared(RUSTC_COVERAGE_TOOLCHAIN),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "rustc-coverage-example"),
        HarnessClock::unavailable(),
    )
}

fn failure(error: impl fmt::Debug) -> ExampleFailure {
    ExampleFailure(format!("{error:?}"))
}

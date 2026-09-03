//! The preflight claims: declared inputs refuse ambient paths, active preflight refuses a wrong compiler or mismatched tools, and the declared supervisor transports every execution class.

use super::support::{
    FuzzRoadFailure, RunScratch, SUPERVISED_MATERIALIZED_INPUT_BYTES, coverage_campaign, external,
    preflight_double, preflight_double_request, process_is_running, rustc_field, rustc_path,
    rustc_profile_request, rustc_profile_request_with_arguments, stop_as, wait_for_crash,
};
use macroonz_harness::descriptor::NamespacedName;
use macroonz_harness::fuzz::{
    CoverageCorpus, CoverageSourceRoot, CoverageSourceRootRefusal, FuzzExecution,
    InstrumentedTarget, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN, RustcProfileRefusal,
    RustcProfileRequest, RustcProfileRequestRefusal, observe_rustc_profile, preflight_ready,
};
use std::cell::Cell;
use std::path::PathBuf;

#[test]
fn declared_execution_inputs_refuse_ambient_paths() -> Result<(), FuzzRoadFailure> {
    assert_eq!(RUSTC_COVERAGE_TOOLCHAIN, "1.98.0");
    assert_eq!(
        InstrumentedTarget::declared(PathBuf::new(), Vec::new()),
        Err(RustcProfileRequestRefusal::Target)
    );
    assert_eq!(
        InstrumentedTarget::declared(PathBuf::from("target"), Vec::new()),
        Err(RustcProfileRequestRefusal::RelativeTarget)
    );
    let Some(logical) = NamespacedName::named("harness", "rustc-coverage").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        CoverageSourceRoot::declared(logical, PathBuf::new()),
        Err(CoverageSourceRootRefusal::EmptyCheckout)
    );
    assert_eq!(
        CoverageSourceRoot::declared(logical, PathBuf::from("checkout")),
        Err(CoverageSourceRootRefusal::RelativeCheckout)
    );
    let traversing = std::env::temp_dir().join("coverage-root").join("..");
    assert_eq!(
        CoverageSourceRoot::declared(logical, traversing),
        Err(CoverageSourceRootRefusal::CheckoutTraversal)
    );

    let absolute = std::env::temp_dir();
    let target =
        InstrumentedTarget::declared(absolute.join("target"), Vec::new()).map_err(external)?;
    let source_root =
        CoverageSourceRoot::declared(logical, absolute.join("checkout")).map_err(external)?;
    let campaign = coverage_campaign()?;
    assert_eq!(
        RustcProfileRequest::declared(
            PathBuf::new(),
            target.clone(),
            source_root.clone(),
            absolute.join("scratch"),
            campaign,
        ),
        Err(RustcProfileRequestRefusal::Rustc)
    );
    assert_eq!(
        RustcProfileRequest::declared(
            PathBuf::from("rustc"),
            target.clone(),
            source_root.clone(),
            absolute.join("scratch"),
            campaign,
        ),
        Err(RustcProfileRequestRefusal::RelativeRustc)
    );
    assert_eq!(
        RustcProfileRequest::declared(
            absolute.join("rustc"),
            target.clone(),
            source_root.clone(),
            PathBuf::new(),
            campaign,
        ),
        Err(RustcProfileRequestRefusal::Scratch)
    );
    assert_eq!(
        RustcProfileRequest::declared(
            absolute.join("rustc"),
            target,
            source_root,
            PathBuf::from("scratch"),
            campaign,
        ),
        Err(RustcProfileRequestRefusal::RelativeScratch)
    );
    Ok(())
}

#[test]
fn active_preflight_refuses_wrong_release_and_mismatched_llvm() -> Result<(), FuzzRoadFailure> {
    let rustc = rustc_path()?;
    let host = rustc_field(&rustc, "host: ")?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = manifest
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?;
    let run = RunScratch::created(
        repository
            .join("target")
            .join("qualification")
            .join(format!(
                "fuzz-preflight-refusal-test-{}",
                std::process::id()
            )),
    )?;

    let wrong_release = preflight_double(
        &rustc,
        &manifest,
        &run.join("wrong-release"),
        &host,
        ["1.97.0", "22.1.8", "22.1.8", "22.1.8"],
    )?;
    let wrong_request =
        preflight_double_request(wrong_release, repository, run.path(), "wrong-release")?;
    let Err(wrong_refusal) = preflight_ready(wrong_request) else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        wrong_refusal,
        PreflightIncomplete::RustcRelease {
            required: RUSTC_COVERAGE_TOOLCHAIN,
            observed: "1.97.0".to_owned(),
        }
    );

    let mismatched_tools = preflight_double(
        &rustc,
        &manifest,
        &run.join("mismatched-tools"),
        &host,
        [RUSTC_COVERAGE_TOOLCHAIN, "22.1.8", "22.1.8", "22.1.9"],
    )?;
    let mismatch_request =
        preflight_double_request(mismatched_tools, repository, run.path(), "mismatched-tools")?;
    let Err(mismatch_refusal) = preflight_ready(mismatch_request) else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        mismatch_refusal,
        PreflightIncomplete::LlvmToolVersionsDiffer {
            profdata: "22.1.8".to_owned(),
            cov: "22.1.9".to_owned(),
        }
    );
    run.removed()?;
    Ok(())
}

#[test]
fn declared_supervisor_transports_crash_timeout_and_resource_classes() -> Result<(), FuzzRoadFailure>
{
    let (ready, run) = rustc_profile_request("classifications")?;
    let mut coverage = CoverageCorpus::opening(&ready);
    assert_eq!(ready.release(), RUSTC_COVERAGE_TOOLCHAIN);
    assert!(!ready.host().is_empty());
    assert!(!ready.llvm_version().is_empty());
    assert!(ready.sysroot().is_absolute());
    assert_eq!(ready.standing().target().target().spelling(), ready.host());
    assert_eq!(
        ready.standing().target().toolchain().spelling(),
        format!("rustc {} LLVM {}", ready.release(), ready.llvm_version())
    );
    let crash = observe_rustc_profile(&ready, &mut coverage, &[0xff], wait_for_crash)?;
    assert!(matches!(crash.execution(), FuzzExecution::Crash(_)));
    assert!(crash.observation().points().is_empty());
    let timeout = observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
        stop_as(child, FuzzExecution::Timeout)
    })?;
    assert_eq!(timeout.execution(), FuzzExecution::Timeout);
    assert!(timeout.observation().points().is_empty());
    let resource = observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
        stop_as(child, FuzzExecution::ResourceExhaustion)
    })?;
    assert_eq!(resource.execution(), FuzzExecution::ResourceExhaustion);
    assert!(resource.observation().points().is_empty());

    let early_process = Cell::new(None);
    assert_eq!(
        observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
            early_process.set(Some(child.id()));
            Ok(FuzzExecution::Timeout)
        }),
        Err(RustcProfileRefusal::SupervisorReturnedBeforeExit)
    );
    let Some(early_process) = early_process.get() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert!(!process_is_running(early_process)?);

    let refused_process = Cell::new(None);
    assert_eq!(
        observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
            refused_process.set(Some(child.id()));
            Err("planted supervisor refusal".to_owned())
        }),
        Err(RustcProfileRefusal::SuperviseTarget(
            "planted supervisor refusal".to_owned()
        ))
    );
    let Some(refused_process) = refused_process.get() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert!(!process_is_running(refused_process)?);

    let (large_input_ready, large_input_run) = rustc_profile_request_with_arguments(
        "large-input-supervision",
        vec!["--park-before-read".to_owned()],
    )?;
    let mut large_input_coverage = CoverageCorpus::opening(&large_input_ready);
    let large_input = observe_rustc_profile(
        &large_input_ready,
        &mut large_input_coverage,
        &vec![0_u8; SUPERVISED_MATERIALIZED_INPUT_BYTES],
        |child| stop_as(child, FuzzExecution::Timeout),
    )?;
    assert_eq!(large_input.execution(), FuzzExecution::Timeout);
    large_input_run.removed()?;
    run.removed()?;
    Ok(())
}

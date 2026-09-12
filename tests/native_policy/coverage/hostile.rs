use super::configure::{debug, request};
use crate::compiler::configure::{bounds, host, root, spelling};
use crate::compiler::types::Host;
use macroonz::harness::fuzz::{
    CoverageCommand, CoverageHostFailure, CoverageTool, FuzzExecution, RustcCommand,
    RustcProfileRefusal,
};
use macroonz::native_coverage::{self, NativeCoverageFailure, NativeCoverageProcessError};
use macroonz::native_process::{ProcessLimits, ProcessRun, ProcessStop, ProcessTool};
use std::path::{Path, PathBuf};
use std::time::Duration;

fn fixture(run: &Path, host: &Host) -> Result<PathBuf, String> {
    let executable = crate::compiler::refusal::standin(
        run,
        host,
        "coverage-tool",
        include_str!("tool_subject.rs"),
    )?;
    let directory = run.join("lib/rustlib").join(&host.triple).join("bin");
    std::fs::create_dir_all(&directory).map_err(debug)?;
    for name in ["llvm-profdata", "llvm-cov"] {
        std::fs::copy(
            &executable,
            directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX)),
        )
        .map_err(debug)?;
    }
    Ok(executable)
}

fn selection(
    run: &Path,
    host: &Host,
    executable: &Path,
    phase: &str,
    mode: &str,
) -> Result<ProcessTool, String> {
    let mut environment = host.environment.clone();
    environment.extend([
        ("COVERAGE_TEST_ROOT".to_owned(), spelling(run)?),
        ("COVERAGE_TEST_HOST".to_owned(), host.triple.clone()),
        ("COVERAGE_TEST_PHASE".to_owned(), phase.to_owned()),
        ("COVERAGE_TEST_MODE".to_owned(), mode.to_owned()),
    ]);
    ProcessTool::informed(
        executable.to_path_buf(),
        run.to_path_buf(),
        environment,
        ProcessLimits::informed(
            Duration::from_millis(500),
            Duration::from_secs(5),
            4096,
            3072,
        )
        .map_err(debug)?,
        &[],
    )
    .map_err(debug)
}

fn interrupted<R: std::fmt::Debug>(
    failure: &NativeCoverageFailure<R>,
    expected: CoverageCommand,
    mode: &str,
) -> Result<(), String> {
    let CoverageHostFailure::Executor {
        operation,
        error: NativeCoverageProcessError::Execution { request, run },
        cleanup: None,
    } = failure.cause()
    else {
        return Err(format!("wrong infrastructure failure: {failure}"));
    };
    assert_eq!(*operation, expected);
    let ProcessRun::Finished(output) = run.as_ref() else {
        return Err(format!("unfinished process: {failure}"));
    };
    assert_eq!(
        output.stop(),
        &if mode == "deadline" {
            ProcessStop::Deadline
        } else {
            ProcessStop::OutputLimit
        }
    );
    assert!(output.stdout().bytes().len() <= request.limits().stdout());
    assert!(output.stderr().bytes().len() <= request.limits().stderr());
    assert!(
        failure.to_string().len() < 512,
        "failure display must not dump retained output or environment"
    );
    Ok(())
}

#[test]
fn every_native_readiness_query_enforces_deadlines_and_both_capture_bounds() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let executable = fixture(&run, &host)?;
    for (phase, operation) in [
        (
            "verbose",
            CoverageCommand::Rustc(RustcCommand::VerboseVersion),
        ),
        ("sysroot", CoverageCommand::Rustc(RustcCommand::Sysroot)),
        (
            "profdata-version",
            CoverageCommand::Version(CoverageTool::Profdata),
        ),
        ("cov-version", CoverageCommand::Version(CoverageTool::Cov)),
    ] {
        for mode in ["deadline", "stdout", "stderr"] {
            let selected = selection(&run, &host, &executable, phase, mode)?;
            let Err(failure) = native_coverage::preflight(
                request(&run, &executable, &executable, "unused")?,
                selected,
                bounds()?,
            ) else {
                return Err("hostile readiness was accepted".to_owned());
            };
            interrupted(&failure, operation, mode)?;
        }
    }
    Ok(())
}

#[test]
fn llvm_merge_and_export_failures_spend_attempts_without_admitting_novelty() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let executable = fixture(&run, &host)?;
    for (phase, operation) in [
        ("merge", CoverageCommand::Merge),
        ("export", CoverageCommand::Export),
    ] {
        for mode in ["deadline", "stdout", "stderr", "failure"] {
            let cases = format!("{phase}-{mode}");
            let coverage = native_coverage::preflight(
                request(&run, &executable, &executable, &cases)?,
                selection(&run, &host, &executable, phase, mode)?,
                bounds()?,
            )
            .map_err(debug)?;
            let mut corpus = coverage.corpus();
            let Err(failure) = native_coverage::observe(&coverage, &mut corpus, &[0]) else {
                return Err("hostile LLVM was accepted".to_owned());
            };
            llvm_failure(&failure, operation, mode)?;
            assert_eq!(corpus.attempted_cases(), 1);
            assert!(corpus.interesting().is_empty());
            assert_eq!(
                std::fs::read_dir(run.join(&cases)).map_err(debug)?.count(),
                0
            );
        }
    }
    Ok(())
}

#[test]
fn native_target_timeout_output_and_exit_failures_cannot_earn_novelty() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let executable = fixture(&run, &host)?;
    for (mode, expected) in [
        ("deadline", FuzzExecution::Timeout),
        ("stdout", FuzzExecution::ResourceExhaustion),
        ("stderr", FuzzExecution::ResourceExhaustion),
        ("failure", FuzzExecution::NonzeroExit(Some(19_i32))),
    ] {
        let selected = selection(&run, &host, &executable, "target", mode)?;
        let limits = selected.limits();
        let coverage = native_coverage::preflight(
            request(&run, &executable, &executable, mode)?,
            selected,
            limits,
        )
        .map_err(debug)?;
        let mut corpus = coverage.corpus();
        let result = native_coverage::observe(&coverage, &mut corpus, &[0]).map_err(debug)?;
        assert_eq!(result.execution(), expected);
        assert!(result.observation().points().is_empty());
        assert!(corpus.admit(result).is_err());
        assert!(corpus.interesting().is_empty());
        assert_eq!(std::fs::read_dir(run.join(mode)).map_err(debug)?.count(), 0);
    }
    Ok(())
}

#[test]
fn missing_native_tools_report_the_exact_starting_role() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let executable = fixture(&run, &host)?;
    let directory = run.join("lib/rustlib").join(&host.triple).join("bin");
    let profdata = directory.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX));
    std::fs::remove_file(profdata).map_err(debug)?;
    let Err(failure) = native_coverage::preflight(
        request(&run, &executable, &executable, "unused")?,
        selection(&run, &host, &executable, "unused", "unused")?,
        bounds()?,
    ) else {
        return Err("missing LLVM was accepted".to_owned());
    };
    assert!(matches!(
        failure.cause(),
        CoverageHostFailure::Executor {
            operation: CoverageCommand::Version(CoverageTool::Profdata),
            error: NativeCoverageProcessError::Start { .. },
            cleanup: None
        }
    ));
    Ok(())
}

fn llvm_failure(
    failure: &NativeCoverageFailure<RustcProfileRefusal>,
    operation: CoverageCommand,
    mode: &str,
) -> Result<(), String> {
    if mode != "failure" {
        return interrupted(failure, operation, mode);
    }
    assert!(
        matches!(
            (failure.cause(), operation),
            (
                CoverageHostFailure::Refused(RustcProfileRefusal::ProfdataFailed(Some(19_i32))),
                CoverageCommand::Merge
            ) | (
                CoverageHostFailure::Refused(RustcProfileRefusal::CovFailed(Some(19_i32))),
                CoverageCommand::Export
            )
        ),
        "wrong tool failure: {failure}"
    );
    Ok(())
}

#[test]
fn the_campaign_export_ceiling_applies_below_the_native_capture_ceiling() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let executable = fixture(&run, &host)?;
    let campaign = super::configure::campaign_with_export(include_bytes!("tool_subject.rs"), 32)?;
    let request = super::configure::request_with_campaign(
        &run,
        &executable,
        &executable,
        "bounded-export",
        campaign,
    )?;
    let coverage = native_coverage::preflight(
        request,
        selection(&run, &host, &executable, "export", "stdout")?,
        bounds()?,
    )
    .map_err(debug)?;
    let mut corpus = coverage.corpus();
    let Err(failure) = native_coverage::observe(&coverage, &mut corpus, &[0]) else {
        return Err("unbounded export was accepted".to_owned());
    };
    interrupted(&failure, CoverageCommand::Export, "stdout")?;
    let CoverageHostFailure::Executor {
        error: NativeCoverageProcessError::Execution {
            run: process_run, ..
        },
        ..
    } = failure.cause()
    else {
        return Err(failure.to_string());
    };
    let ProcessRun::Finished(output) = process_run.as_ref() else {
        return Err(failure.to_string());
    };
    assert_eq!(output.stdout().bytes().len(), 32);
    assert!(corpus.interesting().is_empty());
    Ok(())
}

#[cfg(windows)]
#[test]
fn preflight_rejects_canonical_root_aliases_before_running_a_tool() -> Result<(), String> {
    use macroonz::harness::descriptor::NamespacedName;
    use macroonz::harness::fuzz::PreflightIncomplete;
    use macroonz::harness::fuzz::{
        CoverageRootsRefusal, CoverageSourceRoot, CoverageSourceRoots, InstrumentedTarget,
        RustcProfileRequest,
    };
    let run = root()?;
    let canonical = std::fs::canonicalize(&run).map_err(debug)?;
    let spelling = canonical.to_str().ok_or("non-Unicode source root")?;
    let ordinary = PathBuf::from(spelling.strip_prefix(r"\\?\").ok_or("no verbatim prefix")?);
    let roots = CoverageSourceRoots::declared(vec![
        CoverageSourceRoot::declared(
            NamespacedName::named("test", "first").map_err(debug)?,
            ordinary,
        )
        .map_err(debug)?,
        CoverageSourceRoot::declared(
            NamespacedName::named("test", "alias").map_err(debug)?,
            canonical,
        )
        .map_err(debug)?,
    ])
    .map_err(debug)?;
    let executable = run.join("unexecuted-file");
    std::fs::write(&executable, b"not an executable").map_err(debug)?;
    let request = RustcProfileRequest::mapped(
        executable.clone(),
        InstrumentedTarget::declared(executable, Vec::new()).map_err(debug)?,
        roots,
        run.join("cases"),
        super::configure::campaign(b"alias-control")?,
    )
    .map_err(debug)?;
    let mut called = false;
    let refused = macroonz::harness::fuzz::preflight_ready_with(request, |_| {
        called = true;
        Err(())
    });
    assert!(matches!(
        refused,
        Err(CoverageHostFailure::Refused(
            PreflightIncomplete::SourceRoots(CoverageRootsRefusal::OverlappingPaths {
                left: 0,
                right: 1
            })
        ))
    ));
    assert!(!called);
    Ok(())
}

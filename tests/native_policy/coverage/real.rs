use super::configure::{compiled, debug, ready};
use super::types::Instrumentation;
use crate::compiler::configure::{bounds, host, root};
use macroonz::harness::fuzz::{
    CoverageAdmission, CoverageHostFailure, FuzzExecution, RustcProfileRefusal,
};
use macroonz::native_coverage::{self, NativeCoverageProcessError};
use macroonz::native_process::{ProcessLimits, ProcessRun};
use std::time::Duration;

#[test]
fn actual_llvm_novelty_and_repeat_inputs_survive_source_relocation() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let mut previous = None;
    for name in ["first", "relocated"] {
        let directory = run.join(name);
        std::fs::create_dir(&directory).map_err(debug)?;
        let target = compiled(&directory, &host, Instrumentation::Selected)?;
        let coverage = ready(&directory, &host, &target, "cases", bounds()?)?;
        let mut corpus = coverage.corpus();
        let first = native_coverage::observe(&coverage, &mut corpus, &[0]).map_err(debug)?;
        assert_eq!(first.execution(), FuzzExecution::Success);
        assert!(!first.observation().points().is_empty());
        let points = first.observation().points().to_vec();
        if let Some(previous) = &previous {
            assert_eq!(previous, &points);
        }
        previous = Some(points);
        assert!(matches!(
            corpus.admit(first),
            Ok(CoverageAdmission::Interesting(_))
        ));
        let expanded =
            native_coverage::observe(&coverage, &mut corpus, &[1, 2, 3]).map_err(debug)?;
        assert!(matches!(
            corpus.admit(expanded),
            Ok(CoverageAdmission::Interesting(_))
        ));
        let repeated = native_coverage::observe(&coverage, &mut corpus, &[0]).map_err(debug)?;
        assert_eq!(corpus.admit(repeated), Ok(CoverageAdmission::Known));
        assert_eq!(corpus.attempted_cases(), 3);
        assert_eq!(corpus.interesting().len(), 2);
        assert_eq!(
            std::fs::read_dir(directory.join("cases"))
                .map_err(debug)?
                .count(),
            0
        );
    }
    Ok(())
}

#[test]
fn an_uninstrumented_target_cannot_supply_a_successful_coverage_observation() -> Result<(), String>
{
    let run = root()?;
    let host = host(&run)?;
    let target = compiled(&run, &host, Instrumentation::Absent)?;
    let coverage = ready(&run, &host, &target, "cases", bounds()?)?;
    let mut corpus = coverage.corpus();
    let Err(failure) = native_coverage::observe(&coverage, &mut corpus, &[0]) else {
        return Err("missing profile was accepted".to_owned());
    };
    assert!(matches!(
        failure.cause(),
        CoverageHostFailure::Refused(RustcProfileRefusal::MissingProfile)
    ));
    assert!(corpus.interesting().is_empty());
    assert_eq!(corpus.attempted_cases(), 1);
    assert_eq!(
        std::fs::read_dir(run.join("cases")).map_err(debug)?.count(),
        0
    );
    Ok(())
}

#[test]
fn pending_native_cleanup_keeps_case_input_until_the_child_owner_finishes() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let target = compiled(&run, &host, Instrumentation::Selected)?;
    let limits = ProcessLimits::informed(Duration::from_secs(5), Duration::ZERO, 1024, 1024)
        .map_err(debug)?;
    let coverage = ready(&run, &host, &target, "cases", limits)?;
    let mut corpus = coverage.corpus();
    let Err(failure) = native_coverage::observe(&coverage, &mut corpus, &[0]) else {
        return Err("zero cleanup budget did not retain ownership".to_owned());
    };
    let directory = match failure.cause() {
        CoverageHostFailure::Executor {
            error:
                NativeCoverageProcessError::Execution {
                    run: process_run, ..
                },
            cleanup: Some(cleanup),
            ..
        } if matches!(process_run.as_ref(), ProcessRun::Pending(_)) => {
            cleanup.directory().to_path_buf()
        }
        CoverageHostFailure::Refused(_) | CoverageHostFailure::Executor { .. } => {
            return Err(format!("unexpected failure: {failure}"));
        }
    };
    assert_eq!(
        std::fs::read(directory.join("candidate.bin")).map_err(debug)?,
        [0]
    );
    let finished = failure.finish_cleanup(Duration::from_secs(5));
    assert!(
        matches!(finished.cause(), CoverageHostFailure::Executor { error: NativeCoverageProcessError::Execution { run: process_run, .. }, cleanup: None, .. } if matches!(process_run.as_ref(), ProcessRun::Finished(_)))
    );
    assert_eq!(finished.cleanup_error(), None);
    assert!(!directory.exists());
    assert_eq!(corpus.attempted_cases(), 1);
    assert!(corpus.interesting().is_empty());
    Ok(())
}

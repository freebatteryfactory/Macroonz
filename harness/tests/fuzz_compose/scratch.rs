//! The scratch custody claims: a run directory does not outlive the claim that opened it, and a checked removal reports a failure instead of returning quietly.

use super::support::{FuzzRoadFailure, external, rustc_profile_request, wait_for_exit};
use macroonz_harness::fuzz::{CoverageCorpus, RustcProfileRefusal, observe_rustc_profile};
use std::path::PathBuf;

/// Claim: a claim that refuses before its checked removal leaves no run directory behind.
/// Subject: the run custody every request road hands a claim.
/// Population: one run whose first observation refuses at an existing case directory.
/// Hostile control: the run is dropped without its checked removal, exactly as an early `?` return drops it.
/// Denominator: the one run directory the request road created.
/// Evidence ceiling: a removal that fails on drop is not reported, which is why a claim's success path removes with the checked road.
#[test]
fn an_early_refusal_leaves_no_run_directory() -> Result<(), FuzzRoadFailure> {
    let observed: PathBuf = {
        let (ready, run) = rustc_profile_request("early-refusal")?;
        let case = run.join("cases").join("case-00000000000000000000");
        std::fs::create_dir_all(&case).map_err(external)?;
        let mut coverage = CoverageCorpus::opening(&ready);
        assert!(matches!(
            observe_rustc_profile(&ready, &mut coverage, &[0], wait_for_exit),
            Err(RustcProfileRefusal::CaseAlreadyExists(_))
        ));
        run.path().to_path_buf()
    };
    assert!(!observed.exists());
    Ok(())
}

/// Claim: the checked removal reports a directory it could not remove.
/// Subject: the checked removal road of the run custody.
/// Population: one run whose directory was removed beneath the custody before the checked removal.
/// Hostile control: a removal that swallowed the failure would answer with success here.
/// Denominator: the one refusal the checked road can raise.
#[test]
fn a_checked_removal_reports_a_directory_it_could_not_remove() -> Result<(), FuzzRoadFailure> {
    let (_ready, run) = rustc_profile_request("removed-beneath")?;
    std::fs::remove_dir_all(run.path()).map_err(external)?;
    assert!(matches!(run.removed(), Err(FuzzRoadFailure::External(_))));
    Ok(())
}

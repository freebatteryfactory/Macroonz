//! The budget claims: campaign join and execution budgets refuse before a process starts, an existing case directory keeps its refusal, export and point budgets refuse atomically, both exact ceilings are inclusive, and retention budgets refuse without advancing the frontier.

use super::support::{
    FuzzRoadFailure, alternate_coverage_campaign, coverage_budgets, coverage_campaign_with_budgets,
    coverage_export_size, external, rebound_ready, rustc_profile_request,
    rustc_profile_request_with_campaign, wait_for_exit,
};
use macroonz_harness::fuzz::{
    CoverageAdmission, CoverageAdmissionRefusal, CoverageCorpus, FuzzExecution,
    RustcProfileRefusal, observe_rustc_profile,
};

#[test]
fn campaign_join_and_execution_budgets_refuse_before_process_start() -> Result<(), FuzzRoadFailure>
{
    let (first_ready, first_run) = rustc_profile_request("campaign-join-first")?;
    let (other_ready, other_run) = rustc_profile_request_with_campaign(
        "campaign-join-other",
        Vec::new(),
        alternate_coverage_campaign()?,
    )?;
    let mut first = CoverageCorpus::opening(&first_ready);
    assert_eq!(
        observe_rustc_profile(&other_ready, &mut first, &[0], wait_for_exit),
        Err(RustcProfileRefusal::CampaignMismatch)
    );
    assert_eq!(first.attempted_cases(), 0);

    let case_campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 8, 33_554_432, 1_000_000, 1, 8)?)?;
    let (case_ready, case_run) =
        rustc_profile_request_with_campaign("case-budget", Vec::new(), case_campaign)?;
    let mut cases = CoverageCorpus::opening(&case_ready);
    let first_result = observe_rustc_profile(&case_ready, &mut cases, &[0], wait_for_exit)?;
    assert_eq!(first_result.execution(), FuzzExecution::Success);
    assert_eq!(
        observe_rustc_profile(&case_ready, &mut cases, &[1], wait_for_exit),
        Err(RustcProfileRefusal::CaseBudgetExhausted { bound: 1 })
    );

    let input_campaign =
        coverage_campaign_with_budgets(coverage_budgets(2, 1, 33_554_432, 1_000_000, 1, 8)?)?;
    let (input_ready, input_run) =
        rustc_profile_request_with_campaign("input-budget", Vec::new(), input_campaign)?;
    let mut inputs = CoverageCorpus::opening(&input_ready);
    assert_eq!(
        observe_rustc_profile(&input_ready, &mut inputs, &[0, 1], wait_for_exit),
        Err(RustcProfileRefusal::InputBudgetExhausted {
            bound: 1,
            attempted: 2,
        })
    );
    assert_eq!(inputs.attempted_cases(), 0);

    for run in [first_run, other_run, case_run, input_run] {
        run.removed()?;
    }
    Ok(())
}

#[test]
fn an_existing_case_directory_keeps_its_specific_refusal() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("existing-case-directory")?;
    let case = run.join("cases").join("case-00000000000000000000");
    std::fs::create_dir_all(&case).map_err(external)?;
    let mut coverage = CoverageCorpus::opening(&ready);
    assert_eq!(
        observe_rustc_profile(&ready, &mut coverage, &[0], wait_for_exit),
        Err(RustcProfileRefusal::CaseAlreadyExists(case))
    );
    assert_eq!(coverage.attempted_cases(), 1);
    run.removed()?;
    Ok(())
}

#[test]
fn coverage_export_and_point_budgets_refuse_atomically() -> Result<(), FuzzRoadFailure> {
    let export_campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, 1, 1_000_000, 1, 1)?)?;
    let (export_ready, export_run) =
        rustc_profile_request_with_campaign("export-budget", Vec::new(), export_campaign)?;
    let mut exports = CoverageCorpus::opening(&export_ready);
    let Err(RustcProfileRefusal::CovOutputBudgetExhausted {
        bound,
        observed_at_least,
    }) = observe_rustc_profile(&export_ready, &mut exports, &[0], wait_for_exit)
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(bound, 1);
    assert_eq!(observed_at_least, 2);
    assert!(
        std::fs::read_dir(export_run.join("cases"))
            .map_err(external)?
            .next()
            .is_none()
    );

    let point_campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, 33_554_432, 1, 1, 1)?)?;
    let (point_ready, point_run) =
        rustc_profile_request_with_campaign("point-budget", Vec::new(), point_campaign)?;
    let mut points = CoverageCorpus::opening(&point_ready);
    let result = observe_rustc_profile(&point_ready, &mut points, &[0], wait_for_exit)?;
    let attempted = u64::try_from(result.observation().points().len()).map_err(external)?;
    assert!(attempted > 1);
    assert_eq!(
        points.admit(result),
        Err(CoverageAdmissionRefusal::PointBudgetExhausted {
            bound: 1,
            attempted,
        })
    );
    assert!(points.observed().is_empty());
    assert!(points.interesting().is_empty());

    for run in [export_run, point_run] {
        run.removed()?;
    }
    Ok(())
}

#[test]
fn exact_coverage_export_byte_ceiling_is_inclusive() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("exact-export-bound")?;
    let export_bytes = coverage_export_size(&ready, run.path(), &[0])?;
    if export_bytes == 0 {
        return Err(FuzzRoadFailure::Fixture);
    }
    let campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, export_bytes, 1_000_000, 1, 1)?)?;
    let exact = rebound_ready(&ready, run.path(), "exact-export-cases", campaign)?;
    let mut coverage = CoverageCorpus::opening(&exact);
    let result = observe_rustc_profile(&exact, &mut coverage, &[0], wait_for_exit)?;
    assert_eq!(result.execution(), FuzzExecution::Success);
    assert!(!result.observation().points().is_empty());
    run.removed()?;
    Ok(())
}

#[test]
fn exact_coverage_point_ceiling_is_inclusive() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("exact-point-bound")?;
    let mut discovery = CoverageCorpus::opening(&ready);
    let observed = observe_rustc_profile(&ready, &mut discovery, &[0], wait_for_exit)?;
    let points = u64::try_from(observed.observation().points().len()).map_err(external)?;
    if points == 0 {
        return Err(FuzzRoadFailure::Fixture);
    }
    let campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, 33_554_432, points, 1, 1)?)?;
    let exact = rebound_ready(&ready, run.path(), "exact-point-cases", campaign)?;
    let mut coverage = CoverageCorpus::opening(&exact);
    let result = observe_rustc_profile(&exact, &mut coverage, &[0], wait_for_exit)?;
    assert!(matches!(
        coverage.admit(result)?,
        CoverageAdmission::Interesting(_)
    ));
    assert_eq!(
        u64::try_from(coverage.observed().len()).map_err(external)?,
        points
    );
    run.removed()?;
    Ok(())
}

#[test]
fn coverage_retention_budgets_refuse_without_advancing_the_frontier() -> Result<(), FuzzRoadFailure>
{
    let case_campaign =
        coverage_campaign_with_budgets(coverage_budgets(2, 2, 33_554_432, 1_000_000, 1, 8)?)?;
    let (case_ready, case_run) =
        rustc_profile_request_with_campaign("retained-case-budget", Vec::new(), case_campaign)?;
    let mut cases = CoverageCorpus::opening(&case_ready);
    let case_first = observe_rustc_profile(&case_ready, &mut cases, &[0], wait_for_exit)?;
    assert!(matches!(
        cases.admit(case_first)?,
        CoverageAdmission::Interesting(_)
    ));
    let case_points_before = cases.observed().clone();
    let case_second = observe_rustc_profile(&case_ready, &mut cases, &[1], wait_for_exit)?;
    assert_eq!(
        cases.admit(case_second),
        Err(CoverageAdmissionRefusal::RetainedCaseBudgetExhausted { bound: 1 })
    );
    assert_eq!(cases.observed(), &case_points_before);
    assert_eq!(cases.interesting().len(), 1);

    let byte_campaign =
        coverage_campaign_with_budgets(coverage_budgets(2, 3, 33_554_432, 1_000_000, 2, 1)?)?;
    let (byte_ready, byte_run) =
        rustc_profile_request_with_campaign("retained-byte-budget", Vec::new(), byte_campaign)?;
    let mut bytes = CoverageCorpus::opening(&byte_ready);
    let byte_first = observe_rustc_profile(&byte_ready, &mut bytes, &[0], wait_for_exit)?;
    assert!(matches!(
        bytes.admit(byte_first)?,
        CoverageAdmission::Interesting(_)
    ));
    let byte_points_before = bytes.observed().clone();
    let byte_second = observe_rustc_profile(&byte_ready, &mut bytes, &[1, 0], wait_for_exit)?;
    assert_eq!(
        bytes.admit(byte_second),
        Err(CoverageAdmissionRefusal::RetainedByteBudgetExhausted {
            bound: 1,
            attempted: 3,
        })
    );
    assert_eq!(bytes.observed(), &byte_points_before);
    assert_eq!(bytes.interesting().len(), 1);
    assert_eq!(bytes.retained_bytes(), 1);

    for run in [case_run, byte_run] {
        run.removed()?;
    }
    Ok(())
}

//! Declared failure values preserve distinctions that native success cannot exercise.

use crate::presentation_formats::{field, parsed};
use macroonz::harness::fuzz::{
    CoverageAdmissionRefusal, CoverageReadRefusal, FuzzExecution, PreflightIncomplete,
    RustcCommand, RustcProfileRefusal,
};
use macroonz::presentation;
use serde_json::{Value, json};

#[test]
fn coverage_cleanup_retains_the_original_refusal_and_every_cleanup_layer() -> Result<(), String> {
    let source = RustcProfileRefusal::CleanupCase {
        after: Some(Box::new(RustcProfileRefusal::CleanupCov {
            after: Box::new(RustcProfileRefusal::Coverage(
                CoverageReadRefusal::MalformedBranch { record: 7 },
            )),
            cleanup: "pipe | <cov> & \"wait\"\n".to_owned(),
        })),
        cleanup: "case <directory>".to_owned(),
    };
    let shown = parsed(&presentation::coverage_profile_refusal(&source))?;
    assert_eq!(field(&shown, "/record/phase")?, "profile-observation");
    assert_eq!(
        field(&shown, "/record/cause")?,
        &json!({
            "kind":"cleanup-case", "value": {
                "cleanup":"case <directory>", "after": {
                    "kind":"cleanup-cov", "value": {
                        "cleanup":"pipe | <cov> & \"wait\"\n", "after": {
                            "kind":"coverage", "value": {
                            "kind":"malformed-branch", "value":{"record":7_u64}
                            }
                        }
                    }
                }
            }
        })
    );
    let cleanup_only = parsed(&presentation::coverage_profile_refusal(
        &RustcProfileRefusal::CleanupCase {
            after: None,
            cleanup: String::new(),
        },
    ))?;
    assert_eq!(
        field(&cleanup_only, "/record/cause/value/after")?,
        &Value::Null
    );
    assert_eq!(field(&cleanup_only, "/record/cause/value/cleanup")?, "");
    Ok(())
}

#[test]
fn coverage_unknown_exit_zero_and_classification_are_not_interchangeable() -> Result<(), String> {
    for (execution, kind, code) in [
        (FuzzExecution::NonzeroExit(None), "nonzero-exit", None),
        (
            FuzzExecution::NonzeroExit(Some(0_i32)),
            "nonzero-exit",
            Some(0_i32),
        ),
        (FuzzExecution::Crash(Some(-9_i32)), "crash", Some(-9_i32)),
        (FuzzExecution::Timeout, "timeout", None),
        (
            FuzzExecution::ResourceExhaustion,
            "resource-exhaustion",
            None,
        ),
    ] {
        let shown = parsed(&presentation::coverage_admission_refusal(
            &CoverageAdmissionRefusal::Execution(execution),
        ))?;
        assert_eq!(field(&shown, "/record/phase")?, "frontier-admission");
        assert_eq!(
            field(&shown, "/record/cause/value")?,
            &json!({"kind":kind,"value":code})
        );
    }
    for code in [None, Some(0_i32), Some(19_i32)] {
        let shown = parsed(&presentation::coverage_preflight_refusal(
            &PreflightIncomplete::RustcFailed {
                command: RustcCommand::Sysroot,
                code,
            },
        ))?;
        assert_eq!(
            field(&shown, "/record/cause/value")?,
            &json!({"role":"sysroot","detail":code})
        );
    }
    Ok(())
}

#[test]
fn coverage_spent_work_bounds_and_lower_observation_bounds_stay_distinct() -> Result<(), String> {
    let input = parsed(&presentation::coverage_profile_refusal(
        &RustcProfileRefusal::InputBudgetExhausted {
            bound: 8,
            attempted: u64::MAX,
        },
    ))?;
    let export = parsed(&presentation::coverage_profile_refusal(
        &RustcProfileRefusal::CovOutputBudgetExhausted {
            bound: 8,
            observed_at_least: u64::MAX,
        },
    ))?;
    assert_eq!(
        field(&input, "/record/cause/value")?,
        &json!({"bound":8_u64,"attempted":u64::MAX})
    );
    assert_eq!(
        field(&export, "/record/cause/value")?,
        &json!({"bound":8_u64,"observed_at_least":u64::MAX})
    );
    for (refusal, kind, expected) in [
        (
            CoverageAdmissionRefusal::PointBudgetExhausted {
                bound: 8,
                attempted: 9,
            },
            "point-budget-exhausted",
            json!({"bound":8_u64,"attempted":9_u64}),
        ),
        (
            CoverageAdmissionRefusal::RetainedCaseBudgetExhausted { bound: 0 },
            "retained-case-budget-exhausted",
            json!(0_u32),
        ),
        (
            CoverageAdmissionRefusal::RetainedByteBudgetExhausted {
                bound: 8,
                attempted: 9,
            },
            "retained-byte-budget-exhausted",
            json!({"bound":8_u64,"attempted":9_u64}),
        ),
    ] {
        let shown = parsed(&presentation::coverage_admission_refusal(&refusal))?;
        assert_eq!(field(&shown, "/record/cause/kind")?, kind);
        assert_eq!(field(&shown, "/record/cause/value")?, &expected);
    }
    Ok(())
}

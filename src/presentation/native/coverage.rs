//! Native coverage policy, phase-specific failures and retained cleanup ownership.

use super::{process, refusal};
use crate::harness::fuzz::{CoverageHostFailure, PreflightIncomplete, RustcProfileRefusal};
use crate::native_coverage::{NativeCoverage, NativeCoverageFailure, NativeCoverageProcessError};
use crate::native_process::ProcessRun;
use crate::presentation::{
    Presentation, coverage,
    value::{array, object, optional, tagged},
};
use serde_json::Value;

/// Project established coverage readiness and its explicit native process policies.
pub fn native_coverage(record: &NativeCoverage) -> Presentation {
    let tool = record.tool();
    Presentation::projected(
        "native-coverage",
        "macroonz/native_coverage",
        "recorded",
        object([
            ("ready", coverage::readiness(record.ready())),
            (
                "tool",
                object([
                    ("executable", process::path(tool.executable())),
                    ("directory", process::path(tool.directory())),
                    (
                        "environment",
                        array(tool.environment().iter().map(|(key, value)| {
                            object([
                                ("key", key.as_str().into()),
                                ("value", value.as_str().into()),
                            ])
                        })),
                    ),
                    ("limits", process::limits(tool.limits())),
                ]),
            ),
            ("target_limits", process::limits(record.target_limits())),
        ]),
    )
}

/// Project native preflight failure without claiming readiness or completing cleanup.
pub fn coverage_preflight_error(
    record: &NativeCoverageFailure<PreflightIncomplete>,
) -> Presentation {
    Presentation::projected(
        "native-coverage-preflight-error",
        "macroonz/native_coverage",
        "recorded",
        failure(record, "preflight", coverage::preflight),
    )
}

/// Project a failed native coverage attempt without refunding work or completing cleanup.
pub fn coverage_profile_error(record: &NativeCoverageFailure<RustcProfileRefusal>) -> Presentation {
    Presentation::projected(
        "native-coverage-profile-error",
        "macroonz/native_coverage",
        "recorded",
        failure(record, "profile-observation", coverage::profile),
    )
}

fn failure<R>(
    record: &NativeCoverageFailure<R>,
    phase: &str,
    refused: impl FnOnce(&R) -> Value,
) -> Value {
    let cause = match record.cause() {
        CoverageHostFailure::Refused(value) => tagged("refused", refused(value)),
        CoverageHostFailure::Executor {
            operation,
            error,
            cleanup,
        } => tagged(
            "executor",
            object([
                ("operation", coverage::command(*operation)),
                ("error", process_error(error)),
                (
                    "case_cleanup",
                    optional(cleanup.as_ref(), |value| process::path(value.directory())),
                ),
            ]),
        ),
    };
    object([
        ("phase", phase.into()),
        ("cause", cause),
        (
            "cleanup_error",
            optional(record.cleanup_error(), Value::from),
        ),
    ])
}

fn process_error(record: &NativeCoverageProcessError) -> Value {
    match record {
        NativeCoverageProcessError::Invalid(error) => {
            tagged("invalid", refusal::process_error(error))
        }
        NativeCoverageProcessError::Start { request, error } => tagged(
            "start",
            object([
                ("request", process::request(request)),
                ("error", refusal::process_error(error)),
            ]),
        ),
        NativeCoverageProcessError::Execution { request, run } => {
            let result = match run.as_ref() {
                ProcessRun::Finished(value) => process::output(value),
                ProcessRun::Pending(value) => process::pending(value),
            };
            tagged(
                "execution",
                object([("request", process::request(request)), ("process", result)]),
            )
        }
    }
}

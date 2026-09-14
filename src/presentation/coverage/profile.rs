//! Coverage observation and frontier refusals preserve spent-work and cleanup distinctions.

use super::axes;
use crate::harness::fuzz::{CoverageAdmissionRefusal, CoverageReadRefusal, RustcProfileRefusal};
use crate::presentation::{
    Presentation,
    path::path,
    value::{object, optional, tagged},
};
use serde_json::Value;

/// Project the coverage owner's failed observation without a candidate result.
pub fn coverage_profile_refusal(record: &RustcProfileRefusal) -> Presentation {
    Presentation::projected(
        "coverage-profile-refusal",
        "macroonz-harness/fuzz",
        "recorded",
        object([
            ("phase", "profile-observation".into()),
            ("cause", profile(record)),
        ]),
    )
}

pub(in crate::presentation) fn profile(record: &RustcProfileRefusal) -> Value {
    match record {
        RustcProfileRefusal::UnexpectedExecutorReply => {
            tagged("unexpected-executor-reply", Value::Null)
        }
        RustcProfileRefusal::EmptyCandidate => tagged("empty-candidate", Value::Null),
        RustcProfileRefusal::CampaignMismatch => tagged("campaign-mismatch", Value::Null),
        RustcProfileRefusal::CaseBudgetExhausted { bound } => {
            tagged("case-budget-exhausted", (*bound).into())
        }
        RustcProfileRefusal::InputBudgetExhausted { bound, attempted } => {
            budget("input-budget-exhausted", *bound, *attempted)
        }
        RustcProfileRefusal::CaseAlreadyExists(at) => tagged("case-already-exists", path(at)),
        RustcProfileRefusal::CreateCase(value) => tagged("create-case", value.as_str().into()),
        RustcProfileRefusal::StartTarget(value) => tagged("start-target", value.as_str().into()),
        RustcProfileRefusal::WriteCandidate(value) => {
            tagged("write-candidate", value.as_str().into())
        }
        RustcProfileRefusal::OpenCandidate(value) => {
            tagged("open-candidate", value.as_str().into())
        }
        RustcProfileRefusal::SuperviseTarget(value) => {
            tagged("supervise-target", value.as_str().into())
        }
        RustcProfileRefusal::SupervisorReturnedBeforeExit => {
            tagged("supervisor-returned-before-exit", Value::Null)
        }
        RustcProfileRefusal::InspectTarget(value) => {
            tagged("inspect-target", value.as_str().into())
        }
        RustcProfileRefusal::CleanupTarget {
            after,
            cleanup: detail,
        } => cleanup("cleanup-target", Some(after.as_ref()), detail),
        RustcProfileRefusal::MissingProfile => tagged("missing-profile", Value::Null),
        RustcProfileRefusal::StartProfdata(value) => {
            tagged("start-profdata", value.as_str().into())
        }
        RustcProfileRefusal::ProfdataFailed(code) => tagged("profdata-failed", (*code).into()),
        RustcProfileRefusal::StartCov(value) => tagged("start-cov", value.as_str().into()),
        RustcProfileRefusal::CovOutputBudgetExhausted {
            bound,
            observed_at_least,
        } => tagged(
            "cov-output-budget-exhausted",
            object([
                ("bound", (*bound).into()),
                ("observed_at_least", (*observed_at_least).into()),
            ]),
        ),
        RustcProfileRefusal::ReadCov(value) => tagged("read-cov", value.as_str().into()),
        RustcProfileRefusal::WaitCov(value) => tagged("wait-cov", value.as_str().into()),
        RustcProfileRefusal::CovFailed(code) => tagged("cov-failed", (*code).into()),
        RustcProfileRefusal::CleanupCov {
            after,
            cleanup: detail,
        } => cleanup("cleanup-cov", Some(after.as_ref()), detail),
        RustcProfileRefusal::Coverage(value) => tagged("coverage", read(*value)),
        RustcProfileRefusal::CleanupCase {
            after,
            cleanup: detail,
        } => cleanup("cleanup-case", after.as_deref(), detail),
    }
}

fn cleanup(kind: &str, after: Option<&RustcProfileRefusal>, detail: &str) -> Value {
    tagged(
        kind,
        object([
            ("after", optional(after, profile)),
            ("cleanup", detail.into()),
        ]),
    )
}

fn budget(kind: &str, bound: u64, attempted: u64) -> Value {
    tagged(
        kind,
        object([("bound", bound.into()), ("attempted", attempted.into())]),
    )
}

fn read(value: CoverageReadRefusal) -> Value {
    let (kind, record) = match value {
        CoverageReadRefusal::NonUtf8 => return tagged("non-utf8", Value::Null),
        CoverageReadRefusal::AmbiguousSource { record } => ("ambiguous-source", record),
        CoverageReadRefusal::EmptySource { record } => ("empty-source", record),
        CoverageReadRefusal::RelativeSource { record } => ("relative-source", record),
        CoverageReadRefusal::SourceTraversal { record } => ("source-traversal", record),
        CoverageReadRefusal::SourceOutsideRoot { record } => ("source-outside-root", record),
        CoverageReadRefusal::EmptyRelativeSource { record } => ("empty-relative-source", record),
        CoverageReadRefusal::MissingSource { record } => ("missing-source", record),
        CoverageReadRefusal::MalformedLine { record } => ("malformed-line", record),
        CoverageReadRefusal::MalformedBranch { record } => ("malformed-branch", record),
    };
    tagged(kind, object([("record", record.into())]))
}

/// Project why a completed target result could not advance the novelty frontier.
pub fn coverage_admission_refusal(record: &CoverageAdmissionRefusal) -> Presentation {
    let cause = match record {
        CoverageAdmissionRefusal::CampaignMismatch => tagged("campaign-mismatch", Value::Null),
        CoverageAdmissionRefusal::Execution(value) => tagged("execution", axes::execution(*value)),
        CoverageAdmissionRefusal::EmptyObservation => tagged("empty-observation", Value::Null),
        CoverageAdmissionRefusal::PointBudgetExhausted { bound, attempted } => {
            budget("point-budget-exhausted", *bound, *attempted)
        }
        CoverageAdmissionRefusal::RetainedCaseBudgetExhausted { bound } => {
            tagged("retained-case-budget-exhausted", (*bound).into())
        }
        CoverageAdmissionRefusal::RetainedByteBudgetExhausted { bound, attempted } => {
            budget("retained-byte-budget-exhausted", *bound, *attempted)
        }
    };
    Presentation::projected(
        "coverage-admission-refusal",
        "macroonz-harness/fuzz",
        "recorded",
        object([("phase", "frontier-admission".into()), ("cause", cause)]),
    )
}

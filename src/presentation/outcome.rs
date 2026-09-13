//! Typed attempt and text projections without interpreting foreign prose.

use super::value::{hex, object, optional, tagged};
use crate::harness::report::{
    EmptySelectionReason, FailureClass, Fingerprint, ForeignText, InfrastructureFault,
    NotSelectedReason, RunAttempt, SelectionOutcome, SkipReason, TextFidelity, TrialConclusion,
    TrialFinding, TrialId, Truncation,
};
use serde_json::Value;

pub(super) fn selection(value: SelectionOutcome) -> Value {
    match value {
        SelectionOutcome::Satisfied => tagged("satisfied", Value::Null),
        SelectionOutcome::UnsatisfiedByEmptySelection => {
            tagged("unsatisfied-by-empty-selection", Value::Null)
        }
        SelectionOutcome::EmptyAsStated(reason) => tagged(
            "empty-as-stated",
            match reason {
                EmptySelectionReason::CarriedOverFromAPreviousRun => {
                    "carried-over-from-a-previous-run".into()
                }
                EmptySelectionReason::AskingWhatTheWorldHolds => {
                    "asking-what-the-world-holds".into()
                }
            },
        ),
    }
}

pub(super) fn not_selected(value: NotSelectedReason) -> Value {
    match value {
        NotSelectedReason::OutsideSelection => "outside-selection",
        NotSelectedReason::SuiteNotRun => "suite-not-run",
    }
    .into()
}

pub(super) fn skip(value: SkipReason) -> Value {
    match value {
        SkipReason::BudgetExhausted => "budget-exhausted",
        SkipReason::TargetUnsupported => "target-unsupported",
        SkipReason::PrerequisiteAbsent => "prerequisite-absent",
        SkipReason::SatisfiedByCachedExecution => "satisfied-by-cached-execution",
    }
    .into()
}

pub(super) fn fault(value: InfrastructureFault) -> Value {
    match value {
        InfrastructureFault::GenerationUnavailable => "generation-unavailable",
        InfrastructureFault::SupportAbsent => "support-absent",
        InfrastructureFault::CaptureFailed => "capture-failed",
        InfrastructureFault::BackendUnavailable => "backend-unavailable",
        InfrastructureFault::BackendInitializationFailed => "backend-initialization-failed",
        InfrastructureFault::BackendExecutionUnresolved => "backend-execution-unresolved",
    }
    .into()
}

pub(super) fn class(value: FailureClass) -> Value {
    match value {
        FailureClass::RefusedByCheck => "refused-by-check",
        FailureClass::PropertyDisagreement => "property-disagreement",
        FailureClass::OracleDisagreement => "oracle-disagreement",
        FailureClass::SubjectPanic => "subject-panic",
        FailureClass::BudgetExhausted => "budget-exhausted",
    }
    .into()
}

pub(super) fn fidelity(value: TextFidelity) -> Value {
    match value {
        TextFidelity::Exact => "exact",
        TextFidelity::LossyReplacement => "lossy-replacement",
    }
    .into()
}

pub(super) fn foreign(value: &ForeignText) -> Value {
    let truncation = match value.truncation() {
        Truncation::Complete => tagged("complete", Value::Null),
        Truncation::TruncatedAt { admitted, offered } => tagged(
            "truncated-at",
            object([("admitted", admitted.into()), ("offered", offered.into())]),
        ),
    };
    object([
        ("bytes", hex(value.bytes())),
        ("shown", value.shown().into()),
        ("fidelity", fidelity(value.fidelity())),
        ("truncation", truncation),
    ])
}

pub(super) fn fingerprint(value: Fingerprint) -> Value {
    object([
        ("address", hex(value.address().as_bytes())),
        ("trial", hex(value.trial().address().as_bytes())),
        ("family", value.cause().family().into()),
        ("local", value.cause().local().into()),
        ("class", class(value.class())),
    ])
}

pub(super) fn finding(trial: TrialId, value: &TrialFinding) -> Value {
    object([
        ("fingerprint", fingerprint(Fingerprint::of(trial, value))),
        ("file", value.located().file().into()),
        ("line", value.located().line().into()),
        ("foreign", optional(value.foreign(), foreign)),
    ])
}

pub(super) fn attempt(trial: TrialId, value: &RunAttempt) -> Value {
    match value {
        RunAttempt::Executed(TrialConclusion::Passed) => {
            tagged("executed", tagged("passed", Value::Null))
        }
        RunAttempt::Executed(TrialConclusion::Refused(value)) => {
            tagged("executed", tagged("refused", finding(trial, value)))
        }
        RunAttempt::SkippedWithReason(reason) => tagged("skipped-with-reason", skip(*reason)),
        RunAttempt::TimedOut => tagged("timed-out", Value::Null),
        RunAttempt::InfrastructureFailed(value) => tagged(
            "infrastructure-failed",
            object([
                ("fault", fault(value.fault())),
                ("foreign", optional(value.foreign(), foreign)),
            ]),
        ),
    }
}

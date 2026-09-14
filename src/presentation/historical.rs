//! Archive projections preserve claimed standing and never mint live reports.

use super::value::{array, hex, object, optional, tagged};
use super::{Presentation, clock, context, outcome, record::profile};
use crate::harness::report::archive::{
    ArchivedAccounting, ArchivedAttempt, ArchivedCapsule, ArchivedConclusion, ArchivedDisposition,
    ArchivedFinding, ArchivedFingerprint, ArchivedForeignText, ArchivedRun, ArchivedTrial,
    ArchivedTruncation,
};
use serde_json::Value;

/// Project an admitted historical run with every original census row.
pub fn archived_run(value: &ArchivedRun) -> Presentation {
    Presentation::projected(
        "run",
        "macroonz-harness/report",
        "historical-unauthenticated",
        run_value(value),
    )
}

pub(super) fn run_value(value: &ArchivedRun) -> Value {
    object([
        ("archive_address", hex(value.address().as_bytes())),
        ("denominator", value.census().len().into()),
        ("census", array(value.census().iter().map(accounting))),
        ("posture", context::historical_table(value.posture())),
        ("selection", outcome::selection(value.selection())),
        ("invocation", context::invocation(value.invocation())),
        ("target", context::target(value.target())),
        ("input", optional(value.input(), context::historical_input)),
    ])
}

fn accounting(value: &ArchivedAccounting) -> Value {
    let disposition = match value.disposition() {
        ArchivedDisposition::Selected(report) => tagged("selected", trial_value(report)),
        ArchivedDisposition::NotSelected(reason) => {
            tagged("not-selected", outcome::not_selected(*reason))
        }
    };
    object([
        ("trial", hex(value.trial().as_bytes())),
        ("row", hex(value.row().as_bytes())),
        ("subject", hex(value.subject().as_bytes())),
        ("check", hex(value.check().as_bytes())),
        (
            "claim",
            context::name(value.claim().namespace(), value.claim().stem()),
        ),
        ("disposition", disposition),
    ])
}

/// Project a historical trial without relabeling it as current execution.
pub fn archived_trial(value: &ArchivedTrial) -> Presentation {
    Presentation::projected(
        "trial",
        "macroonz-harness/report",
        "historical-unauthenticated",
        trial_value(value),
    )
}

pub(super) fn trial_value(value: &ArchivedTrial) -> Value {
    let site = value.site();
    object([
        ("archive_address", hex(value.address().as_bytes())),
        ("key", context::historical_execution(value.key())),
        ("posture", context::posture(value.claimed_posture())),
        (
            "site",
            object([
                ("module_path", site.module_path().into()),
                ("file", site.file().into()),
                ("line", site.line().into()),
                ("name", site.name().into()),
            ]),
        ),
        ("attempt", attempt(value.attempt())),
        (
            "measurement",
            clock::historical_measurement(value.measurement()),
        ),
        (
            "clock_attribution",
            clock::attribution(value.clock_attribution()),
        ),
    ])
}

fn attempt(value: &ArchivedAttempt) -> Value {
    match value {
        ArchivedAttempt::Executed(value) => tagged("executed", conclusion(value)),
        ArchivedAttempt::SkippedWithReason(reason) => {
            tagged("skipped-with-reason", outcome::skip(*reason))
        }
        ArchivedAttempt::TimedOut => tagged("timed-out", Value::Null),
        ArchivedAttempt::InfrastructureFailed {
            fault,
            foreign: material,
        } => tagged(
            "infrastructure-failed",
            object([
                ("fault", outcome::fault(*fault)),
                ("foreign", optional(material.as_ref(), foreign)),
            ]),
        ),
    }
}

pub(super) fn conclusion(value: &ArchivedConclusion) -> Value {
    match value {
        ArchivedConclusion::Passed => tagged("passed", Value::Null),
        ArchivedConclusion::Refused(value) => tagged("refused", finding(value)),
    }
}

pub(super) fn foreign(value: &ArchivedForeignText) -> Value {
    let truncation = match value.truncation() {
        ArchivedTruncation::Complete => tagged("complete", Value::Null),
        ArchivedTruncation::TruncatedAt { admitted, offered } => tagged(
            "truncated-at",
            object([("admitted", admitted.into()), ("offered", offered.into())]),
        ),
    };
    object([
        ("bytes", hex(value.bytes())),
        (
            "shown",
            String::from_utf8_lossy(value.bytes()).into_owned().into(),
        ),
        ("fidelity", outcome::fidelity(value.fidelity())),
        ("truncation", truncation),
    ])
}

pub(super) fn fingerprint(value: &ArchivedFingerprint) -> Value {
    object([
        ("address", hex(value.address().as_bytes())),
        ("trial", hex(value.trial().as_bytes())),
        ("family", value.family().into()),
        ("local", value.local().into()),
        ("class", outcome::class(value.class())),
    ])
}

pub(super) fn finding(value: &ArchivedFinding) -> Value {
    object([
        ("fingerprint", fingerprint(value.fingerprint())),
        ("file", value.file().into()),
        ("line", value.line().into()),
        ("foreign", optional(value.foreign(), foreign)),
    ])
}

/// Project an admitted historical witness without granting replay authority.
pub fn archived_capsule(value: &ArchivedCapsule) -> Presentation {
    Presentation::projected(
        "capsule",
        "macroonz-harness/report",
        "historical-unauthenticated",
        capsule_value(value),
    )
}

pub(super) fn capsule_value(value: &ArchivedCapsule) -> Value {
    object([
        ("archive_address", hex(value.address().as_bytes())),
        ("identity", hex(value.identity().as_bytes())),
        ("key", context::historical_execution(value.key())),
        ("fingerprint", fingerprint(value.fingerprint())),
        ("input", hex(value.input())),
        (
            "generation",
            profile(value.generation().name(), value.generation().version()),
        ),
        (
            "minimization",
            profile(value.minimization().name(), value.minimization().version()),
        ),
        ("schema", hex(value.schema().as_bytes())),
        ("posture", context::posture(value.claimed_posture())),
    ])
}

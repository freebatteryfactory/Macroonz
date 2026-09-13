//! Read-only native request, cleanup and capture projections.

use crate::native_process::{
    CaptureEnd, CapturedOutput, PendingProcess, ProcessLimits, ProcessOutput, ProcessRequest,
    ProcessStop,
};
pub(super) use crate::presentation::path::path;
use crate::presentation::{
    Presentation,
    value::{array, hex, object, optional, tagged},
};
use serde_json::Value;
use std::process::ExitStatus;
use std::time::Duration;

/// A finished native process observation with both complete retained prefixes.
pub fn native_process(record: &ProcessOutput) -> Presentation {
    Presentation::projected(
        "native-process",
        "macroonz/native_process",
        "recorded",
        output(record),
    )
}

pub(super) fn request(record: &ProcessRequest) -> Value {
    object([
        ("executable", path(record.executable())),
        ("directory", path(record.directory())),
        (
            "arguments",
            array(
                record
                    .arguments()
                    .iter()
                    .map(|argument| argument.as_str().into()),
            ),
        ),
        (
            "environment",
            array(record.environment().iter().map(|(key, value)| {
                object([
                    ("key", key.as_str().into()),
                    ("value", value.as_str().into()),
                ])
            })),
        ),
        ("limits", limits(record.limits())),
    ])
}

pub(super) fn limits(record: ProcessLimits) -> Value {
    object([
        ("execution", duration(record.execution())),
        ("cleanup", duration(record.cleanup())),
        ("stdout", record.stdout().into()),
        ("stderr", record.stderr().into()),
    ])
}

pub(super) fn output(record: &ProcessOutput) -> Value {
    object([
        ("state", "finished".into()),
        ("stop", stop(record.stop())),
        ("status", status(record.status())),
        ("stdout", capture(record.stdout())),
        ("stderr", capture(record.stderr())),
    ])
}

pub(super) fn pending(record: &PendingProcess) -> Value {
    object([
        ("state", "pending-cleanup".into()),
        ("stop", stop(record.stop())),
        (
            "cleanup_error",
            optional(record.cleanup_error(), Value::from),
        ),
    ])
}

pub(super) fn stop(record: &ProcessStop) -> Value {
    match record {
        ProcessStop::Exited => tagged("exited", Value::Null),
        ProcessStop::Deadline => tagged("deadline", Value::Null),
        ProcessStop::OutputLimit => tagged("output-limit", Value::Null),
        ProcessStop::ObservationFailed(detail) => {
            tagged("observation-failed", detail.as_str().into())
        }
    }
}

pub(super) fn duration(record: Duration) -> Value {
    object([
        ("seconds", record.as_secs().into()),
        ("nanoseconds", record.subsec_nanos().into()),
    ])
}

fn capture(record: &CapturedOutput) -> Value {
    let end = match record.end() {
        CaptureEnd::Eof => tagged("eof", Value::Null),
        CaptureEnd::LimitExceeded => tagged("limit-exceeded", Value::Null),
        CaptureEnd::Failed(detail) => tagged("failed", detail.as_str().into()),
    };
    object([
        ("bytes", hex(record.bytes())),
        ("retained_bytes", record.bytes().len().into()),
        (
            "shown",
            String::from_utf8_lossy(record.bytes()).as_ref().into(),
        ),
        (
            "fidelity",
            if std::str::from_utf8(record.bytes()).is_ok() {
                "utf-8"
            } else {
                "lossy"
            }
            .into(),
        ),
        ("end", end),
    ])
}

fn status(record: ExitStatus) -> Value {
    object([
        ("success", record.success().into()),
        ("code", optional(record.code(), Value::from)),
        ("shown", record.to_string().into()),
        ("unix_wait_status", unix_wait_status(record)),
    ])
}

#[cfg(unix)]
fn unix_wait_status(record: ExitStatus) -> Value {
    use std::os::unix::process::ExitStatusExt;
    record.into_raw().into()
}

#[cfg(not(unix))]
fn unix_wait_status(_record: ExitStatus) -> Value {
    Value::Null
}

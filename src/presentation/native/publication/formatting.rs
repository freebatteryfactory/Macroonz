//! Formatter results and refusals without process or text admission.

use super::super::{process, refusal};
use crate::native_process::ProcessRun;
use crate::native_publication::{FormatError, FormatObservationError, FormatOutput, FormatRun};
use crate::presentation::{
    Presentation,
    value::{hex, object, tagged},
};
use serde_json::Value;

/// Project a formatter run without completing pending cleanup.
pub fn publication_format_run(record: &FormatRun) -> Presentation {
    Presentation::projected(
        "publication-format-run",
        "macroonz/native_publication/formatter",
        "recorded",
        run(record),
    )
}

/// Project finished formatter output and its admitted text or refusal.
pub fn publication_format_output(record: &FormatOutput) -> Presentation {
    Presentation::projected(
        "publication-format-output",
        "macroonz/native_publication/formatter",
        "recorded",
        output(record),
    )
}

/// Project formatter preparation failure with its query and retained cleanup state.
pub fn publication_format_error(record: &FormatError) -> Presentation {
    Presentation::projected(
        "publication-format-error",
        "macroonz/native_publication/formatter",
        "recorded",
        error(record),
    )
}

pub(super) fn run(record: &FormatRun) -> Value {
    match record {
        FormatRun::Finished(value) => tagged("finished", output(value)),
        FormatRun::Pending(value) => tagged("pending-cleanup", process::pending(value.process())),
    }
}

pub(super) fn output(record: &FormatOutput) -> Value {
    let source = match record.source() {
        Ok(source) => tagged(
            "admitted",
            object([
                ("text", source.into()),
                (
                    "published_digest",
                    hex(crate::native_publication::published_digest(source.as_bytes()).as_bytes()),
                ),
            ]),
        ),
        Err(error) => tagged("refused", observation(&error)),
    };
    object([
        ("path", record.path().spelling().into()),
        ("original", record.original().into()),
        (
            "canonical_digest",
            hex(record.canonical_digest().as_bytes()),
        ),
        ("request", process::request(record.request())),
        ("version", record.profile().0.into()),
        ("configuration", hex(record.profile().1)),
        ("process", process::output(record.process())),
        ("source", source),
    ])
}

pub(super) fn error(record: &FormatError) -> Value {
    match record {
        FormatError::Configuration(detail) => tagged("configuration", detail.as_str().into()),
        FormatError::Filesystem(error) => tagged("filesystem", refusal::io_error(error)),
        FormatError::Process(error) => tagged("process", refusal::process_error(error)),
        FormatError::Query { request, run } => tagged(
            "query",
            object([
                ("request", process::request(request)),
                (
                    "process",
                    match run.as_ref() {
                        ProcessRun::Finished(value) => process::output(value),
                        ProcessRun::Pending(value) => process::pending(value),
                    },
                ),
            ]),
        ),
    }
}

pub(super) fn observation(record: &FormatObservationError) -> Value {
    match record {
        FormatObservationError::Process => tagged("process", Value::Null),
        FormatObservationError::Text => tagged("text", Value::Null),
        FormatObservationError::Configuration(detail) => {
            tagged("configuration", detail.as_str().into())
        }
    }
}

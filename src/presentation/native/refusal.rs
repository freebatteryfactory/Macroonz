//! Owner-specific native phases and typed causes.

use super::process;
use crate::native_compiler::{CompilerError, CompilerObservationError, ReadBackError};
use crate::native_process::{ProcessError, ResourceControl};
use crate::presentation::{
    Presentation,
    value::{object, optional, tagged},
};
use serde_json::Value;

/// A compiler configuration or process-start refusal.
pub fn compiler_error(record: &CompilerError) -> Presentation {
    Presentation::projected(
        "compiler-error",
        "macroonz/native_compiler",
        "recorded",
        object([("phase", "start".into()), ("cause", compiler_cause(record))]),
    )
}

pub(super) fn compiler_cause(record: &CompilerError) -> Value {
    match record {
        CompilerError::Configuration(detail) => tagged("configuration", detail.as_str().into()),
        CompilerError::Process(error) => tagged("process", process_error(error)),
    }
}

/// A read-back execution or decoder failure that establishes no semantic verdict.
pub fn read_back_error(record: &ReadBackError) -> Presentation {
    let cause = match record {
        ReadBackError::Interrupted(stop) => tagged("interrupted", process::stop(stop)),
        ReadBackError::ProcessFailure => tagged("process-failure", Value::Null),
        ReadBackError::Decode(detail) => tagged("decode", detail.as_str().into()),
    };
    Presentation::projected(
        "read-back-error",
        "macroonz/native_compiler",
        "recorded",
        object([("phase", "read-back".into()), ("cause", cause)]),
    )
}

pub(super) fn observation(record: &CompilerObservationError) -> Value {
    let cause = match record {
        CompilerObservationError::Interrupted(stop) => tagged("interrupted", process::stop(stop)),
        CompilerObservationError::ProcessFailure => tagged("process-failure", Value::Null),
        CompilerObservationError::InvalidJson { line, detail } => tagged(
            "invalid-json",
            object([("line", (*line).into()), ("detail", detail.as_str().into())]),
        ),
        CompilerObservationError::Protocol(detail) => tagged("protocol", detail.as_str().into()),
        CompilerObservationError::DiagnosticCount(count) => {
            tagged("diagnostic-count", (*count).into())
        }
        CompilerObservationError::PrimarySpanCount(count) => {
            tagged("primary-span-count", (*count).into())
        }
        CompilerObservationError::UncodedDiagnostic => tagged("uncoded-diagnostic", Value::Null),
        CompilerObservationError::Source(detail) => tagged("source", detail.as_str().into()),
    };
    object([("phase", "compiler-observation".into()), ("cause", cause)])
}

pub(super) fn process_error(record: &ProcessError) -> Value {
    match record {
        ProcessError::InvalidRequest(detail) => tagged("invalid-request", detail.as_str().into()),
        ProcessError::Unavailable => tagged("unavailable", Value::Null),
        ProcessError::Unsupported(control) => tagged("unsupported", resource(*control)),
        ProcessError::Start(error) => tagged("start", io_error(error)),
    }
}

pub(super) fn io_error(record: &std::io::Error) -> Value {
    object([
        ("os_code", optional(record.raw_os_error(), Value::from)),
        ("kind", format!("{:?}", record.kind()).into()),
        ("detail", record.to_string().into()),
    ])
}

fn resource(record: ResourceControl) -> Value {
    match record {
        ResourceControl::MemoryBytes(bytes) => tagged("memory-bytes", bytes.into()),
        ResourceControl::CpuTime(duration) => tagged("cpu-time", process::duration(duration)),
        ResourceControl::DenyNetwork => tagged("deny-network", Value::Null),
        ResourceControl::PreventGroupEscape => tagged("prevent-group-escape", Value::Null),
    }
}

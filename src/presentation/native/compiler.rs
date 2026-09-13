//! Native compilation facts without comparison or execution.

use super::{process, refusal};
use crate::native_compiler::{
    CompilerOutput, CompilerRequest, DependencyError, DependencyInfo, PendingCompilation,
};
use crate::presentation::{
    Presentation,
    oracle::compilation,
    value::{array, hex, object, optional, tagged},
};
use serde_json::Value;

/// A native compiler result retaining process, interpretation and dependency standing.
pub fn native_compilation(record: &CompilerOutput) -> Presentation {
    Presentation::projected(
        "native-compilation",
        "macroonz/native_compiler",
        "recorded",
        output(record),
    )
}

pub(super) fn output(record: &CompilerOutput) -> Value {
    let observed = match record.observed() {
        Ok(value) => tagged("observed", compilation::observation(value)),
        Err(error) => tagged("observation-failed", refusal::observation(error)),
    };
    let dependencies = optional(record.dependencies(), |result| match result {
        Ok(value) => tagged("observed", dependency(value)),
        Err(error) => tagged("refused", dependency_error(error)),
    });
    object([
        ("request", request(record.request())),
        ("process", process::output(record.process())),
        ("observation", observed),
        ("executable", optional(record.executable(), process::path)),
        ("cargo_fresh", optional(record.cargo_fresh(), Value::from)),
        ("dependencies", dependencies),
    ])
}

/// A pending compilation's cleanup observation without consuming its resources.
pub fn pending_compilation(record: &PendingCompilation) -> Presentation {
    Presentation::projected(
        "pending-compilation",
        "macroonz/native_compiler",
        "recorded",
        process::pending(record.process()),
    )
}

fn request(record: &CompilerRequest) -> Value {
    object([
        ("process", process::request(record.process())),
        ("locus", record.locus().spelling().into()),
        ("manifest", optional(record.manifest(), process::path)),
        (
            "dependency_file",
            optional(record.dependency_file(), process::path),
        ),
        ("output_root", optional(record.output_root(), process::path)),
    ])
}

fn dependency(record: &DependencyInfo) -> Value {
    object([
        ("bytes", hex(record.bytes())),
        ("artifact", process::path(record.artifact())),
        (
            "files",
            array(record.files().iter().map(|file| process::path(file))),
        ),
    ])
}

fn dependency_error(record: &DependencyError) -> Value {
    match record {
        DependencyError::NotCompiled => tagged("not-compiled", Value::Null),
        DependencyError::Filesystem(detail) => tagged("filesystem", detail.as_str().into()),
        DependencyError::NotRegular => tagged("not-regular", Value::Null),
        DependencyError::ByteBound => tagged("byte-bound", Value::Null),
        DependencyError::FileBound => tagged("file-bound", Value::Null),
        DependencyError::Artifact => tagged("artifact", Value::Null),
        DependencyError::Representation(detail) => tagged("representation", detail.as_str().into()),
    }
}

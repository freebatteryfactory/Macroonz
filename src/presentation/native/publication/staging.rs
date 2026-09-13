//! Staged source and compiler observations retain their separate qualification standing.

use super::super::{compiler, process, refusal, storage};
use super::inventory;
use crate::compiler::Kind;
use crate::native_publication::{
    CompiledPublication, StagingError, StagingObservationError, StagingRun,
};
use crate::presentation::{
    Presentation,
    value::{object, tagged},
};
use serde_json::Value;

/// Project staged compilation, refusal or pending cleanup without installing output.
pub fn publication_staging_run<K: Kind>(record: &StagingRun<K>) -> Presentation {
    Presentation::projected(
        "publication-staging-run",
        "macroonz/native_publication/staging",
        "recorded",
        run(record),
    )
}

/// Project a staged compilation admission or startup failure.
pub fn publication_staging_error(record: &StagingError) -> Presentation {
    Presentation::projected(
        "publication-staging-error",
        "macroonz/native_publication/staging",
        "recorded",
        error(record),
    )
}

pub(super) fn compiled<K: Kind>(record: &CompiledPublication<K>) -> Value {
    object([
        ("prepared", inventory::prepared(record.prepared())),
        ("compiler", compiler::output(record.compiler())),
    ])
}

pub(super) fn run<K: Kind>(record: &StagingRun<K>) -> Value {
    match record {
        StagingRun::Compiled(value) => tagged("compiled", compiled(value)),
        StagingRun::Refused(value) => tagged(
            "refused",
            object([
                ("reason", observation(value.reason())),
                ("compiler", compiler::output(value.compiler())),
                ("source_directory", process::path(value.staged().path())),
                ("prepared", inventory::prepared(value.staged().prepared())),
            ]),
        ),
        StagingRun::Pending(value) => tagged(
            "pending-cleanup",
            process::pending(value.compiler().process()),
        ),
    }
}

pub(super) fn error(record: &StagingError) -> Value {
    match record {
        StagingError::Inventory(value) => tagged("inventory", inventory::error(value)),
        StagingError::Configuration(detail) => tagged("configuration", detail.as_str().into()),
        StagingError::Storage(value) => tagged("storage", storage::cause(value)),
        StagingError::Filesystem(value) => tagged("filesystem", refusal::io_error(value)),
        StagingError::Source(detail) => tagged("source", detail.as_str().into()),
        StagingError::Compiler(value) => tagged("compiler", refusal::compiler_cause(value)),
        StagingError::Unavailable => tagged("unavailable", Value::Null),
    }
}

fn observation(record: &StagingObservationError) -> Value {
    match record {
        StagingObservationError::Compilation => tagged("compilation", Value::Null),
        StagingObservationError::Dependencies(detail) => {
            tagged("dependencies", detail.as_str().into())
        }
        StagingObservationError::Unused(detail) => tagged("unused", detail.as_str().into()),
        StagingObservationError::Source(detail) => tagged("source", detail.as_str().into()),
    }
}

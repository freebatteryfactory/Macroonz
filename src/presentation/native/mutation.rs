//! Native mutation observations and unfinished cleanup.

use super::{mutation_refusal, process};
use crate::native_mutation::{MutationOutput, MutationRequest, PendingMutation};
use crate::presentation::{
    Presentation,
    mutation::backend,
    value::{array, hex, object, tagged},
};
use serde_json::Value;

/// A native mutation result retaining its source material, queries and observation standing.
pub fn native_mutation(record: &MutationOutput) -> Presentation {
    let manifest = match record.manifest() {
        Ok(manifest) => tagged("established", backend::manifest(manifest)),
        Err(error) => tagged("refused", mutation_refusal::observation(error)),
    };
    let qualification = match record.qualification() {
        Ok(qualification) => tagged("established", backend::qualification(qualification)),
        Err(error) => tagged("refused", mutation_refusal::observation(error)),
    };
    Presentation::projected(
        "native-mutation",
        "macroonz/native_mutation",
        "recorded",
        object([
            ("request", request(record.request())),
            ("process", process::output(record.process())),
            (
                "backend_version",
                process::output(record.backend_version_output()),
            ),
            (
                "compiler_version",
                process::output(record.compiler_version_output()),
            ),
            (
                "original_sources",
                array(
                    record.original_sources().map(|(file, bytes)| {
                        object([("file", file.into()), ("bytes", hex(bytes))])
                    }),
                ),
            ),
            ("manifest", manifest),
            ("qualification", qualification),
        ]),
    )
}

/// An unfinished mutation process without an established mutation observation.
pub fn pending_mutation(record: &PendingMutation) -> Presentation {
    Presentation::projected(
        "pending-mutation",
        "macroonz/native_mutation",
        "recorded",
        process::pending(record.process()),
    )
}

fn request(record: &MutationRequest) -> Value {
    object([
        ("process", process::request(record.process())),
        (
            "sources",
            object([
                ("byte_bound", record.sources().byte_bound().into()),
                (
                    "files",
                    array(
                        record
                            .sources()
                            .files()
                            .iter()
                            .map(|file| file.spelling().into()),
                    ),
                ),
            ]),
        ),
        ("output_directory", process::path(record.output_directory())),
        ("version_limits", process::limits(record.version_limits())),
    ])
}

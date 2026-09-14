//! Historical backend manifests and optional original material.

use super::{backend, historical};
use crate::harness::muterprater::backend_archive::{
    ArchivedAdapterProfile, ArchivedBackendInvocation, ArchivedBackendManifest,
};
use crate::presentation::{
    Presentation, context, historical as report,
    value::{array, hex, object, optional, tagged},
};
use serde_json::Value;

/// An integrity-admitted backend manifest with its complete historical population and optional originals.
pub fn archived_backend(record: &ArchivedBackendManifest) -> Presentation {
    Presentation::projected(
        "backend-manifest",
        "harness/muterprater/backend/archive",
        "historical-unauthenticated",
        manifest(record),
    )
}

pub(super) fn manifest(record: &ArchivedBackendManifest) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("invocation", invocation(record.invocation())),
        ("output", hex(record.output().as_bytes())),
        (
            "sources",
            array(record.sources().iter().map(|source| {
                object([
                    ("file", source.file().into()),
                    ("revision", hex(source.revision().as_bytes())),
                    ("original", optional(source.original(), hex)),
                ])
            })),
        ),
        (
            "reading",
            object([
                ("profile", profile(record.profile())),
                ("run", historical::run(record.run())),
                ("announced", backend::announced(record.announced())),
                (
                    "unparsed",
                    array(record.unparsed().iter().map(|line| {
                        object([
                            ("ordinal", line.ordinal().into()),
                            ("text", report::foreign(line.text())),
                        ])
                    })),
                ),
            ]),
        ),
        ("original_console", optional(record.original_console(), hex)),
    ])
}

fn invocation(record: &ArchivedBackendInvocation) -> Value {
    object([
        ("backend", backend::backend(record.backend())),
        ("version", record.version().into()),
        (
            "command",
            object([
                ("executable", record.executable().into()),
                (
                    "arguments",
                    array(record.arguments().iter().map(|arg| arg.as_str().into())),
                ),
            ]),
        ),
        ("target", context::target(record.target())),
    ])
}

fn profile(record: &ArchivedAdapterProfile) -> Value {
    object([
        ("backend", backend::backend(record.backend())),
        ("version", tagged("stated", record.version().into())),
        ("source", backend::source(record.source())),
        ("grammar", record.grammar().number().into()),
        ("ceiling", backend::ceiling(record.ceiling())),
    ])
}

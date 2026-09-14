//! Imported console readings and declared execution manifests.

use super::record;
use crate::harness::muterprater::{
    AdapterProfile, AnnouncedRoster, BackendVersionPosture, ClaimCeiling,
    CompiledSuiteArtifactManifest, MutationBackendInvocation, MutationSourceRevision,
    ReadingSource, WrapReading, WrappedBackend,
};
#[cfg(feature = "native-tooling")]
use crate::harness::muterprater::{AdapterQualification, GrammarStanding};
use crate::presentation::{
    Presentation, context, outcome,
    value::{array, hex, object, tagged},
};
use serde_json::Value;

/// An imported backend reading preserving its announced and parsed populations separately.
pub fn backend_reading(record: &WrapReading) -> Presentation {
    Presentation::projected(
        "backend-reading",
        "harness/muterprater/backend",
        "recorded",
        reading(record),
    )
}

/// An imported backend manifest retaining command tokens, source revisions and its complete reading.
pub fn backend_manifest(record: &CompiledSuiteArtifactManifest) -> Presentation {
    Presentation::projected(
        "backend-manifest",
        "harness/muterprater/backend",
        "recorded",
        manifest(record),
    )
}

pub(in crate::presentation) fn manifest(record: &CompiledSuiteArtifactManifest) -> Value {
    object([
        ("invocation", invocation(record.invocation())),
        ("output", hex(record.output().address().as_bytes())),
        (
            "sources",
            array(record.sources().iter().map(source_revision)),
        ),
        ("reading", reading(record.reading())),
    ])
}

fn reading(record: &WrapReading) -> Value {
    object([
        ("profile", profile(record.profile())),
        ("run", record::run(record.run())),
        ("announced", announced(record.announced())),
        (
            "unparsed",
            array(record.unparsed().iter().map(|line| {
                object([
                    ("ordinal", line.ordinal().into()),
                    ("text", outcome::foreign(line.text())),
                ])
            })),
        ),
    ])
}

fn invocation(record: &MutationBackendInvocation) -> Value {
    object([
        ("backend", backend(record.backend())),
        ("version", record.version().spelling().into()),
        (
            "command",
            object([
                ("executable", record.command().executable().into()),
                (
                    "arguments",
                    array(
                        record
                            .command()
                            .arguments()
                            .iter()
                            .map(|arg| arg.as_str().into()),
                    ),
                ),
            ]),
        ),
        ("target", context::target(record.target())),
    ])
}

pub(super) fn source_revision(record: &MutationSourceRevision) -> Value {
    object([
        ("file", record.file().into()),
        ("revision", hex(record.revision().address().as_bytes())),
    ])
}

pub(super) fn backend(record: WrappedBackend) -> Value {
    match record {
        WrappedBackend::CargoMutants => "cargo-mutants",
    }
    .into()
}

pub(super) fn source(record: ReadingSource) -> Value {
    match record {
        ReadingSource::ConsoleStream => "console-stream",
    }
    .into()
}

pub(super) fn ceiling(record: ClaimCeiling) -> Value {
    match record {
        ClaimCeiling::WitnessRejection => "witness-rejection",
    }
    .into()
}

pub(super) fn announced(record: AnnouncedRoster) -> Value {
    match record {
        AnnouncedRoster::Stated(count) => tagged("stated", count.into()),
        AnnouncedRoster::Unstated => tagged("unstated", Value::Null),
    }
}

fn profile(record: &AdapterProfile) -> Value {
    let version = match record.version() {
        BackendVersionPosture::Stated(version) => tagged("stated", version.spelling().into()),
        BackendVersionPosture::Unstated => tagged("unstated", Value::Null),
    };
    object([
        ("backend", backend(record.backend())),
        ("version", version),
        ("source", source(record.source())),
        ("grammar", record.grammar().number().into()),
        ("ceiling", ceiling(record.ceiling())),
    ])
}

#[cfg(feature = "native-tooling")]
pub(in crate::presentation) fn qualification(record: &AdapterQualification) -> Value {
    let standing = match record.standing() {
        GrammarStanding::Checked(version) => tagged("checked", version.spelling().into()),
        GrammarStanding::Unchecked => tagged("unchecked", Value::Null),
    };
    object([
        ("profile", profile(record.profile())),
        ("standing", standing),
    ])
}

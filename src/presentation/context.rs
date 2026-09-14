//! Shared display projections of existing invocation and input coordinates.

use super::value::{hex, object, optional, tagged};
use crate::harness::descriptor::archive::{
    ArchivedName, ArchivedProvenance, ArchivedRevisionBinding,
};
use crate::harness::descriptor::{NamespacedName, Provenance, RevisionBinding, TablePosture};
use crate::harness::report::archive::{ArchivedExecution, ArchivedInput, ArchivedTablePosture};
use crate::harness::report::{
    ExecutionInput, ExecutionKey, InvocationProfile, ReplayPosture, TargetBinding,
};
use serde_json::Value;

pub(super) fn name(namespace: &str, stem: &str) -> Value {
    object([("namespace", namespace.into()), ("stem", stem.into())])
}

pub(super) fn declared_name(value: NamespacedName) -> Value {
    name(value.namespace().written(), value.stem().written())
}

pub(super) fn historical_name(value: &ArchivedName) -> Value {
    name(value.namespace(), value.stem())
}

pub(super) fn historical_revision(value: ArchivedRevisionBinding) -> Value {
    object([
        ("address", hex(value.revision())),
        ("posture", posture(ReplayPosture::from(value.posture()))),
    ])
}

pub(super) fn revision(value: RevisionBinding) -> Value {
    object([
        ("address", hex(value.revision().as_bytes())),
        ("posture", posture(ReplayPosture::from(value.posture()))),
    ])
}

pub(super) fn provenance(value: Provenance) -> Value {
    match value {
        Provenance::Unproduced => tagged("unproduced", Value::Null),
        Provenance::Produced { producer, schema } => tagged(
            "produced",
            object([
                ("producer", declared_name(producer.name())),
                ("schema", hex(schema.address().as_bytes())),
            ]),
        ),
    }
}

pub(super) fn historical_provenance(value: &ArchivedProvenance) -> Value {
    match value {
        ArchivedProvenance::Unproduced => tagged("unproduced", Value::Null),
        ArchivedProvenance::Produced { producer, schema } => tagged(
            "produced",
            object([
                ("producer", name(producer.namespace(), producer.stem())),
                ("schema", hex(schema)),
            ]),
        ),
    }
}

pub(super) fn invocation(value: InvocationProfile) -> Value {
    object([
        ("cases", value.cases().cases().into()),
        ("bytes", value.bytes().bytes().into()),
        ("nanoseconds", value.time().nanoseconds().into()),
    ])
}

pub(super) fn target(value: &TargetBinding) -> Value {
    object([
        ("target", value.target().spelling().into()),
        ("toolchain", value.toolchain().spelling().into()),
    ])
}

pub(super) fn posture(value: ReplayPosture) -> Value {
    match value {
        ReplayPosture::ExactDerived => "exact-derived",
        ReplayPosture::DeclaredByAuthor => "declared-by-author",
        ReplayPosture::UnavailableBecauseUntracked => "unavailable-because-untracked",
    }
    .into()
}

pub(super) fn table(value: TablePosture) -> Value {
    match value {
        TablePosture::Authored => tagged("authored", Value::Null),
        TablePosture::Staged { parent } => tagged("staged", declared_name(parent.name())),
    }
}

pub(super) fn historical_table(value: &ArchivedTablePosture) -> Value {
    match value {
        ArchivedTablePosture::Authored => tagged("authored", Value::Null),
        ArchivedTablePosture::Staged { parent } => {
            tagged("staged", name(parent.namespace(), parent.stem()))
        }
    }
}

pub(super) fn input(value: ExecutionInput) -> Value {
    let profile = value.profile();
    object([
        ("name", declared_name(profile.name())),
        ("version", profile.version().into()),
        ("schema", hex(profile.schema().as_bytes())),
        ("case", hex(value.case().address().as_bytes())),
        ("decoder", hex(value.decoder().as_bytes())),
        ("posture", posture(ReplayPosture::from(value.posture()))),
    ])
}

pub(super) fn historical_input(value: &ArchivedInput) -> Value {
    object([
        ("name", name(value.namespace(), value.profile().name())),
        ("version", value.profile().version().into()),
        ("schema", hex(value.schema().as_bytes())),
        ("case", hex(value.case().as_bytes())),
        ("decoder", hex(value.decoder().as_bytes())),
        ("posture", posture(value.posture())),
    ])
}

pub(super) fn execution(value: &ExecutionKey) -> Value {
    object([
        ("address", hex(value.address().as_bytes())),
        ("trial", hex(value.trial().address().as_bytes())),
        ("subject", hex(value.subject().address().as_bytes())),
        ("check", hex(value.check().address().as_bytes())),
        ("invocation", invocation(value.invocation())),
        ("target", target(value.target())),
        ("input", optional(value.input(), input)),
    ])
}

pub(super) fn historical_execution(value: &ArchivedExecution) -> Value {
    object([
        ("address", hex(value.address().as_bytes())),
        ("trial", hex(value.trial().as_bytes())),
        ("subject", hex(value.subject().as_bytes())),
        ("check", hex(value.check().as_bytes())),
        ("invocation", invocation(value.invocation())),
        ("target", target(value.target())),
        ("input", optional(value.input(), historical_input)),
    ])
}

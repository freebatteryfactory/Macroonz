//! Complete method-specific historical verdict fields.

use super::{compilation, read_back};
use crate::harness::oracle::{
    ByteDifference,
    archive::{ArchivedOracle, ArchivedStructuralDisagreement, ArchivedVerdict},
};
use crate::presentation::{
    Presentation,
    value::{hex, object, tagged},
};
use serde_json::Value;

/// An integrity-admitted historical oracle verdict without current judgment authority.
pub fn archived_oracle(record: &ArchivedOracle) -> Presentation {
    let (method, verdict) = verdict(record.verdict());
    Presentation::projected(
        "oracle-verdict",
        "harness/oracle/archive",
        "historical-unauthenticated",
        object([
            ("archive_address", hex(record.address().as_bytes())),
            ("method", method.into()),
            ("verdict", verdict),
        ]),
    )
}

fn verdict(record: &ArchivedVerdict) -> (&'static str, Value) {
    match record {
        ArchivedVerdict::VectorAgrees => ("vector", tagged("agrees", Value::Null)),
        ArchivedVerdict::VectorDisagrees(found) => (
            "vector",
            tagged(
                "disagrees",
                object([
                    ("expected", hex(found.expected())),
                    ("produced", hex(found.produced())),
                    ("difference", difference(found.difference())),
                ]),
            ),
        ),
        ArchivedVerdict::TranscriptAgrees => ("transcript", tagged("agrees", Value::Null)),
        ArchivedVerdict::TranscriptDisagrees(found) => (
            "transcript",
            tagged(
                "disagrees",
                object([
                    ("rederived", hex(found.rederived().as_bytes())),
                    ("published", hex(found.published().as_bytes())),
                ]),
            ),
        ),
        ArchivedVerdict::StructuralConforms => ("structural", tagged("conforms", Value::Null)),
        ArchivedVerdict::StructuralUnparsable => ("structural", tagged("unparsable", Value::Null)),
        ArchivedVerdict::StructuralDeviates(cause) => {
            ("structural", tagged("deviates", structural(cause)))
        }
        ArchivedVerdict::CompiledConforms => ("compiled", tagged("conforms", Value::Null)),
        ArchivedVerdict::CompiledDeviates(cause) => (
            "compiled",
            tagged("deviates", read_back::disagreement(cause)),
        ),
        ArchivedVerdict::CompilationConforms => ("compilation", tagged("conforms", Value::Null)),
        ArchivedVerdict::CompilationDeviates(cause) => (
            "compilation",
            tagged("deviates", compilation::disagreement(cause)),
        ),
    }
}

fn difference(record: ByteDifference) -> Value {
    match record {
        ByteDifference::AtByte { at } => tagged("at-byte", object([("at", at.into())])),
        ByteDifference::Length { expected, produced } => tagged(
            "length",
            object([("expected", expected.into()), ("produced", produced.into())]),
        ),
    }
}

fn structural(record: &ArchivedStructuralDisagreement) -> Value {
    match record {
        ArchivedStructuralDisagreement::UnexpectedItem => tagged("unexpected-item", Value::Null),
        ArchivedStructuralDisagreement::OutputCardinality { declared, read } => tagged(
            "output-cardinality",
            object([("declared", (*declared).into()), ("read", (*read).into())]),
        ),
        ArchivedStructuralDisagreement::DuplicateImplementation { at } => {
            positioned("duplicate-implementation", *at)
        }
        ArchivedStructuralDisagreement::ImplementationTarget { at } => {
            positioned("implementation-target", *at)
        }
        ArchivedStructuralDisagreement::TraitPath { at } => positioned("trait-path", *at),
        ArchivedStructuralDisagreement::ImplPosture { at } => {
            positioned("implementation-posture", *at)
        }
        ArchivedStructuralDisagreement::MeaningBearingAttribute { at, attribute } => tagged(
            "meaning-bearing-attribute",
            object([
                ("at", (*at).into()),
                ("attribute", attribute.as_str().into()),
            ]),
        ),
        ArchivedStructuralDisagreement::UnexpectedImplMember { at, member } => {
            member_at("unexpected-member", *at, member)
        }
        ArchivedStructuralDisagreement::DuplicateMember { at, member } => {
            member_at("duplicate-member", *at, member)
        }
        ArchivedStructuralDisagreement::MissingImplMember { at, member } => {
            member_at("missing-member", *at, member)
        }
        ArchivedStructuralDisagreement::MemberValueUnread { at, member } => {
            member_at("member-value-unread", *at, member)
        }
        ArchivedStructuralDisagreement::MemberValue { at, member } => {
            member_at("member-value", *at, member)
        }
    }
}

fn positioned(kind: &str, at: u64) -> Value {
    tagged(kind, object([("at", at.into())]))
}

fn member_at(kind: &str, at: u64, member: &str) -> Value {
    tagged(kind, object([("at", at.into()), ("member", member.into())]))
}

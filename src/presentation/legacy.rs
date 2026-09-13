//! Sparse historical fields remain distinct from current execution coordinates.

use super::{
    Presentation, replay,
    value::{array, hex, object, tagged},
};
use crate::harness::report::legacy::{
    LegacyField, LegacyInputProfile, LegacyPresence, LegacyRecord,
};
use crate::harness::report::replay::{LegacyClaimRelation, LegacyJoinRefusal, LegacyReading};
use serde_json::Value;

/// Project exact legacy source and every missing, null or present historical field.
pub fn legacy_record(record: &LegacyRecord) -> Presentation {
    Presentation::projected(
        "legacy-record",
        "macroonz-harness/report/legacy",
        "historical-unauthenticated",
        object([
            ("source_address", hex(record.source_address().as_bytes())),
            ("source", hex(record.source())),
            ("kind", text(record.kind())),
            ("schema", presence(record.schema(), |value| (*value).into())),
            ("witness", presence(record.witness(), |value| hex(value))),
            ("input_profile", presence(record.input_profile(), profile)),
            ("trial_name", text(record.trial_name())),
            ("subject_name", text(record.subject_name())),
            ("check_name", text(record.check_name())),
            (
                "subject_revision",
                presence(record.subject_revision(), |value| (*value).into()),
            ),
            (
                "check_revision",
                presence(record.check_revision(), |value| (*value).into()),
            ),
            ("target", text(record.target())),
            ("toolchain", text(record.toolchain())),
            (
                "execution_digest",
                presence(record.execution_digest(), |value| hex(value.as_bytes())),
            ),
            (
                "fingerprint_digest",
                presence(record.fingerprint_digest(), |value| hex(value.as_bytes())),
            ),
            ("reported_outcome", text(record.reported_outcome())),
        ]),
    )
}

fn text(value: &LegacyPresence<String>) -> Value {
    presence(value, |value| value.as_str().into())
}

fn presence<T>(record: &LegacyPresence<T>, project: impl FnOnce(&T) -> Value) -> Value {
    match record {
        LegacyPresence::Missing => tagged("missing", Value::Null),
        LegacyPresence::Null => tagged("null", Value::Null),
        LegacyPresence::Present(value) => tagged("present", project(value)),
    }
}

fn profile(record: &LegacyInputProfile) -> Value {
    object([
        ("name", text(record.name())),
        (
            "revision",
            presence(record.revision(), |value| (*value).into()),
        ),
    ])
}

/// Project joined legacy claims without inferring reproduction or repair.
pub fn legacy_comparison(record: &LegacyReading) -> Presentation {
    Presentation::projected(
        "legacy-comparison",
        "macroonz-harness/report/replay",
        "recorded",
        object([
            ("historical", replay::standing(LegacyReading::standing())),
            (
                "claims",
                array(record.claims().iter().map(|(key, value)| {
                    object([
                        ("field", field(*key).into()),
                        ("relation", relation(*value).into()),
                    ])
                })),
            ),
        ]),
    )
}

/// Project an unsuccessful legacy witness join without inventing compared claims.
pub fn legacy_join_refusal(record: LegacyJoinRefusal) -> Presentation {
    let cause = match record {
        LegacyJoinRefusal::MissingWitness => tagged("missing-witness", Value::Null),
        LegacyJoinRefusal::NullWitness => tagged("null-witness", Value::Null),
        LegacyJoinRefusal::Current(value) => tagged("current", replay::join_cause(value).into()),
    };
    Presentation::projected(
        "legacy-join-refusal",
        "macroonz-harness/report/replay",
        "recorded",
        object([("phase", "witness-join".into()), ("cause", cause)]),
    )
}

fn field(record: LegacyField) -> &'static str {
    match record {
        LegacyField::Kind => "kind",
        LegacyField::Schema => "schema",
        LegacyField::Witness => "witness",
        LegacyField::InputProfile => "input_profile",
        LegacyField::TrialName => "trial_name",
        LegacyField::SubjectName => "subject_name",
        LegacyField::CheckName => "check_name",
        LegacyField::SubjectRevision => "subject_revision",
        LegacyField::CheckRevision => "check_revision",
        LegacyField::Target => "target",
        LegacyField::Toolchain => "toolchain",
        LegacyField::ExecutionDigest => "execution_digest",
        LegacyField::FingerprintDigest => "fingerprint_digest",
        LegacyField::ReportedOutcome => "reported_outcome",
        LegacyField::ProfileName => "input_profile.name",
        LegacyField::ProfileRevision => "input_profile.revision",
    }
}

fn relation(record: LegacyClaimRelation) -> &'static str {
    match record {
        LegacyClaimRelation::Missing => "missing",
        LegacyClaimRelation::Null => "null",
        LegacyClaimRelation::ParentMissing => "parent-missing",
        LegacyClaimRelation::ParentNull => "parent-null",
        LegacyClaimRelation::Uninterpreted => "uninterpreted",
        LegacyClaimRelation::SameClaim => "same-claim",
        LegacyClaimRelation::MovedClaim => "moved-claim",
        LegacyClaimRelation::CurrentUnavailable => "current-unavailable",
    }
}

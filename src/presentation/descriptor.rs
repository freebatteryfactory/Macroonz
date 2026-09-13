//! Complete historical descriptor coordinates used by enclosing record projections.

use super::{
    context,
    value::{array, hex, object, tagged},
};
use crate::harness::descriptor::ReplayBearingGround;
use crate::harness::descriptor::archive::{
    ArchivedBinding, ArchivedCandidate, ArchivedOrigin, ArchivedRow, ArchivedSynthesis,
};
use serde_json::Value;

pub(super) fn binding(record: &ArchivedBinding) -> Value {
    object([
        ("row", row(record.row())),
        ("subject", context::historical_name(record.subject())),
        ("check", context::historical_name(record.check())),
        (
            "subject_revision",
            context::historical_revision(record.subject_revision()),
        ),
        (
            "check_revision",
            context::historical_revision(record.check_revision()),
        ),
        (
            "provenance",
            context::historical_provenance(record.provenance()),
        ),
    ])
}

fn row(record: &ArchivedRow) -> Value {
    object([
        ("canonical", hex(record.canonical_bytes())),
        ("claim", context::historical_name(record.claim())),
        (
            "execution_suite",
            context::historical_name(record.execution_suite()),
        ),
        ("subject", context::historical_name(record.subject())),
        ("check", context::historical_name(record.check())),
        ("population", context::historical_name(record.population())),
        (
            "roles",
            array(record.roles().iter().map(context::historical_name)),
        ),
        (
            "tags",
            array(record.tags().iter().map(context::historical_name)),
        ),
        ("origin", origin(record.origin())),
    ])
}

pub(super) fn candidate(record: &ArchivedCandidate) -> Value {
    object([
        ("canonical", hex(record.canonical_bytes())),
        ("claim", context::historical_name(record.claim())),
        (
            "execution_suite",
            context::historical_name(record.execution_suite()),
        ),
        ("subject", context::historical_name(record.subject())),
        ("check", context::historical_name(record.check())),
        ("population", context::historical_name(record.population())),
        (
            "roles",
            array(record.roles().iter().map(context::historical_name)),
        ),
        (
            "tags",
            array(record.tags().iter().map(context::historical_name)),
        ),
        ("synthesis", synthesis(record.synthesis())),
    ])
}

fn synthesis(record: &ArchivedSynthesis) -> Value {
    match record {
        ArchivedSynthesis::Survivor(point) => tagged("survivor", context::historical_name(point)),
        ArchivedSynthesis::ProofGap => tagged("proof-gap", Value::Null),
    }
}

fn origin(record: &ArchivedOrigin) -> Value {
    match record {
        ArchivedOrigin::HandWritten => tagged("hand-written", Value::Null),
        ArchivedOrigin::Generated { door, projection } => tagged(
            "generated",
            object([
                ("door", context::historical_name(door)),
                ("projection", context::historical_name(projection)),
            ]),
        ),
        ArchivedOrigin::Candidate(value) => tagged("candidate", synthesis(value)),
        ArchivedOrigin::AdmittedReplay {
            proposal,
            ground,
            destination,
            replay,
        } => {
            let ground = match ground {
                ReplayBearingGround::MutantKilled => "mutant-killed",
                ReplayBearingGround::ClaimPinned => "claim-pinned",
            };
            tagged(
                "admitted-replay",
                object([
                    ("proposal", hex(proposal)),
                    ("ground", ground.into()),
                    ("destination", context::historical_name(destination)),
                    ("replay", hex(replay)),
                ]),
            )
        }
        ArchivedOrigin::AdmittedDischarge {
            proposal,
            destination,
        } => tagged(
            "admitted-discharge",
            object([
                ("proposal", hex(proposal)),
                ("destination", context::historical_name(destination)),
            ]),
        ),
    }
}

//! Complete historical parity with caller-owned value conventions and actual disposition.

use crate::harness::muterprater::ParityQualificationRefusal;
use crate::harness::muterprater::interpretation_archive::{
    ArchivedEvaluationPair, ArchivedParity, ArchivedParityDisposition, ArchivedSubstrate,
    ArchivedValue,
};
use crate::presentation::{
    Presentation, context, descriptor, historical,
    value::{array, hex, object, tagged},
};
use serde_json::Value;

/// A complete historical parity reading with its recorded qualification disposition.
pub fn archived_parity(record: &ArchivedParity) -> Presentation {
    Presentation::projected(
        "no-mutation-parity",
        "harness/muterprater/interpretation/archive",
        "historical-unauthenticated",
        reading(record),
    )
}

pub(super) fn reading(record: &ArchivedParity) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("pair", pair(record.pair())),
        ("witness", descriptor::binding(record.witness())),
        ("input", encoded_value(record.input())),
        ("production", encoded_value(record.production())),
        ("evaluation", encoded_value(record.evaluation())),
        ("evaluation_firings", record.evaluation_firings().into()),
        ("substrate", substrate(record.substrate())),
        ("conclusion", historical::conclusion(record.conclusion())),
        (
            "production_report",
            historical::trial_value(record.production_report()),
        ),
        (
            "evaluation_report",
            historical::trial_value(record.evaluation_report()),
        ),
        ("disposition", disposition(record.disposition())),
    ])
}

pub(super) fn pair(record: &ArchivedEvaluationPair) -> Value {
    object([
        ("family", context::historical_name(record.family())),
        (
            "production_revision",
            context::historical_revision(record.production_revision()),
        ),
        (
            "evaluation_revision",
            context::historical_revision(record.evaluation_revision()),
        ),
        ("surface", hex(record.surface().as_bytes())),
    ])
}

pub(super) fn encoded_value(record: &ArchivedValue) -> Value {
    let convention = record.convention();
    object([
        ("bytes", hex(record.bytes())),
        (
            "convention",
            object([
                ("name", context::historical_name(convention.name())),
                ("version", convention.version().into()),
                ("schema", hex(convention.schema().as_bytes())),
                (
                    "revision",
                    context::historical_revision(convention.revision()),
                ),
            ]),
        ),
    ])
}

pub(super) fn substrate(record: &ArchivedSubstrate) -> Value {
    match record {
        ArchivedSubstrate::DeclaredIndependent => tagged("declared-independent", Value::Null),
        ArchivedSubstrate::Standing(roster) => tagged(
            "standing",
            array(roster.names().iter().map(context::historical_name)),
        ),
    }
}

fn disposition(record: ArchivedParityDisposition) -> Value {
    match record {
        ArchivedParityDisposition::Raw => tagged("raw", Value::Null),
        ArchivedParityDisposition::Qualified => tagged("qualified", Value::Null),
        ArchivedParityDisposition::Rejected(cause) => tagged("rejected", refusal(cause)),
    }
}

fn refusal(record: ParityQualificationRefusal) -> Value {
    match record {
        ParityQualificationRefusal::ProductionDidNotQualify => {
            tagged("production-did-not-qualify", Value::Null)
        }
        ParityQualificationRefusal::EvaluationDidNotQualify => {
            tagged("evaluation-did-not-qualify", Value::Null)
        }
        ParityQualificationRefusal::NoMutationActivated { firings } => {
            tagged("no-mutation-activated", firings.into())
        }
        ParityQualificationRefusal::MeaningsDisagreed => tagged("meanings-disagreed", Value::Null),
    }
}

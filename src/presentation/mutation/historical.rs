//! Complete historical mutation runs without current outcome authority.

use super::{axes, historical_target};
use crate::harness::muterprater::verdict_archive::{
    ArchivedMutation, ArchivedMutationOutcome, ArchivedMutationRun, ArchivedRejection,
};
use crate::presentation::{
    Presentation, historical,
    value::{array, hex, object, tagged},
};
use serde_json::Value;

/// A historical mutation run retaining its original order and complete census.
pub fn archived_mutation_run(record: &ArchivedMutationRun) -> Presentation {
    Presentation::projected(
        "mutation-run",
        "harness/muterprater/verdict/archive",
        "historical-unauthenticated",
        run(record),
    )
}

/// An individual historical mutation with its recorded axes and evidence claims.
pub fn archived_mutation(record: &ArchivedMutation) -> Presentation {
    Presentation::projected(
        "mutation-report",
        "harness/muterprater/verdict/archive",
        "historical-unauthenticated",
        report(record),
    )
}

pub(super) fn run(record: &ArchivedMutationRun) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("baseline", axes::baseline(record.baseline())),
        ("denominator", record.reports().len().into()),
        ("census", axes::census(record.census())),
        ("reports", array(record.reports().iter().map(report))),
    ])
}

pub(super) fn report(record: &ArchivedMutation) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("target", historical_target::target(record.target())),
        ("baseline", axes::baseline(record.baseline())),
        (
            "materialization",
            axes::materialization(record.materialization()),
        ),
        (
            "activation",
            historical_target::activation(record.activation()),
        ),
        ("execution", axes::execution(record.execution())),
        ("equivalence", axes::equivalence(record.equivalence())),
        ("outcome", outcome(record.outcome())),
    ])
}

fn outcome(record: &ArchivedMutationOutcome) -> Value {
    match record {
        ArchivedMutationOutcome::Killed(rejection) => tagged(
            "killed",
            match rejection {
                ArchivedRejection::Demonstrated(rejection) => {
                    tagged("demonstrated", historical::finding(rejection))
                }
                ArchivedRejection::ReportedByBackend(stated) => {
                    tagged("reported-by-backend", historical::foreign(stated))
                }
            },
        ),
        ArchivedMutationOutcome::Survived => tagged("survived", Value::Null),
        ArchivedMutationOutcome::Inconclusive(cause) => tagged("inconclusive", axes::cause(*cause)),
    }
}

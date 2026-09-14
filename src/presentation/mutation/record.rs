//! Complete current mutation runs and individual reports.

use super::{axes, target};
use crate::harness::muterprater::{
    IntendedRejection, MutationOutcome, MutationReport, MutationRun,
};
use crate::presentation::{
    Presentation, outcome,
    value::{array, object, tagged},
};
use serde_json::Value;

/// A mutation run preserving every report and the owner's complete census.
pub fn mutation_run(record: &MutationRun) -> Presentation {
    Presentation::projected(
        "mutation-run",
        "harness/muterprater/verdict",
        "recorded",
        run(record),
    )
}

/// An individual mutation report with its independent axes and outcome evidence.
pub fn mutation_record(record: &MutationReport) -> Presentation {
    Presentation::projected(
        "mutation-report",
        "harness/muterprater/verdict",
        "recorded",
        report(record),
    )
}

pub(super) fn run(record: &MutationRun) -> Value {
    object([
        ("baseline", axes::baseline(record.baseline().axis())),
        ("denominator", record.reports().len().into()),
        ("census", axes::census(record.census())),
        ("reports", array(record.reports().iter().map(report))),
    ])
}

pub(super) fn report(record: &MutationReport) -> Value {
    object([
        ("target", target::target(record.target())),
        ("baseline", axes::baseline(record.baseline())),
        (
            "materialization",
            axes::materialization(record.materialization()),
        ),
        ("activation", target::activation(record.activation())),
        ("execution", axes::execution(record.execution())),
        ("equivalence", axes::equivalence(record.equivalence())),
        ("outcome", result(record.outcome())),
    ])
}

fn result(record: &MutationOutcome) -> Value {
    match record {
        MutationOutcome::Killed(rejection) => tagged(
            "killed",
            match rejection {
                IntendedRejection::Demonstrated(rejection) => tagged(
                    "demonstrated",
                    outcome::finding(rejection.trial(), rejection.finding()),
                ),
                IntendedRejection::ReportedByBackend { stated } => {
                    tagged("reported-by-backend", outcome::foreign(stated))
                }
            },
        ),
        MutationOutcome::Survived => tagged("survived", Value::Null),
        MutationOutcome::Inconclusive(cause) => tagged("inconclusive", axes::cause(*cause)),
    }
}

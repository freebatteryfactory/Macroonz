//! Mutation preparation and observation failures under their existing phases.

use super::{process, refusal};
use crate::harness::muterprater::QualificationRefusal;
use crate::native_mutation::{MutationError, MutationObservationError, MutationPhase};
use crate::native_process::ProcessRun;
use crate::presentation::{
    Presentation,
    mutation::backend_refusal,
    value::{object, tagged},
};
use serde_json::Value;

/// A mutation preparation failure retaining query context and unfinished process custody.
pub fn mutation_error(record: &MutationError) -> Presentation {
    let cause = match record {
        MutationError::Configuration(detail) => tagged("configuration", detail.as_str().into()),
        MutationError::Filesystem(error) => tagged("filesystem", refusal::io_error(error)),
        MutationError::SourceBound { bound } => tagged("source-bound", (*bound).into()),
        MutationError::Process(error) => tagged("process", refusal::process_error(error)),
        MutationError::Query {
            phase,
            request,
            run,
            cause,
        } => {
            let phase = match phase {
                MutationPhase::BackendVersion => "backend-version",
                MutationPhase::CompilerVersion => "compiler-version",
            };
            let run = match run.as_ref() {
                ProcessRun::Finished(output) => process::output(output),
                ProcessRun::Pending(pending) => process::pending(pending),
            };
            tagged(
                "query",
                object([
                    ("phase", phase.into()),
                    ("request", process::request(request)),
                    ("run", run),
                    ("cause", cause.as_str().into()),
                ]),
            )
        }
    };
    Presentation::projected(
        "mutation-error",
        "macroonz/native_mutation",
        "recorded",
        object([("phase", "preparation".into()), ("cause", cause)]),
    )
}

pub(super) fn observation(record: &MutationObservationError) -> Value {
    let cause = match record {
        MutationObservationError::Process => tagged("process", Value::Null),
        MutationObservationError::Console(detail) => tagged("console", detail.as_str().into()),
        MutationObservationError::Roster => tagged("roster", Value::Null),
        MutationObservationError::Sources(detail) => tagged("sources", detail.as_str().into()),
        MutationObservationError::Artifact(error) => {
            tagged("artifact", backend_refusal::manifest(error))
        }
        MutationObservationError::Qualification(error) => {
            tagged("qualification", qualification(error))
        }
    };
    object([("phase", "mutation-observation".into()), ("cause", cause)])
}

fn qualification(record: &QualificationRefusal) -> Value {
    match record {
        QualificationRefusal::GrammarUnchecked => tagged("grammar-unchecked", Value::Null),
        QualificationRefusal::BackendVersionUnstated => {
            tagged("backend-version-unstated", Value::Null)
        }
        QualificationRefusal::CheckedAgainstAnotherVersion { stated, checked } => tagged(
            "checked-against-another-version",
            object([
                ("stated", stated.spelling().into()),
                ("checked", checked.spelling().into()),
            ]),
        ),
    }
}

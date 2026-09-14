//! Existing retained joins and typed failures without new storage or replay operations.

use super::storage;
use crate::presentation::{
    Presentation, admission, benchmark, historical,
    value::{array, object, tagged},
    workflow,
};
use crate::workflow::{RetentionRefusal, StoredRun};
use serde_json::Value;

/// Project the historical run with its original input and every retained capsule.
pub fn stored_run(record: &StoredRun) -> Presentation {
    let capsules = record
        .report()
        .census()
        .iter()
        .enumerate()
        .filter_map(|(row, _entry)| {
            record.capsule(row).map(|capsule| {
                object([
                    ("row", row.into()),
                    ("capsule", historical::capsule_value(capsule)),
                ])
            })
        });
    Presentation::projected(
        "stored-run",
        "macroonz/workflow",
        "historical-unauthenticated",
        object([
            ("input", workflow::input(record.input())),
            ("report", historical::run_value(record.report())),
            ("capsules", array(capsules)),
        ]),
    )
}

/// Project a run retention or saved-witness refusal with its original owning cause.
pub fn retention_refusal(record: &RetentionRefusal) -> Presentation {
    let cause = match record {
        RetentionRefusal::Storage(error) => tagged("storage", storage::cause(error)),
        RetentionRefusal::Archive(error) => tagged("archive", admission::archive(error)),
        RetentionRefusal::Input(error) => tagged("input", admission::input(error)),
        RetentionRefusal::Members => tagged("members", Value::Null),
        RetentionRefusal::InputJoin => tagged("input-join", Value::Null),
        RetentionRefusal::CapsuleJoin => tagged("capsule-join", Value::Null),
        RetentionRefusal::CapsuleAbsent => tagged("capsule-absent", Value::Null),
    };
    Presentation::projected("retention-refusal", "macroonz/workflow", "recorded", cause)
}

/// Project a benchmark retention refusal without replacing the benchmark's conclusions.
pub fn benchmark_retention_refusal(
    record: &crate::workflow::benchmark::RetentionRefusal,
) -> Presentation {
    use crate::workflow::benchmark::RetentionRefusal;
    let cause = match record {
        RetentionRefusal::Storage(error) => tagged("storage", storage::cause(error)),
        RetentionRefusal::Archive(error) => tagged("archive", benchmark::archive_cause(error)),
        RetentionRefusal::Members => tagged("members", Value::Null),
    };
    Presentation::projected(
        "benchmark-retention-refusal",
        "macroonz/workflow/benchmark",
        "recorded",
        cause,
    )
}

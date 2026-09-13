//! The root workflow's existing input/report join and text selection failures.

use super::{
    Presentation, context, record,
    value::{hex, object, tagged},
};
use crate::harness::input::InputEnvelope;
use crate::workflow::{InputRun, SuiteSelectionRefusal};
use serde_json::Value;

/// Project the original admitted specimen beside its complete run report.
pub fn input_run(value: &InputRun) -> Presentation {
    Presentation::projected(
        "input-run",
        "macroonz/workflow",
        "recorded",
        object([
            ("input", input(value.input())),
            ("report", record::run_value(value.report())),
        ]),
    )
}

pub(super) fn input(value: &InputEnvelope) -> Value {
    let profile = value.profile();
    object([
        ("name", context::declared_name(profile.name())),
        ("version", profile.version().into()),
        ("schema", hex(profile.schema().as_bytes())),
        ("case", hex(value.case().address().as_bytes())),
        ("payload", hex(value.payload())),
        ("encoded", hex(value.encoded())),
    ])
}

/// Project the first refused suite request without inventing a partial selection.
pub fn suite_selection_refusal(record: &SuiteSelectionRefusal) -> Presentation {
    let cause = match record {
        SuiteSelectionRefusal::Empty => tagged("empty", Value::Null),
        SuiteSelectionRefusal::Unknown { position } => tagged("unknown", (*position).into()),
        SuiteSelectionRefusal::Duplicate { position } => tagged("duplicate", (*position).into()),
    };
    Presentation::projected(
        "suite-selection-refusal",
        "macroonz/workflow",
        "recorded",
        cause,
    )
}

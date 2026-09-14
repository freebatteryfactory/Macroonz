//! Declared measurement coordinates shared by current and historical rows.

use super::super::value::{array, hex, object, optional};
use crate::harness::bench::{
    BenchStage, ContentionPosture, DeclaredBudgets, InputSizeAxis, WorkFormula,
};
use serde_json::Value;

pub(super) fn measurement(
    axis: &InputSizeAxis,
    budgets: DeclaredBudgets,
    contention: ContentionPosture,
    formula: Option<&WorkFormula>,
) -> Value {
    object([
        (
            "input_sizes",
            array(axis.sizes().iter().copied().map(Value::from)),
        ),
        ("samples", budgets.samples().into()),
        ("warmups", budgets.warmups().into()),
        (
            "ratio",
            object([
                ("numerator", budgets.ratio().numerator().into()),
                ("denominator", budgets.ratio().denominator().into()),
            ]),
        ),
        (
            "contention",
            match contention {
                ContentionPosture::NoDeclaredContention => "no-declared-contention".into(),
            },
        ),
        ("formula", optional(formula, |value| hex(value.bytes()))),
    ])
}

pub(super) fn stage(value: BenchStage) -> Value {
    match value {
        BenchStage::PreflightRefused => "preflight-refused",
        BenchStage::PlantedWorseNotDistinguished => "planted-worse-not-distinguished",
        BenchStage::PrimaryWorkRefused => "primary-work-refused",
        BenchStage::Qualified => "qualified",
    }
    .into()
}

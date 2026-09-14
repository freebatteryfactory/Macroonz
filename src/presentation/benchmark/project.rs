//! Current benchmark readings without folding away reached failure axes.

use super::super::value::{array, hex, object, optional};
use super::super::{Presentation, context, record};
use super::{declaration, work};
use crate::harness::bench::{BenchOutcome, BenchReading, BenchReport, BenchRow};
use serde_json::Value;

/// Project every benchmark row and every observation its reached stage retains.
pub fn benchmark(value: &BenchReport) -> Presentation {
    Presentation::projected(
        "benchmark",
        "macroonz-harness/bench",
        "recorded",
        object([
            ("table", context::declared_name(value.table().name())),
            ("provenance", context::provenance(value.provenance())),
            ("denominator", value.denominator().into()),
            ("readings", array(value.readings().iter().map(reading))),
        ]),
    )
}

fn reading(value: &BenchReading) -> Value {
    object([
        ("row", row(value.row())),
        ("target", context::target(value.target())),
        ("preflight", record::trial_value(value.preflight())),
        ("outcome", outcome(value.outcome())),
    ])
}

fn row(value: &BenchRow) -> Value {
    object([
        ("key", hex(value.key().address().as_bytes())),
        ("workload", context::declared_name(value.workload().name())),
        (
            "preflight",
            context::declared_name(value.preflight().name()),
        ),
        (
            "planted_worse",
            context::declared_name(value.planted_worse().name()),
        ),
        (
            "complexity",
            context::declared_name(value.complexity().name()),
        ),
        (
            "measurement",
            declaration::measurement(
                value.input_sizes(),
                value.budgets(),
                value.contention(),
                value.formula(),
            ),
        ),
    ])
}

fn outcome(value: &BenchOutcome) -> Value {
    let observed = match value {
        BenchOutcome::PreflightRefused => None,
        BenchOutcome::PlantedWorseNotDistinguished {
            measured,
            planted_worse,
            judgment,
        }
        | BenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        } => Some((measured, planted_worse, judgment, None)),
        BenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            secondary,
            ..
        } => Some((measured, planted_worse, judgment, Some(secondary))),
    };
    object([
        ("stage", declaration::stage(value.stage())),
        (
            "measured",
            optional(observed.map(|reading| reading.0), work::curve),
        ),
        (
            "planted_worse",
            optional(observed.map(|reading| reading.1), work::curve),
        ),
        (
            "judgment",
            optional(observed.map(|reading| reading.2), work::judgment),
        ),
        (
            "secondary",
            optional(observed.and_then(|reading| reading.3), work::secondary),
        ),
    ])
}

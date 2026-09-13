//! Complete historical benchmark observations without current execution authority.

use super::super::value::{array, hex, object, optional};
use super::super::{Presentation, context, historical};
use super::{declaration, historical_work as work};
use crate::harness::bench::archive::{
    ArchivedBenchOutcome, ArchivedBenchReading, ArchivedBenchReport, ArchivedBenchRow,
};
use serde_json::Value;

/// Project an admitted historical benchmark, preserving its full denominator and standing.
pub fn archived_benchmark(value: &ArchivedBenchReport) -> Presentation {
    Presentation::projected(
        "benchmark",
        "macroonz-harness/bench",
        "historical-unauthenticated",
        object([
            ("archive_address", hex(value.address().as_bytes())),
            (
                "table",
                context::name(value.table().namespace(), value.table().stem()),
            ),
            (
                "provenance",
                context::historical_provenance(value.provenance()),
            ),
            ("denominator", value.denominator().into()),
            ("readings", array(value.readings().iter().map(reading))),
        ]),
    )
}

fn reading(value: &ArchivedBenchReading) -> Value {
    object([
        ("row", row(value.row())),
        ("target", context::target(value.target())),
        ("preflight", historical::trial_value(value.preflight())),
        ("outcome", outcome(value.outcome())),
    ])
}

fn row(value: &ArchivedBenchRow) -> Value {
    let measurement = value.measurement();
    object([
        ("key", hex(value.key().as_bytes())),
        (
            "workload",
            context::name(value.workload().namespace(), value.workload().stem()),
        ),
        (
            "preflight",
            context::name(value.preflight().namespace(), value.preflight().stem()),
        ),
        (
            "planted_worse",
            context::name(
                value.planted_worse().namespace(),
                value.planted_worse().stem(),
            ),
        ),
        (
            "complexity",
            context::name(value.complexity().namespace(), value.complexity().stem()),
        ),
        (
            "measurement",
            declaration::measurement(
                measurement.input_sizes(),
                measurement.budgets(),
                measurement.contention(),
                measurement.formula(),
            ),
        ),
    ])
}

fn outcome(value: &ArchivedBenchOutcome) -> Value {
    let observed = match value {
        ArchivedBenchOutcome::PreflightRefused => None,
        ArchivedBenchOutcome::PlantedWorseNotDistinguished {
            measured,
            planted_worse,
            judgment,
        }
        | ArchivedBenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        } => Some((measured, planted_worse, judgment, None)),
        ArchivedBenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            secondary,
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

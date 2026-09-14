//! Complete nested causes of historical benchmark admission failures.

use crate::harness::bench::archive::BenchArchiveRefusal;
use crate::harness::bench::{
    BenchRowRefusal, DeclaredBudgetsRefusal, InputSizeAxisRefusal, WorkFormulaRefusal,
};
use crate::harness::descriptor::EncodeRefusal;
use crate::presentation::{
    Presentation, admission, descriptor_refusal,
    value::{object, tagged},
};
use serde_json::Value;

/// Project a benchmark archive refusal with its complete nested owner cause.
pub fn benchmark_archive_refusal(record: &BenchArchiveRefusal) -> Presentation {
    Presentation::projected(
        "benchmark-archive-refusal",
        "macroonz-harness/bench/archive",
        "recorded",
        cause(record),
    )
}

pub(in crate::presentation) fn cause(record: &BenchArchiveRefusal) -> Value {
    match record {
        BenchArchiveRefusal::Canonical(error) => tagged("canonical", admission::archive(error)),
        BenchArchiveRefusal::Declaration(BenchRowRefusal::Encoding(
            EncodeRefusal::LengthPastEncodingWidth,
        )) => tagged(
            "declaration",
            tagged(
                "encoding",
                tagged("length-past-encoding-width", Value::Null),
            ),
        ),
        BenchArchiveRefusal::Axis(error) => tagged("axis", axis(error)),
        BenchArchiveRefusal::Budgets(error) => tagged("budgets", budgets(*error)),
        BenchArchiveRefusal::Formula(WorkFormulaRefusal::Empty) => {
            tagged("formula", tagged("empty", Value::Null))
        }
        BenchArchiveRefusal::Provenance(error) => {
            tagged("provenance", descriptor_refusal::binding(error))
        }
        BenchArchiveRefusal::EmptyReport => tagged("empty-report", Value::Null),
        BenchArchiveRefusal::TooManyRows => tagged("too-many-rows", Value::Null),
        BenchArchiveRefusal::TooManySizes => tagged("too-many-sizes", Value::Null),
        BenchArchiveRefusal::TooManyObservations => tagged("too-many-observations", Value::Null),
        BenchArchiveRefusal::TooManyMeasurements => tagged("too-many-measurements", Value::Null),
        BenchArchiveRefusal::DuplicateRow => tagged("duplicate-row", Value::Null),
        BenchArchiveRefusal::WorkPopulationMismatch => {
            tagged("work-population-mismatch", Value::Null)
        }
        BenchArchiveRefusal::StageMismatch => tagged("stage-mismatch", Value::Null),
        BenchArchiveRefusal::TargetMismatch => tagged("target-mismatch", Value::Null),
    }
}

fn axis(record: &InputSizeAxisRefusal) -> Value {
    match record {
        InputSizeAxisRefusal::TooShort { found } => tagged("too-short", (*found).into()),
        InputSizeAxisRefusal::DuplicateSize {
            size,
            first,
            duplicate,
        } => tagged(
            "duplicate-size",
            object([
                ("size", (*size).into()),
                ("first", (*first).into()),
                ("duplicate", (*duplicate).into()),
            ]),
        ),
    }
}

fn budgets(record: DeclaredBudgetsRefusal) -> Value {
    tagged(
        match record {
            DeclaredBudgetsRefusal::NoSamples => "no-samples",
            DeclaredBudgetsRefusal::ZeroRatioNumerator => "zero-ratio-numerator",
            DeclaredBudgetsRefusal::ZeroRatioDenominator => "zero-ratio-denominator",
        },
        Value::Null,
    )
}

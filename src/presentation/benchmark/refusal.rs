//! No-report causes and first-row verdicts remain different typed observations.

use super::super::value::{hex, object, tagged};
use super::super::{Presentation, context};
use super::{declaration, work};
use crate::harness::bench::{
    BenchRunRefusal, BenchTargetMismatch, BenchVerdictRefusal, PrimaryWorkPhase,
    SecondaryObservationRefusal, WorkQualificationRefusal, WorkRecordingRefusal,
};
use crate::harness::descriptor::NameRefusal;
use serde_json::Value;

/// Project why a benchmark produced no report, retaining its nested owner cause.
pub fn benchmark_refusal(value: &BenchRunRefusal) -> Presentation {
    let record = match value {
        BenchRunRefusal::PreflightTargetMismatch {
            row,
            mismatch: reason,
        } => tagged(
            "preflight-target-mismatch",
            object([
                ("row", hex(row.address().as_bytes())),
                ("mismatch", mismatch(reason)),
            ]),
        ),
        BenchRunRefusal::WorkNotRecorded {
            row,
            phase,
            refusal,
        } => tagged(
            "work-not-recorded",
            object([
                ("row", hex(row.address().as_bytes())),
                (
                    "phase",
                    match phase {
                        PrimaryWorkPhase::Measured => "measured".into(),
                        PrimaryWorkPhase::PlantedWorse => "planted-worse".into(),
                    },
                ),
                ("refusal", recording(refusal)),
            ]),
        ),
        BenchRunRefusal::SecondaryWorkRefused { row, refusal } => tagged(
            "secondary-work-refused",
            object([
                ("row", hex(row.address().as_bytes())),
                ("refusal", secondary(refusal)),
            ]),
        ),
    };
    Presentation::projected(
        "benchmark-run-refusal",
        "macroonz-harness/bench",
        "recorded",
        record,
    )
}

/// Project the benchmark owner's first nonqualifying row from an existing report.
pub fn benchmark_verdict(value: &BenchVerdictRefusal) -> Presentation {
    Presentation::projected(
        "benchmark-verdict-refusal",
        "macroonz-harness/bench",
        "recorded",
        object([
            ("row", hex(value.row().address().as_bytes())),
            ("stage", declaration::stage(value.stage())),
        ]),
    )
}

fn mismatch(value: &BenchTargetMismatch) -> Value {
    match value {
        BenchTargetMismatch::Target {
            benchmark,
            preflight,
        } => tagged(
            "target",
            object([
                ("benchmark", benchmark.spelling().into()),
                ("preflight", preflight.spelling().into()),
            ]),
        ),
        BenchTargetMismatch::Toolchain {
            benchmark,
            preflight,
        } => tagged(
            "toolchain",
            object([
                ("benchmark", benchmark.spelling().into()),
                ("preflight", preflight.spelling().into()),
            ]),
        ),
    }
}

fn recording(value: &WorkRecordingRefusal) -> Value {
    match value {
        WorkRecordingRefusal::ObservationName(reason) => tagged(
            "observation-name",
            match reason {
                NameRefusal::EmptyNamespace => "empty-namespace".into(),
                NameRefusal::EmptyStem => "empty-stem".into(),
            },
        ),
        WorkRecordingRefusal::UnknownObservation(observation) => tagged(
            "unknown-observation",
            context::declared_name(observation.name()),
        ),
        WorkRecordingRefusal::AmountOverflow {
            observation,
            input_size,
        } => tagged(
            "amount-overflow",
            object([
                ("observation", context::declared_name(observation.name())),
                ("input_size", (*input_size).into()),
            ]),
        ),
        WorkRecordingRefusal::CountOverflow {
            observation,
            current,
            addition,
        } => tagged(
            "count-overflow",
            object([
                ("observation", context::declared_name(observation.name())),
                ("current", (*current).into()),
                ("addition", (*addition).into()),
            ]),
        ),
    }
}

fn secondary(value: &SecondaryObservationRefusal) -> Value {
    match value {
        SecondaryObservationRefusal::Warmup(reason) => tagged("warmup", recording(reason)),
        SecondaryObservationRefusal::Sample(reason) => tagged("sample", recording(reason)),
        SecondaryObservationRefusal::Judgment(reason) => tagged("judgment", qualification(reason)),
    }
}

fn qualification(value: &WorkQualificationRefusal) -> Value {
    match value {
        WorkQualificationRefusal::PlantedWorseNotDistinguished { planted_worse, gap } => tagged(
            "planted-worse-not-distinguished",
            object([
                ("planted_worse", work::conclusion(*planted_worse)),
                ("gap", work::gap(*gap)),
            ]),
        ),
        WorkQualificationRefusal::MeasuredRefused(reason) => {
            tagged("measured-refused", work::conclusion(*reason))
        }
    }
}

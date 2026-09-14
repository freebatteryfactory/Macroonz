//! Exact primary work and independent secondary observations.

use super::super::value::{array, object, tagged};
use super::super::{clock, context};
use crate::harness::bench::{
    SecondaryObservation, WorkConclusion, WorkCurve, WorkGapStanding, WorkJudgment,
};
use crate::harness::report::FindingCause;
use serde_json::Value;

fn cause(value: FindingCause) -> Value {
    object([
        ("family", value.family().into()),
        ("local", value.local().into()),
    ])
}

pub(super) fn conclusion(value: WorkConclusion) -> Value {
    match value {
        WorkConclusion::Satisfied => tagged("satisfied", Value::Null),
        WorkConclusion::Refused(reason) => tagged("refused", cause(reason)),
    }
}

pub(super) fn gap(value: WorkGapStanding) -> Value {
    match value {
        WorkGapStanding::Distinguished => tagged("distinguished", Value::Null),
        WorkGapStanding::NotDistinguished(reason) => tagged("not-distinguished", cause(reason)),
    }
}

pub(super) fn judgment(value: &WorkJudgment) -> Value {
    object([
        ("measured", conclusion(value.measured())),
        ("planted_worse", conclusion(value.planted_worse())),
        ("gap", gap(value.gap())),
    ])
}

pub(super) fn curve(value: &WorkCurve) -> Value {
    array(value.points().iter().map(|point| {
        object([
            ("input_size", point.input_size().into()),
            (
                "counts",
                array(point.counts().iter().map(|count| {
                    object([
                        (
                            "observation",
                            context::declared_name(count.observation().name()),
                        ),
                        ("count", count.count().into()),
                    ])
                })),
            ),
        ])
    }))
}

pub(super) fn secondary(value: &SecondaryObservation) -> Value {
    object([
        ("work", curve(value.work())),
        ("judgment", judgment(&value.judgment())),
        (
            "measurements",
            array(value.measurements().iter().copied().map(clock::measurement)),
        ),
        (
            "clock_attribution",
            clock::attribution(value.clock_attribution()),
        ),
    ])
}

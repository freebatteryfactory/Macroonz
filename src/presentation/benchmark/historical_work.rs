//! Retained work facts retain all judgments and measurement failures.

use super::super::value::{array, object, tagged};
use super::super::{clock, context};
use crate::harness::bench::archive::{
    ArchivedSecondaryObservation, ArchivedWorkCause, ArchivedWorkConclusion, ArchivedWorkCurve,
    ArchivedWorkGap, ArchivedWorkJudgment,
};
use serde_json::Value;

fn cause(value: &ArchivedWorkCause) -> Value {
    object([
        ("family", value.family().into()),
        ("local", value.local().into()),
    ])
}

fn conclusion(value: &ArchivedWorkConclusion) -> Value {
    match value {
        ArchivedWorkConclusion::Satisfied => tagged("satisfied", Value::Null),
        ArchivedWorkConclusion::Refused(reason) => tagged("refused", cause(reason)),
    }
}

fn gap(value: &ArchivedWorkGap) -> Value {
    match value {
        ArchivedWorkGap::Distinguished => tagged("distinguished", Value::Null),
        ArchivedWorkGap::NotDistinguished(reason) => tagged("not-distinguished", cause(reason)),
    }
}

pub(super) fn judgment(value: &ArchivedWorkJudgment) -> Value {
    object([
        ("measured", conclusion(value.measured())),
        ("planted_worse", conclusion(value.planted_worse())),
        ("gap", gap(value.gap())),
    ])
}

pub(super) fn curve(value: &ArchivedWorkCurve) -> Value {
    array(value.points().iter().map(|point| {
        object([
            ("input_size", point.input_size().into()),
            (
                "counts",
                array(point.counts().iter().map(|count| {
                    object([
                        (
                            "observation",
                            context::name(
                                count.observation().namespace(),
                                count.observation().stem(),
                            ),
                        ),
                        ("count", count.count().into()),
                    ])
                })),
            ),
        ])
    }))
}

pub(super) fn secondary(value: &ArchivedSecondaryObservation) -> Value {
    object([
        ("work", curve(value.work())),
        ("judgment", judgment(value.judgment())),
        (
            "measurements",
            array(
                value
                    .measurements()
                    .iter()
                    .copied()
                    .map(clock::historical_measurement),
            ),
        ),
        (
            "clock_attribution",
            clock::attribution(value.clock_attribution()),
        ),
    ])
}

//! Measurement display preserves absence, failure and caller attribution.

use super::value::{object, tagged};
use crate::harness::clock::{ClockAttribution, ClockFailure, MeasurementReading};
use crate::harness::report::archive::{ArchivedClockFailure, ArchivedMeasurement};
use serde_json::Value;

pub(super) fn attribution(value: ClockAttribution) -> Value {
    match value {
        ClockAttribution::Unspecified => "unspecified",
        ClockAttribution::Synthetic => "synthetic",
        ClockAttribution::Monotonic => "monotonic",
    }
    .into()
}

pub(super) fn measurement(value: MeasurementReading) -> Value {
    match value {
        MeasurementReading::Observed(duration) => tagged("observed", duration.nanoseconds().into()),
        MeasurementReading::Unavailable => tagged("unavailable", Value::Null),
        MeasurementReading::Failed(failure) => tagged("failed", clock_failure(failure)),
    }
}

fn clock_failure(value: ClockFailure) -> Value {
    match value {
        ClockFailure::OpeningRefused => "opening-refused".into(),
        ClockFailure::ClosingRefused => "closing-refused".into(),
        ClockFailure::OpeningUnwound => "opening-unwound".into(),
        ClockFailure::ClosingUnwound => "closing-unwound".into(),
        ClockFailure::Regressed { opened, closed } => {
            regression(opened.nanoseconds(), closed.nanoseconds())
        }
    }
}

fn regression(opened: u64, closed: u64) -> Value {
    tagged(
        "regressed",
        object([("opened", opened.into()), ("closed", closed.into())]),
    )
}

pub(super) fn historical_measurement(value: ArchivedMeasurement) -> Value {
    match value {
        ArchivedMeasurement::Observed(nanoseconds) => tagged("observed", nanoseconds.into()),
        ArchivedMeasurement::Unavailable => tagged("unavailable", Value::Null),
        ArchivedMeasurement::Failed(failure) => tagged("failed", historical_failure(failure)),
    }
}

fn historical_failure(value: ArchivedClockFailure) -> Value {
    match value {
        ArchivedClockFailure::OpeningRefused => "opening-refused".into(),
        ArchivedClockFailure::ClosingRefused => "closing-refused".into(),
        ArchivedClockFailure::OpeningUnwound => "opening-unwound".into(),
        ArchivedClockFailure::ClosingUnwound => "closing-unwound".into(),
        ArchivedClockFailure::Regressed { opened, closed } => regression(opened, closed),
    }
}

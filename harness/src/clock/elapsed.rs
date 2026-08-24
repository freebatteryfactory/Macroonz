//! The checked difference between the two ticks of one measurement.
//!
//! A backwards pair refuses as [`ClockFailure::Regressed`] rather than saturating to zero.

use super::{ClockFailure, MeasurementTick, RecordedDuration};

pub(in crate::clock) fn elapsed(
    opened: MeasurementTick,
    closed: MeasurementTick,
) -> Result<RecordedDuration, ClockFailure> {
    let Some(nanoseconds) = closed.nanoseconds().checked_sub(opened.nanoseconds()) else {
        return Err(ClockFailure::Regressed { opened, closed });
    };
    Ok(RecordedDuration::recorded(nanoseconds))
}

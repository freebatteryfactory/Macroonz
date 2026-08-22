//! The pure checked difference between two ticks from one retained measurement start.

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

//! Checked nanosecond readings from one process-local native origin.

use crate::harness::clock::ClockReadRefusal;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

static ORIGIN: OnceLock<Instant> = OnceLock::new();

pub(super) fn native() -> Result<u64, ClockReadRefusal> {
    let origin = ORIGIN.get_or_init(Instant::now);
    offset(*origin, Instant::now())
}

pub(super) fn offset(origin: Instant, reading: Instant) -> Result<u64, ClockReadRefusal> {
    if reading < origin {
        return Err(ClockReadRefusal::Refused);
    }
    let duration = reading
        .checked_duration_since(origin)
        .ok_or(ClockReadRefusal::Refused)?;
    nanoseconds(duration)
}

pub(super) fn nanoseconds(duration: Duration) -> Result<u64, ClockReadRefusal> {
    u64::try_from(duration.as_nanos()).map_err(|_| ClockReadRefusal::Refused)
}

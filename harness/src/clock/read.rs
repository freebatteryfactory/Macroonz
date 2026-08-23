//! The effect boundary that opens and finishes one `TestPak` wall measurement.

use super::elapsed::elapsed;
use super::{
    ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading, MeasurementStart,
    MeasurementTick,
};
use std::panic::catch_unwind;

#[derive(Debug, Clone, Copy)]
pub(in crate::clock) enum ClockSource {
    Unavailable,
    Available(AvailableClockSource),
}

#[derive(Debug, Clone, Copy)]
pub(in crate::clock) enum AvailableClockSource {
    Infallible(fn() -> u64),
    Fallible(fn() -> Result<u64, ClockReadRefusal>),
}

#[derive(Debug)]
pub(in crate::clock) enum MeasurementOpening {
    Unavailable,
    Failed(ClockFailure),
    Opened {
        source: AvailableClockSource,
        tick: MeasurementTick,
    },
}

enum ReadOutcome {
    Tick(MeasurementTick),
    Refused,
    Unwound,
}

fn read(source: AvailableClockSource) -> ReadOutcome {
    match source {
        AvailableClockSource::Infallible(read) => match catch_unwind(read) {
            Ok(nanoseconds) => ReadOutcome::Tick(MeasurementTick::admitted(nanoseconds)),
            Err(_) => ReadOutcome::Unwound,
        },
        AvailableClockSource::Fallible(read) => match catch_unwind(read) {
            Ok(Ok(nanoseconds)) => ReadOutcome::Tick(MeasurementTick::admitted(nanoseconds)),
            Ok(Err(ClockReadRefusal::Refused)) => ReadOutcome::Refused,
            Err(_) => ReadOutcome::Unwound,
        },
    }
}

impl HarnessClock {
    /// Open one wall measurement without granting the reading any semantic authority.
    ///
    /// An unavailable clock performs no source read. A typed refusal or ordinary Rust unwind is retained in the returned start so caller work can still run before [`MeasurementStart::finish`] publishes the final reading.
    pub fn begin(self) -> MeasurementStart {
        let opening = match self.source {
            ClockSource::Unavailable => MeasurementOpening::Unavailable,
            ClockSource::Available(source) => match read(source) {
                ReadOutcome::Tick(tick) => MeasurementOpening::Opened { source, tick },
                ReadOutcome::Refused => MeasurementOpening::Failed(ClockFailure::OpeningRefused),
                ReadOutcome::Unwound => MeasurementOpening::Failed(ClockFailure::OpeningUnwound),
            },
        };
        MeasurementStart { opening }
    }
}

impl MeasurementStart {
    /// Finish this measurement against the exact source retained at opening.
    ///
    /// The operation reads no source for an unavailable or failed opening. A successful close computes elapsed time through checked subtraction, so a regression is a typed failure and an observed zero remains a real zero.
    pub fn finish(self) -> MeasurementReading {
        match self.opening {
            MeasurementOpening::Unavailable => MeasurementReading::Unavailable,
            MeasurementOpening::Failed(failure) => MeasurementReading::Failed(failure),
            MeasurementOpening::Opened { source, tick } => match read(source) {
                ReadOutcome::Tick(closed) => match elapsed(tick, closed) {
                    Ok(duration) => MeasurementReading::Observed(duration),
                    Err(failure) => MeasurementReading::Failed(failure),
                },
                ReadOutcome::Refused => MeasurementReading::Failed(ClockFailure::ClosingRefused),
                ReadOutcome::Unwound => MeasurementReading::Failed(ClockFailure::ClosingUnwound),
            },
        }
    }
}

//! The effect boundary: one source read opens a measurement, a second finishes it.

use super::elapsed::elapsed;
use super::{
    ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading, MeasurementStart,
    MeasurementTick,
};
use std::panic::catch_unwind;

/// What a caller declared: a reader, or nothing at all.
#[derive(Debug, Clone, Copy)]
pub(in crate::clock) enum Source {
    Unavailable,
    Available(Reader),
}

/// The two shapes a caller's reading function may take.
#[derive(Debug, Clone, Copy)]
pub(in crate::clock) enum Reader {
    Infallible(fn() -> u64),
    Fallible(fn() -> Result<u64, ClockReadRefusal>),
}

/// What an opening left behind, including the reader a successful one retained.
#[derive(Debug)]
pub(in crate::clock) enum Opening {
    Unavailable,
    Failed(ClockFailure),
    Opened {
        reader: Reader,
        tick: MeasurementTick,
    },
}

/// What one call into a caller's reader produced.
enum ReadOutcome {
    Tick(MeasurementTick),
    Refused,
    Unwound,
}

fn read_once(reader: Reader) -> ReadOutcome {
    match reader {
        Reader::Infallible(read) => match catch_unwind(read) {
            Ok(nanoseconds) => ReadOutcome::Tick(MeasurementTick::admitted(nanoseconds)),
            Err(_) => ReadOutcome::Unwound,
        },
        Reader::Fallible(read) => match catch_unwind(read) {
            Ok(Ok(nanoseconds)) => ReadOutcome::Tick(MeasurementTick::admitted(nanoseconds)),
            Ok(Err(ClockReadRefusal::Refused)) => ReadOutcome::Refused,
            Err(_) => ReadOutcome::Unwound,
        },
    }
}

fn elapsed_reading(opened: MeasurementTick, closed: MeasurementTick) -> MeasurementReading {
    match elapsed(opened, closed) {
        Ok(duration) => MeasurementReading::Observed(duration),
        Err(failure) => MeasurementReading::Failed(failure),
    }
}

fn close_opened(reader: Reader, opened: MeasurementTick) -> MeasurementReading {
    match read_once(reader) {
        ReadOutcome::Tick(closed) => elapsed_reading(opened, closed),
        ReadOutcome::Refused => MeasurementReading::Failed(ClockFailure::ClosingRefused),
        ReadOutcome::Unwound => MeasurementReading::Failed(ClockFailure::ClosingUnwound),
    }
}

fn finish_opening(opening: &Opening) -> MeasurementReading {
    match opening {
        Opening::Unavailable => MeasurementReading::Unavailable,
        Opening::Failed(failure) => MeasurementReading::Failed(*failure),
        Opening::Opened { reader, tick } => close_opened(*reader, *tick),
    }
}

impl HarnessClock {
    /// Open one wall measurement, granting the reading no authority over anything.
    ///
    /// An unavailable clock reads nothing.
    /// A refusal or an unwind here is retained in the start, so the caller's work still runs before [`MeasurementStart::finish`] publishes the reading.
    pub fn begin(self) -> MeasurementStart {
        let opening = match self.source {
            Source::Unavailable => Opening::Unavailable,
            Source::Available(reader) => match read_once(reader) {
                ReadOutcome::Tick(tick) => Opening::Opened { reader, tick },
                ReadOutcome::Refused => Opening::Failed(ClockFailure::OpeningRefused),
                ReadOutcome::Unwound => Opening::Failed(ClockFailure::OpeningUnwound),
            },
        };
        MeasurementStart { opening }
    }
}

impl MeasurementStart {
    /// Finish this measurement against the exact reader it opened on.
    ///
    /// An unavailable or failed opening reads nothing further.
    /// A second admitted tick becomes a duration by checked subtraction, so an observed zero stays a zero and a backwards pair becomes [`ClockFailure::Regressed`].
    pub fn finish(self) -> MeasurementReading {
        finish_opening(&self.opening)
    }
}

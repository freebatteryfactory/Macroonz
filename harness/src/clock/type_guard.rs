//! Declaring a clock, admitting a tick, and reading a finished measurement.

use super::{
    ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading, MeasurementTick,
    RecordedDuration,
};
use crate::clock::read::{Reader, Source};

impl HarnessClock {
    /// Declare an infallible caller function as the source.
    #[must_use]
    pub const fn reading(read: fn() -> u64) -> Self {
        Self {
            source: Source::Available(Reader::Infallible(read)),
        }
    }

    /// Declare a caller function that may refuse a read instead of unwinding.
    #[must_use]
    pub const fn fallible(read: fn() -> Result<u64, ClockReadRefusal>) -> Self {
        Self {
            source: Source::Available(Reader::Fallible(read)),
        }
    }

    /// Declare that this run offers no wall measurement.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            source: Source::Unavailable,
        }
    }
}

impl MeasurementTick {
    /// Admit one source reading as a tick.
    pub(in crate::clock) const fn admitted(nanoseconds: u64) -> Self {
        Self(nanoseconds)
    }

    /// The reading in nanoseconds, on the caller source's own origin.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.0
    }
}

impl RecordedDuration {
    /// Record a caller-observed duration in nanoseconds.
    #[must_use]
    pub const fn recorded(nanoseconds: u64) -> Self {
        Self(nanoseconds)
    }

    /// The recorded nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.0
    }
}

impl MeasurementReading {
    /// The duration, where both source reads completed in order.
    #[must_use]
    pub const fn duration(self) -> Option<RecordedDuration> {
        match self {
            Self::Observed(duration) => Some(duration),
            Self::Unavailable | Self::Failed(_) => None,
        }
    }

    /// The failure, where an offered measurement did not complete.
    #[must_use]
    pub const fn failure(self) -> Option<ClockFailure> {
        match self {
            Self::Failed(failure) => Some(failure),
            Self::Observed(_) | Self::Unavailable => None,
        }
    }
}

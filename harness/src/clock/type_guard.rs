//! Smart constructors and readers for `TestPak`'s wall-measurement vocabulary.

use super::{HarnessClock, MeasurementReading, MeasurementTick, RecordedDuration};
use crate::clock::read::{AvailableClockSource, ClockSource};

impl HarnessClock {
    /// Declare an infallible caller function as the wall-measurement source.
    #[must_use]
    pub const fn reading(read: fn() -> u64) -> Self {
        Self {
            source: ClockSource::Available(AvailableClockSource::Infallible(read)),
        }
    }

    /// Declare a fallible caller function as the wall-measurement source.
    #[must_use]
    pub const fn fallible(read: fn() -> Result<u64, super::ClockReadRefusal>) -> Self {
        Self {
            source: ClockSource::Available(AvailableClockSource::Fallible(read)),
        }
    }

    /// Declare that this run offers no wall measurement.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            source: ClockSource::Unavailable,
        }
    }
}

impl MeasurementTick {
    /// Admit one caller-source reading as a tick.
    #[must_use]
    pub(in crate::clock) const fn admitted(nanoseconds: u64) -> Self {
        Self(nanoseconds)
    }

    /// The caller-source reading in nanoseconds on its own origin.
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
    /// The observed duration, where both source reads completed in order.
    #[must_use]
    pub const fn duration(self) -> Option<RecordedDuration> {
        match self {
            Self::Observed(duration) => Some(duration),
            Self::Unavailable | Self::Failed(_) => None,
        }
    }

    /// The typed failure, where an offered measurement did not complete.
    #[must_use]
    pub const fn failure(self) -> Option<super::ClockFailure> {
        match self {
            Self::Failed(failure) => Some(failure),
            Self::Observed(_) | Self::Unavailable => None,
        }
    }
}

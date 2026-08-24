//! The declared clock, the readings it produces, and why a reading can fail.

use super::read::{Opening, Source};

#[path = "type_guard.rs"]
mod guard;

/// The wall-measurement source a caller declares for one run.
#[derive(Debug, Clone, Copy)]
pub struct HarnessClock {
    pub(in crate::clock) source: Source,
}

/// An open measurement, finishable exactly once and only against the source it opened on.
#[must_use = "a measurement start must be finished to produce its reading"]
#[derive(Debug)]
pub struct MeasurementStart {
    pub(in crate::clock) opening: Opening,
}

/// One admitted reading in nanoseconds, on the caller source's own origin.
///
/// A tick is not a duration.
/// Only [`MeasurementStart::finish`](crate::clock::MeasurementStart::finish) turns two of them into one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementTick(u64);

/// One observed elapsed duration in nanoseconds.
///
/// Zero is a real observation and never spells unavailable measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordedDuration(u64);

/// A fallible caller source's stated read failure.
#[must_use = "a refusal is the caller source's stated read failure"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockReadRefusal {
    /// The source produced no reading.
    Refused,
}

/// Why an offered measurement produced no duration.
#[must_use = "a failure is why an offered measurement produced no duration"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockFailure {
    /// The opening read returned a typed refusal.
    OpeningRefused,
    /// The closing read returned a typed refusal.
    ClosingRefused,
    /// The opening read unwound.
    OpeningUnwound,
    /// The closing read unwound.
    ClosingUnwound,
    /// The closing tick preceded the opening tick on the same source.
    Regressed {
        /// The admitted opening tick.
        opened: MeasurementTick,
        /// The admitted closing tick.
        closed: MeasurementTick,
    },
}

/// The complete wall reading one run leaves in its report.
#[must_use = "a measurement reading is a report fact"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementReading {
    /// Both ticks were admitted in order, and this is their checked difference.
    Observed(RecordedDuration),
    /// The caller declared no clock for this run.
    Unavailable,
    /// A clock was offered and the measurement did not complete.
    Failed(ClockFailure),
}

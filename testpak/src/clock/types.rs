//! `TestPak`'s caller clock, admitted tick, one-use measurement start, and wall-reading outcomes.

use super::read::{ClockSource, MeasurementOpening};

#[path = "type_guard.rs"]
mod guard;

/// The caller-declared source for `TestPak` wall measurements.
///
/// # Authority
///
/// The source supplies nanosecond readings on its own origin. `TestPak` reads only differences and never turns a clock reading into semantic identity or a verdict.
///
/// # Construction
///
/// [`HarnessClock::reading`] declares an infallible function pointer, [`HarnessClock::fallible`] declares one with a typed refusal, and [`HarnessClock::unavailable`] declares that no measurement is offered.
#[derive(Debug, Clone, Copy)]
pub struct HarnessClock {
    pub(in crate::clock) source: ClockSource,
}

/// One opening posture that can be finished exactly once against its retained clock.
///
/// The value is opaque and its finish operation consumes it, so a caller cannot replace the clock, reverse the tick order, or close one measurement twice.
#[must_use = "a measurement start must be finished to produce its reading"]
#[derive(Debug)]
pub struct MeasurementStart {
    pub(in crate::clock) opening: MeasurementOpening,
}

/// One admitted nanosecond reading on a caller clock's own origin.
///
/// A tick is not a duration. `TestPak` exposes it where a regression must name both readings, while the checked elapsed operation remains behind [`MeasurementStart::finish`](crate::clock::MeasurementStart::finish).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementTick(u64);

/// One observed elapsed duration in nanoseconds.
///
/// A zero value is a real observation and is never used to spell unavailable measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordedDuration(u64);

/// A typed refusal returned by a fallible caller clock read.
///
/// The clock subsystem records whether the refusal occurred while opening or closing the measurement.
#[must_use = "a refusal is the caller clock's stated read failure"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockReadRefusal {
    /// The caller clock did not produce a reading.
    Refused,
}

/// Why an offered wall measurement did not produce a duration.
///
/// Source refusal and ordinary Rust unwind retain their opening or closing stage. A regression retains both admitted ticks instead of collapsing their difference to zero.
#[must_use = "a failure is why an offered wall measurement did not complete"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockFailure {
    /// The first source read returned a typed refusal.
    OpeningRefused,
    /// The second source read returned a typed refusal.
    ClosingRefused,
    /// The first source read unwound through the ordinary Rust unwind mechanism.
    OpeningUnwound,
    /// The second source read unwound through the ordinary Rust unwind mechanism.
    ClosingUnwound,
    /// The closing tick preceded the opening tick on the same caller source.
    Regressed {
        /// The admitted opening tick.
        opened: MeasurementTick,
        /// The admitted closing tick.
        closed: MeasurementTick,
    },
}

/// The complete wall-measurement reading retained by `TestPak` reports.
///
/// `Observed(RecordedDuration::recorded(0))` is distinct from [`MeasurementReading::Unavailable`]. A failed reading preserves why no duration exists rather than inventing one.
#[must_use = "a measurement reading is a report fact"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementReading {
    /// Both ticks were admitted in order and produced this checked duration.
    Observed(RecordedDuration),
    /// The caller declared that this run offered no wall measurement.
    Unavailable,
    /// An offered measurement could not be completed.
    Failed(ClockFailure),
}

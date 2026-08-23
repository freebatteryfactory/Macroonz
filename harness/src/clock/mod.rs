#![doc = include_str!("README.md")]

mod elapsed;
mod read;
mod types;

pub use types::{
    ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading, MeasurementStart,
    MeasurementTick, RecordedDuration,
};

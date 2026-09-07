#![doc = include_str!("README.md")]

mod compare;
mod read;
mod types;

pub use compare::compare;
pub use types::{
    HistoricalReplayRefusal, HistoricalReplayStanding, ReplayCoordinate, ReplayJoinRefusal,
    ReplayMovement, ReplayNonReproduction, ReplayOutcome, ReplayReading, WitnessLineage,
};

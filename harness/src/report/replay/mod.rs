#![doc = include_str!("README.md")]

mod compare;
mod read;
mod types;

pub use compare::{compare, compare_legacy};
pub use types::{
    HistoricalReplayRefusal, HistoricalReplayStanding, LegacyClaimRelation, LegacyJoinRefusal,
    LegacyReading, ReplayCoordinate, ReplayJoinRefusal, ReplayMovement, ReplayNonReproduction,
    ReplayOutcome, ReplayReading, WitnessLineage,
};

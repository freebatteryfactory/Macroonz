#![doc = include_str!("README.md")]

mod explore;
mod material;
mod space;
mod types;

pub use explore::{concluded, explored};
pub use material::{encoded, interpreted};
pub use types::{
    ADDRESSABLE_STRANDS, Counterexample, EXPLORATION_STARVED, EncodingRefusal, ExplorationBound,
    ExplorationBoundRefusal, ExplorationMode, ExplorationReading, ExplorationRefusal,
    ExplorationSite, ExplorationStanding, InterleavedSequence, Interleaving, InterleavingSpace,
    Strand, StrandRefusal, StrandSet, StrandSetRefusal,
};

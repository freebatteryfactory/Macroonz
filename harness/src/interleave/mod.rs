#![doc = include_str!("README.md")]

mod explore;
mod types;

pub use explore::{encoded, explored, interpreted};
pub use types::{
    ADDRESSABLE_STRANDS, Counterexample, EncodingRefusal, ExplorationBound,
    ExplorationBoundRefusal, ExplorationMode, ExplorationReading, ExplorationRefusal,
    ExplorationSite, ExplorationStanding, InterleavedSequence, Interleaving, InterleavingSpace,
    Strand, StrandRefusal, StrandSet, StrandSetRefusal,
};

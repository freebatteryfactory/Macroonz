#![doc = include_str!("README.md")]

mod explore;
mod types;

pub use explore::{concluded, explored};
pub use types::{
    LOOM_PIN, MODEL_BROKE, PreemptionBound, PreemptionBounds, PreemptionBoundsRefusal,
    PreemptionReading, PreemptionVerdict,
};

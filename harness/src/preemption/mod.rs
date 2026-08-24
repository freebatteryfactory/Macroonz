#![doc = include_str!("README.md")]

mod explore;
mod types;

pub use explore::explored;
pub use types::{
    LOOM_PIN, PreemptionBound, PreemptionBounds, PreemptionBoundsRefusal, PreemptionReading,
    PreemptionVerdict,
};

#![doc = include_str!("README.md")]

mod explore;
mod types;

cfg_select! {
    any(
        all(
            unix,
            any(
                target_arch = "aarch64",
                target_arch = "arm",
                target_arch = "x86_64",
                target_arch = "loongarch64",
                target_arch = "riscv64",
                all(target_arch = "powerpc64", target_endian = "little"),
            ),
        ),
        all(windows, any(target_arch = "x86_64", target_arch = "aarch64")),
    ) => {
        #[path = "explore_loom.rs"]
        mod backend;
    }
    _ => {
        #[path = "explore_unavailable.rs"]
        mod backend;
    }
}

pub use explore::{attempted, explored};
pub use types::{
    IncompleteExploration, LOOM_PIN, MODEL_BROKE, PreemptionBound, PreemptionBounds,
    PreemptionBoundsRefusal, PreemptionModelFailure, PreemptionModelResult, PreemptionOutcome,
    PreemptionReading, PreemptionVerdict,
};

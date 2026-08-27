#![doc = include_str!("README.md")]

mod compose;
mod types;

pub use compose::{compose_reduce_replay, preflight_ready};
pub use types::{
    BackendSelection, BackendSelectionRefusal, ComposeRefusal, FRIDA_GUM_CRATE_PIN,
    FRIDA_GUM_WINDOWS_X86_64_DEVKIT, HostDisposition, InterestingBytes, InterestingBytesRefusal,
    LIBAFL_PIN, NamedCeiling, PreflightCapability, PreflightFact, PreflightIncomplete,
    PreflightStatus, ReadyPreflight, SelectedBackend,
};

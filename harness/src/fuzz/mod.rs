#![doc = include_str!("README.md")]

mod compose;
#[cfg(all(
    feature = "fuzz-frida",
    any(windows, target_os = "linux", target_os = "macos")
))]
mod frida;
#[cfg(all(
    feature = "fuzz-frida",
    not(any(windows, target_os = "linux", target_os = "macos"))
))]
mod frida_unsupported;
mod types;

pub use compose::{compose_reduce_replay, preflight_ready};
#[cfg(all(
    feature = "fuzz-frida",
    any(windows, target_os = "linux", target_os = "macos")
))]
pub use frida::run_libafl_frida;
#[cfg(all(
    feature = "fuzz-frida",
    not(any(windows, target_os = "linux", target_os = "macos"))
))]
pub use frida_unsupported::run_libafl_frida;
pub use types::{
    BackendSelection, BackendSelectionRefusal, ComposeRefusal, FRIDA_GUM_CRATE_PIN,
    FRIDA_GUM_WINDOWS_X86_64_DEVKIT, FridaCampaign, FridaCampaignRefusal, FridaCampaignResult,
    FridaModuleName, FridaRunRefusal, FridaTarget, FridaTargetRefusal, FuzzExecution,
    HostDisposition, InterestingBytes, InterestingBytesRefusal, LIBAFL_PIN, NamedCeiling,
    PreflightCapability, PreflightFact, PreflightIncomplete, PreflightStatus, ReadyPreflight,
    SelectedBackend,
};

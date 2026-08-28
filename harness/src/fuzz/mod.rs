#![doc = include_str!("README.md")]

mod compose;
mod coverage;
mod execute;
mod mutate;
mod types;

pub use compose::{compose_reduce_replay, preflight_ready};
pub use coverage::read_lcov;
pub use execute::observe_rustc_profile;
pub use mutate::neighboring_inputs;
pub use types::{
    BackendSelection, BackendSelectionRefusal, ComposeRefusal, CoverageAdmission,
    CoverageAdmissionRefusal, CoverageCorpus, CoverageObservation, CoveragePoint,
    CoverageReadRefusal, FuzzExecution, HostDisposition, InstrumentedTarget, InterestingBytes,
    InterestingBytesRefusal, MutationCandidate, MutationKind, MutationPlan, MutationPlanRefusal,
    MutationRefusal, NamedCeiling, PreflightCapability, PreflightFact, PreflightIncomplete,
    PreflightStatus, RUSTC_COVERAGE_TOOLCHAIN, ReadyPreflight, RustcCoverageTools,
    RustcProfileRefusal, RustcProfileRequest, RustcProfileRequestRefusal, RustcProfileResult,
    SelectedBackend,
};

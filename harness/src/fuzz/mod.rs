#![doc = include_str!("README.md")]

mod compose;
mod coverage;
mod execute;
mod mutate;
mod preflight;
mod types;

pub use compose::compose_reduce_replay;
pub use coverage::read_lcov;
pub use execute::observe_rustc_profile;
pub use mutate::neighboring_inputs;
pub use preflight::preflight_ready;
pub use types::{
    ComposeRefusal, CoverageAdmission, CoverageAdmissionRefusal, CoverageCorpus,
    CoverageObservation, CoveragePoint, CoverageReadRefusal, CoverageSource, CoverageSourceRoot,
    CoverageSourceRootRefusal, CoverageTool, FuzzExecution, InstrumentedTarget, InterestingBytes,
    InterestingBytesRefusal, MutationCandidate, MutationKind, MutationPlan, MutationPlanRefusal,
    MutationRefusal, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN, ReadyPreflight, RustcCommand,
    RustcField, RustcProfileRefusal, RustcProfileRequest, RustcProfileRequestRefusal,
    RustcProfileResult,
};

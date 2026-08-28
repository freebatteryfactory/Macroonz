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
    ComposeRefusal, CoverageAdmission, CoverageAdmissionRefusal, CoverageBudgetRefusal,
    CoverageBudgets, CoverageCampaign, CoverageCorpus, CoverageObservation, CoveragePoint,
    CoverageProfile, CoverageReadRefusal, CoverageSource, CoverageSourceRoot,
    CoverageSourceRootRefusal, CoverageStanding, CoverageTool, FuzzExecution, InstrumentedTarget,
    InterestingBytes, MutationCandidate, MutationKind, MutationPlan, MutationPlanRefusal,
    MutationRefusal, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN, ReadyPreflight, RustcCommand,
    RustcField, RustcProfileRefusal, RustcProfileRequest, RustcProfileRequestRefusal,
    RustcProfileResult,
};

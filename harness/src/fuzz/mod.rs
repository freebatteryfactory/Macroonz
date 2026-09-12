#![doc = include_str!("README.md")]

mod compose;
mod coverage;
mod execute;
mod mutate;
mod preflight;
mod type_contract;
mod types;

pub use compose::compose_reduce_replay;
pub use coverage::{read_lcov, read_lcov_mapped};
pub use execute::{observe_rustc_profile, observe_rustc_profile_with};
pub use mutate::neighboring_inputs;
pub use preflight::{preflight_ready, preflight_ready_with};
pub use types::{
    ComposeRefusal, CoverageAdmission, CoverageAdmissionRefusal, CoverageBudgetRefusal,
    CoverageBudgets, CoverageCampaign, CoverageCaseCleanup, CoverageCommand, CoverageCorpus,
    CoverageHostFailure, CoverageInvocation, CoverageObservation, CoveragePoint, CoverageProfile,
    CoverageReadRefusal, CoverageReply, CoverageRootsRefusal, CoverageSource, CoverageSourceRoot,
    CoverageSourceRootRefusal, CoverageSourceRoots, CoverageStanding, CoverageTool, FuzzExecution,
    InstrumentedTarget, InterestingBytes, MutationCandidate, MutationKind, MutationPlan,
    MutationPlanRefusal, MutationRefusal, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN,
    ReadyPreflight, RustcCommand, RustcField, RustcProfileRefusal, RustcProfileRequest,
    RustcProfileRequestRefusal, RustcProfileResult,
};

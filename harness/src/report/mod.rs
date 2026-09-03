#![doc = include_str!("README.md")]

mod compare;
mod coverage;
mod encode;
mod resolve;
mod stamp;
mod type_contract;
mod types;

pub(crate) use stamp::{
    declare_change_pair, implement_borrowed_change_pair, implement_copy_change_pair,
};

pub use compare::compare;
pub use coverage::claim_coverage;
pub use encode::{
    encode_bytes, encode_length, execution_key_preimage, fingerprint_preimage,
    replay_capsule_preimage, trial_preimage,
};
pub use resolve::{attachment_cache_eligibility, attachment_replay_posture};
pub use types::{
    Baseline, ByteBudget, CacheEligibility, CaseBudget, CensusDelta, CensusDirection,
    CheckRevisionId, ClaimCoverage, ClaimExercise, ConclusionFlip, CoverageRefusal,
    EXECUTION_KEY_TAG, EmptySelectionReason, ExecutionKey, ExecutionRevisionChange,
    ExecutionRevisions, Exercise, FINGERPRINT_TAG, FOREIGN_TEXT_MAX_BYTES, FailureClass,
    FindingCause, FindingLocation, Fingerprint, ForeignText, GenerationProfile, HostTrialRecord,
    InfrastructureFailure, InfrastructureFault, InvocationProfile, InvocationProfileChange,
    MinimizationProfile, NoBaselineReason, NotComparedReason, NotSelectedReason, OutcomeClass,
    ProfiledTrial, REPLAY_CAPSULE_TAG, ROW_REVISION_TAG, ReplayCapsule, ReplayPosture,
    ReportComparison, ReportDiff, ReportExecutionDiff, ReportPopulationDiff, RowRevisionChange,
    RowRevisionId, RunAttempt, RunReport, SelectionDisposition, SelectionExpectation,
    SelectionOutcome, SkipReason, SubjectRevisionId, TRIAL_IDENTITY_TAG, TargetBinding,
    TargetBindingChange, TargetTriple, TextFidelity, TimeBudget, ToolchainIdentity,
    TrialAccounting, TrialConclusion, TrialFinding, TrialId, TrialProfile, TrialReport,
    TrialRunStanding, TrialSite, Truncation,
};

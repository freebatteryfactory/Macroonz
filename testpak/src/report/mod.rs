#![doc = include_str!("README.md")]

mod compare;
mod coverage;
mod encode;
mod resolve;
mod type_contract;
mod types;

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
    EXECUTION_KEY_TAG, EmptySelectionReason, ExecutionKey, Exercise, FINGERPRINT_TAG,
    FOREIGN_TEXT_MAX_BYTES, FailureClass, FindingCause, FindingLocation, Fingerprint, ForeignText,
    GenerationProfile, InfrastructureFault, InvocationProfile, MinimizationProfile,
    NoBaselineReason, NotComparedReason, NotSelectedReason, OutcomeClass, REPLAY_CAPSULE_TAG,
    ROW_REVISION_TAG, RecordedDuration, ReplayCapsule, ReplayPosture, ReportComparison, ReportDiff,
    RowRevisionChange, RowRevisionId, RunAttempt, RunReport, SelectionDisposition,
    SelectionExpectation, SelectionOutcome, SkipReason, SubjectRevisionId, TRIAL_IDENTITY_TAG,
    TargetBinding, TargetTriple, TextFidelity, TimeBudget, ToolchainIdentity, TrialAccounting,
    TrialConclusion, TrialCoordinates, TrialFinding, TrialId, TrialProfile, TrialReport, TrialSite,
    Truncation,
};

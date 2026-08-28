#![doc = include_str!("README.md")]

mod reduce;
pub(super) mod types;

pub use reduce::{capture_replay, reduce, shrink_verdict};
pub use types::{
    ByteReducerExecution, ByteReducerId, FingerprintPreservation, FingerprintProbe, ProbeOutcome,
    ReductionBudget, ReductionCensus, ReductionEvidence, ReductionHalt, ReductionOutcome,
    ReductionPlan, ReductionPlanRefusal, ReductionProbeBinding, ReductionProbeRefusal,
    ReductionRefusal, SemanticCandidateRefusal, SemanticCandidates, SemanticReducerBinding,
    SemanticReducerCall, SemanticReducerExecution, SemanticReducerId, ShrinkVerdict,
};

//! The stable public vocabulary road for generation and reduction.

pub use super::generation::{
    ByteDraw, ByteSource, ByteSourceAddress, CaseIndex, CaseWidth, CaseWidthRefusal, CommandDecode,
    CommandSequence, GENERATION_CHUNK_TAG, GENERATION_DISPOSITION_SEATS, GENERATION_SOURCE_TAG,
    GeneratedSequences, GenerationCensus, GenerationDisposition, GenerationHalt, GenerationPlan,
    GenerationPlanRefusal, InputOrigin, PreconditionVerdict, RejectionAllowance, RootSeed,
    SOURCE_CHUNK_BYTES, SequencePrecondition, SizeProgression, StreamCursor, StreamCursorRefusal,
};
pub use super::reduction::{
    ByteReducerExecution, ByteReducerId, FingerprintPreservation, FingerprintProbe, ProbeOutcome,
    ReductionBudget, ReductionCensus, ReductionEvidence, ReductionHalt, ReductionOutcome,
    ReductionPlan, ReductionPlanRefusal, ReductionProbeBinding, ReductionProbeRefusal,
    ReductionRefusal, SemanticCandidateRefusal, SemanticCandidates, SemanticReducerBinding,
    SemanticReducerCall, SemanticReducerExecution, SemanticReducerId, ShrinkVerdict,
};

#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! [`types`] declares this home's public vocabulary: the generation dispositions and their census, the generation and reduction plans, the deterministic byte source, the driver's seams, and the minimization vocabulary.
//! [`driver`] is the one shared sequence driver every lane drives through.
//! [`reduce`][mod@reduce] is the generic byte reducer and the one law a shrink is admitted under.

pub mod driver;
pub mod reduce;
pub mod types;

pub use driver::{admit_every_sequence, decode_arbitrary, drive};
pub use reduce::{capture_replay, reduce, shrink_verdict};
pub use types::{
    ByteDraw, ByteReducerExecution, ByteReducerId, ByteSource, ByteSourceAddress, CaseIndex,
    CaseWidth, CaseWidthRefusal, CommandDecode, CommandSequence, FingerprintPreservation,
    FingerprintProbe, GENERATION_CHUNK_TAG, GENERATION_DISPOSITION_SEATS, GENERATION_SOURCE_TAG,
    GeneratedSequences, GenerationCensus, GenerationDisposition, GenerationHalt, GenerationPlan,
    GenerationPlanRefusal, InputOrigin, PreconditionVerdict, ProbeOutcome, ReductionBudget,
    ReductionCensus, ReductionEvidence, ReductionHalt, ReductionOutcome, ReductionPlan,
    ReductionPlanRefusal, ReductionProbeBinding, ReductionProbeRefusal, ReductionRefusal,
    RejectionAllowance, RootSeed, SOURCE_CHUNK_BYTES, SemanticCandidateRefusal, SemanticCandidates,
    SemanticReducerBinding, SemanticReducerCall, SemanticReducerExecution, SemanticReducerId,
    SequencePrecondition, ShrinkVerdict, SizeProgression, StreamCursor, StreamCursorRefusal,
};

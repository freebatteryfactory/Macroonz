#![doc = include_str!("README.md")]

mod draw;
mod drive;
mod encode;
pub(super) mod types;

pub use drive::{admit_every_sequence, decode_arbitrary, drive};
pub use types::{
    ByteDraw, ByteSource, ByteSourceAddress, CaseIndex, CaseWidth, CaseWidthRefusal, CommandDecode,
    CommandSequence, GENERATION_CHUNK_TAG, GENERATION_DISPOSITION_SEATS, GENERATION_SOURCE_TAG,
    GeneratedSequences, GenerationCensus, GenerationDisposition, GenerationHalt, GenerationPlan,
    GenerationPlanRefusal, InputOrigin, PreconditionVerdict, RejectionAllowance, RootSeed,
    SOURCE_CHUNK_BYTES, SequencePrecondition, SizeProgression, StreamCursor, StreamCursorRefusal,
};

#![doc = include_str!("README.md")]

mod encode;
mod encode_interpreted;
mod size;
mod size_interpreted;
mod type_contract;
mod types;

pub(crate) use encode::retain_qualified;
pub use encode::{retain_parity, retain_parity_standing};
pub use encode_interpreted::{retain_interpreted, retain_interpreted_with_material};
pub(crate) use size::qualified_size;
pub use types::{
    ArchivedEvaluationPair, ArchivedParity, ArchivedParityDisposition, ArchivedSubstrate,
    ArchivedSubstrateRoster, ArchivedValue, ArchivedValueConvention, PARITY_ARCHIVE_TAG,
    ParityArchiveLimits, ParityArchiveRefusal, ValueEncoder, ValueEncodingRefusal, ValueRole,
    read_parity,
};
pub use types::{
    ArchivedInterpretedEvidence, ArchivedInterpretedTrust, INTERPRETED_ARCHIVE_TAG,
    InterpretedArchiveLimits, InterpretedArchiveRefusal, read_interpreted,
};

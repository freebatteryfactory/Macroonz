#![doc = include_str!("README.md")]

mod encode;
mod size;
mod type_contract;
mod types;

pub use encode::{retain_parity, retain_parity_standing};
pub use types::{
    ArchivedEvaluationPair, ArchivedParity, ArchivedParityDisposition, ArchivedSubstrate,
    ArchivedSubstrateRoster, ArchivedValue, ArchivedValueConvention, PARITY_ARCHIVE_TAG,
    ParityArchiveLimits, ParityArchiveRefusal, ValueEncoder, ValueEncodingRefusal, ValueRole,
    read_parity,
};

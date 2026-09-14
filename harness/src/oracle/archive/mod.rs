#![doc = include_str!("README.md")]

mod encode;
mod encode_compiled;
mod encode_structural;
mod size;
mod type_contract;
mod types;

pub use encode::{
    retain_compilation, retain_compiled, retain_structural, retain_transcript, retain_vector,
};
pub use types::{
    ArchivedMethod, ArchivedOracle, ArchivedStructuralDisagreement, ArchivedTranscriptDisagreement,
    ArchivedVectorDisagreement, ArchivedVerdict, ORACLE_ARCHIVE_TAG, OracleArchiveRefusal,
    read_verdict,
};

#![doc = include_str!("README.md")]

mod encode;
mod size;
mod type_contract;
mod types;

pub use encode::retain_reduction;
pub use types::{
    ArchivedReduction, ArchivedReductionCensus, ArchivedSemanticReducer, REDUCTION_ARCHIVE_TAG,
    ReductionArchiveLimits, ReductionArchiveRefusal, read_reduction,
};

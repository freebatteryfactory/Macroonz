#![doc = include_str!("README.md")]

mod encode;
mod types;

pub use encode::retain_capsule;
pub use types::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedCapsule, ArchivedExecution,
    ArchivedFingerprint, ArchivedInput, ArchivedProfile, CAPSULE_ARCHIVE_TAG, read_capsule,
};

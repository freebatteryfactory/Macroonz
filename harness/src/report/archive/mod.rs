#![doc = include_str!("README.md")]

mod encode;
mod encode_trial;
mod size_trial;
mod types;

pub use encode::retain_capsule;
pub use encode_trial::retain_trial;
pub use types::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedAttempt, ArchivedCapsule,
    ArchivedClockFailure, ArchivedConclusion, ArchivedExecution, ArchivedFinding,
    ArchivedFingerprint, ArchivedForeignText, ArchivedInput, ArchivedMeasurement, ArchivedProfile,
    ArchivedSite, ArchivedTrial, ArchivedTruncation, CAPSULE_ARCHIVE_TAG, TRIAL_ARCHIVE_TAG,
    read_capsule, read_trial,
};

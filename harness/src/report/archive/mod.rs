#![doc = include_str!("README.md")]

mod encode;
mod encode_trial;
mod size_trial;
mod encode_run;
mod size_run;
mod types;

pub use encode::retain_capsule;
pub(crate) use encode::{bounded, capsule_size, sum};
pub use encode_run::retain_run;
pub use encode_trial::retain_trial;
pub(crate) use size_run::name_size;
pub use types::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedAttempt, ArchivedCapsule,
    ArchivedClockFailure, ArchivedConclusion, ArchivedExecution, ArchivedFinding,
    ArchivedFingerprint, ArchivedForeignText, ArchivedInput, ArchivedMeasurement, ArchivedProfile,
    ArchivedSite, ArchivedTrial, ArchivedTruncation, CAPSULE_ARCHIVE_TAG, TRIAL_ARCHIVE_TAG,
    read_capsule, read_trial,
};
pub use types::{
    ArchivedAccounting, ArchivedDisposition, ArchivedName, ArchivedRun, ArchivedTablePosture,
    RUN_ARCHIVE_TAG, RunArchiveLimits, read_run,
};
pub(crate) use types::{claim, cursor, envelope, finish, frame, name, posture};

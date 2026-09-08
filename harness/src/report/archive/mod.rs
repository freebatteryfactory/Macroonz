#![doc = include_str!("README.md")]

mod encode;
mod encode_trial;
mod size_trial;
mod encode_run;
mod size_run;
mod types;

pub use encode::retain_capsule;
pub(crate) use encode::{
    bounded, capsule_size, execution_size, fingerprint_size, sum, write_execution,
};
pub use encode_run::retain_run;
pub use encode_trial::retain_trial;
pub(crate) use encode_trial::{
    attribution_slot, foreign as write_foreign, measurement as write_measurement, write_finding,
};
pub(crate) use size_run::encoded_size as run_size;
pub(crate) use size_run::name_size;
pub(crate) use size_trial::encoded_size as trial_size;
pub(crate) use size_trial::{finding_size, foreign_size, measurement_size};
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
pub(crate) use types::{
    claim, cursor, envelope, execution, finding, fingerprint, finish, foreign, frame, measurement,
    name, posture, read_attribution, text,
};

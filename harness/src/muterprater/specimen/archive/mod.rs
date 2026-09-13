#![doc = include_str!("README.md")]

mod encode;
mod size;
mod type_contract;
mod types;

pub use encode::retain_projection;
pub(crate) use encode::write_artifact;
pub(crate) use size::known_size as projection_known_size;
pub(crate) use size::source_size;
pub(crate) use types::artifact as read_artifact;
pub use types::{
    ArchivedProjectionPressure, ArchivedSpecimenStanding, PROJECTION_ARCHIVE_TAG,
    ProjectionArchiveLimits, ProjectionArchiveRefusal, read_projection,
};

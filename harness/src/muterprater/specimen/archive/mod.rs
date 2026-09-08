#![doc = include_str!("README.md")]

mod encode;
mod size;
mod type_contract;
mod types;

pub use encode::retain_projection;
pub(crate) use size::known_size as projection_known_size;
pub use types::{
    ArchivedProjectionPressure, ArchivedSpecimenStanding, PROJECTION_ARCHIVE_TAG,
    ProjectionArchiveLimits, ProjectionArchiveRefusal, read_projection,
};

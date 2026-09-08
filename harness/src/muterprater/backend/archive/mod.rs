#![doc = include_str!("README.md")]

mod encode;
mod encode_suite;
mod size;
mod types;
mod type_contract;

pub use encode::{retain_backend, retain_backend_with_material};
pub use encode_suite::{retain_suite_pressure, retain_suite_pressure_with_material};
pub(crate) use encode_suite::{retained as retained_suite_pressure, suite_pressure_size};
pub(crate) use types::OriginalMaterial;
pub use types::{
    ArchivedAdapterProfile, ArchivedBackendInvocation, ArchivedBackendManifest,
    ArchivedBackendSource, ArchivedUnparsedLine, BACKEND_ARCHIVE_TAG, BackendArchiveLimits,
    BackendArchiveRefusal, read_backend,
};
pub use types::{
    ArchivedSuitePressure, SUITE_PRESSURE_ARCHIVE_TAG, SuitePressureArchiveLimits,
    SuitePressureArchiveRefusal, read_suite_pressure,
};

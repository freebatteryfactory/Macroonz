#![doc = include_str!("README.md")]

mod encode;
mod size;
mod types;
mod type_contract;

pub use encode::{retain_backend, retain_backend_with_material};
pub use types::{
    ArchivedAdapterProfile, ArchivedBackendInvocation, ArchivedBackendManifest,
    ArchivedBackendSource, ArchivedUnparsedLine, BACKEND_ARCHIVE_TAG, BackendArchiveLimits,
    BackendArchiveRefusal, read_backend,
};

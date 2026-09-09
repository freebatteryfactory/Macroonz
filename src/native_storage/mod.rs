#![doc = include_str!("README.md")]

#[cfg(any(unix, windows))]
mod custody;
#[cfg(any(unix, windows))]
mod read;
mod types;
#[cfg(any(unix, windows))]
mod write;

pub use types::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageRoot,
    StorageTransaction, StoredArtifact,
};

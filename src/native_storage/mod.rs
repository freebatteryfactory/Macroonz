#![doc = include_str!("README.md")]

#[cfg(any(unix, windows))]
mod custody;
#[cfg(any(unix, windows))]
mod read;
mod types;
#[cfg(any(unix, windows))]
mod write;

#[cfg(any(unix, windows))]
pub(crate) use read::bounded as read_bounded;

pub use types::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageRoot,
    StorageTransaction, StoredArtifact,
};

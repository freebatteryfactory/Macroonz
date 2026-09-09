//! Directory-relative access and cooperating process exclusion.

use super::{StorageError, StorageName};
use cap_std::fs::{Dir, OpenOptions};
use std::fs::{File, TryLockError};
use std::path::Path;

pub(super) const PREPARED: &str = ".prepared";
pub(super) const COMMITTED: &str = ".committed";
pub(super) const HEADER: &str = "macroonz-storage/1\n";

pub(super) fn open(path: &Path) -> Result<Dir, StorageError> {
    Dir::open_ambient_dir(path, cap_std::ambient_authority()).map_err(StorageError::Io)
}

pub(super) fn lock(directory: &Dir) -> Result<File, StorageError> {
    let file = directory
        .open_with(
            ".macroonz-storage-lock",
            OpenOptions::new().read(true).write(true).create(true),
        )
        .map_err(StorageError::Io)?
        .into_std();
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(TryLockError::WouldBlock) => Err(StorageError::Busy),
        Err(TryLockError::Error(error)) => Err(StorageError::Io(error)),
    }
}

pub(super) fn physical(name: &StorageName) -> String {
    format!("item-{}", name.spelling())
}

pub(super) fn batch(directory: &Dir, name: &StorageName) -> Result<Dir, StorageError> {
    directory.open_dir(physical(name)).map_err(StorageError::Io)
}

pub(super) fn marker(directory: &Dir, name: &str) -> Result<(), StorageError> {
    let metadata = directory.symlink_metadata(name).map_err(StorageError::Io)?;
    if !metadata.is_file() {
        return Err(StorageError::NotRegular);
    }
    Ok(())
}

pub(super) fn inventory(directory: &Dir, maximum: usize) -> Result<Vec<StorageName>, StorageError> {
    let mut names = Vec::new();
    for entry in directory.entries().map_err(StorageError::Io)? {
        let entry = entry.map_err(StorageError::Io)?;
        if !entry.file_type().map_err(StorageError::Io)?.is_file() {
            return Err(StorageError::NotRegular);
        }
        let spelling = entry
            .file_name()
            .into_string()
            .map_err(|_name| StorageError::UnexpectedEntry)?;
        if matches!(spelling.as_str(), PREPARED | COMMITTED) {
            continue;
        }
        if names.len() >= maximum {
            return Err(StorageError::ArtifactBound);
        }
        let logical = spelling
            .strip_prefix("item-")
            .ok_or(StorageError::UnexpectedEntry)?;
        names.push(StorageName::informed(logical)?);
    }
    names.sort();
    Ok(names)
}

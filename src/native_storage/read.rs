//! Aggregate-bounded loading after the publication boundary.

use super::custody::{self, COMMITTED};
use super::{StorageError, StorageLimits, StorageName, StoredArtifact};
use cap_std::fs::Dir;
use std::io::{ErrorKind, Read};

pub(super) fn load(
    directory: &Dir,
    limits: StorageLimits,
) -> Result<Vec<StoredArtifact>, StorageError> {
    match custody::marker(directory, COMMITTED) {
        Ok(()) => {}
        Err(StorageError::Io(error)) if error.kind() == ErrorKind::NotFound => {
            return Err(StorageError::Incomplete);
        }
        Err(error) => return Err(error),
    }
    let names = published(directory, limits.artifacts)?;
    if names != custody::inventory(directory, limits.artifacts)? {
        return Err(StorageError::InventoryMismatch);
    }
    let mut remaining = limits.bytes;
    let mut artifacts = Vec::new();
    for name in names {
        let bytes = bounded(directory, &custody::physical(&name), remaining)?;
        remaining = remaining
            .checked_sub(bytes.len())
            .ok_or(StorageError::ByteBound)?;
        artifacts.push(StoredArtifact { name, bytes });
    }
    Ok(artifacts)
}

fn published(directory: &Dir, maximum: usize) -> Result<Vec<StorageName>, StorageError> {
    let bound = maximum
        .checked_mul(97)
        .and_then(|bytes| bytes.checked_add(custody::HEADER.len()))
        .ok_or(StorageError::ArtifactBound)?;
    let bytes = bounded(directory, COMMITTED, bound)?;
    let text = std::str::from_utf8(&bytes).map_err(|_error| StorageError::InvalidMarker)?;
    let body = text
        .strip_prefix(custody::HEADER)
        .and_then(|body| body.strip_suffix('\n'))
        .ok_or(StorageError::InvalidMarker)?;
    let mut names = Vec::new();
    for spelling in body.split('\n') {
        if names.len() >= maximum {
            return Err(StorageError::ArtifactBound);
        }
        let name = StorageName::informed(spelling).map_err(|_error| StorageError::InvalidMarker)?;
        if names.last().is_some_and(|previous| previous >= &name) {
            return Err(StorageError::InvalidMarker);
        }
        names.push(name);
    }
    Ok(names)
}

fn bounded(directory: &Dir, name: &str, maximum: usize) -> Result<Vec<u8>, StorageError> {
    let mut file = directory.open(name).map_err(StorageError::Io)?;
    if !file.metadata().map_err(StorageError::Io)?.is_file() {
        return Err(StorageError::NotRegular);
    }
    let maximum = u64::try_from(maximum).map_err(|_error| StorageError::ByteBound)?;
    let mut bytes = Vec::new();
    Read::by_ref(&mut file)
        .take(maximum)
        .read_to_end(&mut bytes)
        .map_err(StorageError::Io)?;
    let mut extra = [0u8; 1];
    if file.read(&mut extra).map_err(StorageError::Io)? != 0usize {
        return Err(StorageError::ByteBound);
    }
    Ok(bytes)
}

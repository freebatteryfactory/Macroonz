//! Exclusive payload creation and batch publication.

use super::custody::{self, COMMITTED, PREPARED};
use super::{StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName};
use cap_std::fs::{Dir, OpenOptions};
use std::io::{ErrorKind, Write};

pub(super) fn reserve(root: &Dir, name: &StorageName) -> Result<Dir, StorageError> {
    match root.create_dir(custody::physical(name)) {
        Ok(()) => custody::batch(root, name),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => Err(StorageError::Collision),
        Err(error) => Err(StorageError::Io(error)),
    }
}

fn create(directory: &Dir, name: &str, bytes: &[u8]) -> Result<(), StorageError> {
    let mut file = directory
        .open_with(name, OpenOptions::new().write(true).create_new(true))
        .map_err(StorageError::Io)?;
    file.write_all(bytes).map_err(StorageError::Io)?;
    file.sync_all().map_err(StorageError::Io)
}

pub(super) fn payload(directory: &Dir, artifact: &StorageArtifact<'_>) -> Result<(), StorageError> {
    create(directory, &custody::physical(artifact.name), artifact.bytes)
}

pub(super) fn publish(directory: &Dir, batch: StorageBatch<'_>) -> Result<(), StorageError> {
    let mut names = batch
        .artifacts()
        .iter()
        .map(|artifact| artifact.name)
        .collect::<Vec<_>>();
    names.sort();
    let mut inventory = custody::HEADER.to_owned();
    for name in names {
        inventory.push_str(name.spelling());
        inventory.push('\n');
    }
    create(directory, PREPARED, inventory.as_bytes())?;
    directory
        .hard_link(PREPARED, directory, COMMITTED)
        .map_err(StorageError::Io)
}

pub(super) fn restart(directory: &Dir, batch: StorageBatch<'_>) -> Result<(), StorageError> {
    match directory.symlink_metadata(COMMITTED) {
        Ok(_metadata) => return Err(StorageError::Collision),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(StorageError::Io(error)),
    }
    let names = custody::inventory(directory, batch.limits().artifacts)?;
    if names.iter().any(|name| {
        !batch
            .artifacts()
            .iter()
            .any(|artifact| artifact.name == name)
    }) {
        return Err(StorageError::UnexpectedEntry);
    }
    for name in names {
        directory
            .remove_file(custody::physical(&name))
            .map_err(StorageError::Io)?;
    }
    match directory.remove_file(PREPARED) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(StorageError::Io(error)),
    }
}

pub(super) fn discard(
    root: &Dir,
    batch_name: &StorageName,
    expected: &[StorageArtifact<'_>],
    limits: StorageLimits,
) -> Result<(), StorageError> {
    let directory = custody::batch(root, batch_name)?;
    let names = custody::inventory(&directory, limits.artifacts)?;
    let mut remaining = limits.bytes;
    for name in &names {
        let artifact = expected
            .iter()
            .find(|artifact| artifact.name == name)
            .ok_or(StorageError::UnexpectedEntry)?;
        let bytes = super::read::bounded(&directory, &custody::physical(name), remaining)?;
        if bytes != artifact.bytes {
            return Err(StorageError::InventoryMismatch);
        }
        remaining = remaining
            .checked_sub(bytes.len())
            .ok_or(StorageError::ByteBound)?;
    }
    for marker in [COMMITTED, PREPARED] {
        match directory.remove_file(marker) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(StorageError::Io(error)),
        }
    }
    for name in names {
        directory
            .remove_file(custody::physical(&name))
            .map_err(StorageError::Io)?;
    }
    drop(directory);
    root.remove_dir(custody::physical(batch_name))
        .map_err(StorageError::Io)
}

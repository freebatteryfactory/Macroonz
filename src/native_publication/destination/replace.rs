use super::types::{InstallationIntent, OwnedFile};
use super::{DestinationError, DestinationLimits, files, inspect};
use crate::native_publication::{PublicationPath, published_digest};
use cap_std::fs::{Dir, OpenOptions};
use std::io::{ErrorKind, Write};

const TEMPORARY: &str = ".macroonz-publication/replacement";
const INCOMING: &str = ".macroonz-publication/incoming";

pub(super) fn compatible(
    root: &Dir,
    intent: &InstallationIntent,
    limits: DestinationLimits,
) -> Result<(), DestinationError> {
    let mut remaining = limits.bytes;
    for path in inspect::paths(intent.before.as_ref(), &intent.after, limits)? {
        let bytes = files::read(root, path.spelling(), remaining)?;
        if let Some(bytes) = bytes {
            remaining = remaining
                .checked_sub(bytes.len())
                .ok_or(DestinationError::Incomplete)?;
            let previous = intent
                .before
                .iter()
                .flat_map(|record| &record.files)
                .find(|file| file.path == path);
            let next = intent.after.files.iter().find(|file| file.path == path);
            if !matches_bytes(previous, &bytes) && !matches_bytes(next, &bytes) {
                return Err(DestinationError::Conflict(path.spelling().to_owned()));
            }
        }
    }
    Ok(())
}

fn matches_bytes(file: Option<&OwnedFile>, bytes: &[u8]) -> bool {
    file.is_some_and(|file| {
        file.bytes == bytes.len() && file.published == *published_digest(bytes).as_bytes()
    })
}

pub(super) fn apply(
    root: &Dir,
    intent: &InstallationIntent,
    path: &PublicationPath,
    limits: DestinationLimits,
) -> Result<(), DestinationError> {
    let next = intent
        .after
        .files
        .iter()
        .position(|file| file.path == *path);
    let previous = intent
        .before
        .iter()
        .flat_map(|record| &record.files)
        .find(|file| file.path == *path);
    let actual = files::read(root, path.spelling(), limits.bytes)?;
    let expected = next.and_then(|at| intent.after.files.get(at));
    if actual
        .as_ref()
        .is_some_and(|bytes| !matches_bytes(previous, bytes) && !matches_bytes(expected, bytes))
    {
        return Err(DestinationError::Conflict(path.spelling().to_owned()));
    }
    if let Some(at) = next {
        let bytes = intent
            .payloads
            .get(at)
            .ok_or(DestinationError::Incomplete)?;
        if actual.as_deref() != Some(bytes.as_slice()) {
            file(root, path.spelling(), bytes)?;
        }
    } else if actual.is_some() {
        root.remove_file(path.spelling())
            .map_err(DestinationError::Filesystem)?;
    }
    Ok(())
}

pub(super) fn completed(
    root: &Dir,
    intent: &InstallationIntent,
    limits: DestinationLimits,
) -> Result<(), DestinationError> {
    let mut remaining = limits.bytes;
    for path in inspect::paths(intent.before.as_ref(), &intent.after, limits)? {
        let actual = files::read(root, path.spelling(), remaining)?;
        let expected = intent.after.files.iter().find(|file| file.path == path);
        let matches = match (&actual, expected) {
            (Some(bytes), Some(file)) => matches_bytes(Some(file), bytes),
            (None, None) => true,
            _ => false,
        };
        if !matches {
            return Err(DestinationError::Conflict(path.spelling().to_owned()));
        }
        if let Some(bytes) = actual {
            remaining = remaining
                .checked_sub(bytes.len())
                .ok_or(DestinationError::Incomplete)?;
        }
    }
    Ok(())
}

pub(super) fn file(root: &Dir, path: &str, bytes: &[u8]) -> Result<(), DestinationError> {
    for (at, _) in path.match_indices('/') {
        let parent = path
            .get(..at)
            .ok_or_else(|| DestinationError::Entry(path.to_owned()))?;
        if !files::directory(root, parent)? {
            root.create_dir(parent)
                .map_err(DestinationError::Filesystem)?;
        }
    }
    let existing = match root.symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() => true,
        Ok(_metadata) => return Err(DestinationError::Entry(path.to_owned())),
        Err(error) if error.kind() == ErrorKind::NotFound => false,
        Err(error) => return Err(DestinationError::Filesystem(error)),
    };
    match root.symlink_metadata(TEMPORARY) {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_metadata) => return Err(DestinationError::Entry(TEMPORARY.to_owned())),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(DestinationError::Filesystem(error)),
    }
    match root.symlink_metadata(INCOMING) {
        Ok(metadata) if metadata.is_file() => {
            root.remove_file(INCOMING)
                .map_err(DestinationError::Filesystem)?;
        }
        Ok(_metadata) => return Err(DestinationError::Entry(INCOMING.to_owned())),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(DestinationError::Filesystem(error)),
    }
    let mut output = root
        .open_with(INCOMING, OpenOptions::new().write(true).create_new(true))
        .map_err(DestinationError::Filesystem)?;
    output
        .write_all(bytes)
        .map_err(DestinationError::Filesystem)?;
    output.sync_all().map_err(DestinationError::Filesystem)?;
    drop(output);
    root.rename(INCOMING, root, TEMPORARY)
        .map_err(DestinationError::Filesystem)?;
    if existing {
        root.rename(TEMPORARY, root, path)
            .map_err(DestinationError::Filesystem)
    } else {
        root.hard_link(TEMPORARY, root, path)
            .map_err(DestinationError::Filesystem)
    }
}

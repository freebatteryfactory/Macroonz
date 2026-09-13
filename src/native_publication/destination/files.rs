use super::DestinationError;
use cap_std::fs::Dir;
use std::io::ErrorKind;

pub(super) const CONTROL: &str = ".macroonz-publication";
pub(super) const CURRENT: &str = ".macroonz-publication/current";
pub(super) const PENDING: &str = ".macroonz-publication/item-pending";

pub(super) fn resolved_output(
    path: &std::path::Path,
) -> Result<std::path::PathBuf, DestinationError> {
    for ancestor in path.ancestors() {
        match ancestor.canonicalize() {
            Ok(resolved) => {
                let tail = path
                    .strip_prefix(ancestor)
                    .map_err(|error| DestinationError::Entry(error.to_string()))?;
                return Ok(resolved.join(tail));
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                match ancestor.symlink_metadata() {
                    Err(missing) if missing.kind() == ErrorKind::NotFound => {}
                    Err(other) => return Err(DestinationError::Filesystem(other)),
                    Ok(_metadata) => return Err(DestinationError::Filesystem(error)),
                }
            }
            Err(error) => return Err(DestinationError::Filesystem(error)),
        }
    }
    Err(DestinationError::Entry(
        "compiler output has no existing root".to_owned(),
    ))
}

pub(super) fn directory(root: &Dir, path: &str) -> Result<bool, DestinationError> {
    match root.symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(true),
        Ok(_metadata) => Err(DestinationError::Entry(path.to_owned())),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(DestinationError::Filesystem(error)),
    }
}

pub(super) fn read(
    root: &Dir,
    path: &str,
    maximum: usize,
) -> Result<Option<Vec<u8>>, DestinationError> {
    for (at, _) in path.match_indices('/') {
        let parent = path
            .get(..at)
            .ok_or_else(|| DestinationError::Entry(path.to_owned()))?;
        if !directory(root, parent)? {
            return Ok(None);
        }
    }
    match root.symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_metadata) => return Err(DestinationError::Entry(path.to_owned())),
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(DestinationError::Filesystem(error)),
    }
    crate::native_storage::read_bounded(root, path, maximum)
        .map(Some)
        .map_err(DestinationError::Storage)
}

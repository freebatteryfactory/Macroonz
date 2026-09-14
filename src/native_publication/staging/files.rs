use super::{StagingError, StagingPlan};
use crate::compiler::Kind;
use cap_std::fs::{Dir, OpenOptions};
use std::collections::BTreeSet;
use std::io::Write;

pub(super) fn populate<K: Kind>(
    directory: &Dir,
    plan: &StagingPlan<K>,
) -> Result<(), StagingError> {
    for (path, bytes) in plan.files() {
        if let Some((parent, _)) = path.spelling().rsplit_once('/') {
            directory
                .create_dir_all(parent)
                .map_err(StagingError::Filesystem)?;
        }
        let mut file = directory
            .open_with(
                path.spelling(),
                OpenOptions::new().write(true).create_new(true),
            )
            .map_err(StagingError::Filesystem)?;
        file.write_all(bytes).map_err(StagingError::Filesystem)?;
        file.sync_all().map_err(StagingError::Filesystem)?;
    }
    compare(directory, plan)
}

pub(super) fn compare<K: Kind>(directory: &Dir, plan: &StagingPlan<K>) -> Result<(), StagingError> {
    let expected = plan
        .files()
        .map(|(path, _)| path.spelling().to_owned())
        .collect::<BTreeSet<_>>();
    let mut directories = BTreeSet::new();
    for path in &expected {
        for (at, _) in path.match_indices('/') {
            if let Some(parent) = path.get(..at) {
                directories.insert(parent.to_owned());
            }
        }
    }
    let actual = inventory(directory, &expected, &directories)?;
    if actual != expected {
        return Err(StagingError::Source("missing staged input".to_owned()));
    }
    for (path, bytes) in plan.files() {
        let observed = crate::native_storage::read_bounded(directory, path.spelling(), bytes.len())
            .map_err(StagingError::Storage)?;
        if observed != bytes {
            return Err(StagingError::Source(path.spelling().to_owned()));
        }
    }
    Ok(())
}

fn inventory(
    root: &Dir,
    files: &BTreeSet<String>,
    directories: &BTreeSet<String>,
) -> Result<BTreeSet<String>, StagingError> {
    let mut observed = BTreeSet::new();
    let mut pending = vec![String::new()];
    while let Some(parent) = pending.pop() {
        let directory = if parent.is_empty() {
            root.try_clone()
        } else {
            root.open_dir(&parent)
        }
        .map_err(StagingError::Filesystem)?;
        for entry in directory.entries().map_err(StagingError::Filesystem)? {
            let entry = entry.map_err(StagingError::Filesystem)?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| StagingError::Source("non-Unicode stage entry".to_owned()))?;
            let path = if parent.is_empty() {
                name
            } else {
                format!("{parent}/{name}")
            };
            let kind = entry.file_type().map_err(StagingError::Filesystem)?;
            if kind.is_dir() && directories.contains(&path) {
                pending.push(path);
            } else if kind.is_file() && files.contains(&path) {
                observed.insert(path);
            } else {
                return Err(StagingError::Source(path));
            }
        }
    }
    Ok(observed)
}

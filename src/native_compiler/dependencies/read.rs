use super::{DependencyError, DependencyInfo};
use crate::native_compiler::types::{DependencyCapture, Observation};
use std::io::Read;
use std::path::Path;

pub(in crate::native_compiler) fn capture(
    selection: &DependencyCapture,
    root: &Path,
    observation: Option<&Observation>,
) -> Result<DependencyInfo, DependencyError> {
    let artifact = observation
        .and_then(|value| value.artifact.as_ref())
        .ok_or(DependencyError::NotCompiled)?;
    let metadata = std::fs::symlink_metadata(&selection.path)
        .map_err(|error| DependencyError::Filesystem(error.to_string()))?;
    if !metadata.is_file() {
        return Err(DependencyError::NotRegular);
    }
    let mut file = std::fs::File::open(&selection.path)
        .map_err(|error| DependencyError::Filesystem(error.to_string()))?;
    if !file
        .metadata()
        .map_err(|error| DependencyError::Filesystem(error.to_string()))?
        .is_file()
    {
        return Err(DependencyError::NotRegular);
    }
    let maximum = u64::try_from(selection.limits.bytes).map_err(|_| DependencyError::ByteBound)?;
    let mut bytes = Vec::new();
    Read::by_ref(&mut file)
        .take(maximum)
        .read_to_end(&mut bytes)
        .map_err(|error| DependencyError::Filesystem(error.to_string()))?;
    let mut extra = [0u8; 1];
    if file
        .read(&mut extra)
        .map_err(|error| DependencyError::Filesystem(error.to_string()))?
        != 0usize
    {
        return Err(DependencyError::ByteBound);
    }
    DependencyInfo::captured(bytes, root, &artifact.files, selection.limits)
}

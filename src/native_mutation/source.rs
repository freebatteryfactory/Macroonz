use super::types::SourceMaterial;
use super::{MutationError, MutationSources};
use std::io::Read;
use std::path::Path;

pub(super) fn capture(
    root: &Path,
    sources: &MutationSources,
) -> Result<Vec<SourceMaterial>, MutationError> {
    let root = root.canonicalize().map_err(MutationError::Filesystem)?;
    if !root.is_dir() {
        return Err(MutationError::Filesystem(std::io::Error::other(
            "source root is not a directory",
        )));
    }
    let mut remaining = sources.bytes;
    let mut material = Vec::with_capacity(sources.files.len());
    for file in &sources.files {
        let path = root
            .join(file.spelling())
            .canonicalize()
            .map_err(MutationError::Filesystem)?;
        if !path.starts_with(&root) {
            return Err(MutationError::Filesystem(std::io::Error::other(format!(
                "source escapes declared root: {}",
                file.spelling()
            ))));
        }
        if !std::fs::metadata(&path)
            .map_err(MutationError::Filesystem)?
            .is_file()
        {
            return Err(MutationError::Filesystem(std::io::Error::other(format!(
                "source is not a regular file: {}",
                file.spelling()
            ))));
        }
        let input = std::fs::File::open(&path).map_err(MutationError::Filesystem)?;
        if !input
            .metadata()
            .map_err(MutationError::Filesystem)?
            .is_file()
        {
            return Err(MutationError::Filesystem(std::io::Error::other(format!(
                "source is not a regular file: {}",
                file.spelling()
            ))));
        }
        let ceiling = u64::try_from(remaining).map_err(|_| MutationError::SourceBound {
            bound: sources.bytes,
        })?;
        let mut bytes = Vec::new();
        input
            .take(ceiling.saturating_add(1))
            .read_to_end(&mut bytes)
            .map_err(MutationError::Filesystem)?;
        remaining = remaining
            .checked_sub(bytes.len())
            .ok_or(MutationError::SourceBound {
                bound: sources.bytes,
            })?;
        material.push(SourceMaterial {
            file: file.spelling().to_owned(),
            bytes,
        });
    }
    Ok(material)
}

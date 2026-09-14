use super::{InventoryError, PublicationLimits, PublicationPath};
use std::collections::BTreeSet;

pub(in crate::native_publication) fn bounded_paths(
    files: impl Iterator<Item = (PublicationPath, usize)>,
    limits: PublicationLimits,
) -> Result<BTreeSet<String>, InventoryError> {
    let mut names = BTreeSet::new();
    let mut remaining = limits.bytes;
    for (path, bytes) in files {
        if names.len() >= limits.files {
            return Err(InventoryError::FileBound);
        }
        if !names.insert(path.spelling().to_ascii_lowercase()) {
            return Err(InventoryError::PathCollision);
        }
        remaining = remaining
            .checked_sub(bytes)
            .ok_or(InventoryError::ByteBound)?;
    }
    if names.iter().any(|name| {
        name.match_indices('/')
            .filter_map(|(at, _)| name.get(..at))
            .any(|prefix| names.contains(prefix))
    }) {
        return Err(InventoryError::PathCollision);
    }
    Ok(names)
}

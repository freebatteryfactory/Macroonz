//! The complete physical roster returns to the canonical historical reader.

use super::{RetentionLimits, RetentionRefusal};
use crate::harness::bench::archive::{ArchivedBenchReport, read_report};
use crate::native_storage::{StorageName, StorageRoot};

/// Read one complete historical benchmark through its canonical archive owner.
///
/// # Errors
/// Refuses storage faults, missing or extra members and invalid or oversized archive bytes.
pub fn load(
    root: &StorageRoot,
    name: &StorageName,
    limits: RetentionLimits,
) -> Result<ArchivedBenchReport, RetentionRefusal> {
    let files = root
        .load(name, limits.storage)
        .map_err(RetentionRefusal::Storage)?;
    let [file] = files.as_slice() else {
        return Err(RetentionRefusal::Members);
    };
    if file.name.spelling() != "benchmark" {
        return Err(RetentionRefusal::Members);
    }
    read_report(&file.bytes, limits.archive).map_err(RetentionRefusal::Archive)
}

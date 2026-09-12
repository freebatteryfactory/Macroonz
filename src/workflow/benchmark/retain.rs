//! Complete benchmark bytes cross the existing storage protocol.

use super::{RetentionLimits, RetentionRefusal};
use crate::harness::bench::BenchReport;
use crate::harness::bench::archive::{BenchArchiveLimits, BenchArchiveRefusal, retain_report};
use crate::harness::report::archive::{ArchiveLimits, ArchiveRefusal};
use crate::native_storage::{
    StorageArtifact, StorageBatch, StorageError, StorageName, StorageRoot, StorageTransaction,
};

/// Publish the complete report under an explicitly selected batch name.
///
/// # Errors
/// Preserves archive and storage refusals without consuming or changing the report.
pub fn retain(
    report: &BenchReport,
    root: &StorageRoot,
    name: &StorageName,
    limits: RetentionLimits,
) -> Result<(), RetentionRefusal> {
    store(report, limits, |batch| {
        StorageTransaction::begin(root, name, batch)
    })
}

/// Replace an unpublished attempt with the complete supplied benchmark report.
///
/// # Errors
/// Preserves archive bounds and the storage owner's explicit recovery refusals.
pub fn recover_retention(
    report: &BenchReport,
    root: &StorageRoot,
    name: &StorageName,
    limits: RetentionLimits,
) -> Result<(), RetentionRefusal> {
    store(report, limits, |batch| {
        StorageTransaction::recover(root, name, batch)
    })
}

fn store(
    report: &BenchReport,
    limits: RetentionLimits,
    open: impl for<'data> FnOnce(StorageBatch<'data>) -> Result<StorageTransaction<'data>, StorageError>,
) -> Result<(), RetentionRefusal> {
    if limits.storage.artifacts == 0 {
        return Err(RetentionRefusal::Storage(StorageError::ArtifactBound));
    }
    let bytes = limits.archive.bytes();
    let bounded = BenchArchiveLimits::declared(
        ArchiveLimits::declared(bytes.envelope().min(limits.storage.bytes), bytes.field()),
        limits.archive.rows(),
        limits.archive.axis(),
        limits.archive.observations(),
        limits.archive.measurements(),
    );
    let archive = retain_report(report, bounded).map_err(|error| {
        if error == BenchArchiveRefusal::Canonical(ArchiveRefusal::EnvelopeTooLarge)
            && limits.storage.bytes < bytes.envelope()
        {
            RetentionRefusal::Storage(StorageError::ByteBound)
        } else {
            RetentionRefusal::Archive(error)
        }
    })?;
    let member = StorageName::informed("benchmark").map_err(RetentionRefusal::Storage)?;
    let artifacts = [StorageArtifact {
        name: &member,
        bytes: archive.encoded(),
    }];
    let batch =
        StorageBatch::informed(&artifacts, limits.storage).map_err(RetentionRefusal::Storage)?;
    let mut transaction = open(batch).map_err(RetentionRefusal::Storage)?;
    while transaction
        .write_next()
        .map_err(RetentionRefusal::Storage)?
        .is_some()
    {}
    transaction.commit().map_err(RetentionRefusal::Storage)
}

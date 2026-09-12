//! Independent retention bounds and the owners that can refuse them.

use crate::harness::bench::archive::{BenchArchiveLimits, BenchArchiveRefusal};
use crate::native_storage::{StorageError, StorageLimits};

/// Independent canonical benchmark and physical storage bounds.
#[derive(Debug, Clone, Copy)]
pub struct RetentionLimits {
    /// Complete archive byte, field and population ceilings.
    pub archive: BenchArchiveLimits,
    /// Physical artifact count and aggregate payload bytes.
    pub storage: StorageLimits,
}

/// Why benchmark retention or historical loading did not finish.
#[derive(Debug)]
pub enum RetentionRefusal {
    /// The native storage owner refused its operation.
    Storage(StorageError),
    /// The benchmark archive owner refused its representation or bounds.
    Archive(BenchArchiveRefusal),
    /// A published batch does not contain exactly the benchmark member.
    Members,
}

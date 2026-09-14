//! Existing archive and storage bounds composed for retained input runs.

use crate::harness::report::archive::{ArchiveLimits, RunArchiveLimits};
use crate::native_storage::StorageLimits;
use crate::workflow::RetentionLimits;

/// The version-one input, archive, census and batch ceilings.
#[must_use]
pub const fn retention_limits() -> RetentionLimits {
    RetentionLimits {
        input: super::input_limits(),
        archive: RunArchiveLimits::declared(ArchiveLimits::declared(16_777_216, 2_097_152), 4096),
        storage: StorageLimits {
            artifacts: 258,
            bytes: 67_108_864,
        },
    }
}

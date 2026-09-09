//! Existing input and report owners joined for ordinary execution and retention.

use crate::harness::{input::InputEnvelope, report::RunReport};
#[cfg(feature = "native-tooling")]
use crate::harness::{
    input::{InputLimits, InputRefusal},
    report::archive::{ArchiveRefusal, ArchivedCapsule, ArchivedRun, RunArchiveLimits},
};
#[cfg(feature = "native-tooling")]
use crate::native_storage::{StorageError, StorageLimits};

#[path = "type_guard.rs"]
mod guard;
pub use guard::run;

/// A complete run report beside the original input that entered its invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputRun {
    report: RunReport,
    input: InputEnvelope,
}

/// Independent input, archive and storage bounds for one retained run.
#[cfg(feature = "native-tooling")]
#[derive(Debug, Clone, Copy)]
pub struct RetentionLimits {
    /// Input envelope and payload admission limits.
    pub input: InputLimits,
    /// Report/capsule envelope, field and complete-run census limits.
    pub archive: RunArchiveLimits,
    /// Batch artifact count and aggregate encoded bytes.
    pub storage: StorageLimits,
}

/// Historical run data joined to its original input and optional reached witnesses.
#[cfg(feature = "native-tooling")]
#[derive(Debug, Clone)]
pub struct StoredRun {
    report: ArchivedRun,
    input: InputEnvelope,
    capsules: std::collections::BTreeMap<usize, ArchivedCapsule>,
}

/// Why a run could not cross retention or saved-witness execution.
#[cfg(feature = "native-tooling")]
#[derive(Debug)]
pub enum RetentionRefusal {
    /// The storage owner refused an effect or physical inventory.
    Storage(StorageError),
    /// The existing archive owner refused its bytes or bounds.
    Archive(ArchiveRefusal),
    /// The existing input owner refused the envelope or current decoder.
    Input(InputRefusal),
    /// The batch does not have the run workflow's mechanical members.
    Members,
    /// The historical run and retained original input name different specimens or profiles.
    InputJoin,
    /// A capsule is not joined to its original failing census row, or repeats that row.
    CapsuleJoin,
    /// The selected census row has no retained replay capsule.
    CapsuleAbsent,
}

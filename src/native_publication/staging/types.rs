use crate::compiler::Kind;
use crate::native_compiler::{CompilerOutput, PendingCompilation};
use crate::native_publication::{InventoryError, PreparedPublication, PublicationPath};
use crate::native_storage::StorageError;
use std::path::PathBuf;

#[path = "type_guard.rs"]
mod guard;

#[derive(Debug, Clone, Copy)]
pub(super) struct StagingSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CacheUse {
    Refuse,
    Compare,
}

/// One explicitly supplied authored input for a private compilation subject.
#[derive(Debug, Clone)]
pub struct AuthoredFile {
    /// Its portable relative position in the staged source tree.
    pub path: PublicationPath,
    /// Its exact caller-owned source, manifest, lockfile or other declared input bytes.
    pub bytes: Vec<u8>,
}

/// A complete bounded union of prepared output and explicitly supplied authored inputs.
#[derive(Debug)]
pub struct StagingPlan<K: Kind> {
    pub(super) prepared: PreparedPublication<K>,
    pub(super) authored: Vec<AuthoredFile>,
}

/// A privately created source tree and the original immutable plan used to populate it.
#[derive(Debug)]
pub struct StagedPublication<K: Kind> {
    pub(super) plan: StagingPlan<K>,
    pub(super) path: PathBuf,
    #[cfg(any(unix, windows))]
    pub(super) directory: cap_std::fs::Dir,
}

/// A refusal before staged compilation begins.
#[derive(Debug)]
pub enum StagingError {
    /// Source paths collide or the union exceeds its shared allowance.
    Inventory(InventoryError),
    /// The selected source, output or compilation request cannot establish this staging boundary.
    Configuration(String),
    /// Directory-relative storage admission or bounded read failed.
    Storage(StorageError),
    /// A private staging filesystem operation failed.
    Filesystem(std::io::Error),
    /// A staged file or directory differs from the complete declared source tree.
    Source(String),
    /// Compiler process admission or startup failed.
    Compiler(crate::native_compiler::CompilerError),
    /// The target has no selected native staging implementation.
    Unavailable,
}

/// Why completed compiler output did not qualify the staged publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StagingObservationError {
    /// The declared compilation did not establish success.
    Compilation,
    /// Compiler dependency capture refused or did not establish the requested roster.
    Dependencies(String),
    /// A prepared file did not occur in the selected artifact's dependency rule.
    Unused(String),
    /// The source tree changed or could not be compared after execution.
    Source(String),
}

/// A successful staged compilation bound to unchanged inputs and every prepared file's dependency membership.
#[derive(Debug)]
pub struct CompiledPublication<K: Kind> {
    pub(super) staged: StagedPublication<K>,
    pub(super) compiler: CompilerOutput,
}

/// A refused staged compilation retaining its actual compiler and source context.
#[derive(Debug)]
pub struct RefusedStaging<K: Kind> {
    pub(super) staged: StagedPublication<K>,
    pub(super) compiler: CompilerOutput,
    pub(super) reason: StagingObservationError,
}

/// Qualified staged material, a completed refusal or retained compiler cleanup.
#[derive(Debug)]
#[must_use]
pub enum StagingRun<K: Kind> {
    /// The selected compilation and staged source checks passed.
    Compiled(Box<CompiledPublication<K>>),
    /// Compiler cleanup finished, but publication qualification refused.
    Refused(Box<RefusedStaging<K>>),
    /// Compiler cleanup remains owned and retryable.
    Pending(Box<PendingStaging<K>>),
}

/// The source tree and still-owned compiler process awaiting cleanup.
#[derive(Debug)]
#[must_use]
pub struct PendingStaging<K: Kind> {
    pub(super) staged: StagedPublication<K>,
    pub(super) compiler: Box<PendingCompilation>,
}

use crate::native_publication::PublicationPath;
#[cfg(any(unix, windows))]
use crate::native_storage::StorageRoot;

#[path = "type_guard.rs"]
mod guard;

/// Bounds for the complete previous and expected destination union and its ownership record.
#[derive(Debug, Clone, Copy)]
pub struct DestinationLimits {
    /// Maximum distinct paths inspected across the previous and expected rosters.
    pub files: usize,
    /// Maximum combined physical bytes inspected in that union.
    pub bytes: usize,
    /// Maximum physical ownership-record bytes.
    pub metadata: usize,
}

/// An explicitly opened publication destination and its operation bounds.
#[derive(Debug)]
pub struct PublicationDestination {
    #[cfg(any(unix, windows))]
    pub(super) root: StorageRoot,
    #[cfg(any(unix, windows))]
    pub(super) path: std::path::PathBuf,
    pub(super) limits: DestinationLimits,
}

/// The complete read-only comparison of expected publication with recorded ownership and current files.
#[derive(Debug)]
pub struct DestinationCheck {
    pub(super) state: DestinationState,
    pub(super) issues: Vec<DestinationIssue>,
}

/// One observed destination discrepancy without overwrite or deletion authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestinationIssue {
    /// The admitted relative path where this discrepancy occurred.
    pub path: PublicationPath,
    /// Its observed relationship to the expected and recorded files.
    pub problem: DestinationProblem,
}

/// The observed relationship between installed ownership and an installation journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationState {
    /// Neither installed ownership nor an installation journal exists.
    Uninitialized,
    /// Installed ownership exists without a pending installation journal.
    Installed,
    /// An initial installation journal exists without installed ownership.
    Preparing,
    /// Both installed ownership and an installation journal exist.
    Updating,
}

/// Why a destination does not match the complete expected publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationProblem {
    /// An expected or previously owned file is absent.
    Missing,
    /// A present expected destination has no previous ownership record.
    Unowned,
    /// The previously installed record or file differs from the new expected output.
    Stale,
    /// Current physical bytes differ from the previously installed commitment.
    Tampered,
    /// A previously owned path is absent from the complete new output roster.
    ExtraOwned,
}

/// Why destination observation or publication custody refused.
#[derive(Debug)]
pub enum DestinationError {
    /// Complete path, count or physical byte admission failed.
    Inventory(crate::native_publication::InventoryError),
    /// An ownership record is malformed, noncanonical or exceeds its byte bound.
    Metadata(String),
    /// An entry is a link or has an incompatible filesystem kind.
    Entry(String),
    /// Directory access, bounded reading or cooperating lock custody failed.
    Storage(crate::native_storage::StorageError),
    /// The operating system refused a destination operation.
    Filesystem(std::io::Error),
    /// An installed record exists without its required cooperating lock.
    MissingLock,
    /// Another committed installation intent requires recovery before a new installation.
    Pending,
    /// A current file or ownership record disagrees with the admitted installation intent.
    Conflict(String),
    /// Not every intended file operation has been completed.
    Incomplete,
    /// This target has no selected native destination implementation.
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OwnedFile {
    pub(super) path: PublicationPath,
    pub(super) canonical: [u8; 32],
    pub(super) published: [u8; 32],
    pub(super) bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Ownership {
    pub(super) files: Vec<OwnedFile>,
}

#[cfg(any(unix, windows))]
pub(super) type WireFile = (String, [u8; 32], [u8; 32], usize);

/// Exclusive custody of a journaled publication and its remaining destination operations.
#[derive(Debug)]
pub struct PublicationInstallation<'root> {
    pub(super) destination: &'root PublicationDestination,
    #[cfg(any(unix, windows))]
    pub(super) lease: std::fs::File,
    pub(super) intent: InstallationIntent,
    pub(super) paths: Vec<PublicationPath>,
    pub(super) cursor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InstallationIntent {
    pub(super) before: Option<Ownership>,
    pub(super) after: Ownership,
    pub(super) payloads: Vec<Vec<u8>>,
}

#[cfg(any(unix, windows))]
pub(super) type WireIntent = (String, Option<Vec<u8>>, Vec<u8>, Vec<Vec<u8>>);

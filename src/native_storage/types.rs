//! Native byte custody and its declared resource bounds.

#[path = "type_guard.rs"]
mod guard;

/// One portable storage name, independent of archive identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageName(String);

/// Maximum artifact count and aggregate payload bytes for one operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageLimits {
    /// Maximum number of payload files.
    pub artifacts: usize,
    /// Maximum combined payload length.
    pub bytes: usize,
}

/// One caller-declared payload under a storage name.
#[derive(Debug, Clone, Copy)]
pub struct StorageArtifact<'data> {
    /// The payload's mechanical name within its batch.
    pub name: &'data StorageName,
    /// Bytes supplied by their existing encoding owner.
    pub bytes: &'data [u8],
}

/// A nonempty, duplicate-free batch within declared count and byte bounds.
#[derive(Debug, Clone, Copy)]
pub struct StorageBatch<'data> {
    artifacts: &'data [StorageArtifact<'data>],
    limits: StorageLimits,
}

/// An opened directory capability for explicitly requested storage operations.
#[derive(Debug)]
pub struct StorageRoot {
    #[cfg(any(unix, windows))]
    directory: cap_std::fs::Dir,
}

/// Exclusive custody of one unpublished batch and its remaining writes.
#[derive(Debug)]
pub struct StorageTransaction<'data> {
    #[cfg(any(unix, windows))]
    directory: cap_std::fs::Dir,
    #[cfg(any(unix, windows))]
    lease: std::fs::File,
    batch: StorageBatch<'data>,
    written: usize,
}

/// Bytes read from a published batch without semantic or execution authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredArtifact {
    /// The mechanical payload name.
    pub name: StorageName,
    /// The bounded bytes observed on disk.
    pub bytes: Vec<u8>,
}

/// Why a native storage operation did not deliver its requested result.
#[derive(Debug)]
pub enum StorageError {
    /// A name violates the storage namespace grammar.
    InvalidName,
    /// A batch contains no payloads.
    EmptyBatch,
    /// An artifact count exceeds the declared bound.
    ArtifactBound,
    /// Payload or control bytes exceed their operation's read or write bound.
    ByteBound,
    /// A batch repeats a payload name.
    DuplicateName,
    /// Another cooperating operation holds the storage root's lock.
    Busy,
    /// A batch already exists, or recovery addressed a published batch.
    Collision,
    /// A batch has not reached publication or still has unwritten payloads.
    Incomplete,
    /// A directory entry is outside the expected mechanical inventory.
    UnexpectedEntry,
    /// The physical payload roster differs from the published roster.
    InventoryMismatch,
    /// A payload or control entry is not an ordinary file.
    NotRegular,
    /// A control file has an invalid representation.
    InvalidMarker,
    /// The target has no selected native storage implementation.
    Unavailable,
    /// The operating system refused an operation.
    Io(std::io::Error),
}

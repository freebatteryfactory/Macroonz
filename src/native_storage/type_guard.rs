//! Informed names, bounded batches and exclusive transaction transitions.

#[cfg(any(unix, windows))]
use super::super::{custody, read, write};
use super::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageRoot,
    StorageTransaction, StoredArtifact,
};
use std::path::Path;

impl StorageName {
    /// Admit one to ninety-six lowercase ASCII letters, digits, underscores or hyphens.
    ///
    /// # Errors
    /// Refuses every other spelling without normalization.
    pub fn informed(spelling: &str) -> Result<Self, StorageError> {
        if spelling.is_empty()
            || spelling.len() > 96usize
            || !spelling.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-')
            })
        {
            return Err(StorageError::InvalidName);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The admitted logical spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl<'data> StorageBatch<'data> {
    /// Admit a complete payload roster before any filesystem operation.
    ///
    /// # Errors
    /// Refuses empty, oversized or duplicate-bearing batches.
    pub fn informed(
        artifacts: &'data [StorageArtifact<'data>],
        limits: StorageLimits,
    ) -> Result<Self, StorageError> {
        if artifacts.is_empty() {
            return Err(StorageError::EmptyBatch);
        }
        if artifacts.len() > limits.artifacts {
            return Err(StorageError::ArtifactBound);
        }
        let mut remaining = limits.bytes;
        let mut names = std::collections::BTreeSet::new();
        for artifact in artifacts {
            if !names.insert(artifact.name) {
                return Err(StorageError::DuplicateName);
            }
            remaining = remaining
                .checked_sub(artifact.bytes.len())
                .ok_or(StorageError::ByteBound)?;
        }
        Ok(Self { artifacts, limits })
    }

    /// The caller's payloads in write order.
    #[must_use]
    pub const fn artifacts(self) -> &'data [StorageArtifact<'data>] {
        self.artifacts
    }

    /// The admitted operation bounds.
    #[must_use]
    pub const fn limits(self) -> StorageLimits {
        self.limits
    }
}

impl StorageRoot {
    /// Bind an existing caller-selected directory without creating or scanning its parents.
    ///
    /// # Errors
    /// Reports directory-open failure or target unavailability.
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        #[cfg(any(unix, windows))]
        {
            Ok(Self {
                directory: custody::open(path)?,
            })
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _path = path;
            Err(StorageError::Unavailable)
        }
    }

    /// Read all payloads of one published batch in lexical name order.
    ///
    /// # Errors
    /// Refuses incomplete batches, bounds, malformed entries, lock contention and operating-system failures.
    pub fn load(
        &self,
        name: &StorageName,
        limits: StorageLimits,
    ) -> Result<Vec<StoredArtifact>, StorageError> {
        #[cfg(any(unix, windows))]
        {
            let lease = custody::lock(&self.directory)?;
            let directory = custody::batch(&self.directory, name)?;
            let result = read::load(&directory, limits);
            drop(lease);
            result
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _inputs = (name, limits);
            Err(StorageError::Unavailable)
        }
    }
}

impl<'data> StorageTransaction<'data> {
    /// Reserve a new batch while retaining exclusive storage-root custody.
    ///
    /// # Errors
    /// Refuses existing names, lock contention and operating-system failures before any payload overwrite.
    pub fn begin(
        root: &StorageRoot,
        name: &StorageName,
        batch: StorageBatch<'data>,
    ) -> Result<Self, StorageError> {
        #[cfg(any(unix, windows))]
        {
            let lease = custody::lock(&root.directory)?;
            let directory = write::reserve(&root.directory, name)?;
            Ok(Self {
                directory,
                lease,
                batch,
                written: 0,
            })
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _inputs = (root, name, batch);
            Err(StorageError::Unavailable)
        }
    }

    /// Restart an unpublished batch from explicitly supplied replacement payloads.
    ///
    /// # Errors
    /// Refuses published batches and undeclared entries before removing any payload; interrupted removal remains recoverable.
    pub fn recover(
        root: &StorageRoot,
        name: &StorageName,
        batch: StorageBatch<'data>,
    ) -> Result<Self, StorageError> {
        #[cfg(any(unix, windows))]
        {
            let lease = custody::lock(&root.directory)?;
            let directory = custody::batch(&root.directory, name)?;
            write::restart(&directory, batch)?;
            Ok(Self {
                directory,
                lease,
                batch,
                written: 0,
            })
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _inputs = (root, name, batch);
            Err(StorageError::Unavailable)
        }
    }

    /// Create and flush the next payload, or report that no write remains.
    ///
    /// # Errors
    /// Reports exclusive-creation or write failure without advancing the cursor.
    pub fn write_next(&mut self) -> Result<Option<&'data StorageName>, StorageError> {
        let Some(artifact) = self.batch.artifacts.get(self.written) else {
            return Ok(None);
        };
        #[cfg(any(unix, windows))]
        {
            write::payload(&self.directory, artifact)?;
            self.written = self
                .written
                .checked_add(1)
                .ok_or(StorageError::ArtifactBound)?;
            Ok(Some(artifact.name))
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _artifact = artifact;
            Err(StorageError::Unavailable)
        }
    }

    /// Publish a fully written batch and release its exclusive custody.
    ///
    /// # Errors
    /// Refuses unwritten payloads or an operating-system failure before publication.
    pub fn commit(self) -> Result<(), StorageError> {
        if self.written != self.batch.artifacts.len() {
            return Err(StorageError::Incomplete);
        }
        #[cfg(any(unix, windows))]
        {
            let result = write::publish(&self.directory, self.batch);
            drop(self.directory);
            drop(self.lease);
            result
        }
        #[cfg(not(any(unix, windows)))]
        {
            Err(StorageError::Unavailable)
        }
    }
}

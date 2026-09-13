use super::{
    DestinationCheck, DestinationError, DestinationIssue, DestinationLimits, DestinationState,
    PublicationDestination,
};
use super::{InstallationIntent, OwnedFile, Ownership, PublicationInstallation};
use crate::compiler::Kind;
use crate::native_publication::{CompiledPublication, PreparedPublication, PublicationPath};
use std::path::Path;

impl Ownership {
    pub(in crate::native_publication::destination) fn admitted(
        files: Vec<OwnedFile>,
        limits: DestinationLimits,
    ) -> Result<Self, DestinationError> {
        if files.windows(2).any(|pair| match pair {
            [left, right] => left.path.spelling() >= right.path.spelling(),
            _ => false,
        }) {
            return Err(DestinationError::Metadata("ownership order".to_owned()));
        }
        drop(
            crate::native_publication::inventory::bounded_paths(
                files.iter().map(|file| (file.path.clone(), file.bytes)),
                crate::native_publication::PublicationLimits {
                    files: limits.files,
                    bytes: limits.bytes,
                },
            )
            .map_err(DestinationError::Inventory)?,
        );
        let record = Self { files };
        if super::super::record::encode(&record)?.len() > limits.metadata {
            return Err(DestinationError::Metadata(
                "ownership byte bound".to_owned(),
            ));
        }
        Ok(record)
    }
}

impl PublicationDestination {
    pub(in crate::native_publication) fn excludes_output(
        &self,
        path: &Path,
    ) -> Result<(), DestinationError> {
        #[cfg(any(unix, windows))]
        {
            let output = super::super::files::resolved_output(path)?;
            if output.starts_with(&self.path) || self.path.starts_with(&output) {
                return Err(DestinationError::Conflict(
                    "compiler outputs must be disjoint from the publication destination".to_owned(),
                ));
            }
            Ok(())
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _path = path;
            Err(DestinationError::Unavailable)
        }
    }

    pub(in crate::native_publication) fn excludes_workspace(
        &self,
        path: &Path,
    ) -> Result<(), DestinationError> {
        #[cfg(any(unix, windows))]
        {
            let workspace = path.canonicalize().map_err(DestinationError::Filesystem)?;
            if workspace.starts_with(&self.path) {
                return Err(DestinationError::Conflict(
                    "the disposable workspace must be outside the publication destination"
                        .to_owned(),
                ));
            }
            Ok(())
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _path = path;
            Err(DestinationError::Unavailable)
        }
    }

    /// Preflights all destinations and retains a committed installation intent before changing output files.
    ///
    /// # Errors
    /// Refuses authored collisions, tampering, bounds, pending committed work and native custody failures.
    pub fn begin<K: Kind>(
        &self,
        compiled: &CompiledPublication<K>,
    ) -> Result<PublicationInstallation<'_>, DestinationError> {
        #[cfg(any(unix, windows))]
        {
            super::super::install::begin(self, compiled.prepared())
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _compiled = compiled;
            Err(DestinationError::Unavailable)
        }
    }

    /// Recovers a committed installation intent or retires an empty completed-preparation remainder.
    ///
    /// # Errors
    /// Refuses malformed intent, incompatible current bytes, incomplete preparation and native custody failures.
    pub fn recover(&self) -> Result<Option<PublicationInstallation<'_>>, DestinationError> {
        #[cfg(any(unix, windows))]
        {
            super::super::install::recover(self)
        }
        #[cfg(not(any(unix, windows)))]
        {
            Err(DestinationError::Unavailable)
        }
    }

    /// Opens the explicitly supplied existing destination without creating files or directories.
    ///
    /// # Errors
    /// Refuses filesystem access failure or an unavailable target.
    pub fn open(path: &Path, limits: DestinationLimits) -> Result<Self, DestinationError> {
        #[cfg(any(unix, windows))]
        {
            let path = path.canonicalize().map_err(DestinationError::Filesystem)?;
            Ok(Self {
                root: crate::native_storage::StorageRoot::open(&path)
                    .map_err(DestinationError::Storage)?,
                path,
                limits,
            })
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _inputs = (path, limits);
            Err(DestinationError::Unavailable)
        }
    }

    /// Compares the complete publication against recorded ownership without writing any file.
    ///
    /// # Errors
    /// Refuses malformed records, bounds, links, incompatible entries and lock or filesystem failure.
    pub fn check<K: Kind>(
        &self,
        expected: &PreparedPublication<K>,
    ) -> Result<DestinationCheck, DestinationError> {
        let expected = super::super::record::expected(expected, self.limits)?;
        #[cfg(any(unix, windows))]
        {
            super::super::inspect::check(self, &expected)
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _expected = expected;
            Err(DestinationError::Unavailable)
        }
    }
}

impl PublicationInstallation<'_> {
    /// Applies the next journaled file replacement or removal and advances only after success.
    ///
    /// # Errors
    /// Refuses unexpected current content, path changes and filesystem failures without losing journal custody.
    pub fn write_next(&mut self) -> Result<Option<&PublicationPath>, DestinationError> {
        let Some(path) = self.paths.get(self.cursor) else {
            return Ok(None);
        };
        #[cfg(any(unix, windows))]
        {
            super::super::install::retained(self)?;
            super::super::replace::apply(
                self.destination.root.directory(),
                &self.intent,
                path,
                self.destination.limits,
            )?;
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _inputs = (path, self.destination, &self.intent);
            return Err(DestinationError::Unavailable);
        }
        #[cfg(any(unix, windows))]
        {
            let at = self.cursor;
            self.cursor = self
                .cursor
                .checked_add(1)
                .ok_or(DestinationError::Incomplete)?;
            Ok(self.paths.get(at))
        }
    }

    /// Verifies every applied output, updates ownership and retires the bounded installation journal.
    ///
    /// # Errors
    /// Refuses remaining operations, changed content, changed journal or filesystem failure; interrupted retirement remains recoverable.
    pub fn commit(self) -> Result<(), DestinationError> {
        if self.cursor != self.paths.len() {
            return Err(DestinationError::Incomplete);
        }
        #[cfg(any(unix, windows))]
        {
            let result = super::super::install::commit(&self);
            drop(self.lease);
            result
        }
        #[cfg(not(any(unix, windows)))]
        {
            Err(DestinationError::Unavailable)
        }
    }
}

impl InstallationIntent {
    #[cfg(any(unix, windows))]
    pub(in crate::native_publication::destination) fn admitted(
        before: Option<Ownership>,
        after: Ownership,
        payloads: Vec<Vec<u8>>,
        limits: DestinationLimits,
    ) -> Result<Self, DestinationError> {
        if after.files.len() != payloads.len() {
            return Err(DestinationError::Metadata(
                "intent payload roster".to_owned(),
            ));
        }
        for (file, bytes) in after.files.iter().zip(&payloads) {
            if file.bytes != bytes.len()
                || file.published != *crate::native_publication::published_digest(bytes).as_bytes()
            {
                return Err(DestinationError::Metadata(
                    "intent payload commitment".to_owned(),
                ));
            }
        }
        let paths = super::super::inspect::paths(before.as_ref(), &after, limits)?;
        drop(
            crate::native_publication::inventory::bounded_paths(
                paths.into_iter().map(|path| {
                    let bytes = before
                        .iter()
                        .flat_map(|record| &record.files)
                        .chain(&after.files)
                        .filter(|file| file.path == path)
                        .map(|file| file.bytes)
                        .max()
                        .unwrap_or(0);
                    (path, bytes)
                }),
                crate::native_publication::PublicationLimits {
                    files: limits.files,
                    bytes: limits.bytes,
                },
            )
            .map_err(DestinationError::Inventory)?,
        );
        Ok(Self {
            before,
            after,
            payloads,
        })
    }
}

impl DestinationCheck {
    /// Whether ownership exists, no installation is pending and every inspected path agrees.
    #[must_use]
    pub fn is_current(&self) -> bool {
        self.state == DestinationState::Installed && self.issues.is_empty()
    }

    /// The observed ownership and pending-installation relationship.
    #[must_use]
    pub const fn state(&self) -> DestinationState {
        self.state
    }

    /// Every reached path discrepancy in lexical path order.
    #[must_use]
    pub fn issues(&self) -> &[DestinationIssue] {
        &self.issues
    }
}

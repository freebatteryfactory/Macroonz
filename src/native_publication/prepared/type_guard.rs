use super::{Material, PreparationError, PreparedFile, PreparedPublication};
use crate::compiler::Kind;
use crate::compiler::identity::Identity;
use crate::native_publication::{
    CanonicalPublicationBytes, FormatOutput, Publication, PublicationPath, PublishedBytes,
    published_digest,
};
use std::collections::BTreeMap;

impl<K: Kind> PreparedPublication<K> {
    /// Prepares the exact unformatted source already admitted by the inventory's shared bounds.
    pub fn unformatted(publication: Publication<K>) -> Self {
        let files = publication
            .files()
            .map(|file| Material::Unformatted {
                path: file.path().clone(),
                canonical: file.canonical_digest(),
                source: file.source(),
            })
            .collect();
        Self { publication, files }
    }

    /// Joins every actual formatter output to the complete original source roster.
    ///
    /// # Errors
    /// Refuses incomplete or repeated outputs, changed source, formatter refusals and aggregate bytes.
    pub fn formatted(
        publication: Publication<K>,
        outputs: Vec<FormatOutput>,
        maximum_bytes: usize,
    ) -> Result<Self, PreparationError> {
        if outputs.len() != publication.files().count() {
            return Err(PreparationError::Inventory);
        }
        let mut by_path = BTreeMap::new();
        for output in outputs {
            if by_path
                .insert(output.path().spelling().to_owned(), output)
                .is_some()
            {
                return Err(PreparationError::Inventory);
            }
        }
        let mut files = Vec::new();
        let mut remaining = maximum_bytes;
        for expected in publication.files() {
            let output = by_path
                .remove(expected.path().spelling())
                .ok_or(PreparationError::Inventory)?;
            if output.canonical_digest() != expected.canonical_digest()
                || output.original() != expected.source()
            {
                return Err(PreparationError::Source);
            }
            let source = output.source().map_err(PreparationError::Formatter)?;
            remaining = remaining
                .checked_sub(source.len())
                .ok_or(PreparationError::ByteBound)?;
            files.push(Material::Formatted(Box::new(output)));
        }
        Ok(Self { publication, files })
    }

    /// The original sealed publication account and declared destinations.
    #[must_use]
    pub const fn publication(&self) -> &Publication<K> {
        &self.publication
    }

    /// Borrows the complete physical file set in its original publication order.
    pub fn files(&self) -> impl Iterator<Item = PreparedFile<'_>> {
        self.files.iter().map(|material| match material {
            Material::Unformatted {
                path,
                canonical,
                source,
            } => PreparedFile {
                path,
                canonical: *canonical,
                bytes: source.as_bytes(),
            },
            Material::Formatted(output) => PreparedFile {
                path: output.path(),
                canonical: output.canonical_digest(),
                bytes: output.process().stdout().bytes(),
            },
        })
    }

    /// Borrows actual formatter observations when that preparation route was selected.
    pub fn formatting(&self) -> impl Iterator<Item = &FormatOutput> {
        self.files.iter().filter_map(|material| match material {
            Material::Unformatted { .. } => None,
            Material::Formatted(output) => Some(output.as_ref()),
        })
    }
}

impl<'source> PreparedFile<'source> {
    /// The admitted relative destination.
    #[must_use]
    pub const fn path(&self) -> &'source PublicationPath {
        self.path
    }

    /// The exact physical bytes to stage, compare or install.
    #[must_use]
    pub const fn bytes(&self) -> &'source [u8] {
        self.bytes
    }

    /// The original canonical token commitment.
    #[must_use]
    pub const fn canonical_digest(&self) -> Identity<CanonicalPublicationBytes> {
        self.canonical
    }

    /// The commitment to these exact physical bytes.
    #[must_use]
    pub fn published_digest(&self) -> Identity<PublishedBytes> {
        published_digest(self.bytes)
    }
}

use crate::compiler::Kind;
use crate::compiler::identity::Identity;
use crate::native_publication::{
    CanonicalPublicationBytes, FormatObservationError, FormatOutput, Publication, PublicationPath,
};

#[path = "type_guard.rs"]
mod guard;

/// A complete bounded physical file set joined to its original sealed publication inventory.
#[derive(Debug)]
pub struct PreparedPublication<K: Kind> {
    publication: Publication<K>,
    files: Vec<Material>,
}

/// One admitted physical file with distinct canonical and published-byte commitments.
#[derive(Debug, Clone, Copy)]
pub struct PreparedFile<'source> {
    path: &'source PublicationPath,
    canonical: Identity<CanonicalPublicationBytes>,
    bytes: &'source [u8],
}

/// Why physical outputs did not establish the complete expected publication set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreparationError {
    /// Formatter outputs are missing, duplicated or outside the expected path roster.
    Inventory,
    /// A formatter output belongs to different original source or canonical material.
    Source,
    /// A finished formatter observation did not establish physical text.
    Formatter(FormatObservationError),
    /// Physical output exceeds the caller's complete byte allowance.
    ByteBound,
}

#[derive(Debug)]
enum Material {
    Unformatted {
        path: PublicationPath,
        canonical: Identity<CanonicalPublicationBytes>,
        source: String,
    },
    Formatted(Box<FormatOutput>),
}

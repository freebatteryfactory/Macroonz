use crate::compiler::stamp::PublishedStamp;
use crate::compiler::{Expansion, GeneratedTree, Kind, OwnerIdentity, RenderedUnit, Role};
use crate::harness::oracle::RelativeSourcePath;

#[path = "type_guard.rs"]
mod guard;

crate::compiler::subjects! {
    stem = "macroonz/native-publication";
    /// The canonical token bytes of one publication file.
    CanonicalPublicationBytes = "canonical-token-bytes",
    /// The exact physical bytes prepared for one publication file.
    PublishedBytes = "published-bytes",
}

/// One normal relative destination with portable publication segments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationPath(RelativeSourcePath);

/// Maximum file count and aggregate unformatted publication source bytes.
#[derive(Debug, Clone, Copy)]
pub struct PublicationLimits {
    /// Maximum number of definitions and landings together.
    pub files: usize,
    /// Maximum aggregate source projection including each file's terminal LF.
    pub bytes: usize,
}

/// One caller-declared physical destination for a logical publication address.
#[derive(Debug, Clone)]
pub struct PublicationBinding {
    /// The address already declared by the compiler request.
    pub address: OwnerIdentity,
    /// The relative generated-file destination.
    pub path: PublicationPath,
}

/// One caller-declared physical destination for a stamp's named landing.
#[derive(Debug, Clone)]
pub struct LandingBinding {
    /// The exact site name declared by the stamp.
    pub site: String,
    /// The relative invocation-file destination.
    pub path: PublicationPath,
}

/// A complete destination binding over a sealed expansion and its admitted stamps.
#[derive(Debug)]
pub struct Publication<K: Kind> {
    expansion: Expansion<K>,
    destinations: Vec<Destination>,
    limits: PublicationLimits,
}

/// A borrowed publication file and the sealed unit that authorizes its material.
#[derive(Debug, Clone, Copy)]
pub struct PublicationFile<'source, R: Role> {
    path: &'source PublicationPath,
    tokens: &'source GeneratedTree,
    unit: &'source RenderedUnit<R>,
    site: Option<&'source str>,
}

/// Why a declared publication set could not be informed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryError {
    /// A path is non-normal, reserved or outside the portable segment grammar.
    Path,
    /// Physical destinations alias or one file occupies another file's parent.
    PathCollision,
    /// Logical-address bindings are missing, duplicated or outside the sealed roster.
    Binding,
    /// A stamp does not agree with its sealed unit or repeats that unit's stamp.
    Stamp,
    /// Landing bindings do not match the stamp's complete named roster.
    Landing,
    /// The complete file set exceeds its count allowance.
    FileBound,
    /// The complete source projection exceeds its aggregate byte allowance.
    ByteBound,
}

#[derive(Debug)]
struct Destination {
    path: PublicationPath,
    stamp: Option<StampFiles>,
}

#[derive(Debug)]
struct StampFiles {
    stamp: PublishedStamp,
    paths: Vec<PublicationPath>,
}

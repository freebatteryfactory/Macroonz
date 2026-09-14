//! Owned historical surfaces with independently bounded canonical material.

use crate::descriptor::archive::ArchivedName;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::archive::{AddressClaim, ArchiveLimits, ArchiveRefusal};

#[path = "type_guard.rs"]
mod guard;
pub(crate) use guard::read_selection;
pub use guard::read_surface;

/// Historical surface, point and alternative claims without executable selection authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSelection {
    surface: AddressClaim,
    point: ArchivedName,
    alternative: AddressClaim,
}

/// The historical evaluation-surface envelope domain.
pub const SURFACE_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-evaluation-surface",
    IdentityProfileVersion::declared(1),
);

/// Independent envelope, field, point and per-point alternative ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceArchiveLimits {
    bytes: ArchiveLimits,
    points: usize,
    alternatives: usize,
}

/// An alternative's historical family and operation with its rederived identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedAlternative {
    identity: ContentAddress,
    family: String,
    operation: Vec<u8>,
}

/// A historical point whose alternatives remain data without executable membership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedMutationPoint {
    name: ArchivedName,
    owner_claim: ArchivedName,
    original: Vec<u8>,
    activation_site: ArchivedName,
    alternatives: Vec<ArchivedAlternative>,
}

/// A complete historical executable-subset roster without its missing discovery or policy preimages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedEvaluationSurface {
    encoded: Vec<u8>,
    address: ContentAddress,
    identity: ContentAddress,
    canonical: Vec<u8>,
    family: ArchivedName,
    policy: AddressClaim,
    points: Vec<ArchivedMutationPoint>,
}

/// Why a historical surface could not be retained or read.
#[must_use = "a refusal explains why historical surface data was not admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceArchiveRefusal {
    /// The envelope, framed fields, names or identity claims refused.
    Record(ArchiveRefusal),
    /// The point count exceeded its independent ceiling.
    TooManyPoints,
    /// A point's alternative count exceeded its independent ceiling.
    TooManyAlternatives,
    /// Points were not strictly ordered by namespace and stem.
    NonCanonicalPoints,
    /// A point had no alternatives or their identities were not strictly ordered.
    NonCanonicalAlternatives,
    /// An original or alternative operation was empty.
    EmptyOperation,
    /// An alternative repeated the original operation.
    AlternativeIsOriginal,
    /// The alternative's stated identity disagreed with its complete preimage.
    AlternativeIdentityMismatch,
    /// The surface's stated identity disagreed with its complete preimage.
    SurfaceIdentityMismatch,
}

//! Owned historical descriptor vocabulary.

#[path = "type_guard.rs"]
mod guard;

pub use guard::{read_candidate, retain_candidate};

/// An owned historical namespace and local spelling without a static-name mint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedName {
    namespace: String,
    stem: String,
}

/// Independent canonical-byte, name-component and per-label-roster ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateArchiveLimits {
    bytes: usize,
    field: usize,
    labels: usize,
}

/// The historical opening a candidate claimed to address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedSynthesis {
    /// A claimed survivor at this historical mutation point.
    Survivor(ArchivedName),
    /// A claimed gap in the independent proof.
    ProofGap,
}

/// An owned canonical candidate descriptor without live row or admission authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedCandidate {
    canonical: Vec<u8>,
    claim: ArchivedName,
    execution_suite: ArchivedName,
    roles: Vec<ArchivedName>,
    tags: Vec<ArchivedName>,
    subject: ArchivedName,
    check: ArchivedName,
    population: ArchivedName,
    synthesis: ArchivedSynthesis,
}

/// Why canonical material could not become a historical candidate descriptor.
#[must_use = "a refusal states why the historical candidate was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateArchiveRefusal {
    /// The complete canonical material exceeds its independent byte ceiling.
    BytesTooLarge,
    /// A name component exceeds its independent byte ceiling.
    FieldTooLarge,
    /// A role or tag roster exceeds its independent population ceiling.
    TooManyLabels,
    /// A declared member extends past the supplied material.
    Truncated,
    /// A declared count cannot be indexed on this platform.
    LengthOutsidePlatform {
        /// The unrepresentable count.
        declared: u64,
    },
    /// The offered row encoding has no reading here.
    UnsupportedFormat {
        /// The offered version.
        found: u32,
    },
    /// The origin is not the candidate arm.
    NotCandidate {
        /// The offered origin slot.
        found: u8,
    },
    /// The candidate synthesis discriminant has no reading here.
    InvalidSynthesis,
    /// A required name component is empty or not UTF-8.
    InvalidName,
    /// A label roster is not strictly ordered by namespace and stem.
    NonCanonicalLabels,
    /// Complete fields leave undeclared material.
    TrailingBytes,
}

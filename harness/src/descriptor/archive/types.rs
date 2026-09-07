//! Owned historical descriptor vocabulary.

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_binding;
pub(crate) use guard::read_revision;
pub use guard::{read_candidate, read_row, retain_candidate, retain_row};

/// Independent binding-byte, name/address-field and per-row-label ceilings.
pub type BindingArchiveLimits = RowArchiveLimits;

/// A historical executable revision claim without its original derivation material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchivedRevisionBinding {
    revision: [u8; 32],
    posture: crate::descriptor::RevisionPosture,
}

/// The producer standing retained beside a historical row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedProvenance {
    /// No producer standing was supplied.
    Unproduced,
    /// A producer and schema were claimed, without proof of their currency.
    Produced {
        /// The historical producer name.
        producer: ArchivedName,
        /// The historical schema address claim.
        schema: [u8; 32],
    },
}

/// A complete historical row and attachment standing with no callable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBinding {
    encoded: Vec<u8>,
    row: ArchivedRow,
    subject: ArchivedName,
    check: ArchivedName,
    subject_revision: ArchivedRevisionBinding,
    check_revision: ArchivedRevisionBinding,
    provenance: ArchivedProvenance,
}

/// Why a historical binding could not be retained or read.
#[must_use = "a refusal states why the historical binding was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingArchiveRefusal {
    /// The framing, names or independent resource bounds refused.
    Canonical(CandidateArchiveRefusal),
    /// The nested row refused its owning grammar.
    Row(RowArchiveRefusal),
    /// The computed encoding size cannot be represented.
    SizeOutsidePlatform,
    /// A revision or provenance slot has no reading.
    InvalidSlot,
    /// An address claim is not exactly thirty-two bytes.
    InvalidAddressWidth,
    /// The attachment subject disagrees with its row.
    SubjectMismatch,
    /// The attachment check disagrees with its row.
    CheckMismatch,
    /// A generated row omitted producer standing.
    GeneratedWithoutSchemaPin,
}

/// Independent canonical-byte, framed-field and per-label-roster ceilings for historical rows.
pub type RowArchiveLimits = CandidateArchiveLimits;

/// The historical provenance a canonical row recorded, without admission authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedOrigin {
    /// A hand-authored row.
    HandWritten,
    /// A producer's declared door and projection.
    Generated {
        /// The historical declaration door.
        door: ArchivedName,
        /// The historical projection.
        projection: ArchivedName,
    },
    /// An unadmitted synthesis opening.
    Candidate(ArchivedSynthesis),
    /// A claimed replay-bearing admission with historical address bytes.
    AdmittedReplay {
        /// The claimed proposal address, with no retained proposal preimage.
        proposal: [u8; 32],
        /// The recorded replay-bearing ground.
        ground: crate::descriptor::ReplayBearingGround,
        /// The recorded destination suite.
        destination: ArchivedName,
        /// The claimed capsule-entry address, with no retained entry preimage.
        replay: [u8; 32],
    },
    /// A claimed discharge admission, with no replay seat.
    AdmittedDischarge {
        /// The claimed proposal address, with no retained proposal preimage.
        proposal: [u8; 32],
        /// The recorded destination suite.
        destination: ArchivedName,
    },
}

/// An owned historical row retaining its exact canonical preimage and origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedRow {
    canonical: Vec<u8>,
    fields: ArchivedRowFields,
    origin: ArchivedOrigin,
}

/// The shared informed prefix of candidate and complete historical row readings.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ArchivedRowFields {
    claim: ArchivedName,
    execution_suite: ArchivedName,
    roles: Vec<ArchivedName>,
    tags: Vec<ArchivedName>,
    subject: ArchivedName,
    check: ArchivedName,
    population: ArchivedName,
}

/// Why canonical material could not become a complete historical row.
#[must_use = "a refusal states why the historical row was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowArchiveRefusal {
    /// The shared descriptor grammar or resource bounds refused.
    Canonical(CandidateArchiveRefusal),
    /// The origin slot has no descriptor reading.
    InvalidOrigin {
        /// The offered origin slot.
        found: u8,
    },
    /// A replay-bearing origin names another ground.
    InvalidReplayGround {
        /// The offered ground slot.
        found: u8,
    },
    /// A historical address field does not contain exactly thirty-two bytes.
    InvalidAddressWidth,
}

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

/// Why the shared canonical descriptor grammar or resource bounds refused.
#[must_use = "a refusal states why the historical candidate was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateArchiveRefusal {
    /// The complete canonical material exceeds its independent byte ceiling.
    BytesTooLarge,
    /// A framed field exceeds its independent byte ceiling.
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

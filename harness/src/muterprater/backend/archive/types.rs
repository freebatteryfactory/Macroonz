//! Historical backend custody vocabulary without current-source or execution authority.

use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::verdict_archive::{
    ArchivedMutation, ArchivedMutationRun, MutationArchiveRefusal, MutationRunArchiveLimits,
};
use crate::muterprater::{AnnouncedRoster, GrammarVersion, ReadingSource, WrappedBackend};
use crate::report::TargetBinding;
use crate::report::archive::{AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedForeignText};

#[path = "type_guard.rs"]
mod guard;
pub use guard::read_backend;
pub use guard::read_suite_pressure;

/// The complete historical backend manifest envelope domain.
pub const BACKEND_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-backend-manifest",
    IdentityProfileVersion::declared(1),
);

/// The historical compiled-suite pressure envelope domain.
pub const SUITE_PRESSURE_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-compiled-suite-pressure",
    IdentityProfileVersion::declared(1),
);

/// Independent outer-envelope and nested backend-record bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuitePressureArchiveLimits {
    bytes: ArchiveLimits,
    backend: BackendArchiveLimits,
}

/// Historical grammar qualification and its manifest's first complete backend-reported kill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSuitePressure {
    encoded: Vec<u8>,
    address: ContentAddress,
    checked_version: String,
    manifest: ArchivedBackendManifest,
    kill_ordinal: u64,
    kill: ArchivedMutation,
}

/// Why complete historical suite pressure could not be retained or read.
#[must_use = "a refusal explains why no historical suite pressure was admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuitePressureArchiveRefusal {
    /// The outer envelope, fields or framing refused.
    Record(ArchiveRefusal),
    /// The complete nested manifest or original material refused.
    Backend(BackendArchiveRefusal),
    /// The checked grammar version or profile contradicted the retained manifest.
    QualificationMismatch,
    /// The retained run had no kill or the selected ordinal was not its first kill.
    FirstKillMismatch,
}

/// Independent byte, report, command, source and unread-line ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendArchiveLimits {
    run: MutationRunArchiveLimits,
    arguments: usize,
    sources: usize,
    unparsed: usize,
}

/// A historical invocation's separate command tokens and target labels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBackendInvocation {
    backend: WrappedBackend,
    version: String,
    executable: String,
    arguments: Vec<String>,
    target: TargetBinding,
}

/// The historical adapter profile joined to its invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedAdapterProfile {
    backend: WrappedBackend,
    version: String,
    source: ReadingSource,
    grammar: GrammarVersion,
}

/// A historical source coordinate, revision claim and optionally joined original bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBackendSource {
    file: String,
    revision: AddressClaim,
    original: Option<Vec<u8>>,
}

/// An unread historical console line with its original ordinal and loss markers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedUnparsedLine {
    ordinal: u64,
    text: ArchivedForeignText,
}

/// An owned historical manifest with optional complete original material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBackendManifest {
    encoded: Vec<u8>,
    address: ContentAddress,
    invocation: ArchivedBackendInvocation,
    profile: ArchivedAdapterProfile,
    output: AddressClaim,
    sources: Vec<ArchivedBackendSource>,
    run: ArchivedMutationRun,
    announced: AnnouncedRoster,
    unparsed: Vec<ArchivedUnparsedLine>,
    original: Option<Vec<u8>>,
}

/// Why bounded historical backend retention refused.
#[must_use = "a refusal explains why no historical backend manifest was admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendArchiveRefusal {
    /// A common envelope or field could not be read.
    Record(ArchiveRefusal),
    /// The nested mutation run could not be read.
    Mutation(MutationArchiveRefusal),
    /// The command argument population exceeds its ceiling.
    TooManyArguments,
    /// The source population exceeds its ceiling.
    TooManySources,
    /// The unread-line population exceeds its ceiling.
    TooManyUnparsed,
    /// Source files repeat or are not in strict spelling order.
    InvalidSourceOrder,
    /// Unread-line ordinals repeat or are not increasing.
    InvalidUnparsedOrder,
    /// The invocation and adapter profile disagree or name unsupported grammar.
    ProfileMismatch,
    /// A mutation claims facts outside the backend's reading road.
    BackendRecordMismatch,
    /// A report names a source absent from the retained roster.
    SourceMissing(String),
    /// A retained or supplied source is outside the expected roster.
    SourceUnexpected(String),
    /// Supplied original material repeats a source file.
    DuplicateMaterialSource(String),
    /// Supplied original bytes do not match this source revision.
    SourceMaterialMismatch(String),
    /// Original console bytes do not match the saved output identity.
    OutputMaterialMismatch,
    /// Original console facts disagree with the retained reading.
    ConsoleReadingMismatch,
}

#[derive(Clone, Copy)]
pub(crate) struct OriginalMaterial<'material> {
    pub(crate) console: &'material str,
    pub(crate) sources: &'material [(&'material str, &'material [u8])],
}

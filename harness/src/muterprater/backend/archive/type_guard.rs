//! Historical read access and declared resource bounds.

use super::{
    ArchivedAdapterProfile, ArchivedBackendInvocation, ArchivedBackendManifest,
    ArchivedBackendSource, ArchivedUnparsedLine, BackendArchiveLimits,
};
use crate::identity::ContentAddress;
use crate::muterprater::verdict_archive::{ArchivedMutationRun, MutationRunArchiveLimits};
use crate::muterprater::{
    AnnouncedRoster, ClaimCeiling, GrammarVersion, ReadingSource, WrappedBackend,
};
use crate::report::TargetBinding;
use crate::report::archive::{AddressClaim, ArchiveLimits, ArchivedForeignText};

#[path = "read.rs"]
mod read;
#[path = "read_members.rs"]
mod members;
#[path = "read_material.rs"]
mod material;
pub use read::read_backend;
#[path = "guard_suite.rs"]
mod suite;
#[path = "read_suite.rs"]
mod suite_read;
pub use suite_read::read_suite_pressure;

impl BackendArchiveLimits {
    /// Declare independent byte and population ceilings.
    #[must_use]
    pub const fn declared(
        run: MutationRunArchiveLimits,
        arguments: usize,
        sources: usize,
        unparsed: usize,
    ) -> Self {
        Self {
            run,
            arguments,
            sources,
            unparsed,
        }
    }

    /// The common envelope and field ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.run.bytes()
    }
}

impl BackendArchiveLimits {
    /// The nested mutation-run ceilings.
    #[must_use]
    pub const fn run(self) -> MutationRunArchiveLimits {
        self.run
    }

    /// The argument population ceiling.
    #[must_use]
    pub const fn arguments(self) -> usize {
        self.arguments
    }

    /// The source population ceiling.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }

    /// The unread-line population ceiling.
    #[must_use]
    pub const fn unparsed(self) -> usize {
        self.unparsed
    }
}

impl ArchivedBackendInvocation {
    /// The historical backend label.
    #[must_use]
    pub const fn backend(&self) -> WrappedBackend {
        self.backend
    }

    /// The historical stated version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// The unflattened executable token.
    #[must_use]
    pub fn executable(&self) -> &str {
        &self.executable
    }

    /// The original argument token order.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    /// The recorded target and toolchain labels.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }
}

impl ArchivedAdapterProfile {
    /// The recorded backend label.
    #[must_use]
    pub const fn backend(&self) -> WrappedBackend {
        self.backend
    }

    /// The recorded backend version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// The recorded output channel.
    #[must_use]
    pub const fn source(&self) -> ReadingSource {
        self.source
    }

    /// The recorded line grammar version.
    #[must_use]
    pub const fn grammar(&self) -> GrammarVersion {
        self.grammar
    }

    /// The existing source owner's verdict ceiling.
    #[must_use]
    pub fn ceiling(&self) -> ClaimCeiling {
        ClaimCeiling::from(self.source)
    }
}

impl ArchivedBackendSource {
    /// The exact historical source spelling.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The saved source revision claim.
    #[must_use]
    pub const fn revision(&self) -> AddressClaim {
        self.revision
    }

    /// Original bytes when the complete material join was retained.
    #[must_use]
    pub fn original(&self) -> Option<&[u8]> {
        self.original.as_deref()
    }
}

impl ArchivedUnparsedLine {
    /// The original zero-based console line position.
    #[must_use]
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }

    /// The retained line bytes and loss markers.
    #[must_use]
    pub const fn text(&self) -> &ArchivedForeignText {
        &self.text
    }
}

impl ArchivedBackendManifest {
    /// The complete bounded historical envelope.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The envelope integrity address.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The retained historical invocation.
    #[must_use]
    pub const fn invocation(&self) -> &ArchivedBackendInvocation {
        &self.invocation
    }

    /// The historical profile joined to that invocation.
    #[must_use]
    pub const fn profile(&self) -> &ArchivedAdapterProfile {
        &self.profile
    }

    /// The original console content claim.
    #[must_use]
    pub const fn output(&self) -> AddressClaim {
        self.output
    }

    /// The exact source roster in spelling order.
    #[must_use]
    pub fn sources(&self) -> &[ArchivedBackendSource] {
        &self.sources
    }

    /// The complete ordered historical mutation run.
    #[must_use]
    pub const fn run(&self) -> &ArchivedMutationRun {
        &self.run
    }

    /// The backend's announcement, independent of parsed population.
    #[must_use]
    pub const fn announced(&self) -> AnnouncedRoster {
        self.announced
    }

    /// Every retained unread line in output order.
    #[must_use]
    pub fn unparsed(&self) -> &[ArchivedUnparsedLine] {
        &self.unparsed
    }

    /// Original console bytes when all original source bytes are also retained.
    #[must_use]
    pub fn original_console(&self) -> Option<&[u8]> {
        self.original.as_deref()
    }
}

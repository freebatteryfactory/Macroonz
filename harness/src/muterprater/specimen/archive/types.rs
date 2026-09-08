//! Owned historical projection data and its independent resource bounds.

use crate::descriptor::archive::ArchivedName;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::discovery_archive::ArchivedSelection;
use crate::muterprater::interpretation_archive::{
    ArchivedEvaluationPair, ArchivedParity, ParityArchiveLimits, ParityArchiveRefusal,
};
use crate::muterprater::verdict_archive::{ArchivedMutation, MutationArchiveRefusal};
use crate::muterprater::{ArtifactContent, ArtifactContentId};
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, ArchivedExecution, ArchivedTrial};

#[path = "type_guard.rs"]
mod guard;
pub use guard::read_projection;

/// The historical compiled-projection-pressure envelope domain.
pub const PROJECTION_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-compiled-projection-pressure",
    IdentityProfileVersion::declared(1),
);

/// Independent outer, nested parity, trial, mutation and source-buffer ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionArchiveLimits {
    bytes: ArchiveLimits,
    parity: ParityArchiveLimits,
    trial: ArchiveLimits,
    mutation: ArchiveLimits,
    source: usize,
}

/// A selected artifact's historical pair, selection, execution and witness check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSpecimenStanding {
    artifact: ArtifactContentId,
    pair: ArchivedEvaluationPair,
    selection: ArchivedSelection,
    execution: ArchivedExecution,
    check: ArchivedName,
}

/// Complete historical compiled projection pressure without live qualification authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedProjectionPressure {
    encoded: Vec<u8>,
    address: ContentAddress,
    parity: ArchivedParity,
    baseline: ArtifactContent,
    selected: ArtifactContent,
    standing: ArchivedSpecimenStanding,
    baseline_report: ArchivedTrial,
    selected_report: ArchivedTrial,
    mutation: ArchivedMutation,
}

/// Why a complete historical projection could not be retained or read.
#[must_use = "a refusal explains why historical projection pressure was not admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionArchiveRefusal {
    /// A common field or envelope refused.
    Record(ArchiveRefusal),
    /// The complete nested parity refused.
    Parity(ParityArchiveRefusal),
    /// The complete nested mutation refused.
    Mutation(MutationArchiveRefusal),
    /// An exact source buffer exceeded its independent ceiling.
    SourceTooLarge,
    /// A stated artifact identity disagreed with its exact bytes.
    ArtifactIdentityMismatch,
    /// Unchanged and selected source have the same content identity.
    ArtifactDidNotChange,
    /// The retained parity disposition was not qualified.
    ParityNotQualified,
    /// Selected standing disagreed with its content, parity or selection surface.
    StandingMismatch,
    /// The compiled reports disagreed with their execution, sites, posture or required outcomes.
    ReportJoinMismatch,
    /// The compiled mutation disagreed with the selection, witness, finding or required axes.
    MutationJoinMismatch,
}

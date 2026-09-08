//! Projection limits and owned historical projections.

use super::{ArchivedProjectionPressure, ArchivedSpecimenStanding, ProjectionArchiveLimits};
use crate::descriptor::archive::ArchivedName;
use crate::identity::ContentAddress;
use crate::muterprater::discovery_archive::ArchivedSelection;
use crate::muterprater::interpretation_archive::{
    ArchivedEvaluationPair, ArchivedParity, ParityArchiveLimits,
};
use crate::muterprater::verdict_archive::ArchivedMutation;
use crate::muterprater::{ArtifactContent, ArtifactContentId};
use crate::report::archive::{ArchiveLimits, ArchivedExecution, ArchivedTrial};

#[path = "read.rs"]
mod read;
#[path = "read_joins.rs"]
mod joins;
pub use read::read_projection;

impl ProjectionArchiveLimits {
    /// Declare independent outer, parity, compiled-trial, mutation and source bounds.
    #[must_use]
    pub const fn declared(
        bytes: ArchiveLimits,
        parity: ParityArchiveLimits,
        trial: ArchiveLimits,
        mutation: ArchiveLimits,
        source: usize,
    ) -> Self {
        Self {
            bytes,
            parity,
            trial,
            mutation,
            source,
        }
    }
    /// The outer envelope and framed-field bounds.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }
    /// The nested parity bounds.
    #[must_use]
    pub const fn parity(self) -> ParityArchiveLimits {
        self.parity
    }
    /// The bounds for each compiled trial.
    #[must_use]
    pub const fn trial(self) -> ArchiveLimits {
        self.trial
    }
    /// The nested mutation bounds.
    #[must_use]
    pub const fn mutation(self) -> ArchiveLimits {
        self.mutation
    }
    /// The independent ceiling for each exact source buffer.
    #[must_use]
    pub const fn source(self) -> usize {
        self.source
    }
}

impl ArchivedSpecimenStanding {
    /// The selected bytes-only content identity.
    #[must_use]
    pub const fn artifact(&self) -> ArtifactContentId {
        self.artifact
    }
    /// The historical pair shared with qualified parity.
    #[must_use]
    pub const fn pair(&self) -> &ArchivedEvaluationPair {
        &self.pair
    }
    /// The historical selection without current activation authority.
    #[must_use]
    pub const fn selection(&self) -> &ArchivedSelection {
        &self.selection
    }
    /// The historical execution shared with the compiled reports.
    #[must_use]
    pub const fn execution(&self) -> &ArchivedExecution {
        &self.execution
    }
    /// The historical witness check.
    #[must_use]
    pub const fn check(&self) -> &ArchivedName {
        &self.check
    }
}

impl ArchivedProjectionPressure {
    /// The integrity address of the historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }
    /// The complete historically qualified parity.
    #[must_use]
    pub const fn parity(&self) -> &ArchivedParity {
        &self.parity
    }
    /// The exact unchanged source and its bytes-only identity.
    #[must_use]
    pub const fn baseline_content(&self) -> &ArtifactContent {
        &self.baseline
    }
    /// The exact selected source and its bytes-only identity.
    #[must_use]
    pub const fn selected_content(&self) -> &ArtifactContent {
        &self.selected
    }
    /// The complete historical selected standing.
    #[must_use]
    pub const fn standing(&self) -> &ArchivedSpecimenStanding {
        &self.standing
    }
    /// The complete passing compiled baseline report.
    #[must_use]
    pub const fn baseline_report(&self) -> &ArchivedTrial {
        &self.baseline_report
    }
    /// The complete rejecting compiled selected report.
    #[must_use]
    pub const fn selected_report(&self) -> &ArchivedTrial {
        &self.selected_report
    }
    /// The complete historical compiled-projection kill.
    #[must_use]
    pub const fn mutation(&self) -> &ArchivedMutation {
        &self.mutation
    }
    /// The complete envelope bytes for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

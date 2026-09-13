//! Bounds and projections of complete historical mutation assessments.

use super::super::ParityArchiveLimits;
use super::super::{
    ArchivedAssessment, ArchivedEvaluationPair, ArchivedSubstrate, ArchivedValue,
    AssessmentArchiveLimits,
};
use crate::descriptor::archive::ArchivedBinding;
use crate::descriptor::archive::BindingArchiveLimits;
use crate::identity::ContentAddress;
use crate::muterprater::ArtifactContent;
use crate::muterprater::discovery_archive::{
    ArchivedEvaluationSurface, ArchivedSelection, SurfaceArchiveLimits,
};
use crate::muterprater::specimen_archive::ProjectionArchiveLimits;
use crate::muterprater::verdict_archive::ArchivedMutation;
use crate::properties::Agreement;
use crate::report::archive::ArchiveLimits;
use crate::report::archive::ArchivedTrial;

impl AssessmentArchiveLimits {
    /// Declare independent complete bytes, surface, witness, report, source, value and substrate ceilings.
    #[must_use]
    pub const fn declared(
        bytes: ArchiveLimits,
        surface: SurfaceArchiveLimits,
        binding: BindingArchiveLimits,
        reports: ArchiveLimits,
        source: usize,
        value: usize,
        substrates: usize,
    ) -> Self {
        Self {
            bytes,
            surface,
            binding,
            reports,
            source,
            value,
            substrates,
        }
    }
    /// The complete surface bounds.
    #[must_use]
    pub const fn surface(self) -> SurfaceArchiveLimits {
        self.surface
    }
    /// The complete envelope and every outer framed field ceiling.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }
    /// The complete historical witness bounds.
    #[must_use]
    pub const fn binding(self) -> BindingArchiveLimits {
        self.binding
    }
    /// The bounds for each trial and mutation report.
    #[must_use]
    pub const fn reports(self) -> ArchiveLimits {
        self.reports
    }
    /// The independent source-buffer ceiling.
    #[must_use]
    pub const fn source(self) -> usize {
        self.source
    }
    /// The independent encoded-value ceiling.
    #[must_use]
    pub const fn value(self) -> usize {
        self.value
    }
    /// The independent substrate population ceiling.
    #[must_use]
    pub const fn substrates(self) -> usize {
        self.substrates
    }

    pub(in crate::muterprater::interpretation::archive) const fn projection(
        self,
    ) -> ProjectionArchiveLimits {
        ProjectionArchiveLimits::declared(
            self.bytes,
            ParityArchiveLimits::declared(
                self.bytes,
                self.binding,
                self.reports,
                self.value,
                self.substrates,
            ),
            self.reports,
            self.reports,
            self.source,
        )
    }
}

impl ArchivedAssessment {
    /// The exact complete historical envelope.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
    /// The historical envelope integrity address.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }
    /// The complete historical surface.
    #[must_use]
    pub const fn surface(&self) -> &ArchivedEvaluationSurface {
        &self.surface
    }
    /// The historical pair and callable revision claims.
    #[must_use]
    pub const fn pair(&self) -> &ArchivedEvaluationPair {
        &self.pair
    }
    /// The exact historical selected membership.
    #[must_use]
    pub const fn selection(&self) -> &ArchivedSelection {
        &self.selection
    }
    /// The exact unchanged compiled source bytes.
    #[must_use]
    pub const fn baseline_content(&self) -> &ArtifactContent {
        &self.baseline_content
    }
    /// The exact selected compiled source bytes.
    #[must_use]
    pub const fn selected_content(&self) -> &ArtifactContent {
        &self.selected_content
    }
    /// The complete historical witness binding.
    #[must_use]
    pub const fn witness(&self) -> &ArchivedBinding {
        &self.witness
    }
    /// The input encoding made at retention time.
    #[must_use]
    pub const fn input(&self) -> &ArchivedValue {
        &self.input
    }
    /// The production, unchanged evaluation, unchanged compiled, selected compiled and selected evaluation encodings.
    #[must_use]
    pub const fn meanings(&self) -> &[ArchivedValue; 5] {
        &self.meanings
    }
    /// The actual witness reports in the same role order as the retained meanings.
    #[must_use]
    pub const fn reports(&self) -> &[ArchivedTrial; 5] {
        &self.reports
    }
    /// The explicitly declared shared foundations of the historical road comparisons.
    #[must_use]
    pub const fn substrate(&self) -> &ArchivedSubstrate {
        &self.substrate
    }
    /// The recorded unchanged-versus-selected comparison under caller-owned semantics.
    #[must_use]
    pub const fn difference(&self) -> Agreement {
        self.difference
    }
    /// The internally consistent historical mutation report without live execution authority.
    #[must_use]
    pub const fn mutation(&self) -> &ArchivedMutation {
        &self.mutation
    }
}

//! Complete historical interpreted bounds and read-only projections.

use super::super::{
    ArchivedInterpretedEvidence, ArchivedInterpretedTrust, ArchivedValue, InterpretedArchiveLimits,
};
use crate::identity::ContentAddress;
use crate::muterprater::backend_archive::{ArchivedSuitePressure, SuitePressureArchiveLimits};
use crate::muterprater::discovery_archive::{ArchivedEvaluationSurface, SurfaceArchiveLimits};
use crate::muterprater::specimen_archive::{ArchivedProjectionPressure, ProjectionArchiveLimits};
use crate::muterprater::verdict_archive::ArchivedMutation;
use crate::report::archive::{ArchiveLimits, ArchivedTrial};

impl InterpretedArchiveLimits {
    /// Declare independent outer, surface, suite, projection, active-report and active-value bounds.
    #[must_use]
    pub const fn declared(
        bytes: ArchiveLimits,
        surface: SurfaceArchiveLimits,
        suite: SuitePressureArchiveLimits,
        projection: ProjectionArchiveLimits,
        trial: ArchiveLimits,
        mutation: ArchiveLimits,
        active: usize,
    ) -> Self {
        Self {
            bytes,
            surface,
            suite,
            projection,
            trial,
            mutation,
            active,
        }
    }
    /// The outer envelope and framed-field limits.
    #[must_use]
    pub const fn bytes(&self) -> ArchiveLimits {
        self.bytes
    }
    /// The complete historical surface limits.
    #[must_use]
    pub const fn surface(&self) -> SurfaceArchiveLimits {
        self.surface
    }
    /// The complete generic suite-pressure limits.
    #[must_use]
    pub const fn suite(&self) -> SuitePressureArchiveLimits {
        self.suite
    }
    /// The complete projection-pressure limits.
    #[must_use]
    pub const fn projection(&self) -> ProjectionArchiveLimits {
        self.projection
    }
    /// The active trial-record limits.
    #[must_use]
    pub const fn trial(&self) -> ArchiveLimits {
        self.trial
    }
    /// The active mutation-record limits.
    #[must_use]
    pub const fn mutation(&self) -> ArchiveLimits {
        self.mutation
    }
    /// The independent byte ceiling for the encoded active meaning.
    #[must_use]
    pub const fn active(&self) -> usize {
        self.active
    }
}

impl ArchivedInterpretedTrust {
    /// The complete historical executable-subset roster.
    #[must_use]
    pub const fn surface(&self) -> &ArchivedEvaluationSurface {
        &self.surface
    }
    /// The complete generic suite pressure at its separate backend context.
    #[must_use]
    pub const fn suite(&self) -> &ArchivedSuitePressure {
        &self.suite
    }
    /// The complete exact selected projection and its parity.
    #[must_use]
    pub const fn projection(&self) -> &ArchivedProjectionPressure {
        &self.projection
    }
}

impl ArchivedInterpretedEvidence {
    /// The complete historical trust inputs.
    #[must_use]
    pub const fn trust(&self) -> &ArchivedInterpretedTrust {
        &self.trust
    }
    /// The active meaning encoding under the shared parity-meaning convention.
    #[must_use]
    pub const fn meaning(&self) -> &ArchivedValue {
        &self.meaning
    }
    /// The complete active trial report.
    #[must_use]
    pub const fn report(&self) -> &ArchivedTrial {
        &self.report
    }
    /// The complete active mutation report.
    #[must_use]
    pub const fn mutation(&self) -> &ArchivedMutation {
        &self.mutation
    }
    /// The integrity address of the complete envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }
    /// The exact complete bytes for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

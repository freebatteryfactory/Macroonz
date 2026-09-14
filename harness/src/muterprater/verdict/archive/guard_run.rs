//! Read-only historical mutation-run projections and independent limits.

use super::super::{ArchivedMutation, ArchivedMutationRun, MutationRunArchiveLimits};
use crate::identity::ContentAddress;
use crate::muterprater::{BaselineAxis, MutationCensus};
use crate::report::archive::ArchiveLimits;

impl MutationRunArchiveLimits {
    /// The independent envelope/field bytes and maximum report population.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, reports: usize) -> Self {
        Self { bytes, reports }
    }

    /// The byte ceilings applied to this run and each nested mutation envelope.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The maximum number of complete historical reports.
    #[must_use]
    pub const fn reports(self) -> usize {
        self.reports
    }
}

impl ArchivedMutationRun {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The integrity address of this historical run.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The historical qualified-baseline claim, without a current qualification mint.
    #[must_use]
    pub const fn baseline(&self) -> BaselineAxis {
        self.baseline
    }

    /// Every complete historical mutation, preserving original order and duplicates.
    #[must_use]
    pub fn reports(&self) -> &[ArchivedMutation] {
        &self.reports
    }

    /// Saturating accounting derived from the retained historical outcomes.
    #[must_use]
    pub const fn census(&self) -> MutationCensus {
        self.census
    }
}

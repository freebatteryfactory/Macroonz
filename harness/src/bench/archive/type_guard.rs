//! Historical construction and read-only projections.

use super::{
    ArchivedBenchOutcome, ArchivedBenchReading, ArchivedBenchReport, ArchivedBenchRow,
    ArchivedSecondaryObservation, ArchivedWorkCause, ArchivedWorkConclusion, ArchivedWorkCount,
    ArchivedWorkCurve, ArchivedWorkGap, ArchivedWorkJudgment, ArchivedWorkPoint,
    BenchArchiveLimits,
};
use crate::bench::{BenchMeasurement, BenchStage};
use crate::clock::ClockAttribution;
use crate::descriptor::archive::{ArchivedName, ArchivedProvenance};
use crate::identity::ContentAddress;
use crate::report::TargetBinding;
use crate::report::archive::{ArchiveLimits, ArchivedMeasurement, ArchivedTrial};

#[path = "read.rs"]
mod read;
#[path = "read_row.rs"]
mod row;
#[path = "read_work.rs"]
mod work;

pub use read::read_report;

impl BenchArchiveLimits {
    /// The independent byte and population ceilings for one complete report.
    #[must_use]
    pub const fn declared(
        bytes: ArchiveLimits,
        rows: usize,
        axis: usize,
        observations: usize,
        measurements: usize,
    ) -> Self {
        Self {
            bytes,
            rows,
            axis,
            observations,
            measurements,
        }
    }
    /// The complete-envelope and field byte ceilings.
    #[must_use]
    pub const fn bytes(&self) -> ArchiveLimits {
        self.bytes
    }
    /// The maximum report row population.
    #[must_use]
    pub const fn rows(&self) -> usize {
        self.rows
    }
    /// The maximum input-size population per row.
    #[must_use]
    pub const fn axis(&self) -> usize {
        self.axis
    }
    /// The maximum observation population per point.
    #[must_use]
    pub const fn observations(&self) -> usize {
        self.observations
    }
    /// The maximum secondary measurement population per row.
    #[must_use]
    pub const fn measurements(&self) -> usize {
        self.measurements
    }
}

impl ArchivedBenchRow {
    /// The exact canonical row preimage.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    /// The row address rederived from its retained preimage.
    #[must_use]
    pub const fn key(&self) -> ContentAddress {
        self.key
    }

    /// The historical workload name.
    #[must_use]
    pub const fn workload(&self) -> &ArchivedName {
        &self.workload
    }

    /// The historical preflight reference.
    #[must_use]
    pub const fn preflight(&self) -> &ArchivedName {
        &self.preflight
    }

    /// The historical control reference.
    #[must_use]
    pub const fn planted_worse(&self) -> &ArchivedName {
        &self.planted_worse
    }

    /// The historical complexity claim.
    #[must_use]
    pub const fn complexity(&self) -> &ArchivedName {
        &self.complexity
    }

    /// The admitted declaration data, without a callable.
    #[must_use]
    pub const fn measurement(&self) -> &BenchMeasurement {
        &self.measurement
    }
}

impl ArchivedWorkCause {
    /// The exact owner-written family text.
    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// The exact owner-written local text.
    #[must_use]
    pub fn local(&self) -> &str {
        &self.local
    }
}

impl ArchivedWorkJudgment {
    /// The historical measured-work conclusion.
    #[must_use]
    pub const fn measured(&self) -> &ArchivedWorkConclusion {
        &self.measured
    }

    /// The historical control conclusion.
    #[must_use]
    pub const fn planted_worse(&self) -> &ArchivedWorkConclusion {
        &self.planted_worse
    }

    /// The historical gap reading.
    #[must_use]
    pub const fn gap(&self) -> &ArchivedWorkGap {
        &self.gap
    }
}

impl ArchivedWorkCount {
    /// The historical observation name.
    #[must_use]
    pub const fn observation(&self) -> &ArchivedName {
        &self.observation
    }

    /// The exact retained count.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }
}

impl ArchivedWorkPoint {
    /// The point's retained input size.
    #[must_use]
    pub const fn input_size(&self) -> u64 {
        self.input_size
    }

    /// The counts in authored observation order.
    #[must_use]
    pub fn counts(&self) -> &[ArchivedWorkCount] {
        &self.counts
    }
}

impl ArchivedWorkCurve {
    /// The complete curve in authored axis order.
    #[must_use]
    pub fn points(&self) -> &[ArchivedWorkPoint] {
        &self.points
    }
}

impl ArchivedSecondaryObservation {
    /// The complete secondary work curve.
    #[must_use]
    pub const fn work(&self) -> &ArchivedWorkCurve {
        &self.work
    }

    /// The historical judgment over the secondary work.
    #[must_use]
    pub const fn judgment(&self) -> &ArchivedWorkJudgment {
        &self.judgment
    }

    /// The measurements in axis order, then sample order.
    #[must_use]
    pub fn measurements(&self) -> &[ArchivedMeasurement] {
        &self.measurements
    }

    /// The caller-declared secondary source classification.
    #[must_use]
    pub const fn clock_attribution(&self) -> ClockAttribution {
        self.clock_attribution
    }
}

impl ArchivedBenchReading {
    /// The complete historical row declaration.
    #[must_use]
    pub const fn row(&self) -> &ArchivedBenchRow {
        &self.row
    }

    /// The target and toolchain retained for this reading.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The complete historical preflight report.
    #[must_use]
    pub const fn preflight(&self) -> &ArchivedTrial {
        &self.preflight
    }

    /// Every retained member of the reached stage.
    #[must_use]
    pub const fn outcome(&self) -> &ArchivedBenchOutcome {
        &self.outcome
    }
}

impl ArchivedBenchReport {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The integrity address of this historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The historical table name.
    #[must_use]
    pub const fn table(&self) -> &ArchivedName {
        &self.table
    }

    /// The retained producer and schema standing.
    #[must_use]
    pub const fn provenance(&self) -> &ArchivedProvenance {
        &self.provenance
    }

    /// One reading per authored row in its original order.
    #[must_use]
    pub fn readings(&self) -> &[ArchivedBenchReading] {
        &self.readings
    }

    /// The complete retained row population.
    #[must_use]
    pub fn denominator(&self) -> usize {
        self.readings.len()
    }
}

impl ArchivedBenchOutcome {
    /// The historical stage, without granting current qualification.
    #[must_use]
    pub const fn stage(&self) -> BenchStage {
        match self {
            Self::PreflightRefused => BenchStage::PreflightRefused,
            Self::PlantedWorseNotDistinguished { .. } => BenchStage::PlantedWorseNotDistinguished,
            Self::PrimaryWorkRefused { .. } => BenchStage::PrimaryWorkRefused,
            Self::Qualified { .. } => BenchStage::Qualified,
        }
    }
}

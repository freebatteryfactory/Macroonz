//! Complete historical benchmark data without executable bindings or current qualification.

use crate::bench::BenchMeasurement;
use crate::clock::ClockAttribution;
use crate::descriptor::archive::{ArchivedName, ArchivedProvenance, BindingArchiveRefusal};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::TargetBinding;
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, ArchivedMeasurement, ArchivedTrial};

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_report;

/// The envelope domain for historical complete benchmark reports.
pub const BENCH_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-benchmark-report",
    IdentityProfileVersion::declared(1),
);

/// Independent byte, row, axis, observation and secondary-sample ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchArchiveLimits {
    bytes: ArchiveLimits,
    rows: usize,
    axis: usize,
    observations: usize,
    measurements: usize,
}

/// Why a complete historical benchmark account was not admitted.
#[must_use = "a refusal states why no historical benchmark report was admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchArchiveRefusal {
    /// Shared report framing or field bounds refused.
    Canonical(ArchiveRefusal),
    /// The canonical row writer refused its declaration.
    Declaration(crate::bench::BenchRowRefusal),
    /// The declared axis refused its owning invariant.
    Axis(crate::bench::InputSizeAxisRefusal),
    /// The budgets refused their owning invariant.
    Budgets(crate::bench::DeclaredBudgetsRefusal),
    /// A present formula refused its owning invariant.
    Formula(crate::bench::WorkFormulaRefusal),
    /// Historical producer standing refused its owning grammar.
    Provenance(BindingArchiveRefusal),
    /// The report has no rows.
    EmptyReport,
    /// The report exceeds its row ceiling.
    TooManyRows,
    /// A row exceeds its input-axis ceiling.
    TooManySizes,
    /// A curve exceeds its per-point observation ceiling.
    TooManyObservations,
    /// A secondary pass exceeds its sample ceiling.
    TooManyMeasurements,
    /// Two readings repeat one canonical row identity.
    DuplicateRow,
    /// Work points or observation rosters disagree with their declared population.
    WorkPopulationMismatch,
    /// A recorded stage disagrees with its retained preflight or judgment.
    StageMismatch,
    /// A row or preflight names a different target or toolchain.
    TargetMismatch,
}

/// A complete historical row declaration with an address rederived from its canonical bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBenchRow {
    canonical: Vec<u8>,
    key: ContentAddress,
    workload: ArchivedName,
    preflight: ArchivedName,
    planted_worse: ArchivedName,
    complexity: ArchivedName,
    measurement: BenchMeasurement,
}

/// Exact owner-written cause text, including lawful empty spellings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedWorkCause {
    family: String,
    local: String,
}

/// A historical conclusion about one work curve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedWorkConclusion {
    /// The source claimed the curve satisfied its owner.
    Satisfied,
    /// The source retained this owner-written cause.
    Refused(ArchivedWorkCause),
}

/// A historical reading of the declared work gap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedWorkGap {
    /// The source claimed the gap distinguished the control.
    Distinguished,
    /// The source retained its cause for an undistinguished gap.
    NotDistinguished(ArchivedWorkCause),
}

/// The three historical work judgments without a live judge or qualification mint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedWorkJudgment {
    measured: ArchivedWorkConclusion,
    planted_worse: ArchivedWorkConclusion,
    gap: ArchivedWorkGap,
}

/// One observation's exact historical count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedWorkCount {
    observation: ArchivedName,
    count: u64,
}

/// One input size and its ordered historical observation counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedWorkPoint {
    input_size: u64,
    counts: Vec<ArchivedWorkCount>,
}

/// One historical curve preserving authored axis and observation order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedWorkCurve {
    points: Vec<ArchivedWorkPoint>,
}

/// The retained secondary pass with its own work, judgment and ordered measurements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSecondaryObservation {
    work: ArchivedWorkCurve,
    judgment: ArchivedWorkJudgment,
    measurements: Vec<ArchivedMeasurement>,
    clock_attribution: ClockAttribution,
}

/// All retained evidence for the stage one historical benchmark reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedBenchOutcome {
    /// The preflight refused before work began.
    PreflightRefused,
    /// The source failed to establish an active control.
    PlantedWorseNotDistinguished {
        /// The measured curve.
        measured: ArchivedWorkCurve,
        /// The deliberately worse curve.
        planted_worse: ArchivedWorkCurve,
        /// The complete historical judgment.
        judgment: ArchivedWorkJudgment,
    },
    /// The control stood and measured work refused.
    PrimaryWorkRefused {
        /// The measured curve.
        measured: ArchivedWorkCurve,
        /// The deliberately worse curve.
        planted_worse: ArchivedWorkCurve,
        /// The complete historical judgment.
        judgment: ArchivedWorkJudgment,
    },
    /// The retained account claims all benchmark stages held.
    Qualified {
        /// The measured primary curve.
        measured: ArchivedWorkCurve,
        /// The deliberately worse curve.
        planted_worse: ArchivedWorkCurve,
        /// The complete primary judgment.
        judgment: ArchivedWorkJudgment,
        /// The complete secondary account.
        secondary: ArchivedSecondaryObservation,
    },
}

/// One complete historical benchmark reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBenchReading {
    row: ArchivedBenchRow,
    target: TargetBinding,
    preflight: ArchivedTrial,
    outcome: ArchivedBenchOutcome,
}

/// An owned complete benchmark report with historical custody only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedBenchReport {
    encoded: Vec<u8>,
    address: ContentAddress,
    table: ArchivedName,
    provenance: ArchivedProvenance,
    readings: Vec<ArchivedBenchReading>,
}

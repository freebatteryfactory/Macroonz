#![doc = include_str!("README.md")]

mod encode;
mod size;
mod types;

pub use encode::retain_report;
pub use types::{
    ArchivedBenchOutcome, ArchivedBenchReading, ArchivedBenchReport, ArchivedBenchRow,
    ArchivedSecondaryObservation, ArchivedWorkCause, ArchivedWorkConclusion, ArchivedWorkCount,
    ArchivedWorkCurve, ArchivedWorkGap, ArchivedWorkJudgment, ArchivedWorkPoint, BENCH_ARCHIVE_TAG,
    BenchArchiveLimits, BenchArchiveRefusal, read_report,
};

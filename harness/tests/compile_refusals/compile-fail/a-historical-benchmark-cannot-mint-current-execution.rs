//! Retained benchmark data supplies no conversion into current execution owners.

use macroonz_harness::bench::{BenchReport, BenchRow, SecondaryObservation, WorkJudgment};
use macroonz_harness::bench::archive::{
    ArchivedBenchReport, ArchivedBenchRow, ArchivedSecondaryObservation, ArchivedWorkJudgment,
};

fn report(value: ArchivedBenchReport) -> BenchReport {
    value.into()
}

fn row(value: ArchivedBenchRow) -> BenchRow {
    value.into()
}

fn judgment(value: ArchivedWorkJudgment) -> WorkJudgment {
    value.into()
}

fn secondary(value: ArchivedSecondaryObservation) -> SecondaryObservation {
    value.into()
}

fn main() {
    let _ = (report, row, judgment, secondary);
}

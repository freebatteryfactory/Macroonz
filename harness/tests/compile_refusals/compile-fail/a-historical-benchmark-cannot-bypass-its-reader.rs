//! Outside callers cannot bypass historical report, row or judgment admission.

use macroonz_harness::bench::archive::{
    ArchivedBenchReport, ArchivedBenchRow, ArchivedWorkJudgment,
};

fn forge(report: ArchivedBenchReport, row: ArchivedBenchRow, judgment: ArchivedWorkJudgment) {
    let _ = ArchivedBenchReport { ..report };
    let _ = ArchivedBenchRow { ..row };
    let _ = ArchivedWorkJudgment { ..judgment };
}

fn main() {
    let _ = forge;
}

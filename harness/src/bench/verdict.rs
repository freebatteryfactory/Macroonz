//! Folding a finished report down to one answer.

use super::types::{BenchReport, BenchStage, BenchVerdictRefusal};

/// Read a finished report for the first row that did not qualify.
///
/// # Errors
///
/// Refuses the first reading outside [`BenchStage::Qualified`], in authored table order.
pub fn bench_verdict(report: &BenchReport) -> Result<(), BenchVerdictRefusal> {
    for reading in report.readings() {
        let stage = reading.outcome().stage();
        if stage != BenchStage::Qualified {
            return Err(BenchVerdictRefusal::row_not_qualified(
                reading.row().key(),
                stage,
            ));
        }
    }
    Ok(())
}

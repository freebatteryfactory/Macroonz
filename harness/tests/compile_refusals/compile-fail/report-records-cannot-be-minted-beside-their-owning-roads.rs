//! Report records and readings are reached through their owning operations, never minted beside them.
//!
//! The types remain public so callers can retain and inspect evidence.
//! Their mints remain private so a caller cannot author a report, replay capsule, comparison, or coverage reading without establishing the facts each owning road joins.

use macroonz_harness::report::{ClaimCoverage, ReplayCapsule, ReportDiff, RunReport, TrialReport};

fn main() {
    let _capsule_mint = ReplayCapsule::captured;
    let _trial_report_mint = TrialReport::recorded;
    let _run_report_mint = RunReport::recorded;
    let _report_diff_mint = ReportDiff::stated;
    let _claim_coverage_mint = ClaimCoverage::read;
}

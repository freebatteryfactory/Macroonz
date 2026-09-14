//! Historical census claims cannot become a live run for coverage or comparison.

use macroonz_harness::report::RunReport;
use macroonz_harness::report::archive::ArchivedRun;

fn promote(historical: ArchivedRun) -> RunReport {
    historical
}

fn main() {}

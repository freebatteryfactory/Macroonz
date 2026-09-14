//! A loaded trial and its clock claims cannot become live report evidence.

use macroonz_harness::clock::MeasurementReading;
use macroonz_harness::report::TrialReport;
use macroonz_harness::report::archive::{ArchivedMeasurement, ArchivedTrial};

fn promote(historical: ArchivedTrial) -> TrialReport {
    historical
}

fn measured(historical: ArchivedMeasurement) -> MeasurementReading {
    historical
}

fn main() {}

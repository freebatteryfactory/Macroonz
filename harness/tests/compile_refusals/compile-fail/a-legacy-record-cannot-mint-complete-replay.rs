//! Sparse source claims do not convert into current or complete historical reproduction.

use macroonz_harness::report::{ReplayCapsule, TrialReport};
use macroonz_harness::report::archive::ArchivedCapsule;
use macroonz_harness::report::legacy::LegacyRecord;
use macroonz_harness::report::replay::{LegacyReading, ReplayReading};

fn report(record: LegacyRecord) -> TrialReport { record.into() }
fn capsule(record: LegacyRecord) -> ReplayCapsule { record.into() }
fn history(record: LegacyRecord) -> ArchivedCapsule { record.into() }
fn comparison(reading: LegacyReading) -> ReplayReading { reading.into() }

fn main() { let _ = (report, capsule, history, comparison); }

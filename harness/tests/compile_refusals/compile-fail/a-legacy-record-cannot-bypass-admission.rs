//! Private fields cannot bypass bounded historical admission or the actual witness join.

use macroonz_harness::report::legacy::{LegacyInputProfile, LegacyRecord};
use macroonz_harness::report::replay::LegacyReading;
use macroonz_harness::runner::LegacyReplayedTrial;

fn forge(record: LegacyRecord, profile: LegacyInputProfile, reading: LegacyReading, run: LegacyReplayedTrial) {
    let _ = LegacyRecord { ..record };
    let _ = LegacyInputProfile { ..profile };
    let _ = LegacyReading { ..reading };
    let _ = LegacyReplayedTrial { ..run };
}

fn main() { let _ = forge; }

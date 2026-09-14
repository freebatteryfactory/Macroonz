//! Generic JSON deserialization cannot bypass source limits or witness admission.

use macroonz_harness::report::legacy::{LegacyInputProfile, LegacyRecord};
use macroonz_harness::report::replay::LegacyReading;
use macroonz_harness::runner::LegacyReplayedTrial;

fn unchecked(bytes: &[u8]) {
    let _: Result<LegacyRecord, _> = serde_json::from_slice(bytes);
    let _: Result<LegacyInputProfile, _> = serde_json::from_slice(bytes);
    let _: Result<LegacyReading, _> = serde_json::from_slice(bytes);
    let _: Result<LegacyReplayedTrial, _> = serde_json::from_slice(bytes);
}

fn main() { let _ = unchecked; }

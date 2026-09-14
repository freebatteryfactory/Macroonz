//! A caller cannot replace a checked replay outcome or manufacture an execution account.

use macroonz_harness::report::replay::{ReplayOutcome, ReplayReading};
use macroonz_harness::runner::ReplayedTrial;

fn relabel(mut reading: ReplayReading) {
    reading.outcome = ReplayOutcome::FixedOnWitness;
}

fn promote(historical: macroonz_harness::report::archive::ArchivedCapsule) -> ReplayedTrial {
    historical
}

fn main() {}

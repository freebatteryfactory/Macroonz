//! A measurement start can publish exactly one reading.

use macroonz_harness::clock::HarnessClock;

fn main() {
    let start = HarnessClock::unavailable().begin();
    let _first = start.finish();
    let _second = start.finish();
}

//! The public one-shot measurement road retains its declared source across both reads.

use macroonz_harness::clock::{
    HarnessClock, MeasurementReading, MeasurementStart, RecordedDuration,
};
use std::sync::atomic::{AtomicU64, Ordering};

const FINISH: fn(MeasurementStart) -> MeasurementReading = MeasurementStart::finish;

static RETAINED_READS: AtomicU64 = AtomicU64::new(0u64);
static FOREIGN_READS: AtomicU64 = AtomicU64::new(0u64);

fn retained_source() -> u64 {
    RETAINED_READS.fetch_add(11u64, Ordering::SeqCst)
}

fn foreign_source() -> u64 {
    FOREIGN_READS.fetch_add(1u64, Ordering::SeqCst)
}

/// The public finish operation consumes one opaque start and takes no replacement source.
#[test]
fn one_start_finishes_against_its_retained_source_only() {
    RETAINED_READS.store(0u64, Ordering::SeqCst);
    FOREIGN_READS.store(0u64, Ordering::SeqCst);

    let start = HarnessClock::reading(retained_source).begin();
    let _unrelated = HarnessClock::reading(foreign_source);
    let reading = FINISH(start);

    assert_eq!(
        reading,
        MeasurementReading::Observed(RecordedDuration::recorded(11u64))
    );
    assert_eq!(RETAINED_READS.load(Ordering::SeqCst), 22u64);
    assert_eq!(FOREIGN_READS.load(Ordering::SeqCst), 0u64);
}

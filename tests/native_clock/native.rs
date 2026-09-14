//! Native observations bounded by separately obtained outer readings.

use macroonz::harness::clock::{ClockAttribution, HarnessClock};
use std::time::{Duration, Instant};

const SOURCE: HarnessClock = macroonz::native_clock::source();

#[test]
fn a_real_interval_is_observed_inside_independent_outer_readings() -> Result<(), String> {
    assert_eq!(SOURCE.attribution(), ClockAttribution::Monotonic);
    let outer_open = Instant::now();
    let start = SOURCE.begin();
    std::thread::sleep(Duration::from_millis(5));
    let reading = start.finish();
    let outer_close = Instant::now();
    let outer = outer_close
        .checked_duration_since(outer_open)
        .ok_or("the independent native interval regressed")?;
    let measured = reading.duration().ok_or_else(|| format!("{reading:?}"))?;
    assert!(measured.nanoseconds() >= 5_000_000);
    assert!(u128::from(measured.nanoseconds()) <= outer.as_nanos());
    Ok(())
}

#[test]
fn concurrent_measurements_keep_independent_consuming_starts() -> Result<(), String> {
    let handles = (0u8..4)
        .map(|_| {
            std::thread::spawn(|| {
                let start = SOURCE.begin();
                std::thread::sleep(Duration::from_millis(1));
                start.finish()
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        let reading = handle.join().map_err(|_| "measurement thread unwound")?;
        assert!(reading.duration().is_some(), "{reading:?}");
    }
    Ok(())
}

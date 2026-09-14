//! A fresh process inspects every reached stage after live report ownership is dropped.

use super::{LIMITS, claims::observe_stages, fixture};
use crate::archive_process;
use macroonz_harness::bench::archive::{read_report, retain_report};
use std::error::Error;
use std::io::Read as _;

#[test]
fn complete_benchmark_data_survives_fresh_process_owned_readback() -> Result<(), Box<dyn Error>> {
    let report = fixture::all_stages()?;
    let retained = retain_report(&report, LIMITS).map_err(|e| format!("retain: {e:?}"))?;
    let original = retained.encoded().to_vec();
    drop(retained);
    drop(report);
    let mut returned = archive_process::round_trip(&original, "archive::process::benchmark_child")?;
    assert_eq!(returned, original);
    let loaded = read_report(&returned, LIMITS).map_err(|e| format!("read: {e:?}"))?;
    returned.fill(0);
    observe_stages(&loaded)
}

#[test]
#[ignore = "launched by the parent with the complete benchmark bytes on stdin"]
fn benchmark_child() -> Result<(), Box<dyn Error>> {
    let mut input = Vec::new();
    std::io::stdin()
        .lock()
        .take(65537)
        .read_to_end(&mut input)?;
    let loaded = read_report(&input, LIMITS).map_err(|e| format!("child: {e:?}"))?;
    input.fill(0);
    observe_stages(&loaded)?;
    archive_process::publish(loaded.encoded())
}

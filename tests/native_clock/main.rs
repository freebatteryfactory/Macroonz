//! The root native source observed through the existing public measurement boundary.

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use macroonz::harness;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod read_boundary;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod native;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod runner;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
#[test]
fn unsupported_target_supplies_declared_unavailability() {
    use macroonz::harness::clock::{ClockAttribution, MeasurementReading};

    let source = macroonz::native_clock::source();
    assert_eq!(source.attribution(), ClockAttribution::Unspecified);
    assert_eq!(source.begin().finish(), MeasurementReading::Unavailable);
}

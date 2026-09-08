//! The native source supplied through the existing harness-clock constructor.

use crate::harness::clock::HarnessClock;

/// The native monotonic source for one caller-selected measurement.
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[must_use]
pub const fn source() -> HarnessClock {
    HarnessClock::fallible_as(
        super::read::native,
        crate::harness::clock::ClockAttribution::Monotonic,
    )
}

/// Declared measurement unavailability on a target without this native adapter.
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
#[must_use]
pub const fn source() -> HarnessClock {
    HarnessClock::unavailable()
}

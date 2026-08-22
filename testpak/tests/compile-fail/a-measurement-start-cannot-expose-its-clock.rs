//! A measurement start does not expose the retained source an outside caller could replace.

use threadpak_testpak::clock::HarnessClock;

fn main() {
    let start = HarnessClock::unavailable().begin();
    let _ = start.opening;
}

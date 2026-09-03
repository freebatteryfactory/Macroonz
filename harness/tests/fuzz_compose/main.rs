//! The fuzz home admits interesting bytes into Macroonz reduction and replay without owning the coverage engine.

#[path = "../support/trial_fixture.rs"]
mod trial_fixture;

mod budget;
mod compose;
mod frontier;
mod lcov;
mod preflight;
mod support;

//! The fuzz home admits interesting bytes into Macroonz reduction and replay without owning the coverage engine.

#[path = "../support/trial_fixture.rs"]
mod trial_fixture;

mod budget;
mod compose;
mod frontier;
mod lcov;
mod preflight;
mod recipe_compilation;
mod recipe_control;
mod recipe_deadline;
mod recipe_grammar;
mod recipe_observation;
mod scratch;
mod support;

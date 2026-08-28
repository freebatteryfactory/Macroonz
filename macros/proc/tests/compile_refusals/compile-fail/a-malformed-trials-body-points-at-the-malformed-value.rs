//! A malformed attribute body refuses through the actual `trials` proc entry at the value the grammar could not read.

#[macroonz_macros::trials(
    support = 7,
    module = greet_trials,
    table = named("proc", "greet-table"),
)]
mod held {}

fn main() {}

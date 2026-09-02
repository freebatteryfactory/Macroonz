//! The zero-ceremony recipe entrance through a renamed facade dependency.

use super::support::{observe_crossing, observed_in_scratch};

#[test]
fn a_renamed_facade_bakes_and_delivers_one_recipe_without_recipe_topology_ceremony()
-> Result<(), String> {
    observed_in_scratch(observe_crossing)
}

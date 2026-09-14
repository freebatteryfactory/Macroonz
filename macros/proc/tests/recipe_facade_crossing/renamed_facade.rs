//! The zero-ceremony recipe entrance through a renamed facade dependency.

use super::support::{
    observe_bench_formula, observe_catalogue, observe_crossing, observed_in_scratch,
};

#[test]
fn a_renamed_facade_bakes_and_delivers_one_recipe_without_recipe_topology_ceremony()
-> Result<(), String> {
    observed_in_scratch(observe_crossing)
}

#[test]
fn co_located_bindings_cross_direct_and_recipe_carriers_without_a_second_row_roster()
-> Result<(), String> {
    observed_in_scratch(observe_catalogue)
}

#[test]
fn benchmark_formula_bytes_reach_the_owner_judge_and_invalid_formulas_refuse() -> Result<(), String>
{
    observed_in_scratch(observe_bench_formula)
}

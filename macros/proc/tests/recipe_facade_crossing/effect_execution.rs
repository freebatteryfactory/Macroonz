//! Caller-owned effect behavior observed through one package-shaped facade crossing.

use super::support::{observe_effect_execution, observed_in_scratch};

#[test]
fn every_effect_form_executes_through_the_same_recipe_dispatch() -> Result<(), String> {
    observed_in_scratch(observe_effect_execution)
}

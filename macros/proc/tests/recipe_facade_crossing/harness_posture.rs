//! Root-recipe availability and typed harness unavailability across facade postures.

use super::support::{observe_harness_refusal, observe_without_harness, observed_in_scratch};

#[test]
fn the_root_recipe_remains_available_without_the_optional_harness() -> Result<(), String> {
    observed_in_scratch(observe_without_harness)
}

#[test]
fn a_harness_projection_is_typed_unavailable_without_the_optional_harness() -> Result<(), String> {
    observed_in_scratch(observe_harness_refusal)
}

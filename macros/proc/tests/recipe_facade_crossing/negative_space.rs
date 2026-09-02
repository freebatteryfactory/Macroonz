//! External package crossings for recipes that intentionally request little or no generated production surface.

use super::support::{observe_negative_space, observed_in_scratch};

#[test]
fn edition_2021_adopters_keep_the_negative_space_recipe_road() -> Result<(), String> {
    observed_in_scratch(|scratch| observe_negative_space(scratch, "2021"))
}

#[test]
fn edition_2024_adopters_keep_the_negative_space_recipe_road() -> Result<(), String> {
    observed_in_scratch(|scratch| observe_negative_space(scratch, "2024"))
}

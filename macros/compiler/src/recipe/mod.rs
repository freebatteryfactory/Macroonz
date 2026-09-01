#![doc = include_str!("README.md")]

mod bake;
mod capture;
mod encode;
mod render;
mod type_contract;
mod types;

pub use bake::{bake, bake_with, bake_wrapped};
pub use types::{
    EffectiveProjection, HarnessPosture, LoweringSource, Offered, ProjectionError,
    ProjectionRequest, ProjectionSink, ProjectionStanding, RECIPE_FACT, Recipe, RecipeBake,
    RecipeError, RecipeIssue, RecipeMember, RecipeProjection, RecipeProjector, RecipeRole,
    RecipeTransition, RecipeView, TRANSITION_LIMIT, VOCABULARY_LIMIT,
};

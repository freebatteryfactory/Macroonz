#![doc = include_str!("README.md")]

mod bake;
mod capture;
mod encode;
mod evidence;
mod render;
mod type_contract;
mod types;

pub(crate) use bake::generated_name_collision;
pub(crate) use evidence::{
    ConfiguredEvidence, EvidenceCompiler, PreparedEvidence, evidence_position,
};

pub use bake::{bake, bake_with, bake_wrapped};
pub use types::{
    EVIDENCE_LIMIT, EffectiveProjection, EvidenceTarget, HarnessPosture, LoweringSource,
    ProjectionDisposition, ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink,
    Recipe, RecipeBake, RecipeEvidence, RecipeMember, RecipeProjection, RecipeProjector,
    RecipeRole, RecipeTransition, RecipeView, TRANSITION_LIMIT, VOCABULARY_LIMIT,
};

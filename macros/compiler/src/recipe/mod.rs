#![doc = include_str!("README.md")]

mod bake;
mod capture;
mod encode;
mod evidence;
mod names;
mod render;
mod render_companions;
mod render_codec;
mod render_dispatch;
mod render_evidence;
mod render_relation_tables;
mod render_tokens;
mod render_typestate;
mod type_contract;
mod types;

pub(crate) use bake::generated_name_collision;
pub(crate) use evidence::evidence_position;
pub(crate) use types::{ConfiguredEvidence, EvidenceCompiler, PreparedEvidence};

pub use bake::{bake, bake_with, bake_wrapped};
pub use types::{
    CODEC_LIMIT, EVIDENCE_LIMIT, EffectiveProjection, EvidenceTarget, HarnessPosture,
    LoweringSource, PROJECTION_LIMIT, ProjectionDisposition, ProjectionError, ProjectionOffered,
    ProjectionRequest, ProjectionSink, RELATION_LIMIT, RELATION_ROW_LIMIT, RELATION_TABLE_LIMIT,
    Recipe, RecipeBake, RecipeCodec, RecipeEvidence, RecipeMember, RecipeProjection,
    RecipeProjector, RecipeRelation, RecipeRelationPayload, RecipeRelationPayloadKind,
    RecipeRelationRequirements, RecipeRelationRow, RecipeRole, RecipeTransitionEffect, RecipeView,
    RecipeVocabulary, RelationTableProjection, TRANSITION_LIMIT, VOCABULARY_LIMIT,
};

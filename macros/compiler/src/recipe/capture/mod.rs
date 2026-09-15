#![doc = include_str!("README.md")]

use super::types::{
    BakeRead, CapturedName, CapturedRelation, RecipeCodec, RecipeError, RecipeIssue, RecipeParts,
    RecipeRelationParts, RecipeVocabularyParts, RequestedEvidence, RequestedProjection,
    RequestedRelationTable,
};
use super::{
    EVIDENCE_LIMIT, EffectiveProjection, HarnessPosture, LoweringSource, RELATION_TABLE_LIMIT,
    Recipe, RecipeMember, RecipeRelationRequirements, RecipeRelationRow, RecipeRole,
    RelationTableProjection,
};
use crate::token::{
    AuthoredItemKind, CapturedDelimiter, CapturedInput, CapturedTokenTree, preserved_tree,
};

mod bake;
mod codec;
mod consuming;
mod dispatch;
mod evidence;
mod module;
mod projection;
mod read;
mod relation;

use read::{fragment_refusal, grammar, identifier_token};

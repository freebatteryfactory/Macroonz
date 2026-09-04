#![doc = include_str!("README.md")]

use super::types::{
    PROJECTION_LIMIT, ProjectionStanding, RecipeCodec, RecipeError, RecipeIssue, RecipeParts,
    RecipeRelationParts, RecipeVocabularyParts,
};
use super::{
    EVIDENCE_LIMIT, EffectiveProjection, EvidenceTarget, HarnessPosture, LoweringSource,
    RELATION_TABLE_LIMIT, Recipe, RecipeEvidence, RecipeMember, RecipeRelationRequirements,
    RecipeRelationRow, RecipeRole, RelationTableProjection,
};
use crate::support::SupportName;
use crate::token::{
    AuthoredItemKind, CapturedDelimiter, CapturedInput, CapturedTokenTree, preserved_tree,
};

mod bake;
mod codec;
mod dispatch;
mod evidence;
mod module;
mod projection;
mod read;
mod relation;

use read::{fragment_refusal, grammar, identifier_token};

/// The private suffix that declares one recipe inside its authored module.
const BAKE: &str = "bake";

/// The mechanically read bake declaration before structural informing.
struct BakeRead {
    vocabularies: Vec<CapturedName>,
    relations: Vec<CapturedRelation>,
    transition_relation: Option<String>,
    codecs: Vec<RecipeCodec>,
    projections: [ProjectionStanding; PROJECTION_LIMIT],
    evidence: [Option<RecipeEvidence>; EVIDENCE_LIMIT],
    support: Option<SupportName>,
}

/// One mechanically read named relation before its endpoint references are informed.
#[derive(Clone)]
struct CapturedRelation {
    name: CapturedName,
    left: CapturedName,
    right: CapturedName,
    rows: Vec<RecipeRelationRow>,
    requirements: RecipeRelationRequirements,
}

/// One exact identifier read from recipe syntax before its structural role is informed.
#[derive(Clone)]
struct CapturedName {
    spelling: String,
    token: crate::token::GeneratedToken,
    at: crate::token::SpanHandle,
}

/// One requested role with its mechanical configuration.
#[derive(Clone)]
struct RequestedProjection {
    role: RecipeRole,
    name: Option<String>,
    subject: Option<String>,
    source: LoweringSource,
    exact: Option<CapturedInput>,
    dispatch_bindings: Option<[String; 2]>,
    relation_tables: Option<Vec<RequestedRelationTable>>,
    at: crate::token::SpanHandle,
}

/// One mechanically requested relation-table surface before its relation is informed.
#[derive(Clone)]
struct RequestedRelationTable {
    relation: String,
    function: Option<String>,
    source: LoweringSource,
    exact: Option<CapturedInput>,
    at: crate::token::SpanHandle,
}

/// One descriptor-native evidence role and its generated or target-unavailable standing.
#[derive(Clone)]
struct RequestedEvidence {
    role: RecipeRole,
    target: Option<String>,
    body: Option<CapturedInput>,
    at: crate::token::SpanHandle,
}

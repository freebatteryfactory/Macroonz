//! Reading the one inline-module recipe grammar into informed structural values.

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
    AuthoredItemKind, CaptureReadRefusal, CapturedDelimiter, CapturedInput, CapturedTokenTree,
    preserved_tree,
};

#[path = "capture_bake.rs"]
mod bake;
#[path = "capture_codec.rs"]
mod codec;
#[path = "capture_dispatch.rs"]
mod dispatch;
#[path = "capture_evidence.rs"]
mod evidence;
#[path = "capture_module.rs"]
mod module;
#[path = "capture_projection.rs"]
mod projection;
#[path = "capture_relation.rs"]
mod relation;

use bake::read_bake;
use module::{authored_record, bake_suffix, collision_free, enum_members};

/// The private suffix that declares one recipe inside its authored module.
const BAKE: &str = "bake";

impl Recipe {
    /// Read one inline authored module and its final `bake!` declaration into an informed recipe.
    ///
    /// # Errors
    ///
    /// Returns the exact structural, grammar, membership, collision, or feature-posture refusal established before planning.
    pub(in crate::recipe) fn read(
        input: &CapturedInput,
        harness: HarnessPosture,
    ) -> Result<Self, RecipeError> {
        let item = input.authored_item().map_err(|refusal| {
            RecipeError::at(RecipeIssue::InlineModuleRequired, refusal.token())
        })?;
        if item.kind() != AuthoredItemKind::Module {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(item.kind_token().span()),
            ));
        }
        let Some((name_token, module_name)) = item.name() else {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(item.kind_token().span()),
            ));
        };
        let Some((CapturedDelimiter::Brace, body)) = item.body() else {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(name_token.span()),
            ));
        };
        let (authored, declaration) = bake_suffix(body)?;
        collision_free(authored)?;
        let read = read_bake(declaration, harness, input.issued())?;
        for codec in &read.codecs {
            let Some(owner) = codec.content().shape.owner().segments().last() else {
                return Err(RecipeError::at(
                    RecipeIssue::CodecOwnerNotRecord {
                        codec: codec.name().to_owned(),
                        owner: "<missing>".to_owned(),
                    },
                    body.enclosing_span(),
                ));
            };
            authored_record(authored, codec.name(), owner)?;
        }
        let vocabularies = read
            .vocabularies
            .into_iter()
            .map(|vocabulary| {
                let members = enum_members(authored, vocabulary.spelling.as_str())?;
                Ok(RecipeVocabularyParts {
                    name: vocabulary.spelling,
                    name_token: vocabulary.token,
                    members,
                    at: vocabulary.at,
                })
            })
            .collect::<Result<Vec<_>, RecipeError>>()?;
        let relations = read
            .relations
            .into_iter()
            .map(|relation| RecipeRelationParts {
                name: relation.name.spelling,
                name_token: relation.name.token,
                name_at: relation.name.at,
                left_vocabulary: relation.left.spelling,
                left_vocabulary_at: relation.left.at,
                right_vocabulary: relation.right.spelling,
                right_vocabulary_at: relation.right.at,
                rows: relation.rows,
                requirements: relation.requirements,
            })
            .collect();

        let attributes = item
            .attributes()
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
        let signature = item
            .signature()
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
        let module_head = attributes
            .joined(&signature)
            .map_err(|_| fragment_refusal(Some(name_token.span())))?;
        let authored_body =
            preserved_tree(authored).map_err(|refusal| fragment_refusal(refusal.token()))?;

        Recipe::informed(RecipeParts {
            module_name: module_name.to_owned(),
            module_name_token: identifier_token(name_token, module_name),
            module_head,
            authored_body,
            module_body_at: body.enclosing_span(),
            vocabularies,
            relations,
            transition_relation: read.transition_relation,
            codecs: read.codecs,
            projections: read.projections,
            evidence: read.evidence,
            support: read.support,
        })
    }
}

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

fn identifier_token(token: &CapturedTokenTree, spelling: &str) -> crate::token::GeneratedToken {
    if token.raw_identifier().is_some() {
        crate::token::GeneratedToken::raw_identifier(spelling)
    } else {
        crate::token::GeneratedToken::word(spelling)
    }
}

fn grammar(refusal: CaptureReadRefusal) -> RecipeError {
    let (issue, at) = refusal.into_parts();
    RecipeError::at(RecipeIssue::Grammar(issue), at)
}

fn fragment_refusal(at: Option<crate::token::SpanHandle>) -> RecipeError {
    RecipeError::at(RecipeIssue::FragmentNotGenerated, at)
}

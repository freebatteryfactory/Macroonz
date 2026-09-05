//! The read from one authored module and its final bake declaration into an informed recipe.

use super::bake::read_bake;
use super::module::{authored_record, bake_suffix, collision_free, enum_members};
use super::{
    AuthoredItemKind, CapturedDelimiter, CapturedInput, CapturedTokenTree, HarnessPosture, Recipe,
    RecipeError, RecipeIssue, RecipeParts, RecipeRelationParts, RecipeVocabularyParts,
    preserved_tree,
};
use crate::token::CaptureReadRefusal;

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
        let authored_declaration = declaration
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
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
            authored_declaration,
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

/// Convert one captured identifier to the generated form that preserves its rawness.
pub(super) fn identifier_token(
    token: &CapturedTokenTree,
    spelling: &str,
) -> crate::token::GeneratedToken {
    if token.raw_identifier().is_some() {
        crate::token::GeneratedToken::raw_identifier(spelling)
    } else {
        crate::token::GeneratedToken::word(spelling)
    }
}

/// Project one capture-reading refusal into the recipe grammar family.
pub(super) fn grammar(refusal: CaptureReadRefusal) -> RecipeError {
    let (issue, at) = refusal.into_parts();
    RecipeError::at(RecipeIssue::Grammar(issue), at)
}

/// Project one generated-fragment refusal at its captured token.
pub(super) fn fragment_refusal(at: Option<crate::token::SpanHandle>) -> RecipeError {
    RecipeError::at(RecipeIssue::FragmentNotGenerated, at)
}

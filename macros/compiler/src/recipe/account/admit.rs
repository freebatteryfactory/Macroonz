//! Admitting authored rosters into keyed collections and naming the first refusal when they do not fit.

use super::settle::{missing_vocabulary, relation_account_refusal};
use super::{
    CODEC_LIMIT, RELATION_LIMIT, RecipeCodec, RecipeError, RecipeIssue, RecipeMember,
    RecipeRelation, RecipeRelationParts, RecipeRole, RecipeVocabularyParts, RelationLowering,
    VOCABULARY_LIMIT,
};
use crate::bounded::{KeyedRoster, KeyedRosterError};
use crate::token::SpanHandle;

pub(super) fn informed_vocabularies(
    offered: Vec<RecipeVocabularyParts>,
) -> Result<Option<KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>, RecipeError> {
    let informed = offered
        .into_iter()
        .map(|vocabulary| {
            super::RecipeVocabulary::informed(
                vocabulary.name,
                vocabulary.name_token,
                vocabulary.members,
                vocabulary.at,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    if informed.is_empty() {
        return Ok(None);
    }
    let informed_for_refusal = informed.clone();
    KeyedRoster::new(informed, |vocabulary| vocabulary.name.clone())
        .map(Some)
        .map_err(|refusal| vocabulary_account_refusal(&informed_for_refusal, refusal))
}

pub(super) fn informed_relations(
    offered: Vec<RecipeRelationParts>,
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<Option<KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>, RecipeError> {
    if offered.is_empty() {
        return Ok(None);
    }
    let Some(vocabularies) = vocabularies else {
        let name = offered
            .first()
            .map_or("<relation>", |relation| relation.left_vocabulary.as_str());
        return Err(missing_vocabulary(name, None));
    };
    let relations = offered
        .into_iter()
        .map(|relation| {
            let lowering = if transition_relation == Some(relation.name.as_str()) {
                RelationLowering::Transition
            } else {
                RelationLowering::Generic
            };
            RecipeRelation::informed(relation, vocabularies, lowering)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let informed_for_refusal = relations.clone();
    KeyedRoster::new(relations, |relation| relation.name.clone())
        .map(Some)
        .map_err(|refusal| relation_account_refusal(&informed_for_refusal, refusal))
}

pub(super) fn informed_codecs(
    codecs: Vec<RecipeCodec>,
) -> Result<Option<KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>, RecipeError> {
    if codecs.is_empty() {
        return Ok(None);
    }
    let offered = codecs.clone();
    KeyedRoster::new(codecs, |codec| codec.name.clone())
        .map(Some)
        .map_err(|refusal| codec_account_refusal(&offered, refusal))
}

pub(super) fn informed_members(
    vocabulary: &str,
    members: Vec<RecipeMember>,
    vocabulary_at: SpanHandle,
) -> Result<KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>, RecipeError> {
    let offered = members.clone();
    KeyedRoster::new(members, |member| member.spelling.clone()).map_err(|refusal| match refusal {
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let duplicate = duplicates.first();
            let at = offered
                .get(*duplicate.repeated_positions().first())
                .map(RecipeMember::at);
            RecipeError::at(
                RecipeIssue::DuplicateMember {
                    vocabulary: vocabulary.to_owned(),
                    member: duplicate.key().clone(),
                },
                at,
            )
        }
        KeyedRosterError::Empty(_) => RecipeError::at(
            RecipeIssue::VocabularyEmpty {
                name: vocabulary.to_owned(),
            },
            Some(vocabulary_at),
        ),
        KeyedRosterError::Overflow(overflow) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: overflow.capacity,
            }),
            offered
                .get(overflow.capacity)
                .map(RecipeMember::at)
                .or(Some(vocabulary_at)),
        ),
    })
}

fn vocabulary_account_refusal(
    offered: &[super::RecipeVocabulary],
    refusal: KeyedRosterError<String, VOCABULARY_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let name = duplicates.first().key();
            let at = offered
                .get(*duplicates.first().repeated_positions().first())
                .map(|vocabulary| vocabulary.at);
            RecipeError::at(RecipeIssue::DuplicateVocabulary { name: name.clone() }, at)
        }
        KeyedRosterError::Empty(_) => RecipeError::at(RecipeIssue::FragmentNotGenerated, None),
        KeyedRosterError::Overflow(overflow) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: overflow.capacity,
            }),
            offered
                .get(overflow.capacity)
                .map(|vocabulary| vocabulary.at),
        ),
    }
}

fn codec_account_refusal(
    offered: &[RecipeCodec],
    refusal: KeyedRosterError<String, CODEC_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterError::Empty(_) => RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Codec,
                expected: "at least one existing-owner codec declaration",
            },
            offered.first().map(RecipeCodec::at),
        ),
        KeyedRosterError::Overflow(_) => RecipeError::at(
            RecipeIssue::CodecDeclaration {
                name: "<recipe>".to_owned(),
                reason: format!("the codec roster exceeds its declared bound of {CODEC_LIMIT}"),
            },
            offered.first().map(RecipeCodec::at),
        ),
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let duplicate = duplicates.first();
            let at = offered
                .get(*duplicate.repeated_positions().first())
                .map(RecipeCodec::at);
            RecipeError::at(
                RecipeIssue::DuplicateCodec {
                    name: duplicate.key().clone(),
                },
                at,
            )
        }
    }
}

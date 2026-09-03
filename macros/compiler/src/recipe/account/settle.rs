//! Settling declared structural postures against computed relation standings and naming relation refusals.

use super::{
    RELATION_LIMIT, RELATION_ROW_LIMIT, RecipeError, RecipeIssue, RecipeMember, RecipeRelation,
    RecipeRelationPayload, RecipeRelationRequirements, RecipeRelationRow, RecipeVocabulary,
    VOCABULARY_LIMIT,
};
use crate::bounded::{KeyedRoster, KeyedRosterError};
use crate::relation::{
    EmptyPosture, KeyedRosterRows, KeyedRosterRowsError, RepetitionPosture, SameRosterRequired,
};
use crate::token::SpanHandle;

pub(super) fn settle_relation_requirements(
    relation: &str,
    rows: &KeyedRosterRows<
        '_,
        RecipeMember,
        String,
        RecipeMember,
        String,
        RecipeRelationRow,
        VOCABULARY_LIMIT,
        VOCABULARY_LIMIT,
        RELATION_ROW_LIMIT,
    >,
    requirements: RecipeRelationRequirements,
    at: Option<SpanHandle>,
) -> Result<(), RecipeError> {
    if let Some(requirement) = requirements.empty.and_then(EmptyPosture::requirement)
        && let Err(mismatch) = requirement.settle(rows.occupancy_standing())
    {
        return Err(relation_posture_mismatch(
            relation,
            "empty",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        ));
    }
    if let Some(requirement) = requirements
        .repetition
        .and_then(RepetitionPosture::requirement)
        && let Err(mismatch) = requirement.settle(rows.repetition_standing())
    {
        return Err(relation_posture_mismatch(
            relation,
            "repetition",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        ));
    }
    if let Some(requirement) = requirements
        .left_completeness()
        .and_then(crate::relation::CompletenessPosture::requirement)
        && let Err(mismatch) = requirement.settle(rows.left_completeness())
    {
        return Err(relation_posture_mismatch(
            relation,
            "left completeness",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        ));
    }
    if let Some(requirement) = requirements
        .right_completeness()
        .and_then(crate::relation::CompletenessPosture::requirement)
        && let Err(mismatch) = requirement.settle(rows.right_completeness())
    {
        return Err(relation_posture_mismatch(
            relation,
            "right completeness",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        ));
    }
    if let Some(requirement) = requirements
        .density
        .and_then(crate::relation::DensityPosture::requirement)
        && let Err(mismatch) = requirement.settle(rows.density_standing())
    {
        return Err(relation_posture_mismatch(
            relation,
            "density",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        ));
    }
    settle_self_relation(relation, rows, requirements, at)?;
    settle_cycle(relation, rows, requirements, at)
}

fn settle_self_relation(
    relation: &str,
    rows: &KeyedRosterRows<
        '_,
        RecipeMember,
        String,
        RecipeMember,
        String,
        RecipeRelationRow,
        VOCABULARY_LIMIT,
        VOCABULARY_LIMIT,
        RELATION_ROW_LIMIT,
    >,
    requirements: RecipeRelationRequirements,
    at: Option<SpanHandle>,
) -> Result<(), RecipeError> {
    let Some(requirement) = requirements
        .self_relation
        .and_then(crate::relation::SelfRelationPosture::requirement)
    else {
        return Ok(());
    };
    let observed = rows
        .self_relation_standing()
        .map_err(|SameRosterRequired| {
            relation_posture_inapplicable(relation, "self relation", at)
        })?;
    requirement.settle(observed).map_err(|mismatch| {
        relation_posture_mismatch(
            relation,
            "self relation",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        )
    })?;
    Ok(())
}

fn settle_cycle(
    relation: &str,
    rows: &KeyedRosterRows<
        '_,
        RecipeMember,
        String,
        RecipeMember,
        String,
        RecipeRelationRow,
        VOCABULARY_LIMIT,
        VOCABULARY_LIMIT,
        RELATION_ROW_LIMIT,
    >,
    requirements: RecipeRelationRequirements,
    at: Option<SpanHandle>,
) -> Result<(), RecipeError> {
    let Some(requirement) = requirements
        .cycle
        .and_then(crate::relation::CyclePosture::requirement)
    else {
        return Ok(());
    };
    let observed = rows
        .cycle_standing()
        .map_err(|SameRosterRequired| relation_posture_inapplicable(relation, "cycle", at))?;
    requirement.settle(observed).map_err(|mismatch| {
        relation_posture_mismatch(
            relation,
            "cycle",
            mismatch.required().name(),
            mismatch.observed().name(),
            at,
        )
    })?;
    Ok(())
}

fn relation_posture_mismatch(
    relation: &str,
    question: &'static str,
    required: &'static str,
    observed: &'static str,
    at: Option<SpanHandle>,
) -> RecipeError {
    RecipeError::at(
        RecipeIssue::RelationPostureMismatch {
            relation: relation.to_owned(),
            question,
            required,
            observed,
        },
        at,
    )
}

fn relation_posture_inapplicable(
    relation: &str,
    question: &'static str,
    at: Option<SpanHandle>,
) -> RecipeError {
    RecipeError::at(
        RecipeIssue::RelationPostureInapplicable {
            relation: relation.to_owned(),
            question,
        },
        at,
    )
}

pub(super) fn referenced_refusal(
    left: &str,
    right: &str,
    offered: &[RecipeRelationRow],
    refusal: KeyedRosterRowsError<String, String, RELATION_ROW_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterRowsError::ForeignLeft(foreign) => {
            let first = foreign.first();
            let at = offered.get(first.offered_position()).map(|row| row.left_at);
            RecipeError::at(
                RecipeIssue::ForeignMember {
                    vocabulary: left.to_owned(),
                    member: first.key().clone(),
                },
                at,
            )
        }
        KeyedRosterRowsError::ForeignRight(foreign) => {
            let first = foreign.first();
            let at = offered
                .get(first.offered_position())
                .map(|row| row.right_at);
            RecipeError::at(
                RecipeIssue::ForeignMember {
                    vocabulary: right.to_owned(),
                    member: first.key().clone(),
                },
                at,
            )
        }
        KeyedRosterRowsError::Overflow(overflow) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: overflow.capacity,
            }),
            offered
                .get(overflow.capacity)
                .map(|row| row.left_at)
                .or_else(|| offered.first().map(|row| row.left_at)),
        ),
    }
}

pub(super) fn relation_account_refusal(
    offered: &[RecipeRelation],
    refusal: KeyedRosterError<String, RELATION_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let duplicate = duplicates.first();
            let at = offered
                .get(*duplicate.repeated_positions().first())
                .map(|relation| relation.name_at);
            RecipeError::at(
                RecipeIssue::DuplicateRelation {
                    name: duplicate.key().clone(),
                },
                at,
            )
        }
        KeyedRosterError::Empty(_) => RecipeError::at(RecipeIssue::FragmentNotGenerated, None),
        KeyedRosterError::Overflow(overflow) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: overflow.capacity,
            }),
            offered
                .get(overflow.capacity)
                .map(|relation| relation.name_at),
        ),
    }
}

pub(super) fn missing_vocabulary(name: &str, at: Option<SpanHandle>) -> RecipeError {
    RecipeError::at(
        RecipeIssue::VocabularyNotFound {
            name: name.to_owned(),
        },
        at,
    )
}

pub(super) fn missing_relation(name: &str) -> RecipeError {
    RecipeError::at(
        RecipeIssue::Grammar(crate::token::CaptureReadIssue::Unexpected(
            crate::token::CaptureExpectation::Word(format!("relation {name}")),
        )),
        None,
    )
}

pub(super) fn validate_transition_relation(
    relation: &RecipeRelation,
    vocabularies: &KeyedRoster<RecipeVocabulary, String, VOCABULARY_LIMIT>,
) -> Result<(), RecipeError> {
    let targets = vocabularies
        .get(relation.left_vocabulary())
        .ok_or_else(|| missing_vocabulary(relation.left_vocabulary(), None))?;
    for row in relation.rows() {
        let RecipeRelationPayload::Transition { target, .. } = row.payload() else {
            return Err(RecipeError::at(
                RecipeIssue::FragmentNotGenerated,
                Some(row.payload_at),
            ));
        };
        if targets.members().get(target.as_str()).is_none() {
            return Err(RecipeError::at(
                RecipeIssue::ForeignMember {
                    vocabulary: targets.name().to_owned(),
                    member: target.clone(),
                },
                Some(row.payload_at),
            ));
        }
    }
    Ok(())
}

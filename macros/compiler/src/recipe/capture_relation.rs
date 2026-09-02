//! Generic vocabulary, relation-row, and structural-posture grammar.

use super::{
    CapturedName, CapturedRelation, RecipeError, RecipeIssue, RecipeRelationRequirements,
    RecipeRelationRow, grammar, identifier_token,
};
use crate::recipe::RecipeRelationPayload;
use crate::relation::{
    AbsencePosture, CompletenessPosture, CyclePosture, DensityPosture, EmptyPosture,
    MembershipPosture, RepetitionPosture, SelfRelationPosture,
};
use crate::token::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedDelimiter,
    CapturedSpacing,
};

pub(super) fn read_vocabularies(
    cursor: &mut CaptureCursor<'_>,
) -> Result<Vec<CapturedName>, RecipeError> {
    cursor.word("vocabularies").map_err(grammar)?;
    let vocabularies = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::VOCABULARY_LIMIT }>(';', read_name)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(vocabularies)
}

pub(super) fn read_relations(
    cursor: &mut CaptureCursor<'_>,
) -> Result<Vec<CapturedRelation>, RecipeError> {
    cursor.word("relations").map_err(grammar)?;
    let relations = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::RELATION_LIMIT }>(';', read_relation)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(relations)
}

fn read_name(cursor: &mut CaptureCursor<'_>) -> Result<CapturedName, CaptureReadRefusal> {
    let (token, spelling) = cursor.identifier()?;
    Ok(CapturedName {
        spelling: spelling.to_owned(),
        token: identifier_token(token, spelling),
        at: token.span(),
    })
}

fn read_relation(cursor: &mut CaptureCursor<'_>) -> Result<CapturedRelation, CaptureReadRefusal> {
    let name = read_name(cursor)?;
    let mut endpoints = cursor.group(CapturedDelimiter::Parenthesis)?;
    let left = read_name(&mut endpoints)?;
    endpoints.punctuation(',', CapturedSpacing::Alone)?;
    let right = read_name(&mut endpoints)?;
    endpoints.finish()?;
    let rows = cursor
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<_, { super::super::RELATION_ROW_LIMIT }>(';', read_row)?
        .as_slice()
        .to_vec();
    Ok(CapturedRelation {
        name,
        left,
        right,
        rows,
        requirements: RecipeRelationRequirements::unspecified(),
    })
}

fn read_row(cursor: &mut CaptureCursor<'_>) -> Result<RecipeRelationRow, CaptureReadRefusal> {
    let mut endpoints = cursor.group(CapturedDelimiter::Parenthesis)?;
    let left = read_name(&mut endpoints)?;
    endpoints.punctuation(',', CapturedSpacing::Alone)?;
    let right = read_name(&mut endpoints)?;
    endpoints.finish()?;
    let (payload, payload_at) = if cursor
        .next_token()
        .is_some_and(|token| token.punct() == Some(';') && token.joint_punct().is_none())
    {
        (RecipeRelationPayload::Unlabeled, left.at)
    } else {
        cursor.word("with")?;
        read_payload(cursor)?
    };
    Ok(RecipeRelationRow::authored(
        (left.spelling, left.token, left.at),
        (right.spelling, right.token, right.at),
        payload,
        payload_at,
    ))
}

fn read_payload(
    cursor: &mut CaptureCursor<'_>,
) -> Result<(RecipeRelationPayload, crate::token::SpanHandle), CaptureReadRefusal> {
    let Some(next) = cursor.next_token() else {
        return Err(CaptureReadRefusal::projected(
            CaptureReadIssue::Missing(CaptureExpectation::Group(CapturedDelimiter::Parenthesis)),
            None,
        ));
    };
    if next
        .group_fragment(CapturedDelimiter::Parenthesis)
        .is_some()
    {
        let mut path = cursor.group(CapturedDelimiter::Parenthesis)?;
        let (fragment, ()) = path.fragment(|path| {
            path.identifier()?;
            while !path.is_finished() {
                path.punctuation(':', CapturedSpacing::Joint)?;
                path.punctuation(':', CapturedSpacing::Alone)?;
                path.identifier()?;
            }
            Ok(())
        })?;
        return fragment
            .generated()
            .map(|payload| (RecipeRelationPayload::Path(payload), next.span()))
            .map_err(|refusal| {
                CaptureReadRefusal::projected(
                    CaptureReadIssue::CursorRangeContradiction,
                    refusal.token(),
                )
            });
    }
    if let Some(fragment) = next.group_fragment(CapturedDelimiter::Brace) {
        cursor.token()?;
        return fragment
            .generated()
            .map(|payload| (RecipeRelationPayload::ExactRust(payload), next.span()))
            .map_err(|refusal| {
                CaptureReadRefusal::projected(
                    CaptureReadIssue::CursorRangeContradiction,
                    refusal.token(),
                )
            });
    }
    Err(CaptureReadRefusal::projected(
        CaptureReadIssue::Unexpected(CaptureExpectation::Group(CapturedDelimiter::Parenthesis)),
        Some(next.span()),
    ))
}

#[derive(Clone)]
struct CapturedPosture {
    relation: String,
    clauses: Vec<CapturedPostureClause>,
    at: crate::token::SpanHandle,
}

#[derive(Clone, Copy)]
struct CapturedPostureClause {
    value: PostureClause,
    at: crate::token::SpanHandle,
}

#[derive(Clone, Copy)]
enum PostureClause {
    Empty(EmptyPosture),
    Repetition(RepetitionPosture),
    Membership(MembershipPosture, MembershipPosture),
    Completeness(CompletenessPosture, CompletenessPosture),
    Density(DensityPosture),
    Absence(AbsencePosture),
    SelfRelation(SelfRelationPosture),
    Cycle(CyclePosture),
}

pub(super) fn read_and_apply_postures(
    cursor: &mut CaptureCursor<'_>,
    relations: &mut [CapturedRelation],
) -> Result<(), RecipeError> {
    let postures = read_postures(cursor)?;
    apply_postures(relations, &postures)
}

fn read_postures(cursor: &mut CaptureCursor<'_>) -> Result<Vec<CapturedPosture>, RecipeError> {
    cursor.word("postures").map_err(grammar)?;
    let postures = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::RELATION_LIMIT }>(';', read_posture)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(postures)
}

fn read_posture(cursor: &mut CaptureCursor<'_>) -> Result<CapturedPosture, CaptureReadRefusal> {
    let (token, relation) = cursor.identifier()?;
    let clauses = cursor
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<_, { super::super::RELATION_QUESTION_LIMIT }>(
            ';',
            read_posture_clause,
        )?
        .as_slice()
        .to_vec();
    Ok(CapturedPosture {
        relation: relation.to_owned(),
        clauses,
        at: token.span(),
    })
}

fn read_posture_clause(
    cursor: &mut CaptureCursor<'_>,
) -> Result<CapturedPostureClause, CaptureReadRefusal> {
    let (token, question) = cursor.identifier()?;
    if !crate::recipe::types::RELATION_QUESTION_NAMES.contains(&question) {
        return Err(CaptureReadRefusal::projected(
            CaptureReadIssue::Unexpected(CaptureExpectation::Word(
                "a structural relation question".to_owned(),
            )),
            Some(token.span()),
        ));
    }
    let value = match question {
        "empty" => read_single(cursor, empty_posture).map(PostureClause::Empty),
        "repetition" => read_single(cursor, repetition_posture).map(PostureClause::Repetition),
        "membership" => read_pair(cursor, membership_posture)
            .map(|(left, right)| PostureClause::Membership(left, right)),
        "completeness" => read_pair(cursor, completeness_posture)
            .map(|(left, right)| PostureClause::Completeness(left, right)),
        "density" => read_single(cursor, density_posture).map(PostureClause::Density),
        "absence" => read_single(cursor, absence_posture).map(PostureClause::Absence),
        "self_relation" => {
            read_single(cursor, self_relation_posture).map(PostureClause::SelfRelation)
        }
        "cycle" => read_single(cursor, cycle_posture).map(PostureClause::Cycle),
        _ => unreachable!("the complete relation-question roster guards this match"),
    }?;
    Ok(CapturedPostureClause {
        value,
        at: token.span(),
    })
}

fn read_single<T>(
    cursor: &mut CaptureCursor<'_>,
    read: impl FnOnce(&str) -> Option<T>,
) -> Result<T, CaptureReadRefusal> {
    let mut answer = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (token, spelling) = answer.identifier()?;
    let value = read(spelling).ok_or_else(|| {
        CaptureReadRefusal::projected(
            CaptureReadIssue::Unexpected(CaptureExpectation::Word(
                "a lawful structural posture".to_owned(),
            )),
            Some(token.span()),
        )
    })?;
    answer.finish()?;
    Ok(value)
}

fn read_pair<T: Copy>(
    cursor: &mut CaptureCursor<'_>,
    read: impl Fn(&str) -> Option<T>,
) -> Result<(T, T), CaptureReadRefusal> {
    let mut answers = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (left_token, left) = answers.identifier()?;
    let left = read(left).ok_or_else(|| posture_word(left_token.span()))?;
    answers.punctuation(',', CapturedSpacing::Alone)?;
    let (right_token, right) = answers.identifier()?;
    let right = read(right).ok_or_else(|| posture_word(right_token.span()))?;
    answers.finish()?;
    Ok((left, right))
}

fn posture_word(at: crate::token::SpanHandle) -> CaptureReadRefusal {
    CaptureReadRefusal::projected(
        CaptureReadIssue::Unexpected(CaptureExpectation::Word(
            "a lawful structural posture".to_owned(),
        )),
        Some(at),
    )
}

fn empty_posture(value: &str) -> Option<EmptyPosture> {
    match value {
        "allowed" => Some(EmptyPosture::Allowed),
        "refused" => Some(EmptyPosture::Refusal),
        _ => None,
    }
}

fn repetition_posture(value: &str) -> Option<RepetitionPosture> {
    match value {
        "allowed" => Some(RepetitionPosture::Allowed),
        "refused" => Some(RepetitionPosture::Refusal),
        _ => None,
    }
}

fn membership_posture(value: &str) -> Option<MembershipPosture> {
    match value {
        "open" => Some(MembershipPosture::Open),
        "closed" => Some(MembershipPosture::Closed),
        _ => None,
    }
}

fn completeness_posture(value: &str) -> Option<CompletenessPosture> {
    match value {
        "partial" => Some(CompletenessPosture::Partial),
        "total" => Some(CompletenessPosture::Total),
        _ => None,
    }
}

fn density_posture(value: &str) -> Option<DensityPosture> {
    match value {
        "sparse" => Some(DensityPosture::Sparse),
        "dense" => Some(DensityPosture::Dense),
        _ => None,
    }
}

fn absence_posture(value: &str) -> Option<AbsencePosture> {
    match value {
        "allowed" => Some(AbsencePosture::Allowed),
        "refused" => Some(AbsencePosture::Refusal),
        _ => None,
    }
}

fn self_relation_posture(value: &str) -> Option<SelfRelationPosture> {
    match value {
        "allowed" => Some(SelfRelationPosture::Allowed),
        "refused" => Some(SelfRelationPosture::Refusal),
        _ => None,
    }
}

fn cycle_posture(value: &str) -> Option<CyclePosture> {
    match value {
        "allowed" => Some(CyclePosture::Allowed),
        "refused" => Some(CyclePosture::Refusal),
        _ => None,
    }
}

fn apply_postures(
    relations: &mut [CapturedRelation],
    postures: &[CapturedPosture],
) -> Result<(), RecipeError> {
    let mut seen = Vec::new();
    for posture in postures {
        if seen.iter().any(|name| name == &posture.relation) {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateRelationPosture {
                    relation: posture.relation.clone(),
                },
                Some(posture.at),
            ));
        }
        seen.push(posture.relation.clone());
        let Some(relation) = relations
            .iter_mut()
            .find(|relation| relation.name.spelling == posture.relation)
        else {
            return Err(RecipeError::at(
                RecipeIssue::RelationNotFound {
                    name: posture.relation.clone(),
                },
                Some(posture.at),
            ));
        };
        let mut requirements = relation.requirements;
        for clause in &posture.clauses {
            let (question, next) = apply_clause(requirements, clause.value);
            let Some(next) = next else {
                return Err(RecipeError::at(
                    RecipeIssue::DuplicateRelationQuestion {
                        relation: posture.relation.clone(),
                        question,
                    },
                    Some(clause.at),
                ));
            };
            requirements = next;
        }
        relation.requirements = requirements;
    }
    Ok(())
}

fn apply_clause(
    requirements: RecipeRelationRequirements,
    clause: PostureClause,
) -> (&'static str, Option<RecipeRelationRequirements>) {
    match clause {
        PostureClause::Empty(value) => ("empty", requirements.with_empty(value)),
        PostureClause::Repetition(value) => ("repetition", requirements.with_repetition(value)),
        PostureClause::Membership(left, right) => {
            ("membership", requirements.with_membership(left, right))
        }
        PostureClause::Completeness(left, right) => {
            ("completeness", requirements.with_completeness(left, right))
        }
        PostureClause::Density(value) => ("density", requirements.with_density(value)),
        PostureClause::Absence(value) => ("absence", requirements.with_absence(value)),
        PostureClause::SelfRelation(value) => {
            ("self_relation", requirements.with_self_relation(value))
        }
        PostureClause::Cycle(value) => ("cycle", requirements.with_cycle(value)),
    }
}

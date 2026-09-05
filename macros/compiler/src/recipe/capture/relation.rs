//! Generic vocabulary, relation-row, and structural-posture grammar.

use super::{
    CapturedName, CapturedRelation, RecipeError, RecipeIssue, RecipeRelationRequirements,
    RecipeRelationRow, grammar, identifier_token,
};
use crate::kind::roster_row;
use crate::recipe::RecipeRelationPayload;
use crate::relation::{
    AbsencePosture, CompletenessPosture, CyclePosture, DensityPosture, EmptyPosture,
    MembershipPosture, RelationQuestion, RepetitionPosture, SelfRelationPosture,
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
        None,
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
    let (token, spelling) = cursor.identifier()?;
    let Some(question) = roster_row(RelationQuestion::ALL, RelationQuestion::name, spelling) else {
        return Err(CaptureReadRefusal::projected(
            CaptureReadIssue::Unexpected(CaptureExpectation::Word(
                "a structural relation question".to_owned(),
            )),
            Some(token.span()),
        ));
    };
    let value = match question {
        RelationQuestion::Empty => {
            read_single(cursor, EmptyPosture::ALL, EmptyPosture::name).map(PostureClause::Empty)
        }
        RelationQuestion::Repetition => {
            read_single(cursor, RepetitionPosture::ALL, RepetitionPosture::name)
                .map(PostureClause::Repetition)
        }
        RelationQuestion::Membership => {
            read_pair(cursor, MembershipPosture::ALL, MembershipPosture::name)
                .map(|(left, right)| PostureClause::Membership(left, right))
        }
        RelationQuestion::Completeness => {
            read_pair(cursor, CompletenessPosture::ALL, CompletenessPosture::name)
                .map(|(left, right)| PostureClause::Completeness(left, right))
        }
        RelationQuestion::Density => read_single(cursor, DensityPosture::ALL, DensityPosture::name)
            .map(PostureClause::Density),
        RelationQuestion::Absence => read_single(cursor, AbsencePosture::ALL, AbsencePosture::name)
            .map(PostureClause::Absence),
        RelationQuestion::SelfRelation => {
            read_single(cursor, SelfRelationPosture::ALL, SelfRelationPosture::name)
                .map(PostureClause::SelfRelation)
        }
        RelationQuestion::Cycle => {
            read_single(cursor, CyclePosture::ALL, CyclePosture::name).map(PostureClause::Cycle)
        }
    }?;
    Ok(CapturedPostureClause {
        value,
        at: token.span(),
    })
}

fn read_single<T>(
    cursor: &mut CaptureCursor<'_>,
    roster: &[T],
    name: fn(T) -> &'static str,
) -> Result<T, CaptureReadRefusal>
where
    T: Copy,
{
    let mut answer = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (token, spelling) = answer.identifier()?;
    let value = roster_row(roster, name, spelling).ok_or_else(|| {
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
    roster: &[T],
    name: fn(T) -> &'static str,
) -> Result<(T, T), CaptureReadRefusal> {
    let mut answers = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (left_token, left) = answers.identifier()?;
    let left = roster_row(roster, name, left).ok_or_else(|| posture_word(left_token.span()))?;
    answers.punctuation(',', CapturedSpacing::Alone)?;
    let (right_token, right) = answers.identifier()?;
    let right = roster_row(roster, name, right).ok_or_else(|| posture_word(right_token.span()))?;
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
        PostureClause::Empty(value) => (
            RelationQuestion::Empty.name(),
            requirements.with_empty(value),
        ),
        PostureClause::Repetition(value) => (
            RelationQuestion::Repetition.name(),
            requirements.with_repetition(value),
        ),
        PostureClause::Membership(left, right) => (
            RelationQuestion::Membership.name(),
            requirements.with_membership(left, right),
        ),
        PostureClause::Completeness(left, right) => (
            RelationQuestion::Completeness.name(),
            requirements.with_completeness(left, right),
        ),
        PostureClause::Density(value) => (
            RelationQuestion::Density.name(),
            requirements.with_density(value),
        ),
        PostureClause::Absence(value) => (
            RelationQuestion::Absence.name(),
            requirements.with_absence(value),
        ),
        PostureClause::SelfRelation(value) => (
            RelationQuestion::SelfRelation.name(),
            requirements.with_self_relation(value),
        ),
        PostureClause::Cycle(value) => (
            RelationQuestion::Cycle.name(),
            requirements.with_cycle(value),
        ),
    }
}

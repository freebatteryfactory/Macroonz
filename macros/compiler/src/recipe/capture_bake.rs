//! The ordered bake declaration clauses for vocabularies, transitions, posture, projections, and evidence.

use super::codec::read_codecs;
use super::evidence::{evidence, read_evidence_block, read_support, support_matches_projections};
use super::projection::{projections, read_projection};
use super::relation::{read_and_apply_postures, read_relations, read_vocabularies};
use super::{
    BakeRead, CapturedName, CapturedRelation, HarnessPosture, RecipeError, RecipeIssue, grammar,
    identifier_token,
};
use crate::recipe::{RecipeRelationPayload, RecipeRelationRequirements, RecipeRelationRow};
use crate::relation::AbsencePosture;
use crate::token::{
    CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedFragment, CapturedSpacing,
};

/// Read the one ordered bake grammar from the bake group.
pub(super) fn read_bake(
    declaration: CapturedFragment<'_>,
    harness: HarnessPosture,
    issued: usize,
) -> Result<BakeRead, RecipeError> {
    let mut cursor = declaration.cursor();
    let vocabularies = if cursor.next_word() == Some("vocabularies") {
        read_vocabularies(&mut cursor)?
    } else {
        Vec::new()
    };
    let transition = if cursor.next_word() == Some("transitions") {
        Some(read_transition_relation(&mut cursor)?)
    } else {
        None
    };
    let transition_relation = transition
        .as_ref()
        .map(|relation| relation.name.spelling.clone());
    let mut relations = transition.into_iter().collect::<Vec<_>>();
    if cursor.next_word() == Some("relations") {
        relations.extend(read_relations(&mut cursor)?);
    }
    if transition_relation.is_some() || cursor.next_word() == Some("absence") {
        let absence_at = cursor
            .next_token()
            .map(crate::token::CapturedTokenTree::span);
        let absence = read_absence(&mut cursor)?;
        let Some(transition_relation_row) = relations
            .iter_mut()
            .find(|relation| transition_relation.as_deref() == Some(&relation.name.spelling))
        else {
            return Err(RecipeError::at(
                RecipeIssue::RelationNotFound {
                    name: "transitions".to_owned(),
                },
                absence_at,
            ));
        };
        transition_relation_row.requirements = RecipeRelationRequirements::transitions(absence);
    }
    if cursor.next_word() == Some("postures") {
        read_and_apply_postures(&mut cursor, &mut relations)?;
    }
    let codecs = read_codecs(&mut cursor)?;
    let requested = read_projections(&mut cursor, issued)?;
    let requested_evidence = read_evidence_block(&mut cursor, issued)?;
    let support = read_support(&mut cursor)?;
    cursor.finish().map_err(grammar)?;
    let transition_subject = transition_relation.as_deref().and_then(|name| {
        relations
            .iter()
            .find(|relation| relation.name.spelling == name)
            .map(|relation| {
                (
                    relation.left.spelling.as_str(),
                    relation.right.spelling.as_str(),
                )
            })
    });
    let projections = projections(
        &requested,
        &requested_evidence,
        harness,
        transition_subject,
        &relations,
    )?;
    let evidence = evidence(&requested_evidence);
    support_matches_projections(&projections, support.as_ref(), declaration.last_span())?;
    Ok(BakeRead {
        vocabularies,
        relations,
        transition_relation,
        codecs,
        projections,
        evidence,
        support,
    })
}

fn read_transition_relation(
    cursor: &mut CaptureCursor<'_>,
) -> Result<CapturedRelation, RecipeError> {
    let (name_token, name) = cursor.identifier().map_err(grammar)?;
    let mut vocabularies = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (left_token, left_name) = vocabularies.identifier().map_err(grammar)?;
    vocabularies
        .punctuation(',', CapturedSpacing::Alone)
        .map_err(grammar)?;
    let (right_token, right_name) = vocabularies.identifier().map_err(grammar)?;
    vocabularies.finish().map_err(grammar)?;
    let rows = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::TRANSITION_LIMIT }>(';', read_transition)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(CapturedRelation {
        name: CapturedName {
            spelling: name.to_owned(),
            token: identifier_token(name_token, name),
            at: name_token.span(),
        },
        left: CapturedName {
            spelling: left_name.to_owned(),
            token: identifier_token(left_token, left_name),
            at: left_token.span(),
        },
        right: CapturedName {
            spelling: right_name.to_owned(),
            token: identifier_token(right_token, right_name),
            at: right_token.span(),
        },
        rows,
        requirements: RecipeRelationRequirements::unspecified(),
    })
}

fn read_projections(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<Vec<super::RequestedProjection>, RecipeError> {
    if cursor.next_word() != Some("projections") {
        return Ok(Vec::new());
    }
    cursor.word("projections").map_err(grammar)?;
    let requested = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::PROJECTION_LIMIT }>(';', |projection| {
            read_projection(projection, issued)
        })
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(requested)
}

fn read_absence(cursor: &mut CaptureCursor<'_>) -> Result<AbsencePosture, RecipeError> {
    cursor.word("absence").map_err(grammar)?;
    let mut clause = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (token, name) = clause.identifier().map_err(grammar)?;
    let absence = match name {
        "allowed" => AbsencePosture::Allowed,
        "refused" => AbsencePosture::Refusal,
        _ => {
            return Err(RecipeError::at(
                RecipeIssue::Grammar(crate::token::CaptureReadIssue::Unexpected(
                    crate::token::CaptureExpectation::Word("allowed or refused".to_owned()),
                )),
                Some(token.span()),
            ));
        }
    };
    clause.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(absence)
}

/// Read one relation row and preserve its caller-owned execution material structurally.
fn read_transition(
    cursor: &mut CaptureCursor<'_>,
) -> Result<RecipeRelationRow, CaptureReadRefusal> {
    let mut endpoints = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (from_token, from) = endpoints.identifier()?;
    endpoints.punctuation(',', CapturedSpacing::Alone)?;
    let (event_token, event) = endpoints.identifier()?;
    endpoints.finish()?;
    cursor.fat_arrow()?;
    let (to_token, to) = cursor.identifier()?;
    cursor.word("with")?;
    let mut effect = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (effect, (binding, segments)) = effect.fragment(|path| {
        let (first, name) = path.identifier()?;
        let binding = identifier_token(first, name);
        let mut segments = 1usize;
        while !path.is_finished() {
            path.punctuation(':', CapturedSpacing::Joint)?;
            path.punctuation(':', CapturedSpacing::Alone)?;
            path.identifier()?;
            segments = segments.saturating_add(1);
        }
        Ok((binding, segments))
    })?;
    let effect = effect.generated().map_err(|refusal| {
        CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::CursorRangeContradiction,
            refusal.token(),
        )
    })?;
    let (payload, payload_at) = if let Some(body) = cursor
        .next_token()
        .and_then(|token| token.group_fragment(CapturedDelimiter::Brace))
    {
        let body_at = cursor.token()?.span();
        if segments != 1 {
            return Err(CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                    "one declared-target binding".to_owned(),
                )),
                Some(body_at),
            ));
        }
        let body = body.generated().map_err(|refusal| {
            CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::CursorRangeContradiction,
                refusal.token(),
            )
        })?;
        (
            RecipeRelationPayload::transition_exact(
                to.to_owned(),
                identifier_token(to_token, to),
                binding,
                body,
            ),
            body_at,
        )
    } else {
        (
            RecipeRelationPayload::transition(
                to.to_owned(),
                identifier_token(to_token, to),
                effect,
            ),
            to_token.span(),
        )
    };
    Ok(RecipeRelationRow::authored(
        (
            from.to_owned(),
            identifier_token(from_token, from),
            from_token.span(),
        ),
        (
            event.to_owned(),
            identifier_token(event_token, event),
            event_token.span(),
        ),
        payload,
        payload_at,
    ))
}

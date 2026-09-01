//! The ordered bake declaration clauses for vocabularies, transitions, posture, projections, and evidence.

use super::evidence::{evidence, read_evidence_block, read_support, support_matches_projections};
use super::projection::{projections, read_projection};
use super::{
    BakeRead, CapturedName, HarnessPosture, RecipeError, RecipeIssue, RecipeTransition,
    VocabularyNames, grammar, identifier_token,
};
use crate::relation::AbsencePosture;
use crate::token::{
    CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedFragment, CapturedSpacing,
};

/// Read the fixed first vertical-slice grammar from the bake group.
pub(super) fn read_bake(
    declaration: CapturedFragment<'_>,
    harness: HarnessPosture,
    issued: usize,
) -> Result<BakeRead, RecipeError> {
    let mut cursor = declaration.cursor();
    let VocabularyNames { states, events } = read_vocabularies(&mut cursor)?;

    cursor.word("transitions").map_err(grammar)?;
    let transitions = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::TRANSITION_LIMIT }>(';', read_transition)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;

    let absence = read_absence(&mut cursor)?;

    cursor.word("projections").map_err(grammar)?;
    let requested = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, 5>(';', |projection| read_projection(projection, issued))
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;

    let requested_evidence = read_evidence_block(&mut cursor, issued)?;
    let support = read_support(&mut cursor)?;
    cursor.finish().map_err(grammar)?;
    let projections = projections(
        &requested,
        &requested_evidence,
        harness,
        &states.spelling,
        &events.spelling,
    )?;
    let evidence = evidence(&requested_evidence);
    support_matches_projections(&projections, support.as_ref(), declaration.last_span())?;
    Ok(BakeRead {
        states,
        events,
        transitions,
        absence,
        projections,
        evidence,
        support,
    })
}

fn read_vocabularies(cursor: &mut CaptureCursor<'_>) -> Result<VocabularyNames, RecipeError> {
    cursor.word("vocabularies").map_err(grammar)?;
    let mut vocabularies = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (states_token, states_name) = vocabularies.identifier().map_err(grammar)?;
    vocabularies
        .punctuation(',', CapturedSpacing::Alone)
        .map_err(grammar)?;
    let (events_token, events_name) = vocabularies.identifier().map_err(grammar)?;
    let names = VocabularyNames {
        states: CapturedName {
            spelling: states_name.to_owned(),
            token: identifier_token(states_token, states_name),
        },
        events: CapturedName {
            spelling: events_name.to_owned(),
            token: identifier_token(events_token, events_name),
        },
    };
    vocabularies.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(names)
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

/// Read one relation row and preserve its effect path structurally.
fn read_transition(cursor: &mut CaptureCursor<'_>) -> Result<RecipeTransition, CaptureReadRefusal> {
    let mut endpoints = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (from_token, from) = endpoints.identifier()?;
    endpoints.punctuation(',', CapturedSpacing::Alone)?;
    let (event_token, event) = endpoints.identifier()?;
    endpoints.finish()?;
    cursor.fat_arrow()?;
    let (to_token, to) = cursor.identifier()?;
    cursor.word("with")?;
    let mut effect = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (effect, ()) = effect.fragment(|path| {
        path.identifier()?;
        while !path.is_finished() {
            path.punctuation(':', CapturedSpacing::Joint)?;
            path.punctuation(':', CapturedSpacing::Alone)?;
            path.identifier()?;
        }
        Ok(())
    })?;
    let effect = effect.generated().map_err(|refusal| {
        CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::CursorRangeContradiction,
            refusal.token(),
        )
    })?;
    Ok(RecipeTransition::authored(
        (from.to_owned(), identifier_token(from_token, from)),
        (event.to_owned(), identifier_token(event_token, event)),
        (to.to_owned(), identifier_token(to_token, to)),
        effect,
        from_token.span(),
    ))
}

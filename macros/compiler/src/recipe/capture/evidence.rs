//! Descriptor-native evidence clauses and authored support addresses.

use super::super::types::RecipeRoleEntrance;
use super::{EVIDENCE_LIMIT, RecipeError, RecipeIssue, RecipeRole, RequestedEvidence, grammar};
use crate::support::SupportName;
use crate::token::{
    CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedInput, CapturedSpacing,
};

pub(super) fn read_evidence_block(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<Vec<RequestedEvidence>, RecipeError> {
    if cursor.next_word() != Some("evidence") {
        return Ok(Vec::new());
    }
    cursor.word("evidence").map_err(grammar)?;
    let rows = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, EVIDENCE_LIMIT>(';', |row| read_evidence(row, issued))
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(rows)
}

fn read_evidence(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<RequestedEvidence, CaptureReadRefusal> {
    let (token, spelling) = cursor.identifier()?;
    let Some(role) = RecipeRole::from_syntax(spelling, RecipeRoleEntrance::Evidence) else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                "a descriptor-native evidence projection".to_owned(),
            )),
            Some(token.span()),
        ));
    };
    if cursor.next_word() == Some("unavailable") {
        cursor.word("unavailable")?;
        return Ok(RequestedEvidence {
            role,
            target: None,
            body: None,
            at: token.span(),
        });
    }
    let target = if role == RecipeRole::Mutation {
        let mut selected = cursor.group(CapturedDelimiter::Parenthesis)?;
        let (_target_token, target) = selected.identifier()?;
        selected.finish()?;
        Some(target.to_owned())
    } else {
        None
    };
    let group = cursor.token()?;
    let Some(fragment) = group.group_fragment(CapturedDelimiter::Brace) else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Group(
                CapturedDelimiter::Brace,
            )),
            Some(group.span()),
        ));
    };
    let body = CapturedInput::selected(fragment, issued).map_err(|_| {
        CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: crate::token::CAPTURED_TOKEN_LIMIT,
            },
            Some(group.span()),
        )
    })?;
    Ok(RequestedEvidence {
        role,
        target,
        body: Some(body),
        at: token.span(),
    })
}

pub(super) fn read_support(
    cursor: &mut CaptureCursor<'_>,
) -> Result<Option<SupportName>, RecipeError> {
    if cursor.is_finished() {
        return Ok(None);
    }
    cursor.word("support").map_err(grammar)?;
    let mut address = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (token, spelling) = address.identifier().map_err(grammar)?;
    let declared = SupportName::declared(spelling).map_err(|_| {
        RecipeError::at(
            RecipeIssue::GeneratedNameNotIdentifier {
                name: spelling.to_owned(),
            },
            Some(token.span()),
        )
    })?;
    address.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(Some(declared))
}

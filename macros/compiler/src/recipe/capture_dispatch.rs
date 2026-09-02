//! Exact caller-authored dispatch signatures and their two row-accounted bindings.

use super::{RecipeError, RecipeIssue, fragment_refusal, identifier_token};
use crate::token::{
    AuthoredItemKind, CapturedDelimiter, CapturedFragment, CapturedInput, CapturedTokenTree,
};

pub(super) fn exact_dispatch(
    input: &CapturedInput,
    at: crate::token::SpanHandle,
    transition_subject: Option<(&str, &str)>,
) -> Result<
    (
        String,
        crate::token::GeneratedTree,
        [crate::token::GeneratedToken; 2],
        [bool; 2],
    ),
    RecipeError,
> {
    exact_function(input, at, transition_subject, ExactFunctionSeat::Dispatch)
}

pub(super) fn exact_relation_table(
    input: &CapturedInput,
    at: crate::token::SpanHandle,
    subject: (&str, &str),
) -> Result<
    (
        String,
        crate::token::GeneratedTree,
        [crate::token::GeneratedToken; 2],
        [bool; 2],
    ),
    RecipeError,
> {
    exact_function(input, at, Some(subject), ExactFunctionSeat::RelationTable)
}

#[derive(Clone, Copy)]
enum ExactFunctionSeat {
    Dispatch,
    RelationTable,
}

fn exact_function(
    input: &CapturedInput,
    at: crate::token::SpanHandle,
    subject: Option<(&str, &str)>,
    seat: ExactFunctionSeat,
) -> Result<
    (
        String,
        crate::token::GeneratedTree,
        [crate::token::GeneratedToken; 2],
        [bool; 2],
    ),
    RecipeError,
> {
    let item = input
        .authored_item()
        .map_err(|refusal| RecipeError::at(seat.function_required(), refusal.token()))?;
    if item.kind() != AuthoredItemKind::Function {
        return Err(RecipeError::at(
            seat.function_required(),
            Some(item.kind_token().span()),
        ));
    }
    if let Some((_, body)) = item.body() {
        return Err(RecipeError::at(
            seat.body_refused(),
            body.enclosing_span().or_else(|| body.first_span()),
        ));
    }
    let Some((name_token, name)) = item.name() else {
        return Err(RecipeError::at(
            seat.function_required(),
            Some(item.kind_token().span()),
        ));
    };
    let parameters = exact_parameters(item, at, seat)?;
    let attributes = item
        .attributes()
        .generated()
        .map_err(|refusal| fragment_refusal(refusal.token()))?;
    let signature = item
        .signature()
        .generated()
        .map_err(|refusal| fragment_refusal(refusal.token()))?;
    let exact = attributes
        .joined(&signature)
        .map_err(|_| fragment_refusal(Some(name_token.span())))?;
    let imports = subject.map_or([false, false], |(left, right)| {
        [
            uses_unqualified_name(item.signature().tokens(), left),
            uses_unqualified_name(item.signature().tokens(), right),
        ]
    });
    Ok((name.to_owned(), exact, parameters, imports))
}

impl ExactFunctionSeat {
    const fn function_required(self) -> RecipeIssue {
        match self {
            Self::Dispatch => RecipeIssue::ExactDispatchFunctionRequired,
            Self::RelationTable => RecipeIssue::ExactRelationTableFunctionRequired,
        }
    }

    const fn body_refused(self) -> RecipeIssue {
        match self {
            Self::Dispatch => RecipeIssue::ExactDispatchBodyRefused,
            Self::RelationTable => RecipeIssue::ExactRelationTableBodyRefused,
        }
    }

    const fn parameter_count(self, observed: usize) -> RecipeIssue {
        match self {
            Self::Dispatch => RecipeIssue::ExactDispatchParameterCount { observed },
            Self::RelationTable => RecipeIssue::ExactRelationTableParameterCount { observed },
        }
    }

    const fn parameter_binding(self, position: usize) -> RecipeIssue {
        match self {
            Self::Dispatch => RecipeIssue::ExactDispatchParameterBinding { position },
            Self::RelationTable => RecipeIssue::ExactRelationTableParameterBinding { position },
        }
    }
}

fn uses_unqualified_name(tokens: &[CapturedTokenTree], sought: &str) -> bool {
    tokens.iter().enumerate().any(|(position, token)| {
        if let Some((_, members)) = token.group()
            && uses_unqualified_name(members, sought)
        {
            return true;
        }
        let spelling = token.word().or_else(|| token.raw_identifier());
        spelling == Some(sought)
            && !preceded_by_path_separator(tokens, position)
            && position
                .checked_sub(1)
                .and_then(|previous| tokens.get(previous))
                .and_then(CapturedTokenTree::word)
                != Some("fn")
    })
}

fn preceded_by_path_separator(tokens: &[CapturedTokenTree], position: usize) -> bool {
    let Some(first_colon) = position.checked_sub(2).and_then(|index| tokens.get(index)) else {
        return false;
    };
    let Some(second_colon) = position.checked_sub(1).and_then(|index| tokens.get(index)) else {
        return false;
    };
    first_colon.joint_punct() == Some(':') && second_colon.punct() == Some(':')
}

fn exact_parameters(
    item: crate::token::AuthoredItem<'_>,
    at: crate::token::SpanHandle,
    seat: ExactFunctionSeat,
) -> Result<[crate::token::GeneratedToken; 2], RecipeError> {
    let Some(parameters) = function_parameters(item) else {
        return Err(RecipeError::at(seat.parameter_count(0), Some(at)));
    };
    let rows = comma_rows(parameters.tokens());
    let [first, second] = rows.as_slice() else {
        return Err(RecipeError::at(
            seat.parameter_count(rows.len()),
            parameters
                .enclosing_span()
                .or_else(|| parameters.first_span()),
        ));
    };
    Ok([
        simple_binding(first, 1, at, seat)?,
        simple_binding(second, 2, at, seat)?,
    ])
}

fn function_parameters(item: crate::token::AuthoredItem<'_>) -> Option<CapturedFragment<'_>> {
    let signature = item.signature().tokens();
    let (name, _) = item.name()?;
    let after_name = signature
        .iter()
        .position(|token| core::ptr::eq(token, name))?
        .checked_add(1)?;
    let start = item.generics().map_or(after_name, |generics| {
        generics
            .tokens()
            .last()
            .and_then(|last| {
                signature
                    .iter()
                    .position(|token| core::ptr::eq(token, last))
            })
            .and_then(|position| position.checked_add(1))
            .unwrap_or(after_name)
    });
    signature
        .get(start)
        .and_then(|token| token.group_fragment(CapturedDelimiter::Parenthesis))
}

fn comma_rows(tokens: &[CapturedTokenTree]) -> Vec<&[CapturedTokenTree]> {
    let mut rows = Vec::new();
    let mut opening = 0usize;
    let mut angle_depth = 0usize;
    for (position, token) in tokens.iter().enumerate() {
        match token.punct().or_else(|| token.joint_punct()) {
            Some('<') => angle_depth = angle_depth.saturating_add(1),
            Some('>') if !thin_arrow_close(tokens, position) => {
                angle_depth = angle_depth.saturating_sub(1);
            }
            Some(',') if angle_depth == 0 => {
                push_comma_row(&mut rows, tokens, opening, position);
                opening = position.saturating_add(1);
            }
            Some(_) | None => {}
        }
    }
    push_comma_row(&mut rows, tokens, opening, tokens.len());
    rows
}

fn thin_arrow_close(tokens: &[CapturedTokenTree], position: usize) -> bool {
    position
        .checked_sub(1)
        .and_then(|previous| tokens.get(previous))
        .and_then(CapturedTokenTree::joint_punct)
        == Some('-')
}

fn push_comma_row<'tokens>(
    rows: &mut Vec<&'tokens [CapturedTokenTree]>,
    tokens: &'tokens [CapturedTokenTree],
    opening: usize,
    closing: usize,
) {
    let Some(row) = (opening < closing)
        .then(|| tokens.get(opening..closing))
        .flatten()
    else {
        return;
    };
    rows.push(row);
}

fn simple_binding(
    row: &[CapturedTokenTree],
    position: usize,
    at: crate::token::SpanHandle,
    seat: ExactFunctionSeat,
) -> Result<crate::token::GeneratedToken, RecipeError> {
    let [binding, colon, rest @ ..] = row else {
        return Err(simple_binding_refusal(row, position, at, seat));
    };
    let Some(spelling) = binding.word().or_else(|| binding.raw_identifier()) else {
        return Err(simple_binding_refusal(row, position, at, seat));
    };
    if colon.punct() != Some(':') || rest.is_empty() {
        return Err(simple_binding_refusal(row, position, at, seat));
    }
    Ok(identifier_token(binding, spelling))
}

fn simple_binding_refusal(
    row: &[CapturedTokenTree],
    position: usize,
    at: crate::token::SpanHandle,
    seat: ExactFunctionSeat,
) -> RecipeError {
    RecipeError::at(
        seat.parameter_binding(position),
        row.first().map(CapturedTokenTree::span).or(Some(at)),
    )
}

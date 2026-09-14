//! Exact caller bindings for the configured consuming typestate seat.

use super::{fragment_refusal, grammar, identifier_token};
use crate::bounded::Bounded;
use crate::recipe::types::{ConsumingParts, RecipeError};
use crate::recipe::{ConsumingMethod, ConsumingParameter, ConsumingProjection, VOCABULARY_LIMIT};
use crate::token::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedDelimiter,
    CapturedInput, CapturedSpacing, GeneratedTree,
};

pub(super) fn read(input: &CapturedInput) -> Result<ConsumingProjection, RecipeError> {
    let mut cursor = input.cursor();
    cursor.word("wrapper").map_err(grammar)?;
    let mut wrapper = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (token, name) = wrapper.identifier().map_err(grammar)?;
    wrapper.finish().map_err(grammar)?;
    semicolon(&mut cursor).map_err(grammar)?;
    let [parameters, arguments, predicates] = if cursor.next_word() == Some("generics") {
        generic_parts(&mut cursor).map_err(grammar)?
    } else {
        [
            Bounded::from_array([]),
            Bounded::from_array([]),
            Bounded::from_array([]),
        ]
    };
    let resource = parameter_clause(&mut cursor, "resource").map_err(grammar)?;
    let runtime = parameter_clause(&mut cursor, "runtime").map_err(grammar)?;
    let refusal = material_clause(&mut cursor, "refusal").map_err(grammar)?;
    let validator = material_clause(&mut cursor, "validate").map_err(grammar)?;
    cursor.word("methods").map_err(grammar)?;
    let methods = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, VOCABULARY_LIMIT>(';', method)
        .map_err(grammar)?;
    semicolon(&mut cursor).map_err(grammar)?;
    cursor.finish().map_err(grammar)?;
    ConsumingProjection::informed(ConsumingParts {
        wrapper: name.to_owned(),
        wrapper_token: identifier_token(token, name),
        parameters,
        arguments,
        predicates,
        resource,
        runtime,
        refusal,
        validator,
        methods,
        at: token.span(),
    })
}

fn parameter_clause(
    cursor: &mut CaptureCursor<'_>,
    word: &str,
) -> Result<ConsumingParameter, CaptureReadRefusal> {
    cursor.word(word)?;
    let mut group = cursor.group(CapturedDelimiter::Parenthesis)?;
    let parameter = parameter(&mut group)?;
    group.finish()?;
    semicolon(cursor)?;
    Ok(parameter)
}

fn parameter(cursor: &mut CaptureCursor<'_>) -> Result<ConsumingParameter, CaptureReadRefusal> {
    let (token, name) = cursor.identifier()?;
    cursor.punctuation(':', CapturedSpacing::Alone)?;
    let kind = material(cursor)?;
    Ok(ConsumingParameter::captured(
        name.to_owned(),
        identifier_token(token, name),
        kind,
        token.span(),
    ))
}

fn material_clause(
    cursor: &mut CaptureCursor<'_>,
    word: &str,
) -> Result<GeneratedTree, CaptureReadRefusal> {
    cursor.word(word)?;
    let mut group = cursor.group(CapturedDelimiter::Parenthesis)?;
    let tree = material(&mut group)?;
    group.finish()?;
    semicolon(cursor)?;
    Ok(tree)
}

fn material(cursor: &mut CaptureCursor<'_>) -> Result<GeneratedTree, CaptureReadRefusal> {
    if cursor.is_finished() {
        return Err(CaptureReadRefusal::projected(
            CaptureReadIssue::Missing(CaptureExpectation::Word(
                "nonempty caller Rust material".to_owned(),
            )),
            None,
        ));
    }
    let (fragment, ()) = cursor.fragment(|tokens| {
        while !tokens.is_finished() {
            tokens.token()?;
        }
        Ok(())
    })?;
    fragment.generated().map_err(|refusal| {
        let error = fragment_refusal(refusal.token());
        CaptureReadRefusal::projected(CaptureReadIssue::CursorRangeContradiction, error.token())
    })
}

fn method(cursor: &mut CaptureCursor<'_>) -> Result<ConsumingMethod, CaptureReadRefusal> {
    let (_, event) = cursor.identifier()?;
    cursor.fat_arrow()?;
    let (token, name) = cursor.identifier()?;
    let mut payload = cursor.group(CapturedDelimiter::Parenthesis)?;
    let payload = if payload.is_finished() {
        None
    } else {
        Some(parameter(&mut payload)?)
    };
    Ok(ConsumingMethod::captured(
        event.to_owned(),
        name.to_owned(),
        identifier_token(token, name),
        payload,
        token.span(),
    ))
}

fn generic_parts(
    cursor: &mut CaptureCursor<'_>,
) -> Result<[Bounded<GeneratedTree, VOCABULARY_LIMIT>; 3], CaptureReadRefusal> {
    cursor.word("generics")?;
    let mut group = cursor.group(CapturedDelimiter::Brace)?;
    let parameters = generic_rows(&mut group, "parameters")?;
    let arguments = generic_rows(&mut group, "arguments")?;
    let predicates = generic_rows(&mut group, "predicates")?;
    group.finish()?;
    semicolon(cursor)?;
    Ok([parameters, arguments, predicates])
}

fn generic_rows(
    cursor: &mut CaptureCursor<'_>,
    word: &str,
) -> Result<Bounded<GeneratedTree, VOCABULARY_LIMIT>, CaptureReadRefusal> {
    cursor.word(word)?;
    let rows = cursor
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<_, VOCABULARY_LIMIT>(';', |row| {
            let mut group = row.group(CapturedDelimiter::Parenthesis)?;
            material(&mut group)
        })?;
    semicolon(cursor)?;
    Ok(rows)
}

fn semicolon(cursor: &mut CaptureCursor<'_>) -> Result<(), CaptureReadRefusal> {
    cursor.punctuation(';', CapturedSpacing::Alone).map(|_| ())
}

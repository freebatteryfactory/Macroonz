//! Mechanical clause readings over already captured tokens.

use super::direct::comma_groups;
use super::types::{Assignment, Clause};
use crate::bounded::first_duplicate_position;
use crate::descriptor::{CaptureCause, DeclarationError, Grammar, Name};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};
use core::convert::Infallible;

/// One grammar-specific projection of a mechanical capture cause.
type Refused<Error> = fn(Grammar, CaptureCause, SpanHandle) -> Error;

/// One grammar-specific projection of a descriptor-vocabulary refusal.
type VocabularyRefused<Error> = fn(Grammar, DeclarationError, SpanHandle) -> Error;

/// One concrete grammar's reader for its nested clause shape.
type NestedReader<'trees, Nested, Error> =
    fn(Grammar, &[&'trees CapturedTokenTree]) -> Result<Option<Nested>, Error>;

/// Read one declaration's assignments and grammar-owned nested clauses.
pub(crate) fn declaration_clauses<'trees, Nested, Error>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
    declarable: &[&str],
    nested: NestedReader<'trees, Nested, Error>,
    refused: Refused<Error>,
) -> Result<Vec<Clause<'trees, Nested>>, Error> {
    let mut clauses = Vec::new();
    for group in comma_groups(grammar, body.iter().copied(), refused)? {
        if let Some(read) = nested(grammar, &group)? {
            clauses.push(Clause::nested(read));
        } else {
            clauses.push(Clause::assigned(assignment(
                grammar, &group, declarable, refused,
            )?));
        }
    }
    distinct(grammar, &clauses, refused)?;
    Ok(clauses)
}

/// Read one body containing assignments alone.
pub(crate) fn assignment_clauses<'trees, Error>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
    declarable: &[&str],
    refused: Refused<Error>,
) -> Result<Vec<Clause<'trees, Infallible>>, Error> {
    let mut clauses = Vec::new();
    for group in comma_groups(grammar, body.iter().copied(), refused)? {
        clauses.push(Clause::assigned(assignment(
            grammar, &group, declarable, refused,
        )?));
    }
    distinct(grammar, &clauses, refused)?;
    Ok(clauses)
}

/// Read one `<key> = <value>` assignment admitted by the concrete grammar's roster.
fn assignment<'trees, Error>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
    declarable: &[&str],
    refused: Refused<Error>,
) -> Result<Assignment<'trees>, Error> {
    let Some((head, rest)) = group.split_first() else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            SpanHandle::at(0),
        ));
    };
    let at = head.span();
    let Some(key) = head.word() else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, at));
    };
    let Some((assigned_by, value)) = rest.split_first() else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, at));
    };
    if assigned_by.punct() != Some('=') || value.is_empty() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            assigned_by.span(),
        ));
    }
    if !declarable.contains(&key) {
        return Err(refused(grammar, CaptureCause::ClauseUndeclared, at));
    }
    Ok(Assignment::admitted(key, value.to_vec(), at))
}

/// Refuse the first repeated assignment key while ignoring grammar-owned nested clauses.
fn distinct<Nested, Error>(
    grammar: Grammar,
    clauses: &[Clause<'_, Nested>],
    refused: Refused<Error>,
) -> Result<(), Error> {
    let assignments = clauses
        .iter()
        .filter_map(Clause::assignment)
        .collect::<Vec<_>>();
    let Some(position) =
        first_duplicate_position(&assignments, |left, right| left.key() == right.key())
    else {
        return Ok(());
    };
    let Some(repeated) = assignments.get(position) else {
        return Ok(());
    };
    Err(refused(grammar, CaptureCause::ClauseDoubled, repeated.at()))
}

/// The value tokens one assigned clause carries, and the token its key sits at.
pub(crate) fn assigned<'trees, 'clauses, Nested>(
    clauses: &'clauses [Clause<'trees, Nested>],
    key: &str,
) -> Option<(&'clauses [&'trees CapturedTokenTree], SpanHandle)> {
    clauses.iter().find_map(|clause| {
        let assignment = clause.assignment()?;
        (assignment.key() == key).then(|| (assignment.value(), assignment.at()))
    })
}

/// Read the one identifier a required assignment carries.
pub(crate) fn identifier<'trees, Nested, Error>(
    grammar: Grammar,
    clauses: &[Clause<'trees, Nested>],
    key: &str,
    at: SpanHandle,
    refused: Refused<Error>,
) -> Result<&'trees str, Error> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, clause));
    };
    (*only)
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))
}

/// Read one assigned `named(<namespace>, <stem>)` reference.
pub(crate) fn named_reference<Nested, Error>(
    grammar: Grammar,
    clauses: &[Clause<'_, Nested>],
    key: &str,
    at: SpanHandle,
    refused: Refused<Error>,
    carried: VocabularyRefused<Error>,
) -> Result<Name, Error> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    named_value(grammar, value, clause, refused, carried)
}

/// Read one `named(<namespace>, <stem>)` reference from its exact captured tokens.
pub(crate) fn named_value<Error>(
    grammar: Grammar,
    value: &[&CapturedTokenTree],
    at: SpanHandle,
    refused: Refused<Error>,
    carried: VocabularyRefused<Error>,
) -> Result<Name, Error> {
    let [word, arguments] = value else {
        return Err(refused(grammar, CaptureCause::ReferenceUnread, at));
    };
    if word.word() != Some("named") {
        return Err(refused(grammar, CaptureCause::ReferenceUnread, word.span()));
    }
    let Some((CapturedDelimiter::Parenthesis, inner)) = arguments.group() else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    let parts = inner.iter().collect::<Vec<_>>();
    let [namespace, separator, stem] = parts.as_slice() else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    if separator.punct() != Some(',') {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            separator.span(),
        ));
    }
    let (Some(owner), Some(spelling)) = (namespace.text(), stem.text()) else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    Name::named(owner, spelling).map_err(|error| carried(grammar, error, arguments.span()))
}

/// Read one unsuffixed decimal assignment at the receiving seat's width.
pub(crate) fn number<Number, Nested, Error>(
    grammar: Grammar,
    clauses: &[Clause<'_, Nested>],
    key: &str,
    at: SpanHandle,
    refused: Refused<Error>,
) -> Result<Number, Error>
where
    Number: core::str::FromStr,
{
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, clause));
    };
    let spelling = only
        .number()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))?;
    if !spelling.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(refused(grammar, CaptureCause::ClauseUnread, only.span()));
    }
    spelling
        .parse::<Number>()
        .map_err(|_| refused(grammar, CaptureCause::NumberBeyondSeat, only.span()))
}

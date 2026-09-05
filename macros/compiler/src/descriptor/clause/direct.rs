//! Mechanical readings for direct descriptor clauses.

use crate::descriptor::{CaptureCause, CaptureIssue, DirectBinding, Grammar};
use crate::token::{CapturedTokenTree, SpanHandle, rendered_identifier, rust_keyword};

/// One grammar-specific projection of a mechanical capture cause.
type Refused<Error> = fn(Grammar, CaptureCause, SpanHandle) -> Error;

/// One grammar-specific projection of a physical-binding refusal.
type BindingRefused<Error> = fn(Grammar, CaptureIssue, SpanHandle) -> Error;

/// One primitive value reader under a grammar's concrete refusal family.
type Reader<Value, Error> =
    fn(Grammar, &[&CapturedTokenTree], Refused<Error>) -> Result<Value, Error>;

/// Cut tokens into comma-separated groups, refusing a separator that separates nothing.
///
/// A trailing comma is lawful and contributes no empty group.
pub(crate) fn comma_groups<'trees, Error>(
    grammar: Grammar,
    trees: impl IntoIterator<Item = &'trees CapturedTokenTree>,
    refused: Refused<Error>,
) -> Result<Vec<Vec<&'trees CapturedTokenTree>>, Error> {
    let mut groups: Vec<Vec<&CapturedTokenTree>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in trees {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            groups.push(core::mem::take(&mut group));
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }
    Ok(groups)
}

/// The token one group opens at, or the declaration opening where none exists.
pub(crate) fn opening(group: &[&CapturedTokenTree]) -> SpanHandle {
    group.first().map_or(SpanHandle::at(0), |tree| tree.span())
}

/// The value trees after one clause's `<key> =` opening.
pub(crate) fn value_of<'group, 'trees>(
    group: &'group [&'trees CapturedTokenTree],
) -> &'group [&'trees CapturedTokenTree] {
    match group {
        [_key, assigned_by, value @ ..] if assigned_by.punct() == Some('=') => value,
        _malformed => &[],
    }
}

/// Read one clause into its empty seat, refusing a doubled key.
pub(crate) fn fill_once<Value, Error>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<Value>,
    read: Reader<Value, Error>,
    refused: Refused<Error>,
) -> Result<(), Error> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    *seat = Some(read(grammar, group, refused)?);
    Ok(())
}

/// Read one direct dependency binding into its empty seat.
pub(crate) fn binding_once<Error>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<DirectBinding>,
    refused: Refused<Error>,
    binding_refused: BindingRefused<Error>,
) -> Result<(), Error> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    let binding = super::super::binding::direct_binding(value_of(group))
        .map_err(|(issue, at)| binding_refused(grammar, issue, at))?;
    *seat = Some(binding);
    Ok(())
}

/// Read the one non-keyword identifier assigned by a clause.
pub(crate) fn assigned_identifier<Error>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    refused: Refused<Error>,
) -> Result<String, Error> {
    let [value] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    let word = value
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, value.span()))?;
    if !rendered_identifier(word) {
        return Err(refused(grammar, CaptureCause::ClauseUnread, value.span()));
    }
    if rust_keyword(word) {
        return Err(refused(grammar, CaptureCause::NameReserved, value.span()));
    }
    Ok(word.to_owned())
}

/// Read the one text literal assigned by a clause.
pub(crate) fn assigned_text<Error>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    refused: Refused<Error>,
) -> Result<String, Error> {
    let [value] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    value
        .text()
        .map(str::to_owned)
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, value.span()))
}

/// Read the one unsigned number assigned by a clause at the receiving seat's width.
pub(crate) fn assigned_number<Number, Error>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    refused: Refused<Error>,
) -> Result<Number, Error>
where
    Number: core::str::FromStr,
{
    let [value] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    let digits = value
        .number()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, value.span()))?;
    digits
        .parse::<Number>()
        .map_err(|_beyond| refused(grammar, CaptureCause::NumberBeyondSeat, value.span()))
}

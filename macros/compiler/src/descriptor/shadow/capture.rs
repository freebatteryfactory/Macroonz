//! Reading one authored shadow declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! <helper>! {
//!     loom = <dependency path>,
//!     names = [<name>, <name>, ...],
//! }
//! ```
//!
//! The Loom path is the adopter's physical binding and each name is one choice from the stated adapter roster.
//! A trailing comma is lawful, declaration order is meaning, and every owned token is consumed.

use super::{SHADOW_ROSTER, ShadowCaptureError, ShadowRow, Shadows};
use crate::descriptor::{CaptureCause, DirectBinding, Grammar};
use crate::token::{CapturedDelimiter, CapturedInput, CapturedTokenTree, SpanHandle};

/// Read one shadow payload out of the declaration's body.
///
/// # Errors
///
/// Returns [`ShadowCaptureError`] where the binding is absent or unreadable, a clause is unknown or doubled, a separator separates nothing, a choice is not one bare name, a name is outside the roster or chosen twice, or the declaration chooses nothing.
pub fn chosen(body: &CapturedInput, grammar: Grammar) -> Result<Shadows, ShadowCaptureError> {
    let groups = comma_groups(grammar, body.trees())?;
    let mut loom: Option<DirectBinding> = None;
    let mut rows: Option<Vec<ShadowRow>> = None;
    for group in &groups {
        match group.first().and_then(|tree| tree.word()) {
            Some("loom") => binding_once(grammar, group, &mut loom)?,
            Some("names") => names_once(grammar, group, &mut rows)?,
            Some(_) => {
                return Err(refused(
                    grammar,
                    CaptureCause::ClauseUndeclared,
                    opening(group),
                ));
            }
            None => return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group))),
        }
    }
    let Some(loom) = loom else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    let Some(rows) = rows else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    Ok(Shadows::declared(loom, rows))
}

/// Cut one body into comma-separated clauses, retaining a trailing comma as ordinary Rust.
fn comma_groups(
    grammar: Grammar,
    trees: &[CapturedTokenTree],
) -> Result<Vec<Vec<&CapturedTokenTree>>, ShadowCaptureError> {
    let mut groups = Vec::new();
    let mut group = Vec::new();
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

/// The token one clause opens at, or the declaration opening where none exists.
fn opening(group: &[&CapturedTokenTree]) -> SpanHandle {
    group.first().map_or(SpanHandle::at(0), |tree| tree.span())
}

/// The value trees after one clause's `name =` opening.
fn value_of<'group, 'trees>(
    group: &'group [&'trees CapturedTokenTree],
) -> &'group [&'trees CapturedTokenTree] {
    match group {
        [_key, assigned_by, value @ ..] if assigned_by.punct() == Some('=') => value,
        _malformed => &[],
    }
}

/// Read the Loom dependency path into its one seat.
fn binding_once(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<DirectBinding>,
) -> Result<(), ShadowCaptureError> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    let binding = crate::descriptor::binding::direct_binding(value_of(group))
        .map_err(|(issue, at)| ShadowCaptureError::binding_refused(grammar, issue, at))?;
    *seat = Some(binding);
    Ok(())
}

/// Read the chosen-name roster into its one seat.
fn names_once(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<Vec<ShadowRow>>,
) -> Result<(), ShadowCaptureError> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    let [roster] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::RosterUnread, opening(group)));
    };
    let Some((CapturedDelimiter::Bracket, members)) = roster.group() else {
        return Err(refused(grammar, CaptureCause::RosterUnread, roster.span()));
    };
    let mut rows = Vec::new();
    let mut expecting_name = true;
    for tree in members {
        if expecting_name {
            let Some(word) = tree.word() else {
                return Err(refused(grammar, CaptureCause::ChoiceUnread, tree.span()));
            };
            let Some(row) = SHADOW_ROSTER.iter().find(|row| row.name() == word) else {
                return Err(refused(grammar, CaptureCause::NameUnshadowed, tree.span()));
            };
            if rows.iter().any(|held: &ShadowRow| held.name() == word) {
                return Err(refused(grammar, CaptureCause::ChoiceDoubled, tree.span()));
            }
            rows.push(*row);
            expecting_name = false;
        } else if tree.punct() == Some(',') {
            expecting_name = true;
        } else {
            return Err(refused(grammar, CaptureCause::ChoiceUnread, tree.span()));
        }
    }
    if rows.is_empty() {
        return Err(refused(grammar, CaptureCause::NothingChosen, roster.span()));
    }
    *seat = Some(rows);
    Ok(())
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> ShadowCaptureError {
    ShadowCaptureError::grammar_refused(grammar, cause, at)
}

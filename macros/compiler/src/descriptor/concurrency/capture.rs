//! Reading one authored concurrency declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! <helper>! {
//!     harness = <dependency path>,
//!     module = <ident>,
//!     namespace = "<owner>",
//!     <row name> {
//!         population = "<stem>",
//!         interleavings = <n>,
//!         samples = <n>,
//!         seed = <n>,
//!     },
//! }
//! ```
//!
//! Clause order is free and is read by key, inside a row and outside one; row order is meaning and is preserved.

use super::{ConcurrencyCaptureError, ConcurrencyDeclaration, ExplorationRow};
use crate::descriptor::clause::{
    assigned_identifier, assigned_number, assigned_text, binding_once, comma_groups, fill_once,
    opening,
};
use crate::descriptor::{CaptureCause, DirectBinding, Grammar};
use crate::token::{
    CapturedDelimiter, CapturedInput, CapturedTokenTree, SpanHandle, rendered_identifier,
    rust_keyword,
};

/// Read one concurrency payload out of the declaration's body.
///
/// # Errors
///
/// Returns [`ConcurrencyCaptureError`] where the tokens do not say a concurrency declaration — an absent or unreadable binding, an unreadable clause, an undeclared key, a doubled key or row, a separator separating nothing, a row missing one of its four facts, a number past its seat's width, a name the language reserves, a declaration with no row at all — each at the token it was established at.
pub fn declared(
    body: &CapturedInput,
    grammar: Grammar,
) -> Result<ConcurrencyDeclaration, ConcurrencyCaptureError> {
    let groups = comma_groups(grammar, body.trees(), refused)?;
    let mut harness: Option<DirectBinding> = None;
    let mut module: Option<String> = None;
    let mut namespace: Option<String> = None;
    let mut rows: Vec<ExplorationRow> = Vec::new();
    for group in &groups {
        match group.as_slice() {
            [key, ..] if key.word() == Some("harness") => {
                binding_once(
                    grammar,
                    group,
                    &mut harness,
                    refused,
                    ConcurrencyCaptureError::binding_refused,
                )?;
            }
            [key, ..] if key.word() == Some("module") => {
                fill_once(grammar, group, &mut module, assigned_identifier, refused)?;
            }
            [key, ..] if key.word() == Some("namespace") => {
                fill_once(grammar, group, &mut namespace, assigned_text, refused)?;
            }
            [name, row] if name.word().is_some() && row.group().is_some() => {
                let read = row_of(grammar, name, row)?;
                if rows.iter().any(|held| held.name() == read.name()) {
                    return Err(refused(grammar, CaptureCause::ChoiceDoubled, name.span()));
                }
                rows.push(read);
            }
            _unread => {
                return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
            }
        }
    }
    let Some(harness) = harness else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    let Some(module) = module else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    let Some(namespace) = namespace else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    if rows.is_empty() {
        return Err(refused(
            grammar,
            CaptureCause::NothingChosen,
            SpanHandle::at(0),
        ));
    }
    Ok(ConcurrencyDeclaration::read(
        harness, module, namespace, rows,
    ))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> ConcurrencyCaptureError {
    ConcurrencyCaptureError::grammar_refused(grammar, cause, at)
}

/// Read one `<name> { clauses }` row.
fn row_of(
    grammar: Grammar,
    name: &CapturedTokenTree,
    row: &CapturedTokenTree,
) -> Result<ExplorationRow, ConcurrencyCaptureError> {
    let Some((CapturedDelimiter::Brace, members)) = row.group() else {
        return Err(refused(grammar, CaptureCause::RowUnread, row.span()));
    };
    let mut population: Option<String> = None;
    let mut interleavings: Option<u32> = None;
    let mut samples: Option<u32> = None;
    let mut seed: Option<u64> = None;
    for group in &comma_groups(grammar, members, refused)? {
        match group.first().and_then(|tree| tree.word()) {
            Some("population") => {
                fill_once(grammar, group, &mut population, assigned_text, refused)?;
            }
            Some("interleavings") => {
                fill_once(grammar, group, &mut interleavings, assigned_number, refused)?;
            }
            Some("samples") => {
                fill_once(grammar, group, &mut samples, assigned_number, refused)?;
            }
            Some("seed") => fill_once(grammar, group, &mut seed, assigned_number, refused)?,
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
    let (Some(population), Some(interleavings), Some(samples), Some(seed)) =
        (population, interleavings, samples, seed)
    else {
        return Err(refused(grammar, CaptureCause::ClauseAbsent, name.span()));
    };
    let Some(spelling) = name.word() else {
        return Err(refused(grammar, CaptureCause::RowUnread, name.span()));
    };
    if !rendered_identifier(spelling) {
        return Err(refused(grammar, CaptureCause::RowUnread, name.span()));
    }
    if rust_keyword(spelling) {
        return Err(refused(grammar, CaptureCause::NameReserved, name.span()));
    }
    Ok(ExplorationRow::declared(
        spelling.to_owned(),
        population,
        interleavings,
        samples,
        seed,
    ))
}

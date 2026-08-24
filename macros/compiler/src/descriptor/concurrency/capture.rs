//! Reading one authored concurrency declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! <helper>! {
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
use crate::descriptor::{CaptureCause, Grammar};
use crate::token::{
    CapturedDelimiter, CapturedInput, CapturedTokenTree, SpanHandle, rendered_identifier,
    rust_keyword,
};

/// Read one concurrency payload out of the declaration's body.
///
/// # Errors
///
/// Returns [`ConcurrencyCaptureError`] where the tokens do not say a concurrency declaration — an unreadable clause, an undeclared key, a doubled key or row, a separator separating nothing, a row missing one of its four facts, a number past its seat's width, a name the language reserves, a declaration with no row at all — each at the token it was established at.
pub fn declared(
    body: &CapturedInput,
    grammar: Grammar,
) -> Result<ConcurrencyDeclaration, ConcurrencyCaptureError> {
    let groups = comma_groups(grammar, body.trees())?;
    let mut module: Option<String> = None;
    let mut namespace: Option<String> = None;
    let mut rows: Vec<ExplorationRow> = Vec::new();
    for group in &groups {
        match group.as_slice() {
            [key, ..] if key.word() == Some("module") => {
                assigned_once(grammar, group, &mut module, assigned_ident)?;
            }
            [key, ..] if key.word() == Some("namespace") => {
                assigned_once(grammar, group, &mut namespace, assigned_text)?;
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
    Ok(ConcurrencyDeclaration::read(module, namespace, rows))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> ConcurrencyCaptureError {
    ConcurrencyCaptureError::grammar_refused(grammar, cause, at)
}

/// Cut one body into its comma-separated groups, refusing a separator that separates nothing.
///
/// A trailing comma after the last group is ordinary Rust and lawful; a leading or doubled comma makes an empty group this reader would otherwise silently drop, so it refuses at the comma's own token.
///
/// # Errors
///
/// Returns [`CaptureCause::SeparatorDangling`] at the comma standing where no clause does.
fn comma_groups(
    grammar: Grammar,
    trees: &[CapturedTokenTree],
) -> Result<Vec<Vec<&CapturedTokenTree>>, ConcurrencyCaptureError> {
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

/// The token one group opens at, or the declaration's own opening for an empty one.
fn opening(group: &[&CapturedTokenTree]) -> SpanHandle {
    group.first().map_or(SpanHandle::at(0), |tree| tree.span())
}

/// Read one `<key> = <value>` clause into its empty seat, refusing a doubled key.
fn assigned_once(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<String>,
    read: fn(Grammar, &[&CapturedTokenTree]) -> Result<String, ConcurrencyCaptureError>,
) -> Result<(), ConcurrencyCaptureError> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    *seat = Some(read(grammar, group)?);
    Ok(())
}

/// The one identifier a `<key> = <ident>` clause assigns, refused where the language already owns the spelling.
///
/// The value becomes a rendered item's own name, so a Rust keyword here would compile into a collision inside the adopter's build — where the expansion's lints are silenced — instead of refusing at the authored token.
fn assigned_ident(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<String, ConcurrencyCaptureError> {
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

/// The one text literal a `<key> = "<text>"` clause assigns.
fn assigned_text(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<String, ConcurrencyCaptureError> {
    let [value] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    value
        .text()
        .map(str::to_owned)
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, value.span()))
}

/// The one unsigned number a `<key> = <n>` clause assigns, parsed at exactly the width of the seat it fills.
///
/// The width is the harness seat's own — the exhaustive ceiling and the sample count are thirty-two bits wide, a seed is sixty-four — and a number past it refuses HERE, at the authored token.
/// Generated code cannot outsource this range to rustc: the overflowing-literal diagnostic is suppressed inside a foreign macro expansion, and an out-of-range literal wraps silently where source-authored Rust would refuse.
fn assigned_number<Number: core::str::FromStr>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<Number, ConcurrencyCaptureError> {
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

/// The value trees past one group's `<key> =` opening, or nothing where no `=` stands second.
fn value_of<'group, 'trees>(
    group: &'group [&'trees CapturedTokenTree],
) -> &'group [&'trees CapturedTokenTree] {
    match group {
        [_key, assigned_by, value @ ..] if assigned_by.punct() == Some('=') => value,
        _malformed => &[],
    }
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
    for group in &comma_groups(grammar, members)? {
        match group.first().and_then(|tree| tree.word()) {
            Some("population") => assigned_once(grammar, group, &mut population, assigned_text)?,
            Some("interleavings") => number_once(grammar, group, &mut interleavings)?,
            Some("samples") => number_once(grammar, group, &mut samples)?,
            Some("seed") => number_once(grammar, group, &mut seed)?,
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

/// Read one `<key> = <n>` clause into its empty seat, refusing a doubled key; the seat's own type is the width the number parses at.
fn number_once<Number: core::str::FromStr>(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<Number>,
) -> Result<(), ConcurrencyCaptureError> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    *seat = Some(assigned_number(grammar, group)?);
    Ok(())
}

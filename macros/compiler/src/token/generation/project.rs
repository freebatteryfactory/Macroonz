//! Conventional Rust projections over informed bounded values.
//!
//! The informed values own membership and order, while the caller owns every token written for one row and every item surrounding the returned expression.

use super::compose::{comma_many, group};
use super::{GENERATED_TOKEN_LIMIT, GeneratedDelimiter, GeneratedRowRefusal, GeneratedToken};
use crate::bounded::{KeyedRoster, KeyedRosterAssignment, NonEmpty, NonEmptyError, Overflow};

/// Projects one keyed roster into a borrowed Rust slice expression in retained order.
///
/// The caller supplies every row's tokens and remains responsible for any item name, visibility, element type, and semantic body surrounding the expression.
///
/// # Errors
///
/// Returns the first [`Overflow`] produced by the row projection or by the completed bracket group.
pub fn keyed_roster_slice<T, K, const N: usize>(
    roster: &KeyedRoster<T, K, N>,
    mut row: impl FnMut(usize, &K, &T) -> Result<Vec<GeneratedToken>, Overflow>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let rows = roster
        .indexed()
        .map(|(index, key, member)| row(index, key, member))
        .collect::<Result<Vec<_>, _>>()?;
    borrowed_slice(rows)
}

/// Projects one keyed roster into a flat item run in retained order.
///
/// The caller supplies every row's complete tokens and remains responsible for the meaning of each item.
///
/// # Errors
///
/// Returns the first [`GeneratedRowRefusal`] where a callback refuses, emits no token, or emits more than the generated-token magnitude.
pub fn keyed_roster_items<T, K, const N: usize>(
    roster: &KeyedRoster<T, K, N>,
    mut row: impl FnMut(usize, &K, &T) -> Result<Vec<GeneratedToken>, Overflow>,
) -> Result<Vec<GeneratedToken>, GeneratedRowRefusal> {
    let mut items = Vec::new();
    for (index, key, member) in roster.indexed() {
        let row = row(index, key, member)
            .map_err(|cause| GeneratedRowRefusal::at(index, NonEmptyError::Overflow(cause)))?;
        let row = NonEmpty::<GeneratedToken, GENERATED_TOKEN_LIMIT>::new(row)
            .map_err(|cause| GeneratedRowRefusal::at(index, cause))?;
        items.extend(row.iter().cloned());
    }
    Ok(items)
}

/// Projects one exact keyed assignment into a borrowed Rust slice expression in denominator order.
///
/// The caller supplies every row's tokens and remains responsible for any item name, visibility, element type, payload meaning, and semantic body surrounding the expression.
///
/// # Errors
///
/// Returns the first [`Overflow`] produced by the row projection or by the completed bracket group.
pub fn keyed_assignment_slice<D, K, P, S, const N: usize>(
    assignment: &KeyedRosterAssignment<D, K, P, S, N>,
    mut row: impl FnMut(usize, &K, &D, &S, &P) -> Result<Vec<GeneratedToken>, Overflow>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let rows = assignment
        .indexed()
        .map(|(index, key, member, seat, payload)| row(index, key, member, seat, payload))
        .collect::<Result<Vec<_>, _>>()?;
    borrowed_slice(rows)
}

/// Projects one exact keyed assignment into a flat item run in denominator order.
///
/// The caller supplies every row's complete tokens and remains responsible for the meaning of each item.
///
/// # Errors
///
/// Returns the first [`GeneratedRowRefusal`] where a callback refuses, emits no token, or emits more than the generated-token magnitude.
pub fn keyed_assignment_items<D, K, P, S, const N: usize>(
    assignment: &KeyedRosterAssignment<D, K, P, S, N>,
    mut row: impl FnMut(usize, &K, &D, &S, &P) -> Result<Vec<GeneratedToken>, Overflow>,
) -> Result<Vec<GeneratedToken>, GeneratedRowRefusal> {
    let mut items = Vec::new();
    for (index, key, member, seat, payload) in assignment.indexed() {
        let row = row(index, key, member, seat, payload)
            .map_err(|cause| GeneratedRowRefusal::at(index, NonEmptyError::Overflow(cause)))?;
        let row = NonEmpty::<GeneratedToken, GENERATED_TOKEN_LIMIT>::new(row)
            .map_err(|cause| GeneratedRowRefusal::at(index, cause))?;
        items.extend(row.iter().cloned());
    }
    Ok(items)
}

/// Writes `&[rows]` without changing the rows or their order.
fn borrowed_slice(rows: Vec<Vec<GeneratedToken>>) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, comma_many(rows))?,
    ])
}

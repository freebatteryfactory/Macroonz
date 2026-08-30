//! Conventional Rust projections over informed bounded values.
//!
//! The informed values own membership and order, while the caller owns every token written for one row and every item surrounding the returned expression.

use super::compose::{comma_many, group};
use super::{GeneratedDelimiter, GeneratedToken};
use crate::bounded::{KeyedRoster, KeyedRosterAssignment, Overflow};

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

/// Writes `&[rows]` without changing the rows or their order.
fn borrowed_slice(rows: Vec<Vec<GeneratedToken>>) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, comma_many(rows))?,
    ])
}

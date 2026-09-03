//! Joining role-bearing rows to the roster and destination facts their role declares.

use super::{Destination, Role};
use crate::bounded::NonEmpty;

/// Which existing order one destination join preserves.
#[derive(Clone, Copy)]
pub(crate) enum JoinOrder<'roster, R> {
    /// Preserve the order in which the rows were offered.
    Offering,
    /// Walk the declared role roster, retaining row order within each role.
    Roster(&'roster [R]),
}

/// The iterator one non-empty collection exposes without surrendering its invariant.
type Rows<'rows, T> = core::iter::Chain<core::iter::Once<&'rows T>, core::slice::Iter<'rows, T>>;

/// Every row standing under one role, in offered order.
pub(crate) fn rows_under<T, R: Role, const N: usize>(
    rows: &NonEmpty<T, N>,
    role: R,
    role_of: fn(&T) -> R,
) -> impl Iterator<Item = &T> {
    rows.iter().filter(move |row| role_of(row) == role)
}

/// The next roster role landing at one destination.
fn destination_role<R: Role>(
    roster: &[R],
    destination: Destination,
    roster_at: &mut usize,
) -> Option<R> {
    loop {
        let role = *roster.get(*roster_at)?;
        *roster_at = roster_at.saturating_add(1);
        if role.destination() == destination {
            return Some(role);
        }
    }
}

/// The role whose rows the roster-ordered join is currently walking.
fn active_role<'rows, T, R: Role, const N: usize>(
    rows: &'rows NonEmpty<T, N>,
    roster: &[R],
    destination: Destination,
    roster_at: &mut usize,
    active: &mut Option<R>,
    under: &mut Rows<'rows, T>,
) -> Option<R> {
    if let Some(role) = *active {
        return Some(role);
    }
    let role = destination_role(roster, destination, roster_at)?;
    *active = Some(role);
    *under = rows.into_iter();
    Some(role)
}

/// The next row in roster order for one destination.
fn next_rostered<'rows, T, R: Role, const N: usize>(
    rows: &'rows NonEmpty<T, N>,
    roster: &[R],
    destination: Destination,
    role_of: fn(&T) -> R,
    roster_at: &mut usize,
    active: &mut Option<R>,
    under: &mut Rows<'rows, T>,
) -> Option<&'rows T> {
    loop {
        let role = active_role(rows, roster, destination, roster_at, active, under)?;
        if let Some(row) = under.find(|row| role_of(row) == role) {
            return Some(row);
        }
        *active = None;
    }
}

/// Every row landing at one destination, in the order the caller selects.
pub(crate) fn rows_to<'rows, T, R: Role, const N: usize>(
    rows: &'rows NonEmpty<T, N>,
    destination: Destination,
    order: JoinOrder<'rows, R>,
    role_of: fn(&T) -> R,
) -> impl Iterator<Item = &'rows T> {
    let mut offering = rows.into_iter();
    let mut roster_at = 0_usize;
    let mut active = None;
    let mut under = rows.into_iter();
    core::iter::from_fn(move || match order {
        JoinOrder::Offering => offering.find(|row| role_of(row).destination() == destination),
        JoinOrder::Roster(roster) => next_rostered(
            rows,
            roster,
            destination,
            role_of,
            &mut roster_at,
            &mut active,
            &mut under,
        ),
    })
}

//! Counting and advancing the finite schedule space owned by one strand set.

use super::types::{InterleavingSpace, StrandSet};

/// How many interleavings the set admits: the multinomial over its strand lengths.
///
/// Computed as a running product of binomials, and surrendered honestly the moment any intermediate leaves the counter's range.
pub(super) fn interleaving_space<Command>(set: &StrandSet<Command>) -> InterleavingSpace {
    let mut placed = 0u128;
    let mut count = 1u128;
    for strand in set.strands() {
        let Ok(commands) = u128::try_from(strand.commands().len()) else {
            return InterleavingSpace::BeyondCount;
        };
        let Some(next_placed) = placed.checked_add(commands) else {
            return InterleavingSpace::BeyondCount;
        };
        placed = next_placed;
        let Some(ways) = choose(placed, commands) else {
            return InterleavingSpace::BeyondCount;
        };
        let Some(product) = count.checked_mul(ways) else {
            return InterleavingSpace::BeyondCount;
        };
        count = product;
    }
    InterleavingSpace::Counted(count)
}

/// Advance the material to the next position string in ascending order, or report the space walked out.
///
/// The rightmost position with room under its own radix steps up and the tail returns to zero, which is always lawful because a zero position names the first live strand.
/// The prefix before the pivot is unchanged, so the pivot's radix, computed under that prefix, still governs it.
pub(super) fn advanced(material: &mut [u8], radixes: &[usize]) -> bool {
    let Some(pivot) = material
        .iter()
        .zip(radixes)
        .rposition(|(&position, &radix)| usize::from(position).saturating_add(1usize) < radix)
    else {
        return false;
    };
    let Some(slot) = material.get_mut(pivot) else {
        return false;
    };
    *slot = slot.saturating_add(1u8);
    if let Some(tail) = material.get_mut(pivot.saturating_add(1usize)..) {
        tail.fill(0u8);
    }
    true
}

/// The binomial coefficient, or nothing where the running product leaves the counter's range.
///
/// The prefix products are themselves binomials, so every division is exact.
fn choose(total: u128, taken: u128) -> Option<u128> {
    let low = taken.min(total.checked_sub(taken)?);
    let mut count = 1u128;
    let mut term = 1u128;
    while term <= low {
        let factor = total.checked_sub(low)?.checked_add(term)?;
        count = count.checked_mul(factor)?.checked_div(term)?;
        term = term.checked_add(1u128)?;
    }
    Some(count)
}

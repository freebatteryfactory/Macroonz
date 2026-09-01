//! The material road: total interpretation of arbitrary bytes and checked encoding of canonical interleavings.

use super::types::{EncodingRefusal, InterleavedSequence, Interleaving, Realization, StrandSet};
use std::collections::VecDeque;

/// Realize one material string over a strand set.
///
/// Total by construction: any bytes are a lawful schedule, spelled canonically in the result.
#[must_use]
pub fn interpreted<Command: Clone>(
    set: &StrandSet<Command>,
    material: &[u8],
) -> InterleavedSequence<Command> {
    let realization = realized(set, material);
    InterleavedSequence::realized(
        Interleaving::declared(realization.choices),
        realization.commands,
    )
}

/// Write one canonical interleaving as the material that realizes it.
///
/// The walk is the validation: an interleaving foreign to the set refuses at the exact step it breaks, and one this home minted over the same set always encodes.
/// [`interpreted`] over the encoded material realizes the same interleaving, which is what makes the pair a replay.
///
/// # Errors
///
/// Refuses a step-count mismatch before any step is walked, then the first choice naming an ordinal no strand owns, then the first choice drawing a strand past its length.
pub fn encoded<Command>(
    set: &StrandSet<Command>,
    interleaving: &Interleaving,
) -> Result<Vec<u8>, EncodingRefusal> {
    if interleaving.choices().len() != set.steps() {
        return Err(EncodingRefusal::StepsMismatch {
            declared: interleaving.choices().len(),
            steps: set.steps(),
        });
    }
    let mut remaining: Vec<usize> = set
        .strands()
        .iter()
        .map(|strand| strand.commands().len())
        .collect();
    let mut material = Vec::with_capacity(set.steps());
    for (at, &choice) in interleaving.choices().iter().enumerate() {
        let Some(stock) = remaining.get(usize::from(choice)).copied() else {
            return Err(EncodingRefusal::ChoiceOutsideStrands { at, choice });
        };
        if stock == 0usize {
            return Err(EncodingRefusal::StrandExhausted { at, choice });
        }
        let position = remaining
            .iter()
            .take(usize::from(choice))
            .filter(|&&held| held > 0usize)
            .count();
        material.push(u8::try_from(position).unwrap_or(u8::MAX));
        if let Some(held) = remaining.get_mut(usize::from(choice)) {
            *held = stock.saturating_sub(1usize);
        }
    }
    Ok(material)
}

/// One command taken from the live strand selected by one material byte.
struct Taken<Command> {
    choice: u8,
    command: Command,
    radix: usize,
    pick: usize,
    strand: StrandAfterTake,
}

/// Whether the selected strand remains live after its next command is taken.
enum StrandAfterTake {
    Live,
    Exhausted,
}

/// Walk one material string over the set, total on any bytes.
///
/// The live list holds the strands that still owe commands, in ordinal order; each step's byte picks a position in it, an exhausted strand leaves it, and the walk ends when it empties.
pub(super) fn realized<Command: Clone>(
    set: &StrandSet<Command>,
    material: &[u8],
) -> Realization<Command> {
    let mut live: Vec<(u8, VecDeque<Command>)> = set
        .strands()
        .iter()
        .enumerate()
        .map(|(ordinal, strand)| {
            (
                u8::try_from(ordinal).unwrap_or(u8::MAX),
                strand.commands().iter().cloned().collect(),
            )
        })
        .collect();
    let mut choices = Vec::with_capacity(set.steps());
    let mut commands = Vec::with_capacity(set.steps());
    let mut radixes = Vec::with_capacity(set.steps());
    let mut fuel = material.iter().copied();
    while !live.is_empty() {
        let Some(taken) = taken(&mut live, fuel.next().unwrap_or(0u8)) else {
            break;
        };
        choices.push(taken.choice);
        commands.push(taken.command);
        radixes.push(taken.radix);
        if matches!(taken.strand, StrandAfterTake::Exhausted) {
            live.remove(taken.pick);
        }
    }
    Realization {
        choices,
        commands,
        radixes,
    }
}

/// Take the command selected by one byte, or report that the live-list invariant did not yield one.
fn taken<Command>(live: &mut [(u8, VecDeque<Command>)], raw: u8) -> Option<Taken<Command>> {
    let radix = live.len();
    let pick = usize::from(raw).checked_rem(radix)?;
    let (ordinal, queue) = live.get_mut(pick)?;
    let choice = *ordinal;
    let command = queue.pop_front()?;
    let strand = if queue.is_empty() {
        StrandAfterTake::Exhausted
    } else {
        StrandAfterTake::Live
    };
    Some(Taken {
        choice,
        command,
        radix,
        pick,
        strand,
    })
}

//! Bounded deterministic neighboring-input enumeration.

use super::{MutationCandidate, MutationKind, MutationPlan, MutationRefusal};
use std::collections::BTreeSet;

const BOUNDARIES: &[u8] = &[0, 1, 0x7f, 0x80, 0xff];

/// Enumerate deterministic nonempty neighbors of one retained seed.
///
/// Candidates are unique, remain beneath the declared byte ceiling, and stop at the declared budget.
///
/// # Errors
///
/// Refuses an empty or oversized seed and an explicitly supplied empty splice partner.
pub fn neighboring_inputs(
    seed: &[u8],
    partner: Option<&[u8]>,
    plan: &MutationPlan,
) -> Result<Vec<MutationCandidate>, MutationRefusal> {
    if seed.is_empty() {
        return Err(MutationRefusal::EmptySeed);
    }
    if seed.len() > plan.byte_limit() {
        return Err(MutationRefusal::SeedExceedsByteLimit);
    }
    if partner.is_some_and(<[u8]>::is_empty) {
        return Err(MutationRefusal::EmptyPartner);
    }
    let mut frontier = Frontier::opening(seed, plan);
    bit_flips(seed, &mut frontier);
    substitutions(seed, &mut frontier);
    checked_steps(seed, &mut frontier);
    deletions(seed, &mut frontier);
    insertions(seed, &mut frontier);
    duplications(seed, &mut frontier);
    if let Some(other) = partner {
        splices(seed, other, &mut frontier);
    }
    dictionary_insertions(seed, &mut frontier);
    Ok(frontier.finish())
}

struct Frontier<'seed, 'plan> {
    seed: &'seed [u8],
    plan: &'plan MutationPlan,
    seen: BTreeSet<Vec<u8>>,
    candidates: Vec<MutationCandidate>,
}

impl<'seed, 'plan> Frontier<'seed, 'plan> {
    fn opening(seed: &'seed [u8], plan: &'plan MutationPlan) -> Self {
        Self {
            seed,
            plan,
            seen: BTreeSet::new(),
            candidates: Vec::new(),
        }
    }

    fn full(&self) -> bool {
        let budget = usize::try_from(self.plan.budget()).unwrap_or(usize::MAX);
        self.candidates.len() >= budget
    }

    fn offer(&mut self, kind: MutationKind, bytes: Vec<u8>) {
        if self.full()
            || bytes.is_empty()
            || bytes.len() > self.plan.byte_limit()
            || bytes == self.seed
            || !self.seen.insert(bytes.clone())
        {
            return;
        }
        self.candidates
            .push(MutationCandidate::established(kind, bytes));
    }

    fn finish(self) -> Vec<MutationCandidate> {
        self.candidates
    }
}

fn bit_flips(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for index in 0..seed.len() {
        for bit in 0..u8::BITS {
            let mut bytes = seed.to_vec();
            let Some(value) = bytes.get_mut(index) else {
                continue;
            };
            let mask = 1_u8.checked_shl(bit).unwrap_or(0);
            *value ^= mask;
            frontier.offer(MutationKind::BitFlip, bytes);
            if frontier.full() {
                return;
            }
        }
    }
}

fn substitutions(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for index in 0..seed.len() {
        for boundary in BOUNDARIES {
            let mut bytes = seed.to_vec();
            let Some(value) = bytes.get_mut(index) else {
                continue;
            };
            *value = *boundary;
            frontier.offer(MutationKind::BoundarySubstitution, bytes);
            if frontier.full() {
                return;
            }
        }
    }
}

fn checked_steps(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for index in 0..seed.len() {
        if let Some(incremented) = seed.get(index).and_then(|value| value.checked_add(1)) {
            let mut bytes = seed.to_vec();
            if let Some(value) = bytes.get_mut(index) {
                *value = incremented;
                frontier.offer(MutationKind::Increment, bytes);
            }
        }
        if let Some(decremented) = seed.get(index).and_then(|value| value.checked_sub(1)) {
            let mut bytes = seed.to_vec();
            if let Some(value) = bytes.get_mut(index) {
                *value = decremented;
                frontier.offer(MutationKind::Decrement, bytes);
            }
        }
        if frontier.full() {
            return;
        }
    }
}

fn deletions(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for index in 0..seed.len() {
        let mut bytes = seed.to_vec();
        bytes.remove(index);
        frontier.offer(MutationKind::Delete, bytes);
        if frontier.full() {
            return;
        }
    }
}

fn insertions(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for index in 0..=seed.len() {
        for boundary in BOUNDARIES {
            let mut bytes = seed.to_vec();
            bytes.insert(index, *boundary);
            frontier.offer(MutationKind::InsertBoundary, bytes);
            if frontier.full() {
                return;
            }
        }
    }
}

fn duplications(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for index in 0..seed.len() {
        let Some(value) = seed.get(index).copied() else {
            continue;
        };
        let mut bytes = seed.to_vec();
        bytes.insert(index.saturating_add(1), value);
        frontier.offer(MutationKind::Duplicate, bytes);
        if frontier.full() {
            return;
        }
    }
}

fn splices(seed: &[u8], partner: &[u8], frontier: &mut Frontier<'_, '_>) {
    for cut in 0..=seed.len() {
        let partner_cut = cut.min(partner.len());
        let bytes = seed
            .iter()
            .copied()
            .take(cut)
            .chain(partner.iter().copied().skip(partner_cut))
            .collect();
        frontier.offer(MutationKind::Splice, bytes);
        if frontier.full() {
            return;
        }
    }
}

fn dictionary_insertions(seed: &[u8], frontier: &mut Frontier<'_, '_>) {
    for token in frontier.plan.dictionary() {
        for index in 0..=seed.len() {
            let bytes = seed
                .iter()
                .copied()
                .take(index)
                .chain(token.iter().copied())
                .chain(seed.iter().copied().skip(index))
                .collect();
            frontier.offer(MutationKind::DictionaryInsert, bytes);
            if frontier.full() {
                return;
            }
        }
    }
}

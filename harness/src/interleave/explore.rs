//! The exploration road: every order a strand set can merge in, judged under one contract.
//!
//! The space is walked exhaustively while its count fits the declared bound, and sampled through the one shared sequence driver ([`crate::generate::drive`]) beyond it — a schedule is a structured input like any other, so its bytes come from the same seeded, seekable stream every generated input comes from, and no loop of the driver's kind grows here.
//!
//! Interpretation is total: every byte string denotes exactly one lawful interleaving.
//! At each step the next byte picks among the strands that still hold commands, a missing tail reads as the first live strand, and surplus bytes go unread.
//! Totality is what lets a reducer remove or zero any window of material and still hold a schedule a fingerprint probe can judge.

use super::types::{
    Counterexample, EXPLORATION_STARVED, EncodingRefusal, ExplorationBound, ExplorationMode,
    ExplorationReading, ExplorationRefusal, ExplorationSite, ExplorationStanding,
    InterleavedSequence, Interleaving, InterleavingSpace, StrandSet,
};
use crate::descriptor::PopulationRef;
use crate::generate::{
    ByteSource, GenerationHalt, GenerationPlan, InputOrigin, RejectionAllowance, RootSeed,
    SizeProgression, admit_every_sequence, decode_arbitrary, drive,
};
use crate::properties::{Holding, TransitionContract, holds_over_history};
use crate::report::{ByteBudget, CaseBudget, FailureClass, GenerationProfile, TrialConclusion};
use std::collections::VecDeque;

/// The generator identity every sampled exploration draws under.
const CHOICE_GENERATOR: &str = "interleaving-choices";

/// The revision of that generator's meaning: how choice bytes become schedules.
const CHOICE_GENERATOR_REVISION: u32 = 1;

/// Explore the orders a strand set can merge in, judging every walked history under the contract.
///
/// The space is counted first.
/// While it fits the bound's interleaving seat, every merge order is enumerated in ascending position order and an all-pass is a statement about the whole space; beyond the seat, the bound's sample seat is drawn through the shared driver under a plan this road sizes exactly, and an all-pass is a statement about the sampled schedules alone.
/// The reading's mode and standing spell which of the two happened, and the first history that breaks a claim ends the walk as a [`Counterexample`].
///
/// The population and the seed belong to the caller, because a sampled schedule is generated material like any other and its lineage is the caller's statement.
///
/// # Errors
///
/// Refuses a sampling byte budget past what a plan can hold, and carries out a refused sampling plan rather than unwrapping one.
pub fn explored<State, Command: Clone>(
    set: &StrandSet<Command>,
    contract: &TransitionContract<State, Command>,
    bound: ExplorationBound,
    population: PopulationRef,
    seed: RootSeed,
) -> Result<ExplorationReading, ExplorationRefusal> {
    let space = interleaving_space(set);
    if within_bound(space, bound) {
        Ok(enumerated(set, contract, space))
    } else {
        sampled(set, contract, bound, population, seed, space)
    }
}

/// Read one exploration into the trial conclusion its evidence earns.
///
/// A counterexample concludes as the refusal its own finding states; an exhausted space concludes as a pass over the whole space; a clean sample concludes as a pass of the declared exploration exactly when the sampling drive met its declared case budget, and refuses as [`EXPLORATION_STARVED`](crate::interleave::EXPLORATION_STARVED) where it stopped short — an all-pass over fewer schedules than were declared is unexercised evidence, not a pass.
///
/// The reading stays the owner of the replay: the conclusion is the verdict alone, and the counterexample's interleaving lives where it always did.
#[must_use]
#[track_caller]
pub fn concluded(reading: &ExplorationReading) -> TrialConclusion {
    match reading.standing() {
        ExplorationStanding::CounterexampleFound(counterexample) => {
            TrialConclusion::Refused(counterexample.finding().clone())
        }
        ExplorationStanding::SpaceExhaustedAllHold => TrialConclusion::Passed,
        ExplorationStanding::SampledAllHold => match reading.mode() {
            ExplorationMode::Sampled {
                halt: GenerationHalt::CaseBudgetMet,
                census: _,
            } => TrialConclusion::Passed,
            ExplorationMode::Exhaustive | ExplorationMode::Sampled { .. } => {
                crate::properties::concluded(
                    Holding::Fails,
                    FailureClass::RefusedByCheck,
                    EXPLORATION_STARVED,
                )
            }
        },
    }
}

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

/// One realized walk: the canonical choices, the merged commands, and how many live strands each step chose among.
struct Realization<Command> {
    /// The canonical choice string: which strand stepped, per step.
    choices: Vec<u8>,
    /// The commands, in merged order.
    commands: Vec<Command>,
    /// The live-strand count each step's byte picked among — the radix the enumerator advances under.
    radixes: Vec<usize>,
}

/// Walk one material string over the set, total on any bytes.
///
/// The live list holds the strands that still owe commands, in ordinal order; each step's byte picks a position in it, an exhausted strand leaves it, and the walk ends when it empties.
/// The `else` exits are unreachable while the list drops emptied strands — ending the merge early there is the one honest thing left if the invariant ever broke.
fn realized<Command: Clone>(set: &StrandSet<Command>, material: &[u8]) -> Realization<Command> {
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
        let raw = fuel.next().unwrap_or(0u8);
        let pick = usize::from(raw).checked_rem(live.len()).unwrap_or(0usize);
        radixes.push(live.len());
        let emptied = {
            let Some((ordinal, queue)) = live.get_mut(pick) else {
                break;
            };
            choices.push(*ordinal);
            match queue.pop_front() {
                Some(command) => commands.push(command),
                None => break,
            }
            queue.is_empty()
        };
        if emptied {
            live.remove(pick);
        }
    }
    Realization {
        choices,
        commands,
        radixes,
    }
}

/// Whether the counted space fits under the bound's exhaustive ceiling.
fn within_bound(space: InterleavingSpace, bound: ExplorationBound) -> bool {
    match space {
        InterleavingSpace::Counted(count) => count <= u128::from(bound.interleavings()),
        InterleavingSpace::BeyondCount => false,
    }
}

/// How many interleavings the set admits: the multinomial over its strand lengths.
///
/// Computed as a running product of binomials, and surrendered honestly the moment any intermediate leaves the counter's range.
fn interleaving_space<Command>(set: &StrandSet<Command>) -> InterleavingSpace {
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

/// Walk the whole space in ascending position order, judging every merged history.
fn enumerated<State, Command: Clone>(
    set: &StrandSet<Command>,
    contract: &TransitionContract<State, Command>,
    space: InterleavingSpace,
) -> ExplorationReading {
    let mut material = vec![0u8; set.steps()];
    let mut explored = 0u64;
    loop {
        let realization = realized(set, &material);
        let ordinal = explored;
        explored = explored.saturating_add(1u64);
        if let TrialConclusion::Refused(finding) =
            holds_over_history(contract, &realization.commands)
        {
            let counterexample = Counterexample::found(
                ExplorationSite::Enumerated { ordinal },
                Interleaving::declared(realization.choices),
                finding,
            );
            return ExplorationReading::read(
                space,
                ExplorationMode::Exhaustive,
                explored,
                ExplorationStanding::CounterexampleFound(counterexample),
            );
        }
        if !advanced(&mut material, &realization.radixes) {
            break;
        }
    }
    ExplorationReading::read(
        space,
        ExplorationMode::Exhaustive,
        explored,
        ExplorationStanding::SpaceExhaustedAllHold,
    )
}

/// Advance the material to the next position string in ascending order, or report the space walked out.
///
/// The rightmost position with room under its own radix steps up and the tail returns to zero, which is always lawful because a zero position names the first live strand.
/// The prefix before the pivot is unchanged, so the pivot's radix — computed under that prefix — still governs it.
fn advanced(material: &mut [u8], radixes: &[usize]) -> bool {
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

/// Draw the bound's sample seat through the shared driver and judge every drawn schedule.
fn sampled<State, Command: Clone>(
    set: &StrandSet<Command>,
    contract: &TransitionContract<State, Command>,
    bound: ExplorationBound,
    population: PopulationRef,
    seed: RootSeed,
    space: InterleavingSpace,
) -> Result<ExplorationReading, ExplorationRefusal> {
    let overflow = ExplorationRefusal::SampleBytesOverflow {
        samples: bound.samples(),
        steps: set.steps(),
    };
    let Ok(step_bytes) = u64::try_from(set.steps()) else {
        return Err(overflow);
    };
    let Some(bytes) = u64::from(bound.samples()).checked_mul(step_bytes) else {
        return Err(overflow);
    };
    let plan = GenerationPlan::declared(
        population,
        GenerationProfile::declared(CHOICE_GENERATOR, CHOICE_GENERATOR_REVISION),
        InputOrigin::Seeded(seed),
        CaseBudget::declared(bound.samples()),
        ByteBudget::declared(bytes),
        RejectionAllowance::NoRejections,
        SizeProgression::Constant { width: set.width() },
    )
    .map_err(ExplorationRefusal::SamplingPlanRefused)?;
    let source = ByteSource::of_plan(&plan);
    let generated = drive(&plan, &source, decode_arbitrary::<u8>, admit_every_sequence);
    let mode = ExplorationMode::Sampled {
        census: generated.census(),
        halt: generated.halt(),
    };
    let mut explored = 0u64;
    for sequence in generated.sequences() {
        let realization = realized(set, sequence.commands());
        explored = explored.saturating_add(1u64);
        if let TrialConclusion::Refused(finding) =
            holds_over_history(contract, &realization.commands)
        {
            let counterexample = Counterexample::found(
                ExplorationSite::Sampled {
                    case: sequence.case(),
                },
                Interleaving::declared(realization.choices),
                finding,
            );
            return Ok(ExplorationReading::read(
                space,
                mode,
                explored,
                ExplorationStanding::CounterexampleFound(counterexample),
            ));
        }
    }
    Ok(ExplorationReading::read(
        space,
        mode,
        explored,
        ExplorationStanding::SampledAllHold,
    ))
}

//! The exploration road: every order a strand set can merge in, judged under one contract.
//!
//! The space is walked exhaustively while its count fits the declared bound, and sampled through the one shared sequence driver ([`crate::generate::drive`]) beyond it — a schedule is a structured input like any other, so its bytes come from the same seeded, seekable stream every generated input comes from, and no loop of the driver's kind grows here.

use super::material::realized;
use super::space::{advanced, interleaving_space};
use super::types::{
    Counterexample, EXPLORATION_STARVED, ExplorationBound, ExplorationMode, ExplorationReading,
    ExplorationRefusal, ExplorationSite, ExplorationStanding, Interleaving, InterleavingSpace,
    StrandSet,
};
use crate::descriptor::PopulationRef;
use crate::generate::{
    ByteSource, CaseIndex, CommandSequence, GenerationHalt, GenerationPlan, InputOrigin,
    RejectionAllowance, RootSeed, SizeProgression, admit_every_sequence, decode_arbitrary, drive,
};
use crate::properties::{Holding, TransitionContract, holds_over_history};
use crate::report::{
    ByteBudget, CaseBudget, FailureClass, GenerationProfile, TrialConclusion, TrialFinding,
};

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
/// A counterexample concludes as the refusal its own finding states; an exhausted space concludes as a pass over the whole space; a clean sample concludes as a pass of the declared exploration exactly when the sampling drive met its declared case budget, and refuses as [`EXPLORATION_STARVED`] where it stopped short — an all-pass over fewer schedules than were declared is unexercised evidence, not a pass.
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
        ExplorationStanding::SampledAllHold => sampled_conclusion(reading.mode()),
    }
}

/// Read a clean sampled standing into the conclusion its exact mode earns.
fn sampled_conclusion(mode: ExplorationMode) -> TrialConclusion {
    match mode {
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
    }
}

/// Whether the counted space fits under the bound's exhaustive ceiling.
fn within_bound(space: InterleavingSpace, bound: ExplorationBound) -> bool {
    match space {
        InterleavingSpace::Counted(count) => count <= u128::from(bound.interleavings()),
        InterleavingSpace::BeyondCount => false,
    }
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
    let (explored, counterexample) = sampled_counterexample(set, contract, generated.sequences());
    let standing = counterexample.map_or(
        ExplorationStanding::SampledAllHold,
        ExplorationStanding::CounterexampleFound,
    );
    Ok(ExplorationReading::read(space, mode, explored, standing))
}

/// Judge sampled sequences in drive order and retain the first counterexample.
fn sampled_counterexample<State, Command: Clone>(
    set: &StrandSet<Command>,
    contract: &TransitionContract<State, Command>,
    sequences: &[CommandSequence<u8>],
) -> (u64, Option<Counterexample>) {
    let mut explored = 0u64;
    for sequence in sequences {
        let realization = realized(set, sequence.commands());
        explored = explored.saturating_add(1u64);
        let TrialConclusion::Refused(finding) = holds_over_history(contract, &realization.commands)
        else {
            continue;
        };
        return (
            explored,
            Some(sampled_found(sequence.case(), realization.choices, finding)),
        );
    }
    (explored, None)
}

/// Mint the first sampled counterexample at its generated case.
fn sampled_found(case: CaseIndex, choices: Vec<u8>, finding: TrialFinding) -> Counterexample {
    Counterexample::found(
        ExplorationSite::Sampled { case },
        Interleaving::declared(choices),
        finding,
    )
}

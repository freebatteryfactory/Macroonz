//! The temporal suite: a generated command sequence drives an owner's transition system, and every claim is read across the whole history.
//!
//! A command sequence is a structured input like any other, so the history a law is read over comes from the one shared sequence driver ([`crate::generate::drive`]) rather than from a loop grown here, and a failing sequence is a counterexample carrying its seed like any other.
//!
//! The state and the command are unbounded type parameters, and this home never learns what a state means.
//! An owner integrates by mapping its own vocabulary into a [`TransitionContract`] at its own layer.
//!
//! What a break does not carry is which step of the history broke the claim.
//! A conclusion carries the typed cause and the class, and the sequence that produced it is what localizes the break: minimization shrinks that sequence while requiring the same fingerprint, so the shortest history that still breaks the claim is the answer to "where", produced by the lane that owns shrinking.

use super::conclude::concluded;
use super::types::{
    Holding, NO_SEQUENCE_DRIVEN, Order, StatePredicate, TemporalDemand, TemporalDriveReading,
    TemporalDriveStanding, TransitionContract,
};
use crate::generate::{
    ByteSource, CommandDecode, CommandSequence, GenerationHalt, GenerationPlan,
    SequencePrecondition, drive,
};
use crate::report::{FailureClass, TrialConclusion};
use core::cmp::Ordering;

/// Drive one command sequence through a contract and read every claim across the whole history it produced.
///
/// The history is the opening state and the state after every command, in order, and the first claim that breaks is the conclusion.
/// A contract cannot be built with no claim at all, so a pass here always means something was demanded.
#[must_use]
#[track_caller]
pub fn holds_over_history<State, Command>(
    contract: &TransitionContract<State, Command>,
    commands: &[Command],
) -> TrialConclusion {
    let history = driven_history(contract, commands);
    for claim in contract.claims() {
        let holding = demand_holding(claim.demand(), &history);
        match concluded(holding, FailureClass::PropertyDisagreement, claim.cause()) {
            TrialConclusion::Passed => {}
            refused @ TrialConclusion::Refused(_) => return refused,
        }
    }
    TrialConclusion::Passed
}

/// Drive a whole generation plan through a contract and retain the evidence behind its temporal standing.
///
/// The plan owns every bound and the source owns every byte; this call owns neither.
/// The sequences arrive from the one shared driver, so the temporal lane, the mutation lane, and the chaos lane all walk histories the same machinery produced.
///
/// A drive that admitted no sequence refuses rather than passing.
/// One counterexample refuses even under a partial halt, but an all-pass prefix stays incomplete unless generation reached the declared case budget.
#[must_use]
#[track_caller]
pub fn holds_over_drive<State, Command>(
    contract: &TransitionContract<State, Command>,
    plan: &GenerationPlan,
    source: &ByteSource,
    decode: CommandDecode<Command>,
    precondition: SequencePrecondition<Command>,
) -> TemporalDriveReading<Command> {
    let generated = drive(plan, source, decode, precondition);
    if generated.sequences().is_empty() {
        let empty = concluded(
            Holding::Fails,
            FailureClass::RefusedByCheck,
            NO_SEQUENCE_DRIVEN,
        );
        let standing = TemporalDriveStanding::Concluded(empty);
        return TemporalDriveReading::from_drive(generated, 0usize, standing);
    }
    let (evaluated, break_found) = first_break(contract, generated.sequences());
    let standing = match break_found {
        Some(refused) => TemporalDriveStanding::Concluded(refused),
        None if generated.halt() == GenerationHalt::CaseBudgetMet => {
            TemporalDriveStanding::Concluded(TrialConclusion::Passed)
        }
        None => TemporalDriveStanding::Incomplete,
    };
    TemporalDriveReading::from_drive(generated, evaluated, standing)
}

/// How many sequences evaluation read, and the first history that broke a claim.
#[track_caller]
fn first_break<State, Command>(
    contract: &TransitionContract<State, Command>,
    sequences: &[CommandSequence<Command>],
) -> (usize, Option<TrialConclusion>) {
    let mut evaluated = 0usize;
    for sequence in sequences {
        evaluated = evaluated.saturating_add(1usize);
        match holds_over_history(contract, sequence.commands()) {
            TrialConclusion::Passed => {}
            refused @ TrialConclusion::Refused(_) => return (evaluated, Some(refused)),
        }
    }
    (evaluated, None)
}

/// The history one command sequence drives: the opening state, and the state after each command in order.
fn driven_history<State, Command>(
    contract: &TransitionContract<State, Command>,
    commands: &[Command],
) -> Vec<State> {
    let apply = contract.apply();
    let mut history: Vec<State> = Vec::with_capacity(commands.len().saturating_add(1));
    let mut state = (contract.opening())();
    for command in commands {
        let next = apply(&state, command);
        history.push(state);
        state = next;
    }
    history.push(state);
    history
}

/// Whether one demand holds of one whole history.
fn demand_holding<State>(demand: &TemporalDemand<State>, history: &[State]) -> Holding {
    match *demand {
        TemporalDemand::Always(predicate) => everywhere(predicate, history),
        TemporalDemand::Never(predicate) => nowhere(predicate, history),
        TemporalDemand::Eventually(predicate) => somewhere(predicate, history),
        TemporalDemand::OnceHoldingAlwaysHolding(predicate) => latched(predicate, history),
        TemporalDemand::NeverDecreases(order) => never_decreasing(order, history),
    }
}

/// Whether the predicate holds of every state.
fn everywhere<State>(predicate: StatePredicate<State>, history: &[State]) -> Holding {
    history.iter().map(predicate).fold(Holding::Holds, both)
}

/// Whether the predicate holds of no state.
fn nowhere<State>(predicate: StatePredicate<State>, history: &[State]) -> Holding {
    history
        .iter()
        .map(predicate)
        .map(opposite)
        .fold(Holding::Holds, both)
}

/// Whether the predicate holds of at least one state.
fn somewhere<State>(predicate: StatePredicate<State>, history: &[State]) -> Holding {
    history.iter().map(predicate).fold(Holding::Fails, either)
}

/// Whether the predicate, once it holds, holds of every later state.
///
/// A history the predicate never holds of satisfies the latch: nothing latched, so nothing was unlatched.
/// What that history fails to establish is the eventually-claim, which is a different claim and states itself.
fn latched<State>(predicate: StatePredicate<State>, history: &[State]) -> Holding {
    let mut seen = Holding::Fails;
    for state in history {
        match (seen, predicate(state)) {
            (Holding::Holds, Holding::Fails) => return Holding::Fails,
            (Holding::Fails, Holding::Holds) => seen = Holding::Holds,
            (Holding::Holds, Holding::Holds) | (Holding::Fails, Holding::Fails) => {}
        }
    }
    Holding::Holds
}

/// Whether no state ranks below the state before it.
fn never_decreasing<State>(order: Order<State>, history: &[State]) -> Holding {
    history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(earlier, later)| match order(earlier, later) {
            Ordering::Less | Ordering::Equal => Holding::Holds,
            Ordering::Greater => Holding::Fails,
        })
        .fold(Holding::Holds, both)
}

/// The conjunction of two demand verdicts.
const fn both(left: Holding, right: Holding) -> Holding {
    match (left, right) {
        (Holding::Holds, Holding::Holds) => Holding::Holds,
        (Holding::Fails, _) | (Holding::Holds, Holding::Fails) => Holding::Fails,
    }
}

/// The disjunction of two demand verdicts.
const fn either(left: Holding, right: Holding) -> Holding {
    match (left, right) {
        (Holding::Holds, _) | (Holding::Fails, Holding::Holds) => Holding::Holds,
        (Holding::Fails, Holding::Fails) => Holding::Fails,
    }
}

/// The opposite demand verdict.
const fn opposite(holding: Holding) -> Holding {
    match holding {
        Holding::Holds => Holding::Fails,
        Holding::Fails => Holding::Holds,
    }
}

//! Judge the caller's balance rule over command orders derived from actual deliveries.

use crate::types::{Account, Move};
use crate::world::baked::exploration;
use macroonz::harness::interleave::{
    ExplorationBoundRefusal, ExplorationMode, ExplorationStanding, InterleavingSpace, Strand,
    StrandSet, encoded, interpreted,
};
use macroonz::harness::network::Delivery;
use macroonz::harness::properties::{
    Holding, TemporalClaim, TemporalDemand, TransitionContract, holds_over_history,
};
use macroonz::harness::report::{FailureClass, FindingCause, TrialConclusion};

const NEVER_OVERDRAWN: FindingCause = FindingCause::named("schedule-example", "never-overdrawn");

pub(super) fn run(deliveries: &[Delivery<Move>]) -> Result<(), String> {
    let strands = strands(deliveries)?;
    let vulnerable = contract(unchecked)?;
    let (reading, conclusion) = exploration::exhaustive(&strands, &vulnerable)
        .map_err(|error| format!("vulnerable exploration: {error:?}"))?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(2u128));
    assert_eq!(reading.mode(), ExplorationMode::Exhaustive);
    let ExplorationStanding::CounterexampleFound(counterexample) = reading.standing() else {
        return Err("the unguarded withdrawal must expose a counterexample".to_owned());
    };
    assert_eq!(counterexample.interleaving().choices(), [1u8, 0u8]);
    let TrialConclusion::Refused(finding) = &conclusion else {
        return Err("the counterexample must retain its refusal".to_owned());
    };
    assert_eq!(finding.cause(), NEVER_OVERDRAWN);
    assert_eq!(finding.class(), FailureClass::PropertyDisagreement);
    let material = encoded(&strands, counterexample.interleaving())
        .map_err(|error| format!("counterexample encoding: {error:?}"))?;
    let replay = interpreted(&strands, &material);
    assert_eq!(
        replay.commands(),
        [Move::Withdraw(5u64), Move::Deposit(5u64)]
    );
    let TrialConclusion::Refused(replayed) = holds_over_history(&vulnerable, replay.commands())
    else {
        return Err("the saved command order must reproduce the no-overdraft failure".to_owned());
    };
    assert_eq!(replayed.cause(), NEVER_OVERDRAWN);
    assert_eq!(replayed.class(), FailureClass::PropertyDisagreement);
    assert!(replayed.foreign().is_none());
    assert_eq!(replayed.located().file(), file!());
    assert_ne!(replayed.located(), finding.located());
    let (again, repeated) = exploration::exhaustive(&strands, &vulnerable)
        .map_err(|error| format!("repeat exploration: {error:?}"))?;
    assert_eq!((again, repeated), (reading, conclusion));
    guarded_controls(&strands, replay.commands())
}

fn guarded_controls(strands: &StrandSet<Move>, replay: &[Move]) -> Result<(), String> {
    let repaired = contract(guarded)?;
    assert_eq!(
        holds_over_history(&repaired, replay),
        TrialConclusion::Passed
    );
    let (reading, conclusion) = exploration::exhaustive(strands, &repaired)
        .map_err(|error| format!("guarded exhaustive exploration: {error:?}"))?;
    assert_eq!(reading.explored(), 2u64);
    assert_eq!(
        reading.standing(),
        &ExplorationStanding::SpaceExhaustedAllHold
    );
    assert_eq!(conclusion, TrialConclusion::Passed);
    let (sampled, sampled_conclusion) = exploration::sampled(strands, &repaired)
        .map_err(|error| format!("guarded sampled exploration: {error:?}"))?;
    assert!(matches!(sampled.mode(), ExplorationMode::Sampled { .. }));
    assert_eq!(sampled.explored(), 3u64);
    assert_eq!(sampled.standing(), &ExplorationStanding::SampledAllHold);
    assert_eq!(sampled_conclusion, TrialConclusion::Passed);
    let (again, repeated) = exploration::sampled(strands, &repaired)
        .map_err(|error| format!("repeat sampling: {error:?}"))?;
    assert_eq!((again, repeated), (sampled, sampled_conclusion));
    assert!(matches!(
        exploration::no_work(strands, &repaired),
        Err(exploration::Fault::Bound(
            ExplorationBoundRefusal::ZeroInterleavings
        ))
    ));
    Ok(())
}

fn strands(deliveries: &[Delivery<Move>]) -> Result<StrandSet<Move>, String> {
    let topology = crate::world::baked::network::topology()
        .map_err(|error| format!("strand topology: {error:?}"))?;
    let strands = topology
        .links()
        .iter()
        .map(|link| {
            Strand::declared(
                link.from().name(),
                deliveries
                    .iter()
                    .filter(|delivery| delivery.link() == *link)
                    .map(|delivery| *delivery.payload())
                    .collect(),
            )
            .map_err(|error| format!("delivery strand: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    StrandSet::declared(strands).map_err(|error| format!("delivery strands: {error:?}"))
}

fn contract(
    apply: fn(&Account, &Move) -> Account,
) -> Result<TransitionContract<Account, Move>, String> {
    TransitionContract::declared(
        opening,
        apply,
        vec![TemporalClaim::declared(
            NEVER_OVERDRAWN,
            TemporalDemand::Never(negative),
        )],
    )
    .map_err(|error| format!("balance contract: {error:?}"))
}

const fn opening() -> Account {
    Account { balance: 0i128 }
}

fn unchecked(state: &Account, command: &Move) -> Account {
    Account {
        balance: match *command {
            Move::Deposit(amount) => state.balance.saturating_add(i128::from(amount)),
            Move::Withdraw(amount) => state.balance.saturating_sub(i128::from(amount)),
        },
    }
}

fn guarded(state: &Account, command: &Move) -> Account {
    match *command {
        Move::Withdraw(amount) if state.balance < i128::from(amount) => *state,
        Move::Deposit(_) | Move::Withdraw(_) => unchecked(state, command),
    }
}

const fn negative(state: &Account) -> Holding {
    if state.balance < 0i128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

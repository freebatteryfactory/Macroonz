//! A direct temporal property over handwritten source with no macro or registration step.

use macroonz_harness::properties::{
    Holding, TemporalClaim, TemporalDemand, TransitionContract, holds_over_history,
};
use macroonz_harness::report::{FindingCause, TrialConclusion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State(u128);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Input(u128);

const BOUND_CAUSE: FindingCause = FindingCause::named("example", "state-remains-bounded");

const fn opening() -> State {
    State(0u128)
}

const fn apply(state: &State, input: &Input) -> State {
    State(state.0.saturating_add(input.0))
}

const fn bounded(state: &State) -> Holding {
    if state.0 <= 3u128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn main() -> Result<(), String> {
    let contract = TransitionContract::declared(
        opening,
        apply,
        vec![TemporalClaim::declared(
            BOUND_CAUSE,
            TemporalDemand::Always(bounded),
        )],
    )
    .map_err(|refusal| format!("the temporal contract was refused: {refusal:?}"))?;

    let lawful = holds_over_history(&contract, &[Input(1u128), Input(2u128)]);
    if lawful != TrialConclusion::Passed {
        return Err(format!("the lawful history was refused: {lawful:?}"));
    }

    let hostile = holds_over_history(&contract, &[Input(4u128)]);
    if matches!(hostile, TrialConclusion::Refused(_)) {
        Ok(())
    } else {
        Err("the hostile history did not produce a typed finding".to_owned())
    }
}

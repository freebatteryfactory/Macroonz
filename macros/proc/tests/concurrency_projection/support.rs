//! Caller-owned strands and transition meaning shared by the generated-crossing claims.

use mh::descriptor::NamespacedName;
use mh::interleave::{Strand, StrandSet};
use mh::properties::{Holding, TemporalClaim, TemporalDemand, TransitionContract};
use mh::report::FindingCause;

/// The cause the caller's always-holding claim is declared under.
const STAYS_LAWFUL: FindingCause = FindingCause::named("concurrency-projection", "stays-lawful");

/// One caller-owned command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Command(i8);

/// One caller-owned state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct State(i8);

/// The state every history opens at.
const fn opening() -> State {
    State(0i8)
}

/// One caller-owned transition.
fn applied(state: &State, command: &Command) -> State {
    State(state.0.saturating_add(command.0))
}

/// The caller's deliberately simple temporal demand.
const fn lawful(_state: &State) -> Holding {
    Holding::Holds
}

/// Two one-command parties, so the exhaustive space contains both orders.
pub(crate) fn strands() -> Option<StrandSet<Command>> {
    let left = Strand::declared(
        NamespacedName::named("concurrency-projection", "left").ok()?,
        vec![Command(1i8)],
    )
    .ok()?;
    let right = Strand::declared(
        NamespacedName::named("concurrency-projection", "right").ok()?,
        vec![Command(-1i8)],
    )
    .ok()?;
    StrandSet::declared(vec![left, right]).ok()
}

/// The transition contract supplied at the generated call boundary.
pub(crate) fn contract() -> Option<TransitionContract<State, Command>> {
    TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            STAYS_LAWFUL,
            TemporalDemand::Always(lawful),
        )],
    )
    .ok()
}

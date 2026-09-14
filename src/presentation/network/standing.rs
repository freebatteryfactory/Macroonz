//! Distinct reproduction, playback exhaustion and exact-address joins.

use crate::harness::network::{
    ReplayExhaustion, ReplayIncomplete, ReproducedReplay, ReproducedReplayRefusal,
    SimulationReproduction,
};
use crate::presentation::{
    Presentation,
    value::{hex, object, tagged},
};
use serde_json::Value;

/// Project the simulation reproduction already established by the network owner.
pub fn network_reproduction(record: &SimulationReproduction) -> Presentation {
    Presentation::projected(
        "network-reproduction",
        "macroonz-harness/network",
        "recorded",
        reproduction(*record),
    )
}

fn reproduction(value: SimulationReproduction) -> Value {
    object([
        ("address", hex(value.address().address().as_bytes())),
        ("actions", value.actions().into()),
        ("rows", value.rows().into()),
        ("final_tick", value.final_tick().ordinal().into()),
    ])
}

/// Project completed playback without claiming the adopter processed any row.
pub fn network_replay_exhaustion(record: &ReplayExhaustion) -> Presentation {
    Presentation::projected(
        "network-replay-exhaustion",
        "macroonz-harness/network",
        "recorded",
        exhaustion(*record),
    )
}

fn exhaustion(value: ReplayExhaustion) -> Value {
    object([
        ("address", hex(value.address().address().as_bytes())),
        ("total", value.total().into()),
        ("final_tick", value.final_tick().ordinal().into()),
    ])
}

/// Project the exact transcript and rows still withheld when playback exhaustion refused.
pub fn network_replay_incomplete(record: &ReplayIncomplete) -> Presentation {
    Presentation::projected(
        "network-replay-incomplete",
        "macroonz-harness/network",
        "recorded",
        object([
            ("address", hex(record.address().address().as_bytes())),
            ("remaining", record.remaining().into()),
        ]),
    )
}

/// Project a joined reproduction and exhausted playback without performing either operation.
pub fn network_reproduced_replay(record: &ReproducedReplay) -> Presentation {
    Presentation::projected(
        "network-reproduced-replay",
        "macroonz-harness/network",
        "recorded",
        object([
            ("reproduction", reproduction(record.reproduction())),
            ("exhaustion", exhaustion(record.exhaustion())),
        ]),
    )
}

/// Project both addresses that prevented reproduction and playback from joining.
pub fn network_replay_join_refusal(record: &ReproducedReplayRefusal) -> Presentation {
    let ReproducedReplayRefusal::AddressMismatch {
        reproduction,
        replay,
    } = record;
    Presentation::projected(
        "network-replay-join-refusal",
        "macroonz-harness/network",
        "recorded",
        tagged(
            "address-mismatch",
            object([
                ("reproduction", hex(reproduction.address().as_bytes())),
                ("replay", hex(replay.address().as_bytes())),
            ]),
        ),
    )
}

//! Joined replay outcomes retain coordinate movement and historical limits independently.

use super::value::{object, tagged};
use super::{Presentation, context};
use crate::harness::report::replay::{
    HistoricalReplayRefusal, HistoricalReplayStanding, ReplayCoordinate, ReplayJoinRefusal,
    ReplayMovement, ReplayNonReproduction, ReplayOutcome, ReplayReading, WitnessLineage,
};
use serde_json::Value;

/// Project the admitted witness comparison without re-executing or authenticating its history.
pub fn replay_comparison(value: &ReplayReading) -> Presentation {
    Presentation::projected(
        "replay-comparison",
        "macroonz-harness/report/replay",
        "recorded",
        object([
            ("movement", movement(value.movement())),
            ("lineage", lineage(value.lineage())),
            ("historical", standing(value.standing())),
            ("outcome", outcome(value.outcome())),
        ]),
    )
}

/// Project why a witness could not join its current report, without inventing a comparison.
pub fn replay_join_refusal(value: ReplayJoinRefusal) -> Presentation {
    Presentation::projected(
        "replay-join-refusal",
        "macroonz-harness/report/replay",
        "recorded",
        object([
            ("phase", "witness-join".into()),
            ("cause", join_cause(value).into()),
        ]),
    )
}

pub(super) fn join_cause(value: ReplayJoinRefusal) -> &'static str {
    match value {
        ReplayJoinRefusal::WitnessBytesDiffer => "witness-bytes-differ",
        ReplayJoinRefusal::CurrentInputUnrecorded => "current-input-unrecorded",
        ReplayJoinRefusal::CurrentProfileDiffers => "current-profile-differs",
        ReplayJoinRefusal::CurrentCaseDiffers => "current-case-differs",
    }
}

fn coordinate(value: ReplayCoordinate) -> Value {
    match value {
        ReplayCoordinate::Same => "same",
        ReplayCoordinate::Moved => "moved",
        ReplayCoordinate::Unrecorded => "unrecorded",
    }
    .into()
}

fn movement(value: ReplayMovement) -> Value {
    object([
        ("trial", coordinate(value.trial())),
        ("subject", coordinate(value.subject())),
        ("check", coordinate(value.check())),
        ("profile", coordinate(value.profile())),
        ("schema", coordinate(value.schema())),
        ("decoder", coordinate(value.decoder())),
        ("target", coordinate(value.target())),
        ("toolchain", coordinate(value.toolchain())),
        ("invocation", coordinate(value.invocation())),
    ])
}

fn lineage(value: WitnessLineage) -> Value {
    match value {
        WitnessLineage::OriginalCase => "original-case",
        WitnessLineage::ReachedWitness => "reached-witness",
        WitnessLineage::ConventionMoved => "convention-moved",
        WitnessLineage::HistoricalInputUnrecorded => "historical-input-unrecorded",
    }
    .into()
}

pub(super) fn standing(value: HistoricalReplayStanding) -> Value {
    match value {
        HistoricalReplayStanding::Comparable(posture) => {
            tagged("comparable", context::posture(posture))
        }
        HistoricalReplayStanding::Unverifiable(reason) => tagged(
            "unverifiable",
            match reason {
                HistoricalReplayRefusal::InputUnrecorded => "input-unrecorded".into(),
                HistoricalReplayRefusal::IncompleteLegacyRecord => {
                    "incomplete-legacy-record".into()
                }
                HistoricalReplayRefusal::Untracked => "untracked".into(),
            },
        ),
    }
}

fn outcome(value: ReplayOutcome) -> Value {
    match value {
        ReplayOutcome::DefectReproduced => tagged("defect-reproduced", Value::Null),
        ReplayOutcome::FixedOnWitness => tagged("fixed-on-witness", Value::Null),
        ReplayOutcome::NotReproduced(reason) => tagged(
            "not-reproduced",
            match reason {
                ReplayNonReproduction::PassedWithoutRepairStanding => {
                    "passed-without-repair-standing".into()
                }
                ReplayNonReproduction::FingerprintMoved => "fingerprint-moved".into(),
                ReplayNonReproduction::TrialMoved => "trial-moved".into(),
                ReplayNonReproduction::DidNotConclude => "did-not-conclude".into(),
            },
        ),
    }
}

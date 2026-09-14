//! Historical offers retain concrete grounds without acquiring admission authority.

use super::{
    Presentation, context, descriptor, historical,
    mutation::historical_target,
    value::{array, hex, object, tagged},
};
use crate::harness::muterprater::proposal_archive::{
    ArchivedDischargeGround, ArchivedKillGround, ArchivedPinGround, ArchivedProposal,
    ArchivedProposalGround,
};
use crate::harness::muterprater::{NoComparisonReason, ObligationLane};
use serde_json::Value;

/// Project a historical offer with its complete concrete ground and comparison.
pub fn archived_proposal(record: &ArchivedProposal) -> Presentation {
    let ground = match record.ground() {
        ArchivedProposalGround::MutantKilled(value) => tagged("mutant-killed", kill(value)),
        ArchivedProposalGround::ClaimPinned(value) => tagged("claim-pinned", pin(value)),
        ArchivedProposalGround::ObligationDischarged(value) => {
            tagged("obligation-discharged", discharge(value))
        }
    };
    Presentation::projected(
        "proposal",
        "macroonz-harness/muterprater/proposal",
        "historical-unauthenticated",
        object([
            ("archive_address", hex(record.address().as_bytes())),
            ("identity", hex(record.identity().as_bytes())),
            ("candidate", descriptor::candidate(record.candidate())),
            (
                "destination",
                context::historical_name(record.destination()),
            ),
            ("ground", ground),
        ]),
    )
}

fn kill(record: &ArchivedKillGround) -> Value {
    object([
        ("target", historical_target::target(record.target())),
        (
            "activation",
            historical_target::activation(record.activation()),
        ),
        ("capsule", historical::capsule_value(record.capsule())),
        ("report", historical::run_value(record.report())),
        (
            "trial_report",
            historical::trial_value(record.trial_report()),
        ),
        ("rejection", historical::finding(record.rejection())),
        (
            "comparison",
            object([
                (
                    "candidate",
                    historical::fingerprint(record.rejection().fingerprint()),
                ),
                (
                    "known",
                    array(record.known().iter().map(historical::fingerprint)),
                ),
            ]),
        ),
    ])
}

fn pin(record: &ArchivedPinGround) -> Value {
    let comparison = match ArchivedPinGround::comparison() {
        NoComparisonReason::GroundCarriesNoFailure => "ground-carries-no-failure",
        NoComparisonReason::NoKnownMaterial => "no-known-material",
    };
    object([
        ("claim", context::historical_name(record.claim())),
        ("capsule", historical::capsule_value(record.capsule())),
        ("before", record.before().into()),
        ("after", record.after().into()),
        ("comparison", tagged(comparison, Value::Null)),
    ])
}

fn discharge(record: &ArchivedDischargeGround) -> Value {
    let lane = match record.lane() {
        ObligationLane::TestRow => "test-row",
        ObligationLane::FuzzSeed => "fuzz-seed",
        ObligationLane::ChaosScenario => "chaos-scenario",
    };
    object([
        ("owed", context::historical_name(record.owed())),
        ("opening_condition", record.opening_condition().into()),
        ("lane", lane.into()),
        ("trial", hex(record.trial().as_bytes())),
        ("key", context::historical_execution(record.key())),
        (
            "comparison",
            object([
                ("owed", context::historical_name(record.compared_owed())),
                ("recorded", Value::Array(Vec::new())),
            ]),
        ),
    ])
}

//! Reached reduction work and search limits without execution or capsule construction.

use super::value::{array, hex, object, tagged};
use super::{Presentation, context, historical, outcome, record::profile};
use crate::harness::generate::reduce::archive::{ArchivedReduction, ArchivedSemanticReducer};
use crate::harness::generate::{
    ByteReducerExecution, ByteReducerId, ReductionEvidence, ReductionHalt, SemanticReducerExecution,
};
use serde_json::Value;

/// Project a completed search's reached witness, candidate census and participant standing.
pub fn reduction(value: &ReductionEvidence) -> Presentation {
    let result = value.outcome();
    let counts = result.census();
    Presentation::projected(
        "reduction",
        "macroonz-harness/generate/reduction",
        "recorded",
        object([
            ("execution", context::execution(value.standing().key())),
            (
                "report_posture",
                context::posture(value.standing().replay()),
            ),
            (
                "generation",
                profile(value.generation().name(), value.generation().version()),
            ),
            (
                "minimization",
                profile(value.minimization().name(), value.minimization().version()),
            ),
            ("schema", hex(value.schema().address().as_bytes())),
            ("probe", context::revision(value.probe_revision())),
            ("budget", value.budget().probes().into()),
            (
                "semantic_reducers",
                array(value.semantic_reducers().iter().copied().map(semantic)),
            ),
            ("byte_reducer", byte_reducer(value.byte_reducer())),
            (
                "outcome",
                object([
                    ("input", hex(result.input())),
                    ("fingerprint", outcome::fingerprint(result.fingerprint())),
                    (
                        "census",
                        census(
                            counts.accepted(),
                            counts.fingerprint_moved(),
                            counts.no_failure(),
                            counts.probes(),
                        ),
                    ),
                    ("halt", halt(result.halt())),
                ]),
            ),
            ("posture", context::posture(value.replay_posture())),
        ]),
    )
}

/// Project an admitted historical reduction without claiming global minimality or replay.
pub fn archived_reduction(value: &ArchivedReduction) -> Presentation {
    let capsule = value.capsule();
    let counts = value.census();
    Presentation::projected(
        "reduction",
        "macroonz-harness/generate/reduction",
        "historical-unauthenticated",
        object([
            ("archive_address", hex(value.address().as_bytes())),
            ("capsule_archive_address", hex(capsule.address().as_bytes())),
            ("capsule_identity", hex(capsule.identity().as_bytes())),
            ("execution", context::historical_execution(capsule.key())),
            ("report_posture", context::posture(value.report_posture())),
            (
                "generation",
                profile(capsule.generation().name(), capsule.generation().version()),
            ),
            (
                "minimization",
                profile(
                    capsule.minimization().name(),
                    capsule.minimization().version(),
                ),
            ),
            ("schema", hex(capsule.schema().as_bytes())),
            (
                "probe",
                object([
                    ("address", hex(value.probe_revision().as_bytes())),
                    ("posture", context::posture(value.probe_posture())),
                ]),
            ),
            ("budget", value.budget().probes().into()),
            (
                "semantic_reducers",
                array(value.semantic_reducers().iter().map(historical_semantic)),
            ),
            ("byte_reducer", byte_reducer(value.byte_reducer())),
            (
                "outcome",
                object([
                    ("input", hex(capsule.input())),
                    (
                        "fingerprint",
                        historical::fingerprint(capsule.fingerprint()),
                    ),
                    (
                        "census",
                        census(
                            counts.accepted(),
                            counts.fingerprint_moved(),
                            counts.no_failure(),
                            counts.probes(),
                        ),
                    ),
                    ("halt", halt(value.halt())),
                ]),
            ),
            ("posture", context::posture(capsule.claimed_posture())),
        ]),
    )
}

fn semantic(value: SemanticReducerExecution) -> Value {
    object([
        ("name", context::declared_name(value.reducer().name())),
        ("revision", context::revision(value.revision())),
        ("candidates", value.candidates().into()),
        ("probes", value.probes().into()),
    ])
}

fn historical_semantic(value: &ArchivedSemanticReducer) -> Value {
    object([
        (
            "name",
            context::name(value.name().namespace(), value.name().stem()),
        ),
        (
            "revision",
            object([
                ("address", hex(value.revision().as_bytes())),
                ("posture", context::posture(value.claimed_posture())),
            ]),
        ),
        ("candidates", value.candidates().into()),
        ("probes", value.probes().into()),
    ])
}

fn byte_reducer(value: ByteReducerExecution) -> Value {
    match value {
        ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing) => {
            tagged("executed", "chunk-removal-and-zeroing".into())
        }
        ByteReducerExecution::NotReachedBecauseBudgetSpent => {
            tagged("not-reached-because-budget-spent", Value::Null)
        }
    }
}

fn halt(value: ReductionHalt) -> Value {
    match value {
        ReductionHalt::FixedPointReached => "fixed-point-reached",
        ReductionHalt::BudgetExhausted => "budget-exhausted",
    }
    .into()
}

fn census(accepted: u32, fingerprint_moved: u32, no_failure: u32, probes: u32) -> Value {
    object([
        ("accepted", accepted.into()),
        ("fingerprint_moved", fingerprint_moved.into()),
        ("no_failure", no_failure.into()),
        ("probes", probes.into()),
    ])
}

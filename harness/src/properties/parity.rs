//! The parity law: two roads to one meaning, driven with one input, demanded to agree.
//!
//! Wherever one meaning is reachable two ways — a maintained result and its recomputed fold, a fused implementation and the steps it fuses, a live run and its reproduction, a generated artifact and its hand-written twin — the suite drives both roads with the same input and demands agreement.
//! The law pins the meaning while leaving both roads free to change.
//!
//! Agreement is silence about everything the two roads share, which is why a suite cannot be built without stating what it stands on.
//!
//! A disagreement names the pair, never the culprit.

use super::conclude::agreement;
use super::types::{
    FUSED_VERSUS_SEPARATE_DISAGREEMENT, LIVE_VERSUS_REPLAYED_DISAGREEMENT, ParityReading,
    ParitySuite, RoadPairing,
};
use crate::report::FindingCause;

/// Drive both of a suite's roads with one input and retain the evidence behind their agreement or disagreement.
#[must_use]
#[track_caller]
pub fn parity<'suite, 'input, Input, Meaning>(
    suite: &'suite ParitySuite<Input, Meaning>,
    input: &'input Input,
) -> ParityReading<'suite, 'input, Input, Meaning> {
    let left = (suite.left())(input);
    let right = (suite.right())(input);
    let conclusion = agreement(suite.same(), &left, &right, cited(suite.pairing()));
    ParityReading::from_run(suite, input, left, right, conclusion)
}

/// The typed cause one pairing's disagreement is cited under.
///
/// The declared arm cites the owner's own name, so a pairing this home has no shape for still reaches a fingerprint that tells it apart from every other pair.
fn cited(pairing: RoadPairing) -> FindingCause {
    match pairing {
        RoadPairing::FusedVersusSeparate => FUSED_VERSUS_SEPARATE_DISAGREEMENT,
        RoadPairing::LiveVersusReplayed => LIVE_VERSUS_REPLAYED_DISAGREEMENT,
        RoadPairing::Declared(name) => {
            FindingCause::named(name.namespace().written(), name.stem().written())
        }
    }
}

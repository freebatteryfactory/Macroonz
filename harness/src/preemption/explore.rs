//! The one road that runs loom: declared bounds in, a typed reading out, with the boundary's own panic discipline between.
//!
//! The model is a bare function pointer, like every owner-supplied seam in this harness; that shape is a stated caller contract — a model reads no ambient fact — and not a structural proof, because a function body can reach the environment on its own.
//! Loom's finding arrives as a panic; the catch at this boundary is what turns it into a value instead of a crashed runner.
//!
//! Every result-affecting seat of loom's builder is set explicitly below, because loom's own constructor reads its `LOOM_*` environment: left standing, an ambient permutation ceiling or wall-clock bound could end exploration before a single schedule ran and still return cleanly — a zero-execution pass wearing the exhaustive claim.
//! The one ambient entrance that remains is the constructor itself, which panics on an unparseable `LOOM_*` spelling before this road holds a builder to correct; that break is loud rather than false, and its typed repair is a separate ruling.

use super::types::{
    MODEL_BROKE, PreemptionBound, PreemptionBounds, PreemptionReading, PreemptionVerdict,
};
use crate::report::{FailureClass, FindingLocation, ForeignText, TrialConclusion, TrialFinding};
use core::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Explore every interleaving of one loom model under the declared bounds.
///
/// Every seat of the builder is set explicitly — the two this home's bounds own, and every other seat at the value a clean environment would give it — so a valid `LOOM_*` variable can no longer change what is explored or end exploration early.
/// The seat roster below is complete for the pinned loom, whose exact version the lane beside this home witnesses; a permutation ceiling, a wall-clock bound, or a checkpoint file left ambient is how a zero-execution run could return cleanly and wear the exhaustive claim.
/// A clean return is loom's exhaustive statement about the bounded space; a caught panic is the model breaking, carried out as the verdict's foreign report.
#[must_use]
pub fn explored(bounds: PreemptionBounds, model: fn()) -> PreemptionReading {
    let mut builder = loom::model::Builder::new();
    builder.max_threads = loom::MAX_THREADS;
    builder.max_branches = usize::try_from(bounds.branches()).unwrap_or(usize::MAX);
    builder.max_permutations = None;
    builder.max_duration = None;
    builder.preemption_bound = match bounds.preemptions() {
        PreemptionBound::Exhaustive => None,
        PreemptionBound::AtMost(depth) => Some(usize::try_from(depth).unwrap_or(usize::MAX)),
    };
    builder.checkpoint_file = None;
    builder.checkpoint_interval = 20_000usize;
    builder.expect_explicit_explore = false;
    builder.location = false;
    builder.log = false;
    let outcome = catch_unwind(AssertUnwindSafe(move || builder.check(model)));
    let verdict = match outcome {
        Ok(()) => PreemptionVerdict::AllInterleavingsHeld,
        Err(payload) => PreemptionVerdict::ModelBroke {
            report: payload_text(payload.as_ref())
                .map(|text| ForeignText::admitted(text.as_bytes())),
        },
    };
    PreemptionReading::read(bounds, verdict)
}

/// Read one exploration into the trial conclusion its evidence earns.
///
/// A clean walk of the bounded space concludes as a pass; a broken model concludes as a refusal under [`MODEL_BROKE`](crate::preemption::MODEL_BROKE), classed as the subject's own panic — which is how loom's finding arrives — with loom's report riding the finding as foreign text.
#[must_use]
pub fn concluded(reading: &PreemptionReading) -> TrialConclusion {
    match reading.verdict() {
        PreemptionVerdict::AllInterleavingsHeld => TrialConclusion::Passed,
        PreemptionVerdict::ModelBroke { report } => {
            TrialConclusion::Refused(TrialFinding::established(
                FailureClass::SubjectPanic,
                MODEL_BROKE,
                FindingLocation::at(file!(), line!()),
                report.clone(),
            ))
        }
    }
}

/// The panic payload, in the two shapes a payload is safely readable in.
///
/// A payload of any other type reads as absent rather than through a road that would have to guess at its bytes.
fn payload_text(payload: &(dyn Any + Send)) -> Option<&str> {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

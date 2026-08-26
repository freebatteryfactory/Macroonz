//! The qualified Loom 0.7.2 implementation of the preemption explore road.

use super::types::{
    IncompleteExploration, PreemptionBound, PreemptionBounds, PreemptionModelFailure,
    PreemptionModelResult, PreemptionOutcome, PreemptionReading, PreemptionVerdict,
};
use crate::report::ForeignText;
use core::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// The private payload that proves an unwind came from a typed model return rather than foreign panic text.
#[derive(Debug)]
struct ModelRefusal(PreemptionModelFailure);

/// Run the pinned backend with every result-affecting builder seat forced explicitly.
///
/// The declaration supplies branch and preemption ceilings.
/// The road fixes the backend's thread ceiling, removes permutation, duration, and checkpoint early stops, restores the pinned checkpoint interval, disables explicit-explore, location, and log modes, and admits no ambient value after construction.
#[must_use]
pub(super) fn explored(
    bounds: PreemptionBounds,
    model: fn() -> PreemptionModelResult,
) -> PreemptionReading {
    let builder = catch_unwind(loom::model::Builder::new);
    let Ok(mut builder) = builder else {
        let report = builder
            .err()
            .and_then(|payload| foreign_panic_report(payload.as_ref()));
        return PreemptionReading::read(
            bounds,
            PreemptionOutcome::Incomplete(IncompleteExploration::InitializationFailed { report }),
        );
    };

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

    let outcome = catch_unwind(AssertUnwindSafe(move || scheduled(&builder, model)));
    let outcome = match outcome {
        Ok(()) => PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld),
        Err(payload) => match payload.downcast::<ModelRefusal>() {
            Ok(failure) => PreemptionOutcome::Completed(PreemptionVerdict::ModelBroke {
                report: failure.0.report().cloned(),
            }),
            Err(payload) => {
                let report = foreign_panic_report(payload.as_ref());
                PreemptionOutcome::Incomplete(IncompleteExploration::ExecutionUnresolved { report })
            }
        },
    };
    PreemptionReading::read(bounds, outcome)
}

/// Walk the schedules admitted by the configured builder.
fn scheduled(builder: &loom::model::Builder, model: fn() -> PreemptionModelResult) {
    builder.check(move || checked(model));
}

/// Turn one typed model refusal into the backend's private stop signal.
fn checked(model: fn() -> PreemptionModelResult) {
    if let Err(failure) = model() {
        resume_unwind(Box::new(ModelRefusal(failure)));
    }
}

/// Admit a foreign unwind payload only in the two standard text shapes, without interpreting its words.
fn foreign_panic_report(payload: &(dyn Any + Send)) -> Option<ForeignText> {
    let text = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str));
    text.map(|material| ForeignText::admitted(material.as_bytes()))
}

//! The one road that runs loom: declared bounds in, a typed reading out, with the boundary's own panic discipline between.
//!
//! The model is a bare function pointer, like every owner-supplied seam in this harness, so nothing ambient rides in with it.
//! Loom's finding arrives as a panic; the catch at this boundary is what turns it into a value instead of a crashed runner.

use super::types::{PreemptionBound, PreemptionBounds, PreemptionReading, PreemptionVerdict};
use crate::report::ForeignText;
use core::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Explore every interleaving of one loom model under the declared bounds.
///
/// The seats this home owns — the preemption bound and the branch budget — are set explicitly on the builder and always win over anything ambient.
/// A clean return is loom's exhaustive statement about the bounded space; a caught panic is the model breaking, carried out as the verdict's foreign report.
#[must_use]
pub fn explored(bounds: PreemptionBounds, model: fn()) -> PreemptionReading {
    let mut builder = loom::model::Builder::new();
    builder.preemption_bound = match bounds.preemptions() {
        PreemptionBound::Exhaustive => None,
        PreemptionBound::AtMost(depth) => Some(usize::try_from(depth).unwrap_or(usize::MAX)),
    };
    builder.max_branches = usize::try_from(bounds.branches()).unwrap_or(usize::MAX);
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

/// The panic payload, in the two shapes a payload is safely readable in.
///
/// A payload of any other type reads as absent rather than through a road that would have to guess at its bytes.
fn payload_text(payload: &(dyn Any + Send)) -> Option<&str> {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

//! One caller-owned counter compiled with ordinary or modeled synchronization.

mod increment;
mod observation;
#[path = "synchronization/mod.rs"]
mod synchronization;

use std::io::Write;

#[cfg(not(loom))]
fn main() -> Result<(), String> {
    observation::sequential()?;
    observation::concurrent(increment::indivisible)?;
    std::io::stdout()
        .write_all(b"ordinary: sequential check and concurrent repair agree\n")
        .map_err(|error| error.to_string())
}

#[cfg(loom)]
fn main() -> Result<(), String> {
    use macroonz::harness::preemption::{
        PreemptionBound, PreemptionBounds, PreemptionOutcome, PreemptionVerdict, explored,
    };

    let bounds = PreemptionBounds::declared(PreemptionBound::AtMost(2u32), 1_000u32)
        .map_err(|error| format!("{error:?}"))?;
    let sequential = explored(bounds, observation::sequential_model);
    if sequential.outcome()
        != &PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld)
    {
        return Err(format!("sequential control: {:?}", sequential.outcome()));
    }
    let broken = explored(bounds, observation::separate_model);
    if !matches!(
        broken.outcome(),
        PreemptionOutcome::Completed(PreemptionVerdict::ModelBroke { report: Some(report) })
            if report.bytes() == b"two completed increments must leave two"
    ) {
        return Err(format!("lost-update control: {:?}", broken.outcome()));
    }
    let repaired = explored(bounds, observation::indivisible_model);
    if repaired.outcome() != &PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld)
    {
        return Err(format!("indivisible repair: {:?}", repaired.outcome()));
    }
    std::io::stdout()
        .write_all(
            b"shadow: sequential check holds; lost update caught; repair holds within bounds\n",
        )
        .map_err(|error| error.to_string())
}

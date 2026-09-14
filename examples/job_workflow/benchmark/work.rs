//! Executed Job dispatch calls and a deliberately repeated workload.

use crate::job::{Event, Stage, baked};
use macroonz::harness::bench::{WorkObservationRef, WorkRecorder, WorkRecordingRefusal};

pub(super) fn counted_dispatches(
    size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = WorkObservationRef::named("neutral-job", "dispatch-invocations")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    for _ in 0..size {
        let _queued = std::hint::black_box(baked::apply(Stage::Draft, Event::Queue));
        recorder.record(observation, 1)?;
        let _completed = std::hint::black_box(baked::apply(Stage::Queued, Event::Complete));
        recorder.record(observation, 1)?;
    }
    Ok(())
}

pub(super) fn quadratic_dispatches(
    size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    for _ in 0..size {
        counted_dispatches(size, recorder)?;
    }
    Ok(())
}

//! Running a bound callable into a work curve, and handing curves to the owner's judge.

use super::types::{
    BenchBinding, BenchCall, BenchInvocation, SecondaryObservation, SecondaryObservationRefusal,
    WorkCurve, WorkJudgment, WorkJudgmentInput, WorkRecorder, WorkRecordingRefusal,
};

/// Record one callable's work at every size on the row's axis.
///
/// # Errors
///
/// Refuses whatever the callable's own recorder refused, at the first size that refuses it.
pub(super) fn curve(
    call: BenchCall,
    binding: &BenchBinding,
) -> Result<WorkCurve, WorkRecordingRefusal> {
    let row = binding.row();
    let mut points = Vec::new();
    for input_size in row.input_sizes().sizes().iter().copied() {
        let mut recorder = WorkRecorder::scoped(binding.attachment().observations());
        for _sample in 0..row.budgets().samples() {
            call(input_size, &mut recorder)?;
        }
        points.push(recorder.finish(input_size));
    }
    Ok(WorkCurve::recorded(points))
}

/// Ask the row's own judge to read a pair of curves together.
pub(super) fn judge(
    binding: &BenchBinding,
    measured: &WorkCurve,
    planted_worse: &WorkCurve,
) -> WorkJudgment {
    let row = binding.row();
    let input = WorkJudgmentInput::over(
        row.formula(),
        row.complexity(),
        row.budgets(),
        measured,
        planted_worse,
    );
    (binding.attachment().judge().judge())(&input)
}

/// Take the timed pass: warmups discarded, one clock measurement per sample, then judged again.
///
/// The control's curve is the one already recorded, so the clock is read for the measured callable and for nothing else.
///
/// # Errors
///
/// Refuses a warmup or a sample that could not record, then a pass that no longer qualifies under the same judge.
pub(super) fn timed_pass(
    binding: &BenchBinding,
    invocation: &BenchInvocation,
    planted_worse: &WorkCurve,
) -> Result<SecondaryObservation, SecondaryObservationRefusal> {
    let row = binding.row();
    let attachment = binding.attachment();
    let mut points = Vec::new();
    let mut measurements = Vec::new();
    for input_size in row.input_sizes().sizes().iter().copied() {
        for _warmup in 0..row.budgets().warmups() {
            let mut discarded = WorkRecorder::scoped(attachment.observations());
            (attachment.measured())(input_size, &mut discarded)
                .map_err(SecondaryObservationRefusal::Warmup)?;
        }
        let mut recorder = WorkRecorder::scoped(attachment.observations());
        for _sample in 0..row.budgets().samples() {
            let measurement = invocation.clock().begin();
            (attachment.measured())(input_size, &mut recorder)
                .map_err(SecondaryObservationRefusal::Sample)?;
            measurements.push(measurement.finish());
        }
        points.push(recorder.finish(input_size));
    }
    let timed = WorkCurve::recorded(points);
    let judgment = judge(binding, &timed, planted_worse);
    SecondaryObservation::recorded(timed, judgment, measurements)
}

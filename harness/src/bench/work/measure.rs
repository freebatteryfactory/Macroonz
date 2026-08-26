//! Recording primary work curves and handing them to the owner's judge.

use super::super::declaration::BenchRow;
use super::{
    BenchAttachment, BenchCall, WorkCurve, WorkJudgment, WorkJudgmentInput, WorkRecorder,
    WorkRecordingRefusal,
};

/// Record one callable's work at every size on the row's axis.
///
/// # Errors
///
/// Refuses whatever the callable's own recorder refused, at the first size that refuses it.
pub(in crate::bench) fn curve(
    call: BenchCall,
    row: &BenchRow,
    attachment: &BenchAttachment,
) -> Result<WorkCurve, WorkRecordingRefusal> {
    let mut points = Vec::new();
    for input_size in row.input_sizes().sizes().iter().copied() {
        let mut recorder = WorkRecorder::scoped(attachment.observations());
        for _sample in 0..row.budgets().samples() {
            call(input_size, &mut recorder)?;
        }
        points.push(recorder.finish(input_size));
    }
    Ok(WorkCurve::recorded(points))
}

/// Ask the row's own judge to read a pair of curves together.
pub(in crate::bench) fn judge(
    row: &BenchRow,
    attachment: &BenchAttachment,
    measured: &WorkCurve,
    planted_worse: &WorkCurve,
) -> WorkJudgment {
    let input = WorkJudgmentInput::over(
        row.formula(),
        row.complexity(),
        row.budgets(),
        measured,
        planted_worse,
    );
    (attachment.judge().judge())(&input)
}

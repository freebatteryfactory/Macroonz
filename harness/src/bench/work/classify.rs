//! The shared control-before-measured classification of three informed work readings.

use super::types::WorkStage;

pub(in crate::bench) const fn stage<Cause>(
    measured: Result<(), &Cause>,
    planted_worse: Result<(), &Cause>,
    gap: Result<(), &Cause>,
) -> WorkStage {
    match (measured, planted_worse, gap) {
        (_, Ok(()), _) | (_, _, Err(_)) => WorkStage::ControlNotDistinguished,
        (Err(_), Err(_), Ok(())) => WorkStage::MeasuredRefused,
        (Ok(()), Err(_), Ok(())) => WorkStage::Qualified,
    }
}

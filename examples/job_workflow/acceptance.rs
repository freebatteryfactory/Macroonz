//! This caller requires every declared trial for its complete-check result.

use macroonz::harness::report::RunReport;
use macroonz::harness::runner::{SeatOutcome, seat_verdict};

pub(super) fn all_declared(report: &RunReport) -> Result<(), String> {
    match seat_verdict(report) {
        Ok(SeatOutcome::EveryTrialConcluded {
            selected,
            denominator,
        }) if selected == denominator => Ok(()),
        Ok(SeatOutcome::EveryTrialConcluded { .. }) => {
            Err("required declared trials were not selected".to_owned())
        }
        Ok(SeatOutcome::NoWorkAsStated { .. }) => {
            Err("required declared trials were not executed".to_owned())
        }
        Err(refusal) => Err(format!("required trial execution refused: {refusal:?}")),
    }
}

#![doc = include_str!("README.md")]

mod assemble;
mod catch;
mod execute;
mod record;
mod replay;
mod resolve;
mod select;
mod types;
mod verdict;

pub use execute::{run_all, run_one};
pub use record::{record_all, record_one};
pub use replay::{replay, replay_legacy};
pub use resolve::{execution_key, trial_identity};
pub use types::{
    FailedTrial, Invocation, LegacyReplayRefusal, LegacyReplayedTrial, ReplayedTrial,
    ReportRecordingRefusal, SUBJECT_PANIC_CAUSE, SeatFailure, SeatOutcome, SeatRefusal, Selection,
    SelectionPlan, TrialBinding, TrialCall, TrialTable, TrialTableView,
};
pub use verdict::{lens_verdict, seat_verdict};

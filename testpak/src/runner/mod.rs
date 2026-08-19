#![doc = include_str!("README.md")]

mod catch;
mod execute;
mod resolve;
mod select;
mod types;
mod verdict;

pub use execute::{run_all, run_one};
pub use resolve::{execution_key, trial_identity};
pub use types::{
    FailedTrial, HostClock, Invocation, SUBJECT_PANIC_CAUSE, SeatFailure, SeatRefusal, Selection,
    TrialBinding, TrialCall, TrialTable, TrialTableView,
};
pub use verdict::{lens_verdict, seat_verdict};

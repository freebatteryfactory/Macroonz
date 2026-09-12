#![doc = include_str!("README.md")]

mod capture;
mod execute;
mod platform;
mod type_contract;
mod types;

pub use execute::run;
pub use types::{
    CaptureEnd, CapturedOutput, PendingProcess, ProcessError, ProcessLimits, ProcessOutput,
    ProcessRequest, ProcessRun, ProcessStop, ProcessTool, ResourceControl,
};

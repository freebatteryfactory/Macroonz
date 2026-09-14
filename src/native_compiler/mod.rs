#![doc = include_str!("README.md")]

mod decode;
mod dependencies;
mod diagnose;
mod execute;
mod read_back;
mod source;
mod type_contract;
mod types;

pub use dependencies::{DependencyError, DependencyInfo, DependencyLimits};
pub use execute::compile;
pub use read_back::{compared_read_back, observed_read_back};
pub use types::{
    CargoFixture, CargoTarget, CompilerError, CompilerObservationError, CompilerOutput,
    CompilerRequest, CompilerRun, PendingCompilation, ReadBackError,
};

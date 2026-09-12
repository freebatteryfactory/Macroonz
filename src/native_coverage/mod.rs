#![doc = include_str!("README.md")]

mod cleanup;
mod execute;
mod preflight;
mod type_contract;
mod types;

pub use execute::observe;
pub use preflight::preflight;
pub use types::{NativeCoverage, NativeCoverageFailure, NativeCoverageProcessError};

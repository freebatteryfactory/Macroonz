#![doc = include_str!("README.md")]

mod read;
mod type_contract;
mod types;

pub(super) use read::capture;
pub use types::{DependencyError, DependencyInfo, DependencyLimits};

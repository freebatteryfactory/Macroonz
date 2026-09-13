#![doc = include_str!("README.md")]

#[cfg(feature = "native-tooling")]
pub mod benchmark;

#[cfg(feature = "native-tooling")]
mod load;
#[cfg(feature = "native-tooling")]
mod retain;
mod select;
mod types;

pub use select::{select_suites, suites};
pub use types::{InputRun, SuiteSelectionRefusal, run};
#[cfg(feature = "native-tooling")]
pub use types::{RetentionLimits, RetentionRefusal, StoredRun};

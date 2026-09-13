#![doc = include_str!("README.md")]

pub(super) mod types;

mod plan;

pub use plan::{admission, admission_for, planned, unrealized_families};

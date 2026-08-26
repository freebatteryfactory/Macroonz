#![doc = include_str!("README.md")]

pub(super) mod types;

mod plan;

pub use plan::{admission, planned, unrealized_families};

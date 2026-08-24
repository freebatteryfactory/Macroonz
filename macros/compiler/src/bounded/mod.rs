#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{Bounded, Capped, Capping, Empty, NonEmpty, NonEmptyError, Overflow};

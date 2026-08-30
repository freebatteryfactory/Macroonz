#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    Bounded, Capped, Capping, DuplicateKey, Empty, KeyedRoster, KeyedRosterError, NonEmpty,
    NonEmptyError, Overflow,
};

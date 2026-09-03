#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub(crate) use types::first_duplicate_position;
pub use types::{
    Bounded, Capped, Capping, DuplicateKey, Empty, ForeignRosterReference, KeyedRoster,
    KeyedRosterAssignment, KeyedRosterAssignmentError, KeyedRosterError, NonEmpty, NonEmptyError,
    Overflow, UnassignedRosterMember,
};

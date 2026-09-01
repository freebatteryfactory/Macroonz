#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    Bounded, Capped, Capping, DuplicateKey, Empty, ForeignRosterReference, KeyedRoster,
    KeyedRosterAssignment, KeyedRosterAssignmentError, KeyedRosterError, KeyedRosterRelation,
    KeyedRosterRows, KeyedRosterRowsError, NonEmpty, NonEmptyError, Overflow, RepeatedRelationPair,
    RepeatedRelationPairs, UnassignedRosterMember,
};

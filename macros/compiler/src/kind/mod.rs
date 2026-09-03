#![doc = include_str!("README.md")]

mod join;
mod type_contract;
mod types;

pub(crate) use join::{JoinOrder, rows_to, rows_under};
pub(crate) use type_contract::roster_row;
pub use types::{
    Answer, CanonicalContent, Destination, Disposition, DispositionRecord, DispositionSet,
    DispositionSetError, Kind, KindSet, NoQuestions, Question, Role, SoleRole,
};

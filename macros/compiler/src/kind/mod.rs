#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub(crate) use type_contract::roster_row;
pub use types::{
    Answer, CanonicalContent, Destination, Disposition, DispositionRecord, DispositionSet,
    DispositionSetError, Kind, KindSet, NoQuestions, Question, Role, SoleRole,
};

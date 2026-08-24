#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    Answer, Destination, Disposition, Kind, KindSet, NoQuestions, Question, Role, SoleRole,
};

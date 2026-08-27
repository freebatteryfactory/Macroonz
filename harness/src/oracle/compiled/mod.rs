#![doc = include_str!("README.md")]

mod compare;
mod conclude;
mod types;

pub use compare::compared;
pub use types::{
    CompiledDisagreement, CompiledObservation, CompiledVerdict, DeclaredBehavior, DeclaredReadBack,
    DeclaredReadBackRoster, DeclaredReadBackRosterRefusal, ObservedMember, ObservedValue,
};

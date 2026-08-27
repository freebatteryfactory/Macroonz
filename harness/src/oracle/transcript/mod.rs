#![doc = include_str!("README.md")]

mod conclude;
mod types;

pub use types::{
    ContextRefusal, DerivedIdentity, SpecifiedContext, TranscriptDerivation,
    TranscriptDisagreement, TranscriptMember, TranscriptVerdict,
};

#![doc = include_str!("README.md")]

mod execute;
mod observe;
mod type_contract;
mod types;

pub use types::{
    FormatError, FormatObservationError, FormatOutput, FormatRun, Formatter, PendingFormat,
};

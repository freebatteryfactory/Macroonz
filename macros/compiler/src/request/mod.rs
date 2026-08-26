#![doc = include_str!("README.md")]

mod decide;
mod explain;
mod types;

pub use decide::{bound_content, committed};
pub use types::{CrateBinding, Door, Producer, RUST_DECLARATION_PROFILE, Request, SELECTION_FACT};

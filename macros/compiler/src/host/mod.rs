#![doc = include_str!("README.md")]

mod capture;
mod emit;
mod encode;
mod expand;
mod place;
mod type_contract;
mod types;

pub use capture::capture;
pub use emit::{emit, emit_tree};
pub use expand::{expand, expand_on};
pub use place::place;
pub use types::{CaptureError, EmissionError, Emittable, Spans};

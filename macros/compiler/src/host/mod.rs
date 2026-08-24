#![doc = include_str!("README.md")]

mod capture;
mod emit;
mod encode;
mod expand;
mod place;
mod type_contract;
mod types;

pub use capture::capture;
pub use emit::emit;
pub use expand::expand;
pub use place::place;
pub use types::{CaptureError, Emittable, Spans};

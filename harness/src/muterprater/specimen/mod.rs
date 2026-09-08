#![doc = include_str!("README.md")]

pub(crate) mod types;
pub mod archive;

mod demonstrate;

pub use demonstrate::demonstrate_compiled_projection;

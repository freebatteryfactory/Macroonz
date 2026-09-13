#![doc = include_str!("README.md")]

pub(crate) mod types;
pub mod archive;

mod demonstrate;
mod observe;

pub use demonstrate::demonstrate_compiled_projection;
pub use observe::observe_mutation;

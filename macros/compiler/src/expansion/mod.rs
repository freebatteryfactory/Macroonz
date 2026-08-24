#![doc = include_str!("README.md")]

mod encode;
mod type_contract;
mod types;

pub use types::{Accounted, BINDING_FACT, BindError, Expansion};

#![doc = include_str!("README.md")]

pub mod capsules;
#[path = "operator_family/bank.rs"]
pub mod operator_families;
#[path = "type_separation/bank.rs"]
pub mod swap_pairs;
pub mod types;

mod operator_family;
mod type_separation;

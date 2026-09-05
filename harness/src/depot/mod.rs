#![doc = include_str!("README.md")]

pub mod capsules;
#[path = "operator_family/bank.rs"]
pub mod operator_families;
#[path = "type_separation/bank.rs"]
pub mod swap_pairs;
pub mod types;

pub(crate) use artifact_mutation::artifact_mutation_bank;
pub(crate) use producer_field::generated_support_field_banks;

mod artifact_mutation;
mod operator_family;
mod producer_field;
mod type_separation;

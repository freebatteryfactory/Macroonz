#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub(in crate::recipe) use types::{
    ExactFunctionIssue, ExactProjectionSeat, RecipeError, RecipeIssue,
};

//! The recipe refusal vocabulary: one issue roster, one refusal carrier, and their diagnostic contracts.

mod type_contract;
mod types;

pub(in crate::recipe) use types::{
    ExactFunctionIssue, ExactProjectionSeat, RecipeError, RecipeIssue,
};

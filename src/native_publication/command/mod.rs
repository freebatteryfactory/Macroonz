#![doc = include_str!("README.md")]

mod budget;
mod execute;
mod prepare;
mod type_contract;
mod types;
mod workspace;

pub use execute::bake;
pub use types::{
    BakeCause, BakeCommand, BakeError, BakeFormatter, BakeGeneration, BakeOutput, BakePreparation,
    BakeToolBudget,
};

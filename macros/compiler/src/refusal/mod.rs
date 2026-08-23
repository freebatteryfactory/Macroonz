#![doc = include_str!("README.md")]

mod encode;
mod type_contract;
mod types;

pub use types::{
    BoundAxis, ContradictionPair, PlanningIssueLimit, ProjectionPlanning, ProjectionPlanningIssue,
};

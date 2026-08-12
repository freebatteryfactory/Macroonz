#![doc = include_str!("README.md")]

mod establish;
mod type_contract;
mod types;

pub use types::{
    BoundAxis, ContradictionPair, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue,
};

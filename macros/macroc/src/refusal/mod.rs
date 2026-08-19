#![doc = include_str!("README.md")]

mod encode;
mod type_contract;
mod types;

pub use types::{
    BoundAxis, ContradictionPair, PlanSeat, PlanningIssueLimit, ProjectionPlanning,
    ProjectionPlanningIssue,
};

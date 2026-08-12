#![doc = include_str!("README.md")]

mod establish;
mod project;
mod type_contract;
mod types;

pub use establish::kind_admits;
pub use types::{
    ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue, ProjectionExplanation,
    ProjectionExplanationView,
};

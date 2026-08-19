#![doc = include_str!("README.md")]

mod encode;
mod establish;
mod project;
mod type_contract;
mod types;

pub use establish::kind_admits;
pub use types::{
    ClosureProofSeal, ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue,
    ExplanationIssueLimit, ExplanationSeatLimit, ProjectionExplanation, ProjectionExplanationView,
    ProvedClosure,
};

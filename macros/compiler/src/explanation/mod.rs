#![doc = include_str!("README.md")]

mod encode;
mod establish;
mod project;
mod type_contract;
mod types;

pub use types::{
    ASSUMPTION_LIMIT, DECLARED_QUESTION_LIMIT, EXPLANATION_ISSUE_LIMIT, ExplanationError,
    ExplanationIssue, RELATED_KIND_LIMIT, RelatedDisposition, UNIVERSAL_QUESTION_COUNT,
    UniversalAnswer, UniversalQuestion, View,
};

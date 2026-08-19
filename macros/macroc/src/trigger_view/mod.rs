#![doc = include_str!("README.md")]

mod establish;
mod type_contract;
mod types;

pub use types::{
    SelectionCitationLimit, TriggerCitations, TriggerOmission, TriggerSelection,
    TriggerViewComposition, TriggerViewIssue, TriggerViewIssueLimit, WrapperTriggerView,
};

#![doc = include_str!("README.md")]
mod encode;
mod establish;
mod type_contract;
mod types;
pub use types::{
    ASSEMBLY_FACT, ASSEMBLY_ISSUE_LIMIT, AssemblyError, AssemblyIssue, SupportAssembly,
};

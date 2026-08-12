#![doc = include_str!("README.md")]

mod plan;
mod types;

pub use plan::plan_scope_guard_stamp;
pub use types::{ScopeGuardOwnerFacts, ScopeGuardStampAnchors};

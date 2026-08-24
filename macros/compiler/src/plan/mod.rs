#![doc = include_str!("README.md")]

mod anchor;
mod encode;
mod type_contract;
mod types;

pub use types::{
    Account, BoundAxis, Context, ContradictionPair, DEPENDENCY_LIMIT, DigestContract, Intent,
    InvalidationSet, InvalidationTrigger, MEMBERSHIP_LIMIT, Membership, NONCLAIM_LIMIT,
    PLAN_ISSUE_LIMIT, Plan, PlanDecisions, PlanError, PlanIssue, PlannedMember, PlannedOutput,
    TRIGGER_LIMIT,
};

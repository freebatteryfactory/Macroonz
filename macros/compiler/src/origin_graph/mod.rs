#![doc = include_str!("README.md")]

mod encode;
mod type_contract;
mod types;

pub use types::{
    DecisionTrace, Nonclaim, OriginEdge, OriginEdgeLimit, OriginRelation, OriginTrail,
    TraceDecision, TraceEntry,
};

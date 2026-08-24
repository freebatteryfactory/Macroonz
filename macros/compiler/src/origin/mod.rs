#![doc = include_str!("README.md")]

mod encode;
mod type_contract;
mod types;

pub use types::{
    DecisionTrace, Nonclaim, ORIGIN_EDGE_LIMIT, OriginEdge, OriginRelation, OriginTrail,
    TRACE_ENTRY_LIMIT, TraceDecision, TraceEntry, TrailError,
};

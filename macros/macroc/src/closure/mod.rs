#![doc = include_str!("README.md")]

mod prove;
mod type_contract;
mod types;

pub use types::{
    ClosureIssue, ProjectionClosure, ProjectionClosureRefusal, RenderedProjection, RenderedUnit,
    RenderingRefusal,
};

#![doc = include_str!("README.md")]

mod encode;
mod type_contract;
mod types;

pub use types::{Output, RENDERED_BYTE_LIMIT, RenderError, RenderedProjection, RenderedUnit};

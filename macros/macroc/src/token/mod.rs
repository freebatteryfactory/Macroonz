#![doc = include_str!("README.md")]

mod encode;
mod inspect;
mod literal;
mod resolve;
mod text;
mod types;

pub use literal::capture_literal;
pub use types::{
    CaptureBound, CaptureWalk, CaptureWorkLimit, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, CapturedTreeTokenLimit, GeneratedDelimiter, GeneratedSpacing,
    GeneratedToken, GeneratedTree, LiteralReadCause, SpanHandle, SpanResolutionRefusal, SpanTable,
    TextCapture, TextReadCause, TextReadRefusal, TokenPath, TokenPathDepthLimit,
};

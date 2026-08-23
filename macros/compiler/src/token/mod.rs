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
    CapturedTokenTree, CapturedTreeTokenLimit, CoordinateRole, GeneratedDelimiter,
    GeneratedSpacing, GeneratedToken, GeneratedTree, LiteralReadCause, SourceCoordinate,
    SpanHandle, SpanResolutionRefusal, SpanTable, TextCapture, TextReadCause, TextReadRefusal,
    TokenPath, TokenPathDepthLimit,
};

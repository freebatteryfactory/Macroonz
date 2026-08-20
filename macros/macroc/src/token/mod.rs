#![doc = include_str!("README.md")]

mod encode;
mod inspect;
mod resolve;
mod text;
mod types;

pub use types::{
    CaptureBound, CaptureWalk, CaptureWorkLimit, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, CapturedTreeTokenLimit, GeneratedDelimiter, GeneratedSpacing,
    GeneratedToken, GeneratedTree, SpanHandle, SpanResolutionRefusal, SpanTable, TextCapture,
    TextReadCause, TextReadRefusal, TokenPath, TokenPathDepthLimit,
};

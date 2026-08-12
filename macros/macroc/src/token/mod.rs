#![doc = include_str!("README.md")]

mod encode;
mod inspect;
mod resolve;
mod text;
mod types;

pub use types::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
    SpanHandle, SpanResolutionRefusal, SpanTable, TextCapture, TextReadCause, TextReadRefusal,
    TokenPath,
};

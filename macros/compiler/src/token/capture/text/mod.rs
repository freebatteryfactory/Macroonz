//! The callable text producer over the pinned low-level lexer.

mod read;
mod type_contract;
mod types;

use super::{
    CaptureBound, CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CapturedAtom,
    CapturedDelimiter, CapturedInput, LiteralReadCause, SpanHandle, SpanTable, TokenPath,
    capture_literal,
};
pub use types::{
    TEXT_SOURCE_BYTE_LIMIT, TextCapture, TextLexicalCause, TextReadCause, TextReadRefusal,
};

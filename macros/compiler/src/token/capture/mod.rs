#![doc = include_str!("README.md")]

mod cursor;
mod encode;
mod fragment;
mod item;
mod literal;
mod resolve;
mod text;
mod type_contract;
mod types;

pub use cursor::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedSpacing,
};
#[cfg(feature = "host")]
pub(crate) use encode::encode_token_path;
pub use literal::capture_literal;
pub use types::{
    AuthoredItem, AuthoredItemKind, AuthoredItemReadIssue, AuthoredItemReadRefusal,
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound,
    CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CaptureWalk, CapturedAtom,
    CapturedDelimiter, CapturedFragment, CapturedInput, CapturedPayload, CapturedTokenTree,
    CoordinateRole, LiteralReadCause, SourceCoordinate, SpanHandle, SpanResolutionRefusal,
    SpanTable, TEXT_SOURCE_BYTE_LIMIT, TOKEN_PATH_DEPTH_LIMIT, TextCapture, TextLexicalCause,
    TextReadCause, TextReadRefusal, TokenPath,
};

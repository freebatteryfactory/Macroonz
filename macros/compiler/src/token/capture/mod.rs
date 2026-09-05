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
pub use item::{AuthoredItem, AuthoredItemKind, AuthoredItemReadIssue, AuthoredItemReadRefusal};
pub use literal::capture_literal;
pub use text::{
    TEXT_SOURCE_BYTE_LIMIT, TextCapture, TextLexicalCause, TextReadCause, TextReadRefusal,
};
pub use types::{
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound,
    CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CaptureWalk, CapturedAtom,
    CapturedDelimiter, CapturedFragment, CapturedInput, CapturedPayload, CapturedTokenTree,
    CoordinateRole, LiteralReadCause, SourceCoordinate, SpanHandle, SpanResolutionRefusal,
    SpanTable, TOKEN_PATH_DEPTH_LIMIT, TokenPath,
};

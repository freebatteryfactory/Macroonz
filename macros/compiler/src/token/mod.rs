#![doc = include_str!("README.md")]

mod compose;
mod encode;
mod inspect;
mod literal;
mod resolve;
mod text;
mod type_contract;
mod types;

pub use compose::{
    absolute_path, and_all, attribute, bound_local, bound_path, call, comma, comma_many, constant,
    documentation, equality, function, group, metavariable, method_call, method_chain,
    rendered_identifier, rendered_name, result_type, roster, rust_keyword, text_pair, twin_path,
};
pub use literal::capture_literal;
pub use types::{
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound, CaptureWalk,
    CapturedDelimiter, CapturedInput, CapturedPayload, CapturedTokenTree, CoordinateRole,
    GENERATED_TOKEN_LIMIT, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
    LiteralReadCause, SourceCoordinate, SpanHandle, SpanResolutionRefusal, SpanTable,
    TOKEN_PATH_DEPTH_LIMIT, TextCapture, TextReadCause, TextReadRefusal, TokenPath,
};

#![doc = include_str!("README.md")]

mod capture;
mod generation;

#[cfg(feature = "host")]
pub(crate) use capture::encode_token_path;
pub use capture::{
    AuthoredItem, AuthoredItemKind, AuthoredItemReadIssue, AuthoredItemReadRefusal,
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound,
    CaptureBuildRefusal, CaptureBuilder, CaptureCursor, CaptureExpectation, CaptureLevel,
    CaptureReadIssue, CaptureReadRefusal, CaptureWalk, CapturedAtom, CapturedDelimiter,
    CapturedFragment, CapturedInput, CapturedPayload, CapturedSpacing, CapturedTokenTree,
    CoordinateRole, LiteralReadCause, SourceCoordinate, SpanHandle, SpanResolutionRefusal,
    SpanTable, TEXT_SOURCE_BYTE_LIMIT, TOKEN_PATH_DEPTH_LIMIT, TextCapture, TextLexicalCause,
    TextReadCause, TextReadRefusal, TokenPath, capture_literal,
};
#[cfg(feature = "host")]
pub(crate) use generation::GeneratedLiteralForm;
pub(crate) use generation::segmented_twin_path;
pub use generation::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GENERATED_TOKEN_LIMIT, GeneratedDelimiter,
    GeneratedLiteral, GeneratedLiteralRefusal, GeneratedSpacing, GeneratedToken, GeneratedTree,
    absolute_path, and_all, attribute, bound_local, bound_path, call, comma, comma_many, constant,
    documentation, equality, function, group, keyed_assignment_slice, keyed_roster_slice,
    metavariable, method_call, method_chain, rendered_identifier, rendered_name, result_type,
    roster, rust_keyword, text_pair, twin_path,
};

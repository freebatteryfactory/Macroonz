#![doc = include_str!("README.md")]

mod bank;
mod capture;
mod generation;
mod types;

pub use bank::rust_keyword;
pub(crate) use types::RENDERED_PATH_SEGMENT_LIMIT;

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
pub use generation::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GENERATED_TOKEN_LIMIT, GeneratedDelimiter,
    GeneratedLiteral, GeneratedLiteralRefusal, GeneratedRowRefusal, GeneratedSpacing,
    GeneratedToken, GeneratedTree, absolute_path, and_all, associated_constant,
    associated_function, associated_type, attribute, bound_local, bound_path, call, comma,
    comma_many, constant, consuming_receiver, decorated, documentation, enumeration, equality,
    exclusive_receiver, function, function_item, function_signature, generic_parameters, group,
    implementation, inline_module, keyed_assignment_items, keyed_assignment_slice,
    keyed_roster_items, keyed_roster_slice, match_arm, match_expression, metavariable, method_call,
    method_chain, named_field, named_struct, named_variant, pinned_receiver, rendered_identifier,
    rendered_name, result_type, roster, shared_receiver, text_pair, trait_declaration,
    tuple_struct, tuple_variant, twin_path, type_alias, typed_parameter, unit_struct, unit_variant,
    use_item, where_clause,
};
pub(crate) use generation::{preserved_tree, segmented_twin_path, vector};

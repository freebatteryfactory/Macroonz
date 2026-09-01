#![doc = include_str!("README.md")]

mod behavior;
mod compose;
mod encode;
mod inspect;
mod items;
mod preserve;
mod project;
mod traits;
mod type_contract;
mod types;

pub use behavior::{
    consuming_receiver, exclusive_receiver, function, function_item, function_signature, match_arm,
    match_expression, pinned_receiver, shared_receiver, typed_parameter,
};
pub(crate) use compose::segmented_twin_path;
pub use compose::{
    absolute_path, and_all, attribute, bound_local, bound_path, call, comma, comma_many, constant,
    documentation, equality, group, metavariable, method_call, method_chain, rendered_identifier,
    rendered_name, result_type, roster, rust_keyword, text_pair, twin_path,
};
pub use items::{
    decorated, enumeration, generic_parameters, inline_module, named_field, named_struct,
    named_variant, tuple_struct, tuple_variant, type_alias, unit_struct, unit_variant, use_item,
    where_clause,
};
pub(crate) use preserve::preserved_tokens;
pub use project::{keyed_assignment_items, keyed_roster_items};
pub use project::{keyed_assignment_slice, keyed_roster_slice};
pub use traits::{
    associated_constant, associated_function, associated_type, implementation, trait_declaration,
};
pub(crate) use types::GeneratedLiteralForm;
pub use types::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GENERATED_TOKEN_LIMIT, GeneratedDelimiter,
    GeneratedLiteral, GeneratedLiteralRefusal, GeneratedRowRefusal, GeneratedSpacing,
    GeneratedToken, GeneratedTree,
};

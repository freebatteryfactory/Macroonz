#![doc = include_str!("README.md")]

mod compose;
mod encode;
mod inspect;
mod preserve;
mod project;
mod type_contract;
mod types;

pub(crate) use compose::segmented_twin_path;
pub use compose::{
    absolute_path, and_all, attribute, bound_local, bound_path, call, comma, comma_many, constant,
    documentation, equality, function, group, metavariable, method_call, method_chain,
    rendered_identifier, rendered_name, result_type, roster, rust_keyword, text_pair, twin_path,
};
pub(crate) use preserve::preserved_tokens;
pub use project::{keyed_assignment_slice, keyed_roster_slice};
pub(crate) use types::GeneratedLiteralForm;
pub use types::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GENERATED_TOKEN_LIMIT, GeneratedDelimiter,
    GeneratedLiteral, GeneratedLiteralRefusal, GeneratedSpacing, GeneratedToken, GeneratedTree,
};

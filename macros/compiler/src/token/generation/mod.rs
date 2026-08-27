#![doc = include_str!("README.md")]

mod compose;
mod encode;
mod inspect;
mod types;

pub(crate) use compose::segmented_twin_path;
pub use compose::{
    absolute_path, and_all, attribute, bound_local, bound_path, call, comma, comma_many, constant,
    documentation, equality, function, group, metavariable, method_call, method_chain,
    rendered_identifier, rendered_name, result_type, roster, rust_keyword, text_pair, twin_path,
};
pub use types::{
    GENERATED_TOKEN_LIMIT, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
};

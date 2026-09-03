#![doc = include_str!("README.md")]

mod capture;
mod direct;
mod types;

pub(crate) use capture::{
    assigned, assignment_clauses, declaration_clauses, identifier, named_reference, named_value,
    number,
};
pub(crate) use direct::{
    assigned_identifier, assigned_number, assigned_text, binding_once, comma_groups, fill_once,
    opening, value_of,
};
pub(crate) use types::Clause;

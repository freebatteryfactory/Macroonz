#![doc = include_str!("README.md")]

mod capture;

pub(crate) use capture::{
    assigned_identifier, assigned_number, assigned_text, binding_once, comma_groups, fill_once,
    opening, value_of,
};

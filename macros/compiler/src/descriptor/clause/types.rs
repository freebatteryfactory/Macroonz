//! Informed assignment and clause shapes shared by descriptor grammars.

use crate::token::{CapturedTokenTree, SpanHandle};

#[path = "type_guard.rs"]
mod guard;

/// One admitted `<key> = <value>` assignment.
pub(crate) struct Assignment<'trees> {
    key: &'trees str,
    value: Vec<&'trees CapturedTokenTree>,
    at: SpanHandle,
}

/// One declaration clause after mechanical assignment reading and grammar-owned nested reading agree.
pub(crate) enum Clause<'trees, Nested> {
    /// One mechanically admitted assignment.
    Assigned(Assignment<'trees>),
    /// One nested shape whose meaning remains with the concrete descriptor grammar.
    Nested(Nested),
}

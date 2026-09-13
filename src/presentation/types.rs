//! The opaque read-only presentation value.

#[path = "type_guard.rs"]
mod guard;

/// A complete display projection of one typed record.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Presentation {
    value: serde_json::Value,
}

/// The selected markup language's escaping rules.
#[derive(Clone, Copy)]
pub(super) enum Markup {
    Markdown,
    Html,
}

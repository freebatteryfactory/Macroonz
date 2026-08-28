//! The generation home's declarations.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.

use crate::bounded::Bounded;

#[path = "type_guard.rs"]
mod guard;

/// Tokens one generated tree may carry at any one nesting level.
pub const GENERATED_TOKEN_LIMIT: usize = 4096;

/// Whether one generated punctuation mark joins the token after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedSpacing {
    /// It joins what follows: `::`, `->`, `'static`.
    Joint,
    /// It stands alone.
    Alone,
}

/// The delimiter one generated group is written with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedDelimiter {
    /// `( … )`.
    Parenthesis,
    /// `{ … }`.
    Brace,
    /// `[ … ]`.
    Bracket,
}

/// One token a renderer writes.
///
/// A renderer states a literal's value and never its spelling, and the quoting, the escaping, and the absence of a suffix are the tree's business.
///
/// # Ordering
///
/// The roster grows at its end and nowhere else: each arm's slot is a byte of [`GeneratedTree::canonical_bytes`], which is what a rendered unit's identity is derived over.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratedToken {
    /// An ordinary identifier-shaped word.
    Word(String),
    /// One punctuation character and whether it joins what follows.
    Punct {
        /// The character.
        mark: char,
        /// Whether it joins what follows.
        spacing: GeneratedSpacing,
    },
    /// A text literal, stated as its text; the quoting is the tree's business.
    Text(String),
    /// A delimited group.
    Group {
        /// The delimiter.
        delimiter: GeneratedDelimiter,
        /// The tokens inside.
        tokens: Bounded<GeneratedToken, GENERATED_TOKEN_LIMIT>,
    },
    /// A byte-string literal's material, written `b"…"`.
    ByteText(Vec<u8>),
    /// An unsuffixed integer literal.
    Number(u64),
    /// A raw identifier's name without its `r#` spelling marker.
    RawIdentifier(String),
}

/// One generated token tree: the artifact a renderer produces.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedTree {
    tokens: Bounded<GeneratedToken, GENERATED_TOKEN_LIMIT>,
}

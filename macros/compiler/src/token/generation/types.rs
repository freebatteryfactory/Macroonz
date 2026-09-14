//! The generation home's declarations.
//!
//! Declarations only.
//! The invariant nucleus lives in `type_guard.rs`, and nonsemantic source restoration lives in `provenance.rs`; both are this file's children so no public field is opened.

use crate::bounded::{Bounded, NonEmptyError, Overflow};
use crate::token::SpanHandle;

#[path = "type_guard.rs"]
mod guard;

#[path = "provenance.rs"]
mod provenance;

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
    /// A compiler token group with no written delimiter.
    ///
    /// This row exists for preserving an invisible group the compiler supplied, not for inventing grouping a caller did not declare.
    Bare,
}

/// One exact literal form carried from declared Rust into generated Rust.
///
/// The private value is admitted only through its typed constructors or the preserved-fragment road.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedLiteral {
    value: GeneratedLiteralValue,
}

/// The exact literal forms that need more custody than the older semantic constructors provide.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GeneratedLiteralValue {
    Number(String),
    Character(char),
    Byte(u8),
    NulTerminatedText(Vec<u8>),
}

/// A crate-internal read of one admitted generated literal.
#[derive(Clone, Copy)]
pub(crate) enum GeneratedLiteralForm<'literal> {
    Number(&'literal str),
    Character(char),
    Byte(u8),
    NulTerminatedText(&'literal [u8]),
}

/// Why an exact literal could not be admitted for generation.
#[must_use = "a generated-literal refusal names the exact literal contract that disagreed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedLiteralRefusal {
    /// The spelling is not one numeric literal admitted by the capture owner.
    NotANumber,
    /// C-string material contains an interior NUL and therefore has no lawful C-string literal value.
    InteriorNul,
}

/// Why one preserved captured fragment could not become a generated tree.
#[must_use = "a fragment-generation issue names the exact token conversion that refused"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentGenerationIssue {
    /// One captured literal could not enter its generated literal form.
    Literal(GeneratedLiteralRefusal),
    /// The generated tree would exceed its declared token magnitude.
    Unbounded,
    /// One captured word, raw identifier or punctuation has no token in its stated role.
    Token(GeneratedTokenIssue),
}

/// The lexical role whose offered spelling cannot be emitted as one Rust token.
#[must_use = "a generated-token issue names the lexical role that refused"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedTokenIssue {
    /// The spelling is not one ordinary identifier-shaped token or keyword.
    Word,
    /// The name is malformed or forbidden behind the raw-identifier marker.
    RawIdentifier,
    /// The character is not a Rust punctuation token.
    Punctuation,
}

/// Why an offered token run could not become a completed generated tree.
#[must_use = "a generated-tree refusal prevents malformed or unbounded output from acquiring an identity"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedTreeRefusal {
    /// The top-level run exceeds its declared token magnitude.
    Unbounded(Overflow),
    /// One offered token has no spelling in its stated lexical role.
    Token {
        /// The zero-based position in pre-order, counting groups before their children.
        position: usize,
        /// The lexical role that refused.
        issue: GeneratedTokenIssue,
    },
}

/// One refused captured-fragment projection with the exact source span it belongs to.
#[must_use = "a fragment-generation refusal carries its issue and exact captured span"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FragmentGenerationRefusal {
    pub(super) issue: FragmentGenerationIssue,
    pub(super) at: Option<SpanHandle>,
}

/// Why one flat keyed-row projection could not produce exactly one non-empty item run.
#[must_use = "a generated-row refusal names the exact row and non-empty token disagreement"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratedRowRefusal {
    position: usize,
    cause: NonEmptyError,
}

/// One offered token a renderer composes before generated-tree admission.
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
    /// An exact caller-authored literal admitted through [`GeneratedLiteral`].
    Literal(GeneratedLiteral),
}

/// One bounded generated token tree whose words, raw identifiers and punctuation hold their lexical roles.
#[derive(Clone)]
pub struct GeneratedTree {
    tokens: Bounded<GeneratedToken, GENERATED_TOKEN_LIMIT>,
    source_spans: Vec<Option<SpanHandle>>,
}

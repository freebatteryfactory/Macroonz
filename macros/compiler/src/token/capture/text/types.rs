//! The text home's declarations: the source-byte magnitude, why a text read refuses, and the read it produces.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `read.rs`, this file's own child.

use super::super::{
    CaptureBound, CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CapturedAtom,
    CapturedDelimiter, CapturedInput, LiteralReadCause, SpanHandle, SpanTable, TokenPath,
    capture_literal,
};

#[path = "read.rs"]
mod read;

/// Source bytes one text capture may read before tokenization.
///
/// This magnitude is independent of token count, tree depth, and capture work so a hostile trivia-only input cannot evade every structural bound by producing no retained token.
pub const TEXT_SOURCE_BYTE_LIMIT: usize = 65_536;

crate::roster! {
    /// Why the low-level lexer could not normalize one spelling.
    #[must_use = "a lexical refusal names the spelling distinction that could not be normalized"]
    pub enum TextLexicalCause {
        /// A block comment was not terminated.
        BlockCommentNotTerminated = "block-comment-not-terminated",
        /// An identifier contains a character the compiler lexer rejects.
        InvalidIdentifier = "invalid-identifier",
        /// A prefix is reserved or not meaningful without an edition-aware parser.
        UnknownPrefix = "unknown-prefix",
        /// A lifetime prefix is reserved or not meaningful without an edition-aware parser.
        UnknownLifetimePrefix = "unknown-lifetime-prefix",
        /// A guarded-string prefix requires parser context this boundary does not own.
        GuardedStringPrefix = "guarded-string-prefix",
        /// A literal carries a malformed low-level spelling.
        MalformedLiteral = "malformed-literal",
        /// A lifetime begins with a number.
        LifetimeStartsWithNumber = "lifetime-starts-with-number",
        /// Frontmatter is not Rust token input at this boundary.
        Frontmatter = "frontmatter",
        /// The lexer reported a character with no lawful Rust token kind.
        UnknownToken = "unknown-token",
    }
}

/// Why one text read refused.
///
/// Dependent checks: there is no group to balance until the characters were cut, and no magnitude to exceed until the trees were built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextReadCause {
    /// A literal was never closed.
    NotTerminated,
    /// A literal carries an escape sequence the literal owner could not read.
    NotEscapeFree,
    /// A delimited group was never closed.
    NotBalanced,
    /// A closing delimiter arrived with no group open.
    NotOpened,
    /// The declared text exceeds the independent source-byte magnitude.
    SourceBytesUnbounded,
    /// The low-level lexer established a malformed or context-dependent spelling.
    Lexical(TextLexicalCause),
    /// The read exceeds a declared magnitude, and this is which one — a reader told only "unbounded" cannot tell a tree that nests too deep from one that spends the walk's budget.
    Unbounded(CaptureBound),
}

/// One refused text read: the established cause, and the byte it sits at.
#[must_use = "a read refusal carries the established cause and the byte it sits at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextReadRefusal {
    /// The established cause.
    pub cause: TextReadCause,
    /// The byte position the cause was established at.
    pub at: u64,
}

/// One declared input read from text: the captured trees, and the byte offsets that resolve every handle the read issued.
///
/// The callable route — a compiler is one producer of captured input, a test is another, and text is the third — so that the reproduction route a diagnostic names is a real road and not a promise.
/// The two seats are visible to the text read alone, and that read establishes the relationship between them: the offsets table resolves exactly the handles the capture beside it issued.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextCapture {
    /// The captured input the read produced.
    input: CapturedInput,
    /// The byte offsets that resolve the handles that read issued.
    spans: SpanTable,
}

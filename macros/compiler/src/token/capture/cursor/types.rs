//! The cursor home's declarations: the mechanical read cursor, the shapes it asks for, and how a read refuses.
//!
//! Declarations only.

use super::{CapturedDelimiter, CapturedTokenTree, SpanHandle};

/// A read cursor over one normalized captured-token sequence.
///
/// The cursor borrows an already bounded capture and advances only after one requested mechanical shape is present.
/// It knows token structure and no declaration vocabulary beyond an exact word the caller asks it to read.
#[derive(Debug, Clone)]
pub struct CaptureCursor<'tokens> {
    pub(super) tokens: &'tokens [CapturedTokenTree],
    pub(super) next: usize,
    pub(super) end: Option<SpanHandle>,
}

/// Whether one punctuation seat is joined to what follows or stands alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapturedSpacing {
    /// The punctuation stands alone.
    Alone,
    /// The punctuation is joined to the following token.
    Joint,
}

/// The mechanical token shape one capture read asked for.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptureExpectation {
    /// Any one captured token.
    Token,
    /// One exact ordinary word.
    Word(String),
    /// One ordinary or raw identifier.
    Identifier,
    /// One numeric literal spelling.
    Number,
    /// One punctuation character with its adjacency stated.
    Punctuation {
        /// The punctuation character.
        mark: char,
        /// Whether the character joins what follows.
        spacing: CapturedSpacing,
    },
    /// One group carrying this delimiter.
    Group(CapturedDelimiter),
}

/// Why generic captured-token grammar mechanics could not complete one read.
///
/// These rows state only token structure.
/// The caller retains declaration vocabulary, clause meaning, and diagnostic policy.
#[must_use = "a capture read issue names the exact mechanical disagreement"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptureReadIssue {
    /// The sequence ended before the requested shape appeared.
    Missing(CaptureExpectation),
    /// The next token did not carry the requested shape.
    Unexpected(CaptureExpectation),
    /// Finishing found an unconsumed token.
    InputRemaining,
    /// A separated sequence exceeded the caller's declared magnitude.
    SequenceUnbounded {
        /// The maximum admitted member count.
        limit: usize,
    },
    /// A separated-sequence reader returned one member without consuming a token.
    SequenceMemberDidNotAdvance,
    /// A cursor's consumed range no longer belongs to the captured sequence it reads.
    CursorRangeContradiction,
}

/// One refused mechanical read with the exact producer span available at that site.
///
/// A root end-of-input refusal has no token to point at.
/// An end-of-group refusal points at the group token whose closing boundary was reached.
#[must_use = "a capture read refusal carries its mechanical issue and exact available span"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaptureReadRefusal {
    pub(super) issue: CaptureReadIssue,
    pub(super) at: Option<SpanHandle>,
}

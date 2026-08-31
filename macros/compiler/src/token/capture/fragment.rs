//! Borrowed exact fragments of one normalized captured-token sequence.
//!
//! A fragment never copies or reparses source.
//! It retains the captured tokens and producer span handles already owned by its source boundary, while canonical bytes continue to exclude producer-local coordinates.

use super::{CaptureCursor, CapturedFragment, CapturedInput, CapturedTokenTree, SpanHandle};

impl CapturedInput {
    /// Borrow this complete captured input as one exact fragment.
    #[must_use]
    pub fn fragment(&self) -> CapturedFragment<'_> {
        CapturedFragment::over(self.trees(), None)
    }
}

impl<'tokens> CapturedFragment<'tokens> {
    /// Construct one fragment inside the capture owner.
    #[must_use]
    pub(super) const fn over(
        tokens: &'tokens [CapturedTokenTree],
        end: Option<SpanHandle>,
    ) -> Self {
        Self { tokens, end }
    }

    /// The exact captured tokens this fragment borrows.
    #[must_use]
    pub const fn tokens(self) -> &'tokens [CapturedTokenTree] {
        self.tokens
    }

    /// Open the generic mechanical cursor over this exact fragment.
    #[must_use]
    pub fn cursor(self) -> CaptureCursor<'tokens> {
        let mut cursor = CaptureCursor::over(self.tokens);
        cursor.end = self.end;
        cursor
    }

    /// How many tokens this fragment carries at its current level.
    #[must_use]
    pub const fn len(self) -> usize {
        self.tokens.len()
    }

    /// Whether this fragment carries no token at its current level.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.tokens.is_empty()
    }

    /// The first token's producer span, where the fragment is nonempty.
    #[must_use]
    pub fn first_span(self) -> Option<SpanHandle> {
        self.tokens.first().map(CapturedTokenTree::span)
    }

    /// The last token's producer span, where the fragment is nonempty.
    #[must_use]
    pub fn last_span(self) -> Option<SpanHandle> {
        self.tokens.last().map(CapturedTokenTree::span)
    }

    /// The enclosing group span used when a read reaches this fragment's end.
    ///
    /// A top-level fragment has no enclosing group and therefore answers `None`.
    #[must_use]
    pub const fn enclosing_span(self) -> Option<SpanHandle> {
        self.end
    }

    /// The canonical captured bytes of exactly this fragment.
    ///
    /// Producer spans remain excluded, so a span-only movement does not move these bytes.
    #[must_use]
    pub fn canonical_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for token in self.tokens {
            super::encode::encode_captured(token, &mut bytes);
        }
        bytes
    }
}

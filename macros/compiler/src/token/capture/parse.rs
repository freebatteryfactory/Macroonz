//! Generic mechanical reads over an already normalized captured-token sequence.
//!
//! This file knows token structure and nothing about the declaration written with it.
//! A caller supplies exact words, chooses which operations compose one clause, and maps a typed mechanical refusal into its own diagnostic policy.

use super::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedDelimiter,
    CapturedInput, CapturedSpacing, CapturedTokenTree,
};
use crate::bounded::Bounded;

impl CapturedInput {
    /// Open a mechanical read cursor over the top-level captured sequence.
    #[must_use]
    pub fn cursor(&self) -> CaptureCursor<'_> {
        CaptureCursor::over(self.trees())
    }
}

impl<'tokens> CaptureCursor<'tokens> {
    /// Open the top-level cursor while keeping raw slice construction inside the capture owner.
    pub(super) const fn over(tokens: &'tokens [CapturedTokenTree]) -> Self {
        Self {
            tokens,
            next: 0,
            end: None,
        }
    }

    /// Whether this sequence has no token left to read.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.next == self.tokens.len()
    }

    /// Read the next ordinary word without advancing this cursor.
    #[must_use]
    pub(crate) fn next_word(&self) -> Option<&'tokens str> {
        self.tokens.get(self.next).and_then(CapturedTokenTree::word)
    }

    /// Read the next captured token without advancing this cursor.
    #[must_use]
    pub(crate) fn next_token(&self) -> Option<&'tokens CapturedTokenTree> {
        self.tokens.get(self.next)
    }

    /// Read one caller-defined shape and retain the exact captured run it consumed.
    ///
    /// The callback may consume no token where an empty exact Rust seat is lawful.
    /// The returned fragment still belongs to this cursor's original captured sequence and retains its enclosing group boundary.
    ///
    /// # Errors
    ///
    /// Returns the callback's exact typed mechanical refusal without advancing past that refusal.
    pub fn fragment<T>(
        &mut self,
        read: impl FnOnce(&mut Self) -> Result<T, CaptureReadRefusal>,
    ) -> Result<(super::CapturedFragment<'tokens>, T), CaptureReadRefusal> {
        let start = self.next;
        let value = read(self)?;
        let tokens = self
            .tokens
            .get(start..self.next)
            .ok_or(CaptureReadRefusal {
                issue: CaptureReadIssue::CursorRangeContradiction,
                at: self.current_span(),
            })?;
        Ok((super::CapturedFragment::over(tokens, self.end), value))
    }

    /// Read any one captured token.
    ///
    /// # Errors
    ///
    /// Returns a typed missing-token refusal at the current sequence boundary.
    pub fn token(&mut self) -> Result<&'tokens CapturedTokenTree, CaptureReadRefusal> {
        self.take(|| CaptureExpectation::Token, |_| true)
    }

    /// Read one exact ordinary word.
    ///
    /// A raw identifier does not satisfy this operation because rawness is declaration material.
    /// Use [`CaptureCursor::identifier`] where either identifier form is lawful.
    ///
    /// # Errors
    ///
    /// Returns a typed missing or unexpected-token refusal at the exact available span.
    pub fn word(
        &mut self,
        expected: &str,
    ) -> Result<&'tokens CapturedTokenTree, CaptureReadRefusal> {
        self.take(
            || CaptureExpectation::Word(expected.to_owned()),
            |token| token.word() == Some(expected),
        )
    }

    /// Read one ordinary or raw identifier token.
    ///
    /// The returned token retains which form it carried through [`CapturedTokenTree::word`] and [`CapturedTokenTree::raw_identifier`].
    ///
    /// # Errors
    ///
    /// Returns a typed missing or unexpected-token refusal at the exact available span.
    pub fn identifier(
        &mut self,
    ) -> Result<(&'tokens CapturedTokenTree, &'tokens str), CaptureReadRefusal> {
        let token = self.take(
            || CaptureExpectation::Identifier,
            |token| token.word().is_some() || token.raw_identifier().is_some(),
        )?;
        let ((Some(spelling), None) | (None, Some(spelling))) =
            (token.word(), token.raw_identifier())
        else {
            return Err(CaptureReadRefusal {
                issue: CaptureReadIssue::Unexpected(CaptureExpectation::Identifier),
                at: Some(token.span()),
            });
        };
        Ok((token, spelling))
    }

    /// Read one numeric literal spelling.
    ///
    /// # Errors
    ///
    /// Returns a typed missing or unexpected-token refusal at the exact available span.
    pub fn number(
        &mut self,
    ) -> Result<(&'tokens CapturedTokenTree, &'tokens str), CaptureReadRefusal> {
        let token = self.take(
            || CaptureExpectation::Number,
            |token| token.number().is_some(),
        )?;
        match token.number() {
            Some(spelling) => Ok((token, spelling)),
            None => Err(CaptureReadRefusal {
                issue: CaptureReadIssue::Unexpected(CaptureExpectation::Number),
                at: Some(token.span()),
            }),
        }
    }

    /// Read one punctuation seat with both character and adjacency stated.
    ///
    /// # Errors
    ///
    /// Returns a typed missing or unexpected-token refusal at the exact available span.
    pub fn punctuation(
        &mut self,
        mark: char,
        spacing: CapturedSpacing,
    ) -> Result<&'tokens CapturedTokenTree, CaptureReadRefusal> {
        self.take(
            || CaptureExpectation::Punctuation { mark, spacing },
            |token| match spacing {
                CapturedSpacing::Alone => {
                    token.punct() == Some(mark) && token.joint_punct().is_none()
                }
                CapturedSpacing::Joint => token.joint_punct() == Some(mark),
            },
        )
    }

    /// Read the two punctuation seats of `->`.
    ///
    /// # Errors
    ///
    /// Returns the exact first seat that is missing or disagrees.
    pub fn thin_arrow(&mut self) -> Result<[&'tokens CapturedTokenTree; 2], CaptureReadRefusal> {
        let dash = self.punctuation('-', CapturedSpacing::Joint)?;
        let arrow = self.punctuation('>', CapturedSpacing::Alone)?;
        Ok([dash, arrow])
    }

    /// Read the two punctuation seats of `=>`.
    ///
    /// # Errors
    ///
    /// Returns the exact first seat that is missing or disagrees.
    pub fn fat_arrow(&mut self) -> Result<[&'tokens CapturedTokenTree; 2], CaptureReadRefusal> {
        let equals = self.punctuation('=', CapturedSpacing::Joint)?;
        let arrow = self.punctuation('>', CapturedSpacing::Alone)?;
        Ok([equals, arrow])
    }

    /// Read one group and open a cursor over its captured members.
    ///
    /// A missing token inside the returned cursor points at this group token, which is the exact source boundary the capture retains for an empty or exhausted group.
    ///
    /// # Errors
    ///
    /// Returns a typed missing or unexpected-token refusal at the exact available span.
    pub fn group(&mut self, delimiter: CapturedDelimiter) -> Result<Self, CaptureReadRefusal> {
        let token = self.take(
            || CaptureExpectation::Group(delimiter),
            |token| token.group().is_some_and(|(found, _)| found == delimiter),
        )?;
        match token.group() {
            Some((_, members)) => Ok(Self {
                tokens: members,
                next: 0,
                end: Some(token.span()),
            }),
            None => Err(CaptureReadRefusal {
                issue: CaptureReadIssue::Unexpected(CaptureExpectation::Group(delimiter)),
                at: Some(token.span()),
            }),
        }
    }

    /// Read a sequence whose every member is followed by one standalone separator.
    ///
    /// Empty standing is deliberately left to the caller: this operation returns an empty [`Bounded`] where the sequence is empty.
    /// The member reader owns one member's shape and meaning, while this operation owns cursor progress, the separator seat, and the declared member magnitude.
    ///
    /// # Errors
    ///
    /// Returns the member reader's refusal, an exact separator refusal, a nonadvancing-reader refusal, or a refusal at the first member beyond `LIMIT`.
    pub fn trailing_separated<T, const LIMIT: usize>(
        mut self,
        separator: char,
        mut read: impl FnMut(&mut Self) -> Result<T, CaptureReadRefusal>,
    ) -> Result<Bounded<T, LIMIT>, CaptureReadRefusal> {
        let mut members = Bounded::empty();
        while !self.is_finished() {
            let member_at = self.current_span();
            if members.len() >= LIMIT {
                return Err(CaptureReadRefusal {
                    issue: CaptureReadIssue::SequenceUnbounded { limit: LIMIT },
                    at: member_at,
                });
            }
            let before = self.next;
            let member = read(&mut self)?;
            if self.next == before {
                return Err(CaptureReadRefusal {
                    issue: CaptureReadIssue::SequenceMemberDidNotAdvance,
                    at: member_at,
                });
            }
            self.punctuation(separator, CapturedSpacing::Alone)?;
            members.try_push(member).map_err(|_| CaptureReadRefusal {
                issue: CaptureReadIssue::SequenceUnbounded { limit: LIMIT },
                at: member_at,
            })?;
        }
        Ok(members)
    }

    /// Finish this sequence after every token has been consumed.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureReadIssue::InputRemaining`] at the first unconsumed token.
    pub fn finish(self) -> Result<(), CaptureReadRefusal> {
        match self.tokens.get(self.next) {
            Some(token) => Err(CaptureReadRefusal {
                issue: CaptureReadIssue::InputRemaining,
                at: Some(token.span()),
            }),
            None => Ok(()),
        }
    }

    /// Read one token satisfying a mechanical expectation without advancing on disagreement.
    fn take(
        &mut self,
        expected: impl FnOnce() -> CaptureExpectation,
        accepts: impl FnOnce(&CapturedTokenTree) -> bool,
    ) -> Result<&'tokens CapturedTokenTree, CaptureReadRefusal> {
        let Some(token) = self.tokens.get(self.next) else {
            return Err(CaptureReadRefusal {
                issue: CaptureReadIssue::Missing(expected()),
                at: self.end,
            });
        };
        if !accepts(token) {
            return Err(CaptureReadRefusal {
                issue: CaptureReadIssue::Unexpected(expected()),
                at: Some(token.span()),
            });
        }
        self.next = self.next.saturating_add(1);
        Ok(token)
    }

    /// The current token's span, or the enclosing group span at its end.
    fn current_span(&self) -> Option<super::SpanHandle> {
        self.tokens
            .get(self.next)
            .map(CapturedTokenTree::span)
            .or(self.end)
    }
}

impl CaptureReadRefusal {
    /// Bind one compiler-owned higher grammar operation to an already established mechanical issue and span.
    pub(crate) const fn projected(issue: CaptureReadIssue, at: Option<super::SpanHandle>) -> Self {
        Self { issue, at }
    }

    /// The mechanical disagreement this read established.
    pub const fn issue(&self) -> &CaptureReadIssue {
        &self.issue
    }

    /// The exact producer span available at the refusal site.
    ///
    /// Root end of input has no token and therefore no span.
    #[must_use]
    pub const fn token(&self) -> Option<super::SpanHandle> {
        self.at
    }

    /// Consume this refusal into its mechanical issue and exact available span.
    pub(crate) fn into_parts(self) -> (CaptureReadIssue, Option<super::SpanHandle>) {
        (self.issue, self.at)
    }
}

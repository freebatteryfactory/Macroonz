//! The seam's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, so the walk's budget, the route's steps, the captured trees, and the generated tree's tokens are reachable here and nowhere else.
//! Each magnitude is settled at the moment a value is made, which is why nothing downstream re-checks one.
//!
//! The two byte-producing seats read their private collections here and hand each element to the walkers in `encode.rs` and `inspect.rs`: the value's own bytes are the value's business, and walking a token is the walker's.

use super::super::encode::{encode_captured, encode_generated};
use super::super::inspect::inspect_token;
use super::{
    CAPTURE_WORK_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound, CaptureWalk, CapturedDelimiter,
    CapturedInput, CapturedPayload, CapturedTokenTree, GeneratedDelimiter, GeneratedSpacing,
    GeneratedToken, GeneratedTree, SpanHandle, TokenPath,
};
use crate::bounded::{Bounded, Overflow};

impl SpanHandle {
    /// The handle at one index of the producer's table.
    #[must_use]
    pub const fn at(index: u32) -> Self {
        Self(index)
    }

    /// The index this handle names.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }
}

impl TokenPath {
    /// The root route: the declared input itself, before any step into it.
    #[must_use]
    pub const fn root() -> Self {
        Self {
            steps: Bounded::empty(),
        }
    }

    /// The route to one token of the group this route names.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Depth`] where the route would run past the declared nesting magnitude.
    /// The step refuses rather than saturating, because a saturated depth makes two different tokens share one route.
    pub fn stepped(&self, index: u32) -> Result<Self, CaptureBound> {
        let mut steps = self.steps.as_slice().to_vec();
        steps.push(index);
        Bounded::new(steps)
            .map(|steps| Self { steps })
            .map_err(|_| CaptureBound::Depth)
    }

    /// The route's steps, from the root inward.
    #[must_use]
    pub fn steps(&self) -> &[u32] {
        self.steps.as_slice()
    }

    /// How deep this route runs; the root is zero.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.steps.len()
    }

    /// Whether this route names the declared input itself.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.steps.is_empty()
    }
}

impl CaptureWalk {
    /// A fresh walk, holding the whole declared budget and nothing taken.
    #[must_use]
    pub const fn declared() -> Self {
        Self {
            remaining: CAPTURE_WORK_LIMIT,
            taken: 0,
        }
    }

    /// Spend one unit of the declared budget on looking at one token.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Work`] where the budget is spent.
    pub fn examined(&mut self) -> Result<(), CaptureBound> {
        self.remaining = self.remaining.checked_sub(1).ok_or(CaptureBound::Work)?;
        Ok(())
    }

    /// Count one token against the whole-tree magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Tree`] where the tree outgrows its declared magnitude.
    pub fn took(&mut self) -> Result<(), CaptureBound> {
        let taken = self.taken.checked_add(1).ok_or(CaptureBound::Tree)?;
        if taken > CAPTURED_TREE_TOKEN_LIMIT {
            return Err(CaptureBound::Tree);
        }
        self.taken = taken;
        Ok(())
    }

    /// How many tokens the whole tree has taken so far.
    #[must_use]
    pub const fn taken(self) -> usize {
        self.taken
    }

    /// How much of the declared budget is left.
    #[must_use]
    pub const fn remaining(self) -> usize {
        self.remaining
    }
}

impl CapturedTokenTree {
    /// Capture one token.
    #[must_use]
    pub const fn captured(payload: CapturedPayload, path: TokenPath, span: SpanHandle) -> Self {
        Self {
            payload,
            path,
            span,
        }
    }

    /// Capture one delimited group.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Level`] where the group carries more tokens than the declared magnitude admits.
    /// A group that does not fit refuses rather than capturing as an empty one: an empty group is a declaration with no body, not a shorter declaration, and the two must never read alike.
    pub fn group_of(
        delimiter: CapturedDelimiter,
        trees: Vec<Self>,
        path: TokenPath,
        span: SpanHandle,
    ) -> Result<Self, CaptureBound> {
        Bounded::new(trees)
            .map(|trees| Self::captured(CapturedPayload::Group { delimiter, trees }, path, span))
            .map_err(|_| CaptureBound::Level)
    }

    /// What this token carries.
    #[must_use]
    pub const fn payload(&self) -> &CapturedPayload {
        &self.payload
    }

    /// The route from the root of the declared input to this token.
    #[must_use]
    pub const fn path(&self) -> &TokenPath {
        &self.path
    }

    /// The handle into the producer's span table.
    #[must_use]
    pub const fn span(&self) -> SpanHandle {
        self.span
    }

    /// The word this token spells, where it is a word.
    #[must_use]
    pub fn word(&self) -> Option<&str> {
        match &self.payload {
            CapturedPayload::Word(word) => Some(word.as_str()),
            CapturedPayload::Punct(_)
            | CapturedPayload::Text(_)
            | CapturedPayload::Number(_)
            | CapturedPayload::Group { .. }
            | CapturedPayload::ByteText(_)
            | CapturedPayload::Character(_)
            | CapturedPayload::Byte(_)
            | CapturedPayload::NulTerminatedText(_) => None,
        }
    }

    /// The punctuation character this token spells, where it is one.
    #[must_use]
    pub const fn punct(&self) -> Option<char> {
        match &self.payload {
            CapturedPayload::Punct(mark) => Some(*mark),
            CapturedPayload::Word(_)
            | CapturedPayload::Text(_)
            | CapturedPayload::Number(_)
            | CapturedPayload::Group { .. }
            | CapturedPayload::ByteText(_)
            | CapturedPayload::Character(_)
            | CapturedPayload::Byte(_)
            | CapturedPayload::NulTerminatedText(_) => None,
        }
    }

    /// The text this token carries, where it is a text literal.
    ///
    /// The text and never the spelling, so a raw text answers here on the same terms as a quoted one and the escape a quoted one carried is already read.
    ///
    /// # Nonclaims
    ///
    /// A byte string, a C string, a character, and a byte are not text and do not answer here; each is a different value at the seat it is written to, and a road wanting one asks for it by name.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match &self.payload {
            CapturedPayload::Text(text) => Some(text.as_str()),
            CapturedPayload::Word(_)
            | CapturedPayload::Punct(_)
            | CapturedPayload::Number(_)
            | CapturedPayload::Group { .. }
            | CapturedPayload::ByteText(_)
            | CapturedPayload::Character(_)
            | CapturedPayload::Byte(_)
            | CapturedPayload::NulTerminatedText(_) => None,
        }
    }

    /// The numeric spelling this token carries, where it is a numeric literal.
    ///
    /// The spelling exactly as written — the base, the digit separators, and the suffix are part of what the declaration says — so a grammar wanting a value states which spellings it reads and refuses the rest itself.
    #[must_use]
    pub fn number(&self) -> Option<&str> {
        match &self.payload {
            CapturedPayload::Number(spelling) => Some(spelling.as_str()),
            CapturedPayload::Word(_)
            | CapturedPayload::Punct(_)
            | CapturedPayload::Text(_)
            | CapturedPayload::Group { .. }
            | CapturedPayload::ByteText(_)
            | CapturedPayload::Character(_)
            | CapturedPayload::Byte(_)
            | CapturedPayload::NulTerminatedText(_) => None,
        }
    }

    /// The group this token opens, where it is one.
    #[must_use]
    pub fn group(&self) -> Option<(CapturedDelimiter, &[Self])> {
        match &self.payload {
            CapturedPayload::Group { delimiter, trees } => Some((*delimiter, trees.as_slice())),
            CapturedPayload::Word(_)
            | CapturedPayload::Punct(_)
            | CapturedPayload::Text(_)
            | CapturedPayload::Number(_)
            | CapturedPayload::ByteText(_)
            | CapturedPayload::Character(_)
            | CapturedPayload::Byte(_)
            | CapturedPayload::NulTerminatedText(_) => None,
        }
    }
}

impl CapturedInput {
    /// Take one captured input.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Level`] where the top level carries more trees than the declared magnitude admits.
    pub fn taken(trees: Vec<CapturedTokenTree>, issued: u32) -> Result<Self, CaptureBound> {
        Bounded::new(trees)
            .map(|trees| Self { trees, issued })
            .map_err(|_| CaptureBound::Level)
    }

    /// The top-level trees, in the order they were written.
    #[must_use]
    pub fn trees(&self) -> &[CapturedTokenTree] {
        self.trees.as_slice()
    }

    /// How many top-level trees were captured.
    #[must_use]
    pub fn len(&self) -> usize {
        self.trees.len()
    }

    /// Whether nothing at all was captured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }

    /// How many span handles the producer issued; a handle at or past this index names no token.
    #[must_use]
    pub const fn issued(&self) -> u32 {
        self.issued
    }

    /// The canonical bytes of this capture — what an identity over the declared input is derived from.
    ///
    /// Deterministic and independent of span handles, so two captures of one declaration from different producers encode identically.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for tree in self.trees.as_slice() {
            encode_captured(tree, &mut bytes);
        }
        bytes
    }
}

impl GeneratedToken {
    /// One word.
    #[must_use]
    pub fn word(spelling: &str) -> Self {
        Self::Word(spelling.to_owned())
    }

    /// One punctuation mark that joins what follows.
    #[must_use]
    pub const fn joint(mark: char) -> Self {
        Self::Punct {
            mark,
            spacing: GeneratedSpacing::Joint,
        }
    }

    /// One punctuation mark that stands alone.
    #[must_use]
    pub const fn alone(mark: char) -> Self {
        Self::Punct {
            mark,
            spacing: GeneratedSpacing::Alone,
        }
    }

    /// One text literal.
    #[must_use]
    pub fn text(content: &str) -> Self {
        Self::Text(content.to_owned())
    }

    /// One byte-string literal, over the material a caller holds.
    ///
    /// The material is taken as bytes and stays bytes, so material that is not text crosses without a lossy road existing for it to take.
    #[must_use]
    pub fn byte_text(material: &[u8]) -> Self {
        Self::ByteText(material.to_vec())
    }

    /// One unsuffixed integer literal.
    ///
    /// Total: every `u64` is a lawful unsuffixed integer literal, so there is no value to refuse and no refusal branch to invent.
    #[must_use]
    pub const fn number(value: u64) -> Self {
        Self::Number(value)
    }

    /// One delimited group.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] where the group carries more tokens than the declared magnitude admits.
    pub fn group(delimiter: GeneratedDelimiter, tokens: Vec<Self>) -> Result<Self, Overflow> {
        Bounded::new(tokens).map(|tokens| Self::Group { delimiter, tokens })
    }
}

impl GeneratedTree {
    /// Assemble one generated tree.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] where the tree carries more top-level tokens than the declared magnitude admits.
    pub fn assembled(tokens: Vec<GeneratedToken>) -> Result<Self, Overflow> {
        Bounded::new(tokens).map(|tokens| Self { tokens })
    }

    /// The top-level tokens, in the order they were written.
    #[must_use]
    pub fn tokens(&self) -> &[GeneratedToken] {
        self.tokens.as_slice()
    }

    /// How many top-level tokens the tree carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Whether the tree carries nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Join one tree onto another, producing the tree that carries both.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] where the joined tree outgrows the declared magnitude.
    pub fn joined(&self, other: &Self) -> Result<Self, Overflow> {
        let mut tokens = self.tokens.as_slice().to_vec();
        tokens.extend_from_slice(other.tokens.as_slice());
        Self::assembled(tokens)
    }

    /// The Rust source text this tree projects, for a person to read.
    ///
    /// A projection and only a projection: nothing reads it back, no identity is derived from it, and a caller comparing two trees compares the trees.
    #[must_use]
    pub fn inspected(&self) -> String {
        let mut rendered = String::new();
        for token in self.tokens.as_slice() {
            inspect_token(token, &mut rendered);
        }
        rendered
    }

    /// The tree's canonical bytes — what a digest over the rendered unit is taken from.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for token in self.tokens.as_slice() {
            encode_generated(token, &mut bytes);
        }
        bytes
    }
}

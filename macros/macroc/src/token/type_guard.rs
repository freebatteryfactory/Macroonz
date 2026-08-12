//! The token seam's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, so the walk's budget, the
//! route's steps, the captured trees, and the generated tree's tokens are
//! reachable here and nowhere else. Each magnitude is settled at the moment a
//! value is made, which is why nothing downstream re-checks one.
//!
//! The two byte-producing seats — a capture's canonical bytes and a generated
//! tree's — read their private collections here and hand each element to the
//! walkers in `encode.rs` and `inspect.rs`. The value's own bytes are the
//! value's business; walking a token is the walker's.

use super::super::encode::{encode_captured, encode_generated};
use super::super::inspect::inspect_token;
use super::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
    SpanHandle, TokenPath,
};
use crate::plane::{AuthoringLimitProfile, CapturedTokenLimit, CapturedTreeTokenLimit};
use threadpak::types::{AdmittedLimit, Bounded, BoundedConstruction, ConstLimit};

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
    /// Returns [`CaptureBound::DepthUnbounded`] when the route would run past
    /// the declared nesting magnitude. The step refuses rather than saturating:
    /// a saturated depth makes two different tokens share one route, which is
    /// the defect this type exists to end.
    pub fn stepped(&self, index: u32) -> Result<Self, CaptureBound> {
        let mut steps: Vec<u32> = self.steps.iter().copied().collect();
        steps.push(index);
        Bounded::admitted_const(
            steps,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|steps| Self { steps })
        .map_err(|_| CaptureBound::DepthUnbounded)
    }

    /// The route's steps, from the root inward.
    pub fn steps(&self) -> impl Iterator<Item = &u32> {
        self.steps.iter()
    }

    /// How deep this route runs. The root is zero.
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
    /// The declared capture-work budget, in units of one examined token.
    ///
    /// Four times the whole-tree magnitude, because a walk may look at more than
    /// it keeps. A budget at the tree magnitude exactly would refuse a lawful
    /// input the moment its producer looked twice at anything.
    pub const DECLARED_WORK: u32 = 65_536;

    /// A fresh walk, holding the whole declared budget and nothing taken.
    #[must_use]
    pub const fn declared() -> Self {
        Self {
            remaining: Self::DECLARED_WORK,
            taken: 0,
        }
    }

    /// Spend one unit of the declared budget on looking at one token.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::WorkUnbounded`] when the budget is spent.
    pub fn examined(&mut self) -> Result<(), CaptureBound> {
        self.remaining = self
            .remaining
            .checked_sub(1)
            .ok_or(CaptureBound::WorkUnbounded)?;
        Ok(())
    }

    /// Count one token against the whole-tree magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::TreeUnbounded`] when the tree outgrows its
    /// declared magnitude.
    pub fn took(&mut self) -> Result<(), CaptureBound> {
        let taken = self
            .taken
            .checked_add(1)
            .ok_or(CaptureBound::TreeUnbounded)?;
        let magnitude =
            u32::try_from(CapturedTreeTokenLimit::MAX).map_err(|_| CaptureBound::TreeUnbounded)?;
        if taken > magnitude {
            return Err(CaptureBound::TreeUnbounded);
        }
        self.taken = taken;
        Ok(())
    }

    /// How many tokens the whole tree has taken so far.
    #[must_use]
    pub const fn taken(self) -> u32 {
        self.taken
    }

    /// How much of the declared budget is left.
    #[must_use]
    pub const fn remaining(self) -> u32 {
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
            | CapturedPayload::Group { .. } => None,
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
            | CapturedPayload::Group { .. } => None,
        }
    }

    /// The text this token carries, where it is a text literal.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match &self.payload {
            CapturedPayload::Text(text) => Some(text.as_str()),
            CapturedPayload::Word(_)
            | CapturedPayload::Punct(_)
            | CapturedPayload::Number(_)
            | CapturedPayload::Group { .. } => None,
        }
    }

    /// Capture one delimited group.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::LevelUnbounded`] when the group carries more
    /// tokens than the declared magnitude admits. A group that does not fit
    /// refuses rather than capturing as an empty one: an empty group is not a
    /// shorter declaration, it is a declaration with no body, and the two must
    /// never read alike.
    pub fn group_of(
        delimiter: CapturedDelimiter,
        trees: Vec<Self>,
        path: TokenPath,
        span: SpanHandle,
    ) -> Result<Self, CaptureBound> {
        Bounded::admitted_const(
            trees,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|trees| Self::captured(CapturedPayload::Group { delimiter, trees }, path, span))
        .map_err(|_| CaptureBound::LevelUnbounded)
    }

    /// The group this token opens, where it is one.
    #[must_use]
    pub fn group(
        &self,
    ) -> Option<(
        CapturedDelimiter,
        &Bounded<CapturedTokenTree, CapturedTokenLimit>,
    )> {
        match &self.payload {
            CapturedPayload::Group { delimiter, trees } => Some((*delimiter, trees)),
            CapturedPayload::Word(_)
            | CapturedPayload::Punct(_)
            | CapturedPayload::Text(_)
            | CapturedPayload::Number(_) => None,
        }
    }
}

impl CapturedInput {
    /// Take one captured input.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::LevelUnbounded`] when the top level carries more
    /// trees than the declared magnitude admits. A capture that does not fit
    /// refuses rather than reading part of a declaration.
    pub fn taken(trees: Vec<CapturedTokenTree>, issued: u32) -> Result<Self, CaptureBound> {
        Bounded::admitted_const(
            trees,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|trees| Self { trees, issued })
        .map_err(|_| CaptureBound::LevelUnbounded)
    }

    /// The top-level trees, in the order they were written.
    pub fn trees(&self) -> impl Iterator<Item = &CapturedTokenTree> {
        self.trees.iter()
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

    /// How many span handles the producer issued. A handle at or past this
    /// index names no token.
    #[must_use]
    pub const fn issued(&self) -> u32 {
        self.issued
    }

    /// The canonical bytes of this capture — what a plane identity over the
    /// declared input is derived from. Deterministic, and independent of span
    /// handles: two captures of the same declaration from different producers
    /// encode identically.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for tree in self.trees.iter() {
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

    /// One delimited group.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the group carries more
    /// tokens than the declared magnitude admits.
    pub fn group(
        delimiter: GeneratedDelimiter,
        tokens: Vec<Self>,
    ) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(
            tokens,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|tokens| Self::Group { delimiter, tokens })
    }

    /// The absolute path `::a::b::c`, as the tokens that spell it.
    ///
    /// Rendering a path by hand is where a renderer starts writing Rust text; a
    /// path stated as segments cannot be mis-spaced, cannot lose a colon, and
    /// cannot be built out of a string a caller supplied.
    #[must_use]
    pub fn absolute_path(segments: &[&str]) -> Vec<Self> {
        let mut tokens = Vec::new();
        for segment in segments {
            tokens.push(Self::joint(':'));
            tokens.push(Self::alone(':'));
            tokens.push(Self::word(segment));
        }
        tokens
    }

    /// The path `a::b::c` relative to the caller's own crate binding, as the
    /// tokens that spell it. The first segment is written as a plain word, so a
    /// caller that renamed its dependency is named the way it named itself.
    #[must_use]
    pub fn bound_path(binding: &str, segments: &[&str]) -> Vec<Self> {
        let mut tokens = vec![Self::word(binding)];
        for segment in segments {
            tokens.push(Self::joint(':'));
            tokens.push(Self::alone(':'));
            tokens.push(Self::word(segment));
        }
        tokens
    }
}

impl GeneratedTree {
    /// Assemble one generated tree.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the tree carries more
    /// top-level tokens than the declared magnitude admits.
    pub fn assembled(tokens: Vec<GeneratedToken>) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(
            tokens,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|tokens| Self { tokens })
    }

    /// The top-level tokens, in the order they were written.
    pub fn tokens(&self) -> impl Iterator<Item = &GeneratedToken> {
        self.tokens.iter()
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
    /// Returns [`BoundedConstruction::OverLimit`] when the joined tree outgrows
    /// the declared magnitude.
    pub fn joined(&self, other: &Self) -> Result<Self, BoundedConstruction> {
        let mut tokens: Vec<GeneratedToken> = self.tokens.iter().cloned().collect();
        tokens.extend(other.tokens.iter().cloned());
        Self::assembled(tokens)
    }

    /// The Rust source text this tree projects, for a person to read.
    ///
    /// A projection and only a projection. Nothing reads it back, no identity is
    /// derived from it, and a caller comparing two trees compares the trees.
    #[must_use]
    pub fn inspected(&self) -> String {
        let mut rendered = String::new();
        for token in self.tokens.iter() {
            inspect_token(token, &mut rendered);
        }
        rendered
    }

    /// The tree's canonical bytes — what a digest over the rendered unit is
    /// taken from.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for token in self.tokens.iter() {
            encode_generated(token, &mut bytes);
        }
        bytes
    }
}

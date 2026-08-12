//! The typed token seam: what the services read, and what they write.
//!
//! # Why the services carry their own token vocabulary
//!
//! `proc_macro` is a proc-macro-crate-only API. A crate that is not compiled as
//! a proc-macro cannot name its types at all, so the services — which are
//! ordinary callable Rust and must stay so — cannot take a `TokenStream` and
//! cannot hand one back. The answer is not to fall back to strings: a string is
//! a token stream with its structure thrown away, and everything the capture
//! then has to do is re-derive structure that the compiler already had.
//!
//! So the seam is typed on both sides.
//!
//! **Reading.** [`CapturedTokenTree`] is what one token of a declared input is:
//! a payload, a **stable local coordinate** naming exactly where it sits in the
//! tree, and an opaque [`SpanHandle`] indexing the producer's own span table.
//! Delimited groups stay groups; nothing is re-lexed and no balance is
//! re-discovered.
//!
//! **Writing.** [`GeneratedTree`] is what a renderer produces. The human Rust
//! text is [`GeneratedTree::inspected`] — a PROJECTION of the tree, produced for
//! a person to read, never the artifact itself. The artifact is the tree.
//!
//! # The span handle is opaque, and deliberately so
//!
//! A [`SpanHandle`] means "the token at this index of the table the producer
//! built while capturing". The services never resolve one: they carry it into a
//! diagnostic so that whoever produced the input can map it back to the exact
//! compiler span. That is what puts a `compile_error!` on the offending token
//! rather than on the first token of the declaration.

use crate::plane::{CapturedTokenLimit, GeneratedTokenLimit, encode_bytes, encode_length};
use threadpak::declaration::{CoordinateRole, SourceCoordinate};
use threadpak::types::{Bounded, BoundedConstruction};

// ---------------------------------------------------------------------------
// Reading: the captured token tree.
// ---------------------------------------------------------------------------

/// An opaque index into the producer's span table.
///
/// It carries no position, no file, and no length. It is a handle and only a
/// handle: the producer built the table while capturing, and only the producer
/// can turn one back into a compiler span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanHandle(u32);

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

/// The delimiter one captured group is written with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapturedDelimiter {
    /// `( … )`.
    Parenthesis,
    /// `{ … }`.
    Brace,
    /// `[ … ]`.
    Bracket,
    /// A group with no delimiter written — the invisible grouping a compiler
    /// inserts around a captured fragment. It is a real group and is never
    /// flattened away.
    Bare,
}

/// Where one captured token sits, independently of any span.
///
/// Stable under everything a span is not stable under: the coordinate is the
/// same whether the input arrived from a compiler or from text, whether the
/// file moved, and whether anything was reformatted. Two captures of the same
/// declaration agree on every coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalCoordinate {
    /// How many groups enclose this token. Top level is zero.
    pub depth: u32,
    /// This token's position among its enclosing group's own tokens.
    pub index: u32,
}

/// What one captured token carries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapturedPayload {
    /// An identifier-shaped word.
    Word(String),
    /// One punctuation character.
    Punct(char),
    /// A text literal, with its quotes removed.
    Text(String),
    /// A numeric literal, exactly as written.
    Number(String),
    /// A delimited group and the tokens inside it.
    Group {
        /// The delimiter written around the group.
        delimiter: CapturedDelimiter,
        /// The tokens inside, in the order they were written.
        trees: Bounded<CapturedTokenTree, CapturedTokenLimit>,
    },
}

/// One captured token: what it carries, where it sits, and how to reach the
/// compiler span it came from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedTokenTree {
    payload: CapturedPayload,
    coordinate: LocalCoordinate,
    span: SpanHandle,
}

impl CapturedTokenTree {
    /// Capture one token.
    #[must_use]
    pub const fn captured(
        payload: CapturedPayload,
        coordinate: LocalCoordinate,
        span: SpanHandle,
    ) -> Self {
        Self {
            payload,
            coordinate,
            span,
        }
    }

    /// What this token carries.
    #[must_use]
    pub const fn payload(&self) -> &CapturedPayload {
        &self.payload
    }

    /// Where this token sits.
    #[must_use]
    pub const fn coordinate(&self) -> LocalCoordinate {
        self.coordinate
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
    /// Returns [`BoundedConstruction::OverLimit`] when the group carries more
    /// tokens than the declared magnitude admits. A group that does not fit
    /// refuses rather than capturing as an empty one: an empty group is not a
    /// shorter declaration, it is a declaration with no body, and the two must
    /// never read alike.
    pub fn group_of(
        delimiter: CapturedDelimiter,
        trees: Vec<Self>,
        coordinate: LocalCoordinate,
        span: SpanHandle,
    ) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(trees).map(|trees| {
            Self::captured(
                CapturedPayload::Group { delimiter, trees },
                coordinate,
                span,
            )
        })
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

/// One captured declared input: the top-level token trees, and how many span
/// handles the producer issued.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedInput {
    trees: Bounded<CapturedTokenTree, CapturedTokenLimit>,
    issued: u32,
}

impl CapturedInput {
    /// Take one captured input.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the top level carries
    /// more trees than the declared magnitude admits. A capture that does not
    /// fit refuses rather than reading part of a declaration.
    pub fn taken(trees: Vec<CapturedTokenTree>, issued: u32) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(trees).map(|trees| Self { trees, issued })
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

/// Encode one captured tree into the canonical byte form, spans excluded.
fn encode_captured(tree: &CapturedTokenTree, into: &mut Vec<u8>) {
    match tree.payload() {
        CapturedPayload::Word(word) => {
            into.push(1);
            encode_text(word, into);
        }
        CapturedPayload::Punct(mark) => {
            into.push(2);
            let mut buffer = [0u8; 4];
            encode_text(mark.encode_utf8(&mut buffer), into);
        }
        CapturedPayload::Text(text) => {
            into.push(3);
            encode_text(text, into);
        }
        CapturedPayload::Number(number) => {
            into.push(4);
            encode_text(number, into);
        }
        CapturedPayload::Group { delimiter, trees } => {
            into.push(5);
            into.push(match delimiter {
                CapturedDelimiter::Parenthesis => 0,
                CapturedDelimiter::Brace => 1,
                CapturedDelimiter::Bracket => 2,
                CapturedDelimiter::Bare => 3,
            });
            encode_length(trees.len(), into);
            for inner in trees.iter() {
                encode_captured(inner, into);
            }
        }
    }
}

/// Encode one length-prefixed text under the plane's one length framing.
fn encode_text(text: &str, into: &mut Vec<u8>) {
    encode_bytes(text.as_bytes(), into);
}

// ---------------------------------------------------------------------------
// Resolving a handle back to a position.
// ---------------------------------------------------------------------------

/// How a producer answers "where is the token this handle names?".
///
/// Not an option and not a default. A producer either knows byte offsets into
/// the text it read, or it holds the compiler's own spans and resolves handles
/// on its own side — and the services never invent a position for a handle they
/// cannot resolve. The second posture is the honest one for an expansion shell:
/// the shell owns the spans, so the shell does the mapping, and a diagnostic
/// coordinate that read `byte 0` would be a fiction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpanTable {
    /// Byte offsets into the declared input, one per issued handle.
    ByteOffsets(Bounded<u64, CapturedTokenLimit>),
    /// The producer holds the compiler's spans and resolves handles itself.
    ProducerHeld,
}

impl SpanTable {
    /// Where the token one handle names sits, in whatever coordinate role this
    /// producer speaks.
    #[must_use]
    pub fn coordinate_of(&self, span: SpanHandle) -> SourceCoordinate {
        match self {
            Self::ByteOffsets(offsets) => {
                let index = usize::try_from(span.index()).unwrap_or(usize::MAX);
                offsets.iter().nth(index).map_or(
                    SourceCoordinate {
                        role: CoordinateRole::SemanticOrigin,
                        position: u64::from(span.index()),
                    },
                    |offset| SourceCoordinate {
                        role: CoordinateRole::Byte,
                        position: *offset,
                    },
                )
            }
            Self::ProducerHeld => SourceCoordinate {
                role: CoordinateRole::SemanticOrigin,
                position: u64::from(span.index()),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// The text route into the captured tree.
// ---------------------------------------------------------------------------

/// Why one text read refused. Dependent checks: there is no group to balance
/// until the characters were cut, and no magnitude to exceed until the trees
/// were built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextReadCause {
    /// A text literal was never closed.
    NotTerminated,
    /// A text literal carries an escape sequence. The grammar admits none, so
    /// what is captured renders back without a quoting question ever arising.
    NotEscapeFree,
    /// A delimited group was never closed.
    NotBalanced,
    /// A closing delimiter arrived with no group open.
    NotOpened,
    /// The read exceeds a declared magnitude.
    Unbounded,
}

/// One refused text read: the established cause, and the byte it sits at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextReadRefusal {
    /// The established cause.
    pub cause: TextReadCause,
    /// The byte position the cause was established at.
    pub at: u64,
}

/// One declared input read from TEXT: the captured trees, and the byte offsets
/// that resolve every handle the read issued.
///
/// This is the callable route. A compiler is one producer of captured input; a
/// test is another; text is the third, and it exists so that the
/// callable-services reproduction route a diagnostic names is a real road and
/// not a promise.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextCapture {
    input: CapturedInput,
    spans: SpanTable,
}

impl TextCapture {
    /// Read one declared input from source text.
    ///
    /// # Errors
    ///
    /// Returns [`TextReadRefusal`] naming the established cause and the byte it
    /// sits at.
    pub fn read(source: &str) -> Result<Self, TextReadRefusal> {
        let mut reader = TextReader {
            offsets: Vec::new(),
        };
        let mut characters = source.char_indices().peekable();
        let trees = reader.read_group(&mut characters, None, 0)?;
        let issued = u32::try_from(reader.offsets.len()).unwrap_or(u32::MAX);
        let offsets = Bounded::admitted_const(reader.offsets).map_err(|_| TextReadRefusal {
            cause: TextReadCause::Unbounded,
            at: 0,
        })?;
        let input = CapturedInput::taken(trees, issued).map_err(|_| TextReadRefusal {
            cause: TextReadCause::Unbounded,
            at: 0,
        })?;
        Ok(Self {
            input,
            spans: SpanTable::ByteOffsets(offsets),
        })
    }

    /// The captured input.
    #[must_use]
    pub const fn input(&self) -> &CapturedInput {
        &self.input
    }

    /// The table that resolves this read's handles.
    #[must_use]
    pub const fn spans(&self) -> &SpanTable {
        &self.spans
    }
}

/// The bounded hand-rolled text reader's running state: the byte offset issued
/// for each handle, in handle order.
struct TextReader {
    offsets: Vec<u64>,
}

/// One character stream over source text, with lookahead.
type Characters<'source> = core::iter::Peekable<core::str::CharIndices<'source>>;

impl TextReader {
    /// Issue the next handle for a token starting at one byte offset.
    fn issue(&mut self, at: u64) -> SpanHandle {
        let index = u32::try_from(self.offsets.len()).unwrap_or(u32::MAX);
        self.offsets.push(at);
        SpanHandle::at(index)
    }

    /// Read the tokens of one group, stopping at `closing` where one is given.
    fn read_group(
        &mut self,
        characters: &mut Characters<'_>,
        closing: Option<(char, u64)>,
        depth: u32,
    ) -> Result<Vec<CapturedTokenTree>, TextReadRefusal> {
        let mut trees: Vec<CapturedTokenTree> = Vec::new();
        loop {
            let Some(&(offset, character)) = characters.peek() else {
                return match closing {
                    Some((_, at)) => Err(TextReadRefusal {
                        cause: TextReadCause::NotBalanced,
                        at,
                    }),
                    None => Ok(trees),
                };
            };
            let at = u64::try_from(offset).unwrap_or(u64::MAX);
            if character.is_whitespace() {
                let _consumed = characters.next();
                continue;
            }
            if matches!(character, ')' | ']' | '}') {
                let expected = closing.map(|(close, _)| close);
                if expected == Some(character) {
                    let _consumed = characters.next();
                    return Ok(trees);
                }
                return Err(TextReadRefusal {
                    cause: TextReadCause::NotOpened,
                    at,
                });
            }
            let index = u32::try_from(trees.len()).unwrap_or(u32::MAX);
            let coordinate = LocalCoordinate { depth, index };
            let tree = self.read_token(characters, at, character, coordinate, depth)?;
            trees.push(tree);
        }
    }

    /// Read one token, whatever kind it is.
    fn read_token(
        &mut self,
        characters: &mut Characters<'_>,
        at: u64,
        character: char,
        coordinate: LocalCoordinate,
        depth: u32,
    ) -> Result<CapturedTokenTree, TextReadRefusal> {
        if let Some(delimiter) = opening(character) {
            let span = self.issue(at);
            let _consumed = characters.next();
            let inner = self.read_group(
                characters,
                Some((closing_of(delimiter), at)),
                depth.saturating_add(1),
            )?;
            let trees = Bounded::admitted_const(inner).map_err(|_| TextReadRefusal {
                cause: TextReadCause::Unbounded,
                at,
            })?;
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Group { delimiter, trees },
                coordinate,
                span,
            ));
        }
        if character.is_alphabetic() || character == '_' {
            let span = self.issue(at);
            let mut word = String::new();
            while let Some(&(_, next)) = characters.peek() {
                if next.is_alphanumeric() || next == '_' {
                    word.push(next);
                    let _consumed = characters.next();
                } else {
                    break;
                }
            }
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Word(word),
                coordinate,
                span,
            ));
        }
        if character.is_ascii_digit() {
            let span = self.issue(at);
            let mut number = String::new();
            while let Some(&(_, next)) = characters.peek() {
                if next.is_alphanumeric() || next == '_' || next == '.' {
                    number.push(next);
                    let _consumed = characters.next();
                } else {
                    break;
                }
            }
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Number(number),
                coordinate,
                span,
            ));
        }
        if character == '"' {
            let span = self.issue(at);
            let _consumed = characters.next();
            let mut text = String::new();
            loop {
                let Some((_, next)) = characters.next() else {
                    return Err(TextReadRefusal {
                        cause: TextReadCause::NotTerminated,
                        at,
                    });
                };
                if next == '"' {
                    break;
                }
                if next == '\\' {
                    return Err(TextReadRefusal {
                        cause: TextReadCause::NotEscapeFree,
                        at,
                    });
                }
                text.push(next);
            }
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Text(text),
                coordinate,
                span,
            ));
        }
        let span = self.issue(at);
        let _consumed = characters.next();
        Ok(CapturedTokenTree::captured(
            CapturedPayload::Punct(character),
            coordinate,
            span,
        ))
    }
}

/// The delimiter one opening character writes, where it opens a group.
const fn opening(character: char) -> Option<CapturedDelimiter> {
    match character {
        '(' => Some(CapturedDelimiter::Parenthesis),
        '[' => Some(CapturedDelimiter::Bracket),
        '{' => Some(CapturedDelimiter::Brace),
        _ => None,
    }
}

/// The character that closes one delimiter.
const fn closing_of(delimiter: CapturedDelimiter) -> char {
    match delimiter {
        CapturedDelimiter::Parenthesis => ')',
        CapturedDelimiter::Bracket => ']',
        CapturedDelimiter::Brace | CapturedDelimiter::Bare => '}',
    }
}

// ---------------------------------------------------------------------------
// Writing: the generated token tree.
// ---------------------------------------------------------------------------

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratedToken {
    /// An identifier-shaped word.
    Word(String),
    /// One punctuation character and whether it joins what follows.
    Punct {
        /// The character.
        mark: char,
        /// Whether it joins what follows.
        spacing: GeneratedSpacing,
    },
    /// A text literal. The renderer states the text; the quoting is the tree's
    /// business, so no caller ever composes a quoted string by hand.
    Text(String),
    /// A delimited group.
    Group {
        /// The delimiter.
        delimiter: GeneratedDelimiter,
        /// The tokens inside.
        tokens: Bounded<GeneratedToken, GeneratedTokenLimit>,
    },
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
        Bounded::admitted_const(tokens).map(|tokens| Self::Group { delimiter, tokens })
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

/// One generated token tree: the artifact a renderer produces.
///
/// The Rust source text a person reads is [`GeneratedTree::inspected`], and it
/// is a projection of this value rather than the other way round. Nothing in the
/// services parses that text back, and no identity is derived from it: the
/// digest is taken over [`GeneratedTree::canonical_bytes`], which is the tree's
/// own encoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedTree {
    tokens: Bounded<GeneratedToken, GeneratedTokenLimit>,
}

impl GeneratedTree {
    /// Assemble one generated tree.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the tree carries more
    /// top-level tokens than the declared magnitude admits.
    pub fn assembled(tokens: Vec<GeneratedToken>) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(tokens).map(|tokens| Self { tokens })
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

/// Project one generated token into Rust source text.
fn inspect_token(token: &GeneratedToken, into: &mut String) {
    match token {
        GeneratedToken::Word(word) => {
            into.push_str(word);
            into.push(' ');
        }
        GeneratedToken::Punct { mark, spacing } => {
            into.push(*mark);
            if *spacing == GeneratedSpacing::Alone {
                into.push(' ');
            }
        }
        GeneratedToken::Text(text) => {
            into.push('"');
            for character in text.chars() {
                if character == '"' || character == '\\' {
                    into.push('\\');
                }
                into.push(character);
            }
            into.push('"');
            into.push(' ');
        }
        GeneratedToken::Group { delimiter, tokens } => {
            let (open, close) = match delimiter {
                GeneratedDelimiter::Parenthesis => ('(', ')'),
                GeneratedDelimiter::Brace => ('{', '}'),
                GeneratedDelimiter::Bracket => ('[', ']'),
            };
            into.push(open);
            into.push(' ');
            for inner in tokens.iter() {
                inspect_token(inner, into);
            }
            into.push(close);
            into.push(' ');
        }
    }
}

/// Encode one generated token into the canonical byte form.
fn encode_generated(token: &GeneratedToken, into: &mut Vec<u8>) {
    match token {
        GeneratedToken::Word(word) => {
            into.push(1);
            encode_text(word, into);
        }
        GeneratedToken::Punct { mark, spacing } => {
            into.push(2);
            into.push(match spacing {
                GeneratedSpacing::Joint => 0,
                GeneratedSpacing::Alone => 1,
            });
            let mut buffer = [0u8; 4];
            encode_text(mark.encode_utf8(&mut buffer), into);
        }
        GeneratedToken::Text(text) => {
            into.push(3);
            encode_text(text, into);
        }
        GeneratedToken::Group { delimiter, tokens } => {
            into.push(4);
            into.push(match delimiter {
                GeneratedDelimiter::Parenthesis => 0,
                GeneratedDelimiter::Brace => 1,
                GeneratedDelimiter::Bracket => 2,
            });
            encode_length(tokens.len(), into);
            for inner in tokens.iter() {
                encode_generated(inner, into);
            }
        }
    }
}

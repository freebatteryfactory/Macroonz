//! The seam's declarations: what one captured declaration is, how a producer's span table answers, how a text read refuses, and what a renderer writes.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is where all five magnitudes below are settled.

use crate::bounded::Bounded;

#[path = "type_guard.rs"]
mod guard;

/// Steps one token path may carry, and so how deeply a declared input may nest.
///
/// A width bound alone bounds each level and says nothing about the depth, so an input nested a million groups deep would satisfy it at every level while the walk reading it did not terminate.
pub const TOKEN_PATH_DEPTH_LIMIT: usize = 32;

/// Token trees one captured input may carry at any one nesting level.
pub const CAPTURED_TOKEN_LIMIT: usize = 4096;

/// Tokens one captured input may carry across the whole tree, and positions one span table may hold.
///
/// The level bound and the depth bound multiply, so the total is bounded in its own right rather than left as the product of two other magnitudes; a table is not a level, so it stands here too.
pub const CAPTURED_TREE_TOKEN_LIMIT: usize = 16_384;

/// Units of capture work one walk may spend, one unit per examined token.
///
/// Deliberately wider than the whole-tree magnitude, because a walk may look at more than it keeps, and a budget at the tree magnitude exactly would refuse a lawful input the moment its producer looked twice at anything.
pub const CAPTURE_WORK_LIMIT: usize = 65_536;

/// Tokens one generated tree may carry at any one nesting level.
pub const GENERATED_TOKEN_LIMIT: usize = 4096;

/// An opaque index into the producer's span table.
///
/// It carries no position, no file, and no length: the producer built the table while capturing, and only the producer can turn one back into a compiler span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanHandle(u32);

/// The coordinate system one source position is counted in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordinateRole {
    /// A zero-based byte offset in the captured text.
    Byte,
    /// A zero-based ordinal retained by the source producer.
    SemanticOrigin,
}

/// One compiler-local source position with its coordinate system stated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceCoordinate {
    /// The coordinate system in which the position is counted.
    pub role: CoordinateRole,
    /// The zero-based position in that coordinate system.
    pub position: u64,
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
    /// A group with no delimiter written — the invisible grouping a compiler inserts around a captured fragment.
    ///
    /// It is a real group and is never flattened away, and a reader of text can never write one, because text that carries no delimiter carries no group.
    Bare,
}

/// Which declared magnitude one capture ran past.
///
/// Every row refuses before any partial tree exists: a truncated capture is a different declaration, and capturing one would put everything downstream to work on material nobody wrote.
#[must_use = "a bound refusal names which declared magnitude the capture would have passed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptureBound {
    /// The declared input nests deeper than the declared magnitude.
    Depth,
    /// One nesting level carries more token trees than the declared magnitude.
    Level,
    /// The whole tree carries more tokens than the declared magnitude.
    Tree,
    /// The walk spent the declared capture-work budget.
    Work,
}

/// Where one captured token sits, as the index route from the root of the declared input.
///
/// The route is unique by construction: `[3, 0, 5]` is the sixth token of the first token of the fourth top-level token, and nothing else in the tree spells that.
/// It is stable under everything a span is not stable under — which producer read the input, where the file moved, how the source was formatted — so two captures of one declaration agree on every route.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenPath {
    steps: Bounded<u32, TOKEN_PATH_DEPTH_LIMIT>,
}

/// The running state of one capture walk: what the walk has spent, and how much of the whole-tree magnitude it has taken.
///
/// The two are charged separately, because a producer that reads material it discards — a frontend skipping trivia, a reader backtracking over an alternative — spends work the result never shows, and the budget is the only magnitude that can see it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CaptureWalk {
    remaining: usize,
    taken: usize,
}

/// One non-group value a capture producer offers to the checked builder.
///
/// Groups have their own builder operation so no caller can smuggle child trees carrying foreign paths or handles through an atom seat.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapturedAtom {
    /// An identifier-shaped word.
    Word(String),
    /// One punctuation character.
    Punct(char),
    /// A text literal's text.
    Text(String),
    /// A numeric literal, exactly as written.
    Number(String),
    /// A byte-string literal's material.
    ByteText(Vec<u8>),
    /// One character literal's character.
    Character(char),
    /// One byte literal's byte.
    Byte(u8),
    /// A C string literal's material without its terminating NUL.
    NulTerminatedText(Vec<u8>),
}

/// What one captured token carries.
///
/// An arm carries a literal's value and never the characters it was spelled with, so `"x"` and `r"x"` are one text and which prefix a producer read is not a fact the tree keeps.
///
/// # Ordering
///
/// The roster grows at its end and nowhere else: each arm's slot is a byte of the canonical bytes a captured declaration's identity is derived over.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapturedPayload {
    /// An identifier-shaped word.
    Word(String),
    /// One punctuation character.
    Punct(char),
    /// A text literal's text: `"…"` and `r"…"` alike, escapes read and quotes removed.
    Text(String),
    /// A numeric literal, exactly as written: the base, the digit separators, and the suffix that types it are all part of what the declaration says.
    Number(String),
    /// A delimited group and the tokens inside it.
    Group {
        /// The delimiter written around the group.
        delimiter: CapturedDelimiter,
        /// The tokens inside, in the order they were written.
        trees: Bounded<CapturedTokenTree, CAPTURED_TOKEN_LIMIT>,
    },
    /// A byte-string literal's material: `b"…"` and `br"…"`, kept as bytes because material that is not text crosses without a lossy road existing for it to take.
    ByteText(Vec<u8>),
    /// One character literal's character: `'…'`.
    Character(char),
    /// One byte literal's byte: `b'…'`.
    Byte(u8),
    /// A C string literal's material: `c"…"` and `cr"…"`, without the terminating NUL, which is the literal form's and never the value's.
    NulTerminatedText(Vec<u8>),
}

/// Why one literal spelling could not be read into the value it names.
///
/// Neither row is the caller's mistake: every spelling that reaches this road was already lexed by a compiler, so a refusal is this crate saying it does not read what the compiler admitted.
#[must_use = "a literal refusal names why the spelling could not be read into a value"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiteralReadCause {
    /// The spelling opens with no literal form this grammar has a row for.
    NotAKnownForm,
    /// The form is one this grammar reads, and its body carries material this grammar could not read the value of.
    NotReadable,
}

/// One captured token: what it carries, where it sits, and how to reach the compiler span it came from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedTokenTree {
    payload: CapturedPayload,
    path: TokenPath,
    span: SpanHandle,
}

/// One captured declared input: the top-level token trees, and how many span handles the producer issued.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedInput {
    trees: Bounded<CapturedTokenTree, CAPTURED_TOKEN_LIMIT>,
    issued: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CaptureBuilderStanding {
    Ready,
    Refused { retained_before_capture: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CaptureLevelStanding {
    Open,
    Finished,
}

/// The only state that issues capture handles and retains the producer's matching source positions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaptureBuilder<Position> {
    positions: Vec<Position>,
    walk: CaptureWalk,
    standing: CaptureBuilderStanding,
}

/// One nesting level borrowed from a [`CaptureBuilder`].
///
/// A producer can append atoms or groups and cannot state a path, a handle, or a denominator.
/// Every operation consumes the level, and only a successful operation returns it, so a refused partial level cannot be finished.
pub struct CaptureLevel<'capture, Position> {
    positions: &'capture mut Vec<Position>,
    walk: &'capture mut CaptureWalk,
    builder_standing: &'capture mut CaptureBuilderStanding,
    retained_before_capture: usize,
    path: TokenPath,
    trees: Bounded<CapturedTokenTree, CAPTURED_TOKEN_LIMIT>,
    standing: CaptureLevelStanding,
}

/// Why a checked capture was not completed.
#[must_use = "a capture refusal names whether a declared bound or the producer's own reading stopped construction"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptureBuildRefusal<Position, ProducerRefusal> {
    /// One declared capture magnitude was exceeded at this producer position.
    Unbounded {
        /// The magnitude exceeded.
        bound: CaptureBound,
        /// The producer's own position for the token that reached it.
        at: Position,
    },
    /// The producer could not read one token after the builder issued its handle.
    ProducerRefused {
        /// The producer's typed reason.
        cause: ProducerRefusal,
        /// The handle already bound to the token's retained source position.
        at: SpanHandle,
    },
}

/// Why one span table could not say where a handle sits.
///
/// A caller holding the handle and the table's reach can tell a handle issued by another producer from a handle issued past the end of a truncated table, which is the whole of what is knowable from this side.
#[must_use = "a resolution refusal carries the handle and how far the table reaches"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanResolutionRefusal {
    /// The handle the table was asked to resolve.
    pub handle: SpanHandle,
    /// How many positions the table carries; a handle at or past this index names no position in it.
    pub reaches: usize,
}

/// How a producer answers "where is the token this handle names?".
///
/// Not an option and not a default: nothing here invents a position for a handle it cannot resolve, and a diagnostic coordinate reading `byte 0` under a producer-held table would be a fiction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpanTable {
    /// Byte offsets into the declared input, one per issued handle.
    ByteOffsets(Bounded<u64, CAPTURED_TREE_TOKEN_LIMIT>),
    /// The producer holds the compiler's spans and resolves handles itself.
    ProducerHeld,
}

/// Why one text read refused.
///
/// Dependent checks: there is no group to balance until the characters were cut, and no magnitude to exceed until the trees were built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextReadCause {
    /// A text literal was never closed.
    NotTerminated,
    /// A text literal carries an escape sequence, which this route admits none of.
    ///
    /// The captured grammar is wider than the route: a compiler shell hands escaped spellings over and `capture_literal` reads them, so what this cause bounds is this reader's own alphabet and never what a captured text may hold.
    NotEscapeFree,
    /// A delimited group was never closed.
    NotBalanced,
    /// A closing delimiter arrived with no group open.
    NotOpened,
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
/// The two seats are visible to `text.rs` alone, and that read establishes the relationship between them: the offsets table resolves exactly the handles the capture beside it issued.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextCapture {
    /// The captured input the read produced.
    pub(super) input: CapturedInput,
    /// The byte offsets that resolve the handles that read issued.
    pub(super) spans: SpanTable,
}

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
/// That is what keeps a caller from composing `b"…"` out of a word and a quoted string — a pair that is two tokens where the address reading it matches one.
///
/// # Ordering
///
/// The roster grows at its end and nowhere else: each arm's slot is a byte of [`GeneratedTree::canonical_bytes`], which is what a rendered unit's identity is derived over.
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
    /// A text literal, stated as its text; the quoting is the tree's business.
    Text(String),
    /// A delimited group.
    Group {
        /// The delimiter.
        delimiter: GeneratedDelimiter,
        /// The tokens inside.
        tokens: Bounded<GeneratedToken, GENERATED_TOKEN_LIMIT>,
    },
    /// A byte-string literal's material, written `b"…"`, because a clause declared to take `b"…"` does not take `"…"`.
    ByteText(Vec<u8>),
    /// An integer literal, written unsuffixed: plain digits and nothing else.
    ///
    /// The seat it lands in is what types it, so one renderer writes a count into a `u32` seat, a `u64` seat, and a `usize` seat without being told which, where a suffix would state a second type beside the one the address declares.
    /// It carries no sign and no fraction, and a value the destination seat cannot hold is refused at that seat by the reader's own type.
    Number(u64),
}

/// One generated token tree: the artifact a renderer produces.
///
/// # Nonclaims
///
/// The Rust source text a person reads is [`GeneratedTree::inspected`], a projection of this value rather than the other way round: nothing parses it back, and the digest is taken over [`GeneratedTree::canonical_bytes`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedTree {
    tokens: Bounded<GeneratedToken, GENERATED_TOKEN_LIMIT>,
}

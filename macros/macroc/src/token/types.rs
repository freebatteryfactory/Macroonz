//! The token seam's declarations: what a captured declaration is, how a
//! producer's span table answers, how a text read refuses, and what a renderer
//! writes.
//!
//! Declarations only. The roads that reach a private field — the capture walk's
//! budget, the route's steps, the read that builds a table — live in
//! `type_guard.rs`, this file's own child.

use crate::plane::{CapturedTokenLimit, GeneratedTokenLimit, TokenPathDepthLimit};
use threadpak::types::Bounded;

#[path = "type_guard.rs"]
mod guard;

/// An opaque index into the producer's span table.
///
/// It carries no position, no file, and no length. It is a handle and only a
/// handle: the producer built the table while capturing, and only the producer
/// can turn one back into a compiler span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanHandle(u32);

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

threadpak::closed_register! {
    /// How one capture refuses on a declared magnitude.
    ///
    /// Four magnitudes and four causes, because they are four different facts
    /// about a declared input and repairing one of them tells a caller nothing
    /// about the other three. Every one of them refuses BEFORE any partial tree
    /// exists: a truncated capture is a different declaration, and capturing one
    /// would put the whole road downstream to work on material nobody wrote.
    ///
    /// `described` is the bound rendered for a person — a projection of the
    /// typed value that nothing reads back, carried so that a producer reporting
    /// a refused capture composes no sentence of its own.
    #[must_use = "a bound refusal names which declared magnitude the capture would have passed"]
    pub enum CaptureBound {
        /// The declared input nests deeper than the declared magnitude.
        DepthUnbounded = "depth-unbounded",
            "threadpak refusal-family derive: the declared input nests deeper than the \
             declared magnitude";
        /// One nesting level carries more token trees than the declared magnitude.
        LevelUnbounded = "level-unbounded",
            "threadpak refusal-family derive: one nesting level of the declared input carries \
             more tokens than the declared magnitude";
        /// The whole tree carries more tokens than the declared magnitude.
        TreeUnbounded = "tree-unbounded",
            "threadpak refusal-family derive: the declared input carries more tokens than the \
             declared magnitude";
        /// The walk spent the declared capture-work budget.
        WorkUnbounded = "work-unbounded",
            "threadpak refusal-family derive: reading the declared input spent the declared \
             capture-work budget";
    }
}

/// Where one captured token sits, as the index route from the root of the
/// declared input.
///
/// # A depth and an index do not locate a token
///
/// The pair that stood here named two tokens with one value. The first token of
/// one group and the first token of its sibling both sit at depth one, index
/// zero, so a diagnostic, an origin mapping, or an inspection reading that pair
/// was pointing at whichever of them the reader guessed. The route from the root
/// is unique by construction: `[3, 0, 5]` is the sixth token of the first token
/// of the fourth top-level token, and nothing else in the tree spells that.
///
/// Stable under everything a span is not stable under: the route is the same
/// whether the input arrived from a compiler or from text, whether the file
/// moved, and whether anything was reformatted. Two captures of the same
/// declaration agree on every route.
///
/// Bounded by [`TokenPathDepthLimit`], so a route is never longer than the
/// nesting a declared input is allowed to reach.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenPath {
    steps: Bounded<u32, TokenPathDepthLimit>,
}

/// The running state of one capture walk: what the walk has spent, and how much
/// of the whole-tree magnitude it has taken.
///
/// # Why a budget sits beside the three magnitudes
///
/// The depth, level, and tree magnitudes bound the RESULT — how deep it nests,
/// how wide each level is, how many tokens it holds. The budget bounds the WALK,
/// and the two are charged separately: [`CaptureWalk::examined`] is spent on
/// every token a producer LOOKS AT, and [`CaptureWalk::took`] counts only the
/// tokens a producer KEEPS.
///
/// Both of today's producers keep every token they look at, so the tree
/// magnitude is the one that bites for them. That is the honest state and not an
/// argument for dropping the budget: a producer that reads material it discards
/// — a frontend skipping trivia, a reader backtracking over an alternative —
/// spends work the result never shows, and the budget is the only magnitude that
/// can see it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CaptureWalk {
    remaining: u32,
    taken: u32,
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
    path: TokenPath,
    span: SpanHandle,
}

/// One captured declared input: the top-level token trees, and how many span
/// handles the producer issued.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedInput {
    trees: Bounded<CapturedTokenTree, CapturedTokenLimit>,
    issued: u32,
}

/// Why one span table could not say where a handle sits.
///
/// One cause and two facts, because a table that reaches a handle answers and a
/// table that does not has exactly one thing to say: which handle ran past it,
/// and how far it reaches. A caller holding both can tell a handle issued by
/// another producer from a handle issued past the end of a truncated table,
/// which is the whole of what is knowable from this side.
#[must_use = "a resolution refusal carries the handle and how far the table reaches"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanResolutionRefusal {
    /// The handle the table was asked to resolve.
    pub handle: SpanHandle,
    /// How many positions the table carries. A handle at or past this index
    /// names no position in it.
    pub reaches: u32,
}

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

/// Why one text read refused. Dependent checks: there is no group to balance
/// until the characters were cut, and no magnitude to exceed until the trees
/// were built.
#[must_use = "an established cause is why the text read refused"]
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
    /// The read exceeds a declared magnitude, and this is which one. The bound
    /// travels rather than collapsing to one word: a reader told only
    /// "unbounded" cannot tell a tree that nests too deep from one that spends
    /// the walk's budget, and the two are repaired differently.
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

/// One declared input read from TEXT: the captured trees, and the byte offsets
/// that resolve every handle the read issued.
///
/// This is the callable route. A compiler is one producer of captured input; a
/// test is another; text is the third, and it exists so that the
/// callable-services reproduction route a diagnostic names is a real road and
/// not a promise.
///
/// The two seats are visible to the text route alone, and that route is what
/// establishes the relationship between them: the offsets table resolves exactly
/// the handles the capture beside it issued. Nothing outside `token/` can name
/// either seat, and nothing inside it builds the pair any other way.
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

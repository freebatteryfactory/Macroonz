//! The token seam's declarations: what a captured declaration is, how a
//! producer's span table answers, how a text read refuses, and what a renderer
//! writes.
//!
//! Declarations only.
//! The roads that reach a private field live in `type_guard.rs`, this file's
//! own child, which is where all four declared magnitudes are settled.
//!
//! Three of those four are declared BELOW, because only this seam asks what they
//! bound; the per-level magnitude is the compiler plane's, because the
//! refusal-family derive asks it too.

use crate::plane::{CapturedTokenLimit, GeneratedTokenLimit};
use threadpak::types::Bounded;

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many steps one token path may carry — how
    /// deeply a declared input may nest.
    ///
    /// # Bounds
    ///
    /// Thirty-two. A level bound alone bounds the WIDTH of each level and
    /// nothing about the depth, so an input nested a million groups deep
    /// satisfies it at every level while the walk that reads it does not
    /// terminate in any useful time. A route that would run past this refuses
    /// rather than saturating: a saturated depth makes two different tokens
    /// share one route.
    TokenPathDepthLimit = 32,
    /// The magnitude governing how many tokens one captured input may carry
    /// ACROSS the whole tree.
    ///
    /// # Bounds
    ///
    /// Sixteen thousand three hundred and eighty-four. The level bound and the
    /// depth bound MULTIPLY: four thousand tokens at each of thirty-two levels
    /// is a tree nobody declared and nobody wants captured, so the total is
    /// bounded in its own right rather than left as the product of two other
    /// magnitudes.
    ///
    /// A producer's span table stands under this magnitude too — one entry per
    /// handle it issued, across every level at once — because a table is not a
    /// level.
    CapturedTreeTokenLimit = 16384,
    /// The magnitude governing how many units of capture work one walk may
    /// spend, one unit per examined token.
    ///
    /// # Bounds
    ///
    /// Sixty-five thousand five hundred and thirty-six, and DELIBERATELY wider
    /// than the whole-tree magnitude: a walk may LOOK at more than it keeps, and
    /// a budget at the tree magnitude exactly would refuse a lawful input the
    /// moment its producer looked twice at anything.
    ///
    /// Four units for every token [`CapturedTreeTokenLimit`] admits, which is
    /// the room a producer that backtracks over an alternative or skips trivia
    /// needs and no more. That magnitude is the one this number stands over, so
    /// the two are moved together or not at all: a wider tree under this budget
    /// would refuse lawful declarations naming a bound they never approached,
    /// and this is the number that would have to move to keep the tree magnitude
    /// reachable. The two rows sit side by side here for exactly that reason.
    CaptureWorkLimit = 65536,
}

/// An opaque index into the producer's span table.
///
/// It carries no position, no file, and no length.
/// A handle and only a handle: the producer built the table while capturing,
/// and only the producer can turn one back into a compiler span.
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
    ///
    /// The one producer that writes it is a compiler shell, which is handed the
    /// grouping already made. A reader of text can never write one, because
    /// there are no characters to read: text that carries no delimiter carries
    /// no group, so the text route's alphabet has no row for this and needs
    /// none.
    Bare,
}

threadpak::closed_register! {
    /// How one capture refuses on a declared magnitude.
    ///
    /// One cause per magnitude, because each is a different fact about a
    /// declared input and repairing one tells a caller nothing about the rest.
    /// Every one of them refuses before any partial tree exists: a truncated
    /// capture is a different declaration, and capturing one would put the whole
    /// road downstream to work on material nobody wrote.
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
/// The route is unique by construction: `[3, 0, 5]` is the sixth token of the
/// first token of the fourth top-level token, and nothing else in the tree
/// spells that.
/// A depth and an index would not locate a token: the first token of one group
/// and the first token of its sibling both sit at depth one, index zero.
///
/// Stable under everything a span is not stable under: the route is the same
/// whether the input arrived from a compiler or from text, whether the file
/// moved, and whether anything was reformatted.
/// Two captures of the same declaration agree on every route.
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
/// # Bounds
///
/// The depth, level, and tree magnitudes bound the result — how deep it nests,
/// how wide each level is, how many tokens it holds.
/// The budget bounds the walk, and the two are charged separately:
/// [`CaptureWalk::examined`] is spent on every token a producer looks at, and
/// [`CaptureWalk::took`] counts only the tokens a producer keeps.
///
/// A producer that reads material it discards — a frontend skipping trivia, a
/// reader backtracking over an alternative — spends work the result never
/// shows, and the budget is the only magnitude that can see it.
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
/// A table that does not reach a handle has exactly one thing to say: which
/// handle ran past it, and how far it reaches.
/// A caller holding both can tell a handle issued by another producer from a
/// handle issued past the end of a truncated table, which is the whole of what
/// is knowable from this side.
#[must_use = "a resolution refusal carries the handle and how far the table reaches"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanResolutionRefusal {
    /// The handle the table was asked to resolve.
    pub handle: SpanHandle,
    /// How many positions the table carries.
    /// A handle at or past this index names no position in it.
    pub reaches: usize,
}

/// How a producer answers "where is the token this handle names?".
///
/// A producer either knows byte offsets into the text it read, or it holds the
/// compiler's own spans and resolves handles on its own side.
/// Not an option and not a default: the services never invent a position for a
/// handle they cannot resolve, and a diagnostic coordinate reading `byte 0`
/// under a producer-held table would be a fiction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpanTable {
    /// Byte offsets into the declared input, one per issued handle.
    ///
    /// Bounded by the WHOLE-TREE magnitude, because that is what the table
    /// counts: a producer issues one handle per token it keeps, across every
    /// level of the tree at once, so the table grows with the tree and never
    /// with any one level of it.
    /// Under the per-level magnitude the table would refuse a lawful
    /// declaration of four thousand and ninety-seven tokens while naming a
    /// magnitude that declaration never approached — a bound that bites is only
    /// evidence when it is the bound the input actually overran.
    ByteOffsets(Bounded<u64, CapturedTreeTokenLimit>),
    /// The producer holds the compiler's spans and resolves handles itself.
    ProducerHeld,
}

/// Why one text read refused.
///
/// Dependent checks: there is no group to balance until the characters were
/// cut, and no magnitude to exceed until the trees were built.
#[must_use = "an established cause is why the text read refused"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextReadCause {
    /// A text literal was never closed.
    NotTerminated,
    /// A text literal carries an escape sequence.
    /// The grammar admits none, so what is captured renders back without a
    /// quoting question ever arising.
    NotEscapeFree,
    /// A delimited group was never closed.
    NotBalanced,
    /// A closing delimiter arrived with no group open.
    NotOpened,
    /// The read exceeds a declared magnitude, and this is which one.
    /// The bound travels rather than collapsing to one word: a reader told only
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

/// One declared input read from text: the captured trees, and the byte offsets
/// that resolve every handle the read issued.
///
/// The callable route.
/// A compiler is one producer of captured input, a test is another, and text is
/// the third — it exists so that the callable-services reproduction route a
/// diagnostic names is a real road and not a promise.
///
/// The two seats are visible to the text route alone, and that route is what
/// establishes the relationship between them: the offsets table resolves
/// exactly the handles the capture beside it issued.
/// Nothing outside `token/` can name either seat, and nothing inside it builds
/// the pair any other way.
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
/// A renderer states a literal's VALUE and never its spelling: a text literal is
/// its text, a byte-string literal is its bytes, an integer literal is its
/// number.
/// The quoting, the escaping, and the absence of a suffix are the tree's
/// business, which is what keeps a caller from composing `b"…"` out of a word
/// and a quoted string — that pair is two tokens where the address reading it
/// matches one.
///
/// # Ordering
///
/// The roster grows at its END and nowhere else.
/// Each arm's slot is written in the seam's `encode.rs`, a slot is a byte of
/// [`GeneratedTree::canonical_bytes`], and those bytes are the content a
/// rendered unit's plane identity is derived over — so an arm inserted among the
/// existing ones would renumber every slot after it and rename identities that
/// were already derived.
/// Appending renames nothing: a tree that was spellable before an arm was added
/// encodes to the same bytes after it.
/// The captured roster next door is ordered differently for the same reason it
/// is ordered at all — it was declared once, in one act, and has never had to
/// grow under material already encoded.
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
    /// A text literal.
    /// The renderer states the text; the quoting is the tree's business, so no
    /// caller ever composes a quoted string by hand.
    Text(String),
    /// A delimited group.
    Group {
        /// The delimiter.
        delimiter: GeneratedDelimiter,
        /// The tokens inside.
        tokens: Bounded<GeneratedToken, GeneratedTokenLimit>,
    },
    /// A BYTE-STRING literal: the material, written `b"…"`.
    ///
    /// Bytes rather than text, because the two are different literals at the
    /// address they are written to.
    /// A clause declared to take `b"…"` does not take `"…"`, and a text literal
    /// carrying the same characters is a different value at that seat — so a
    /// producer holding thirty-two bytes of a pinned identity has one arm that
    /// says what it holds and one that says something else.
    ///
    /// The renderer states the material and never the spelling: the `b`, the
    /// quotes, and every escape are the tree's, exactly as the quoting of
    /// [`GeneratedToken::Text`] is.
    ByteText(Vec<u8>),
    /// An integer literal, written UNSUFFIXED: plain digits and nothing else.
    ///
    /// Unsuffixed because the consumer's type position is what types it.
    /// The literal is written into a seat the address already declares — a
    /// constructor parameter, a roster element, an attribute argument — and an
    /// unsuffixed literal takes the type that seat demands, so one renderer
    /// writes a count into a `u32` seat, a `u64` seat, and a `usize` seat
    /// without being told which.
    /// A suffix would state a second type beside the one the address declares,
    /// and where the two disagreed the consumer would be shown a mismatch this
    /// producer invented rather than a fact about its own declaration.
    ///
    /// The payload is a `u64` because that is the widest count the services
    /// carry.
    /// A value the destination seat cannot hold is refused at that seat, by the
    /// consumer's own type, rather than by a narrower payload here that would
    /// refuse it in this producer's name.
    ///
    /// # Nonclaims
    ///
    /// It carries no sign and no fraction: the arm is what the services actually
    /// have to write — counts and byte magnitudes — and an arm that admitted
    /// spellings no renderer produces would be a grammar nobody exercises.
    Number(u64),
}

/// One generated token tree: the artifact a renderer produces.
///
/// # Nonclaims
///
/// The Rust source text a person reads is [`GeneratedTree::inspected`], a
/// projection of this value rather than the other way round.
/// Nothing in the services parses that text back, and no identity is derived
/// from it: the digest is taken over [`GeneratedTree::canonical_bytes`], which
/// is the tree's own encoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedTree {
    tokens: Bounded<GeneratedToken, GeneratedTokenLimit>,
}

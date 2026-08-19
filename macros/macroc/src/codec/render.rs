//! The token half of the road: the decode refusal this home declares, the
//! encode road that writes one declared shape's canonical bytes, the decode road
//! that reads them back and refuses, and the placement that carries them.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every brace is a group, and no function
//! here composes Rust source. The Rust a person reads is
//! [`GeneratedTree::inspected`](crate::token::GeneratedTree::inspected), a
//! projection of what is emitted rather than the thing itself.
//!
//! # The framing, stated once
//!
//! Every variable-length member is written as the framing width's worth of
//! big-endian length followed by the bytes — the plane's own framing, the same
//! one every canonical encoding in these services is written through. Two
//! members can therefore never be re-cut at a different boundary and produce one
//! byte string. A NESTED member is framed on exactly those terms rather than run
//! to the end of the input, because a nested value that consumed the remainder
//! would make the member after it unreadable.
//!
//! # No numeric literal is written anywhere here
//!
//! The generated-token roster carries four arms — word, punctuation, text, group
//! — and no numeric one. Rather than refuse, every place a number would have
//! stood is written as the language's own road to the same value: the framing
//! width is `::core::mem::size_of::<u64>()`, a presence byte is `u8::from(false)`
//! and `u8::from(true)`, a repeated member stops on a length comparison, and a
//! closed choice's admitted slots are the owner's OWN declared roster walked and
//! compared by `slot()`.
//!
//! That last one is the road worth reading twice: this home never writes a table
//! of slots. A roster that gained an arm gains it in the decode road too, without
//! this home ever learning what the arms are — and a slot no arm answers to
//! refuses rather than electing a neighbour.
//!
//! # The decode road IS the validator
//!
//! Every read refuses at the member it is standing at, naming that member as a
//! text literal, and the road refuses once more where material remains after the
//! last declared member. "These bytes are a lawful value" is therefore exactly
//! "the decode road returned one", and there is no second pass to run.

use super::type_contract::covers;
use super::{
    AssemblyPosture, CodecMember, CodecMemberShape, CodecPlacement, CodecRoad, CodecShape,
    CodecSurfaceIssue, CodecTypePath, PathRooting,
};
use crate::plane::GeneratedTokenLimit;
use crate::planning::CodecDirection;
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::schema::FieldCardinality;
use threadpak::types::ConstLimit;

// ---------------------------------------------------------------------------
// The spellings this home writes at the address it renders into.
// ---------------------------------------------------------------------------

/// The road that writes one declared shape's canonical bytes.
pub const ENCODE_ROAD: &str = "encode_canonical";

/// The road that reads those bytes back, and refuses where they are not the
/// shape's.
pub const DECODE_ROAD: &str = "decode_canonical";

/// The encode road's one parameter: the sink the bytes are appended to.
pub const INTO_PARAMETER: &str = "into";

/// The decode road's one parameter: the material read.
pub const MATERIAL_PARAMETER: &str = "material";

/// The decode road's running cursor over what it has not yet read.
pub const REMAINING_BINDING: &str = "remaining";

/// The binding one member's own value stands under while it is being written or
/// read.
pub const CARRIED_BINDING: &str = "carried";

/// The binding a framed member's declared length stands under.
pub const LENGTH_BINDING: &str = "length";

/// The binding a framed member's length stands under once it is an addressable
/// width.
pub const WIDTH_BINDING: &str = "width";

/// The binding a nested member's own encoding stands under before it is framed.
pub const NESTED_BINDING: &str = "nested";

/// The binding a repeated member's elements are gathered into.
pub const COLLECTED_BINDING: &str = "collected";

/// The binding one arm of a closed roster stands under while the walk is
/// comparing it.
pub const CANDIDATE_BINDING: &str = "candidate";

/// The binding a borrowed single byte — a slot, a presence — stands under.
pub const CHOSEN_BINDING: &str = "chosen";

/// The binding the elected arm of a closed roster stands under.
pub const ELECTED_BINDING: &str = "elected";

/// The binding an optional member's presence byte stands under.
pub const PRESENT_BINDING: &str = "present";

/// The seat every member-bearing refusal arm names the member through.
pub const MEMBER_SEAT: &str = "member";

/// The refusal arm a decode road answers with where material remains after the
/// last declared member.
pub const TRAILING_BYTES_ARM: &str = "TrailingBytes";

/// The refusal arm a checked assembly road's own refusal is carried under.
pub const NOT_ASSEMBLED_ARM: &str = "NotAssembled";

/// The roster constant a closed choice's admitted arms are walked through.
pub const ROSTER_CONSTANT: &str = "ALL";

/// The road one arm of a closed roster answers its declared position through.
pub const SLOT_ROAD: &str = "slot";

/// The one import a published codec module's head writes.
///
/// A wrapped surface names the owner's type and every member's type in the scope
/// the module sits IN rather than the scope it sits in itself, so the module's
/// head brings that scope with it. One import and no more: a module that reached
/// further would be deciding what else a consumer's generated module can see.
pub const MODULE_PRELUDE_ROOT: &str = "super";

/// One arm of the decode refusal this home declares, and the sentence it carries
/// for a reader.
///
/// Every arm here names the MEMBER it refused at, because a caller told only that
/// decoding failed has nothing to look at — and the member's spelling is a text
/// literal, which is the one literal arm the generated-token roster carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecodeRefusalArm {
    /// The variant's rendered spelling.
    pub spelling: &'static str,
    /// The sentence the variant documents itself with.
    pub sentence: &'static str,
}

/// The complete member-bearing roster of the decode refusal this home renders.
///
/// Nine arms, in the order a read establishes them: the material ran out, a
/// declared length ran past what remained or past what a machine can address, a
/// count did not fit the member's own width, text was not UTF-8, the member's own
/// type refused what was read, a slot named no admitted arm, a nested codec
/// refused, and a presence byte was neither of the two the encode road writes.
///
/// [`TRAILING_BYTES_ARM`] is not on this roster because it names no member: it is
/// a fact about the whole material rather than about one read.
pub const DECODE_REFUSAL_ARMS: [DecodeRefusalArm; 9] = [
    DecodeRefusalArm {
        spelling: TRUNCATED_ARM,
        sentence: "The material ended inside this member.",
    },
    DecodeRefusalArm {
        spelling: LENGTH_PAST_REMAINING_ARM,
        sentence: "This member's declared length runs past the material that remains.",
    },
    DecodeRefusalArm {
        spelling: LENGTH_PAST_WIDTH_ARM,
        sentence: "This member's declared length does not fit an addressable width.",
    },
    DecodeRefusalArm {
        spelling: COUNT_PAST_WIDTH_ARM,
        sentence: "This member's declared count does not fit the width the member is held at.",
    },
    DecodeRefusalArm {
        spelling: TEXT_NOT_UTF8_ARM,
        sentence: "This member's framed bytes are not UTF-8.",
    },
    DecodeRefusalArm {
        spelling: MEMBER_NOT_ADMITTED_ARM,
        sentence: "The member's own type refused what was read for it.",
    },
    DecodeRefusalArm {
        spelling: SLOT_NOT_ADMITTED_ARM,
        sentence: "The slot read for this member names no arm of the roster it was declared over.",
    },
    DecodeRefusalArm {
        spelling: NESTED_REFUSED_ARM,
        sentence: "The nested codec this member carries refused the framed material.",
    },
    DecodeRefusalArm {
        spelling: PRESENCE_NOT_ADMITTED_ARM,
        sentence: "This member's presence byte is neither of the two the encode road writes.",
    },
];

/// The refusal arm a read answers with where the material ended inside a member.
pub const TRUNCATED_ARM: &str = "Truncated";

/// The refusal arm a framed read answers with where the declared length runs
/// past what remains.
pub const LENGTH_PAST_REMAINING_ARM: &str = "LengthPastRemaining";

/// The refusal arm a framed read answers with where the declared length does not
/// fit an addressable width.
pub const LENGTH_PAST_WIDTH_ARM: &str = "LengthPastAddressableWidth";

/// The refusal arm a count read answers with where the declared count does not
/// fit the member's own width.
pub const COUNT_PAST_WIDTH_ARM: &str = "CountPastDeclaredWidth";

/// The refusal arm a text read answers with where the framed bytes are not
/// UTF-8.
pub const TEXT_NOT_UTF8_ARM: &str = "TextNotUtf8";

/// The refusal arm a read answers with where the member's own type refused what
/// was read for it.
pub const MEMBER_NOT_ADMITTED_ARM: &str = "MemberNotAdmitted";

/// The refusal arm a closed-choice read answers with where the slot names no arm
/// of the owner's declared roster.
pub const SLOT_NOT_ADMITTED_ARM: &str = "SlotNotAdmitted";

/// The refusal arm a nested read answers with where the nested codec refused.
pub const NESTED_REFUSED_ARM: &str = "NestedMemberRefused";

/// The refusal arm an optional read answers with where the presence byte is
/// neither of the two the encode road writes.
pub const PRESENCE_NOT_ADMITTED_ARM: &str = "PresenceNotAdmitted";

/// The sentence the rendered decode refusal documents itself with.
const REFUSAL_SENTENCE: &str = "Why one decode of this shape's canonical bytes refused. Holding \
     one is the whole of what went wrong: the arm says which read established it and, where the \
     read was about one member, which member it was standing at.";

/// The sentence the rendered encode road documents itself with.
const ENCODE_SENTENCE: &str = "Append this value's canonical bytes. Every variable-length member \
     is written length-prefixed at the framing width, in the order the shape declares its members, \
     so two values this shape considers different never encode identically.";

/// The sentence the rendered decode road documents itself with.
const DECODE_SENTENCE: &str = "Read one value back from its canonical bytes, refusing where the \
     material is not this shape's. A refusal names the member the read was standing at, and \
     material remaining after the last declared member is itself a refusal.";

/// The sentence a published codec module documents itself with.
const MODULE_SENTENCE: &str = "ThreadPak codec projection: the canonical encode and decode roads \
     for one declared shape, published here rather than spliced beside the declaration. Its head \
     imports the scope the module sits in, which is where the shape's own names live.";

/// The sentence the whole-material refusal arm documents itself with.
const TRAILING_SENTENCE: &str = "Material remains after the last declared member. A canonical \
     encoding is the whole of what a value writes, so a longer input is not this value with \
     something after it.";

/// The sentence the assembly refusal arm documents itself with.
const ASSEMBLY_SENTENCE: &str = "Every member was read, and the road that assembles them refused. \
     The refusal is the owner's own, carried exactly.";

// ---------------------------------------------------------------------------
// The token primitives.
// ---------------------------------------------------------------------------

/// The issue a tree that outgrew the declared token magnitude amounts to.
#[must_use]
pub fn unbounded() -> CodecSurfaceIssue {
    CodecSurfaceIssue::SurfaceTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One delimited group, with a tree past the declared magnitude refused in this
/// home's own vocabulary.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the group carries
/// more tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, CodecSurfaceIssue> {
    GeneratedToken::group(delimiter, tokens).map_err(|_| unbounded())
}

/// One path a caller declared, spelled from the rooting it stated.
#[must_use]
pub fn type_path(path: &CodecTypePath) -> Vec<GeneratedToken> {
    let segments: Vec<&str> = path.segments().map(String::as_str).collect();
    match path.rooting() {
        PathRooting::CrateAbsolute => GeneratedToken::absolute_path(&segments),
        PathRooting::InScope => in_scope_path(&segments),
    }
}

/// One path resolved in the scope the surface lands in: the first segment as a
/// plain word, and every later one behind a separator.
fn in_scope_path(segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for segment in segments {
        if tokens.is_empty() {
            tokens.push(GeneratedToken::word(segment));
            continue;
        }
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word(segment));
    }
    tokens
}

/// One path rooted at the language's own crates, spelled absolutely.
#[must_use]
pub fn language_path(segments: &[&str]) -> Vec<GeneratedToken> {
    GeneratedToken::absolute_path(segments)
}

/// One `::` separator followed by a word — the road from a path to an associated
/// item on it.
#[must_use]
pub fn associated(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(spelling),
    ]
}

/// One attribute over the body a caller spelled.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn attribute(body: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// One `#[doc = "…"]` attribute, as the tokens that spell it.
///
/// Every public item this home renders carries one, because a lint wall that
/// denies an undocumented public item is the wall a consumer's own crate is most
/// likely to be standing behind.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn doc_attribute(sentence: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    attribute(vec![
        GeneratedToken::word("doc"),
        GeneratedToken::alone('='),
        GeneratedToken::text(sentence),
    ])
}

/// The framing width, as the language's own road to it rather than as a number.
#[must_use]
pub fn framing_width() -> Vec<GeneratedToken> {
    let mut tokens = language_path(&["core", "mem", "size_of"]);
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::alone('<'));
    tokens.push(GeneratedToken::word("u64"));
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// One byte's width, as the language's own road to it rather than as a number.
#[must_use]
pub fn byte_width() -> Vec<GeneratedToken> {
    let mut tokens = language_path(&["core", "mem", "size_of"]);
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::alone('<'));
    tokens.push(GeneratedToken::word("u8"));
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// One qualified road: `<Path as Trait>::road`.
///
/// Qualified rather than plain, so the rendered call names the exact trait the
/// member contract bills for and never resolves onto an inherent road that
/// happened to share a spelling.
#[must_use]
pub fn qualified(
    subject: Vec<GeneratedToken>,
    contract: Vec<GeneratedToken>,
    road: &str,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('<')];
    tokens.extend(subject);
    tokens.push(GeneratedToken::word("as"));
    tokens.extend(contract);
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(associated(road));
    tokens
}

/// One generic argument list: `<…>`.
#[must_use]
pub fn generics(arguments: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('<')];
    tokens.extend(arguments);
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// One statement: the tokens a caller spelled, closed with a semicolon.
#[must_use]
pub fn statement(mut tokens: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One member read off `self`, as the tokens that spell it.
#[must_use]
pub fn self_member(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(spelling),
    ]
}

/// One member read off `self`, borrowed and parenthesized so a shape's write
/// road always stands over a reference whatever the cardinality supplied it.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the expression
/// outgrows the declared token magnitude.
pub fn borrowed_self_member(spelling: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut inner = vec![GeneratedToken::alone('&')];
    inner.extend(self_member(spelling));
    Ok(vec![group(GeneratedDelimiter::Parenthesis, inner)?])
}

/// One method call: `<receiver>.<road>(<arguments>)`.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn call(
    receiver: Vec<GeneratedToken>,
    road: &str,
    arguments: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = receiver;
    tokens.push(GeneratedToken::alone('.'));
    tokens.push(GeneratedToken::word(road));
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(tokens)
}

/// One member-bearing refusal construction: `<Refusal>::<Arm> { member: "…" }`.
///
/// The member's spelling is a TEXT literal, which is the one literal arm the
/// generated-token roster carries — so a refusal this home renders always names
/// the member the read was standing at.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the construction
/// outgrows the declared token magnitude.
pub fn member_refusal(
    refusal: &str,
    arm: &str,
    member: &str,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = vec![GeneratedToken::word(refusal)];
    tokens.extend(associated(arm));
    tokens.push(group(
        GeneratedDelimiter::Brace,
        vec![
            GeneratedToken::word(MEMBER_SEAT),
            GeneratedToken::alone(':'),
            GeneratedToken::text(member),
            GeneratedToken::alone(','),
        ],
    )?);
    Ok(tokens)
}

/// One payload-free refusal construction: `<Refusal>::<Arm>`.
#[must_use]
pub fn sole_refusal(refusal: &str, arm: &str) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(refusal)];
    tokens.extend(associated(arm));
    tokens
}

/// `.map_err(|_| <refusal>)?` — the road a fallible step takes to this home's own
/// refusal.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the road outgrows
/// the declared token magnitude.
pub fn mapped(refusal: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut closure = vec![
        GeneratedToken::alone('|'),
        GeneratedToken::word("_"),
        GeneratedToken::alone('|'),
    ];
    closure.extend(refusal);
    let mut tokens = vec![GeneratedToken::alone('.'), GeneratedToken::word("map_err")];
    tokens.push(group(GeneratedDelimiter::Parenthesis, closure)?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// `.ok_or(<refusal>)?` — the road an absent read takes to this home's own
/// refusal.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the road outgrows
/// the declared token magnitude.
pub fn absent(refusal: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = vec![GeneratedToken::alone('.'), GeneratedToken::word("ok_or")];
    tokens.push(group(GeneratedDelimiter::Parenthesis, refusal)?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// `&u64::try_from(<expression>.len()).unwrap_or(u64::MAX).to_be_bytes()` — one
/// framed length, at the framing width, written without a numeric literal.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the expression
/// outgrows the declared token magnitude.
pub fn framed_length(
    material: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let counted = call(material, "len", Vec::new())?;
    let mut tokens = vec![GeneratedToken::alone('&'), GeneratedToken::word("u64")];
    tokens.extend(associated("try_from"));
    tokens.push(group(GeneratedDelimiter::Parenthesis, counted)?);
    let mut ceiling = vec![GeneratedToken::word("u64")];
    ceiling.extend(associated("MAX"));
    tokens = call(tokens, "unwrap_or", ceiling)?;
    call(tokens, "to_be_bytes", Vec::new())
}

/// `into.extend_from_slice(<expression>);` — one appended run.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the statement
/// outgrows the declared token magnitude.
pub fn appended(material: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let call_tokens = call(
        vec![GeneratedToken::word(INTO_PARAMETER)],
        "extend_from_slice",
        material,
    )?;
    Ok(statement(call_tokens))
}

// ---------------------------------------------------------------------------
// The rendered decode refusal.
// ---------------------------------------------------------------------------

/// `#[derive(Debug, Clone, PartialEq, Eq)]`, as the tokens that spell it.
///
/// Four derives and no more: a refusal is shown in a failure report, cloned into
/// one, and compared against an expectation, and nothing about a decode refusal
/// needs ordering or hashing. `Copy` is absent because the assembly arm may carry
/// a refusal the owner declared, and this home does not decide whether that one
/// copies.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn derive_attribute() -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let named = group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word("Debug"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Clone"),
            GeneratedToken::alone(','),
            GeneratedToken::word("PartialEq"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Eq"),
        ],
    )?;
    attribute(vec![GeneratedToken::word("derive"), named])
}

/// One member-bearing variant of the rendered decode refusal.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the variant outgrows
/// the declared token magnitude.
pub fn member_variant(arm: DecodeRefusalArm) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = doc_attribute(arm.sentence)?;
    tokens.push(GeneratedToken::word(arm.spelling));
    tokens.push(group(
        GeneratedDelimiter::Brace,
        vec![
            GeneratedToken::word(MEMBER_SEAT),
            GeneratedToken::alone(':'),
            GeneratedToken::alone('&'),
            GeneratedToken::joint('\''),
            GeneratedToken::word("static"),
            GeneratedToken::word("str"),
            GeneratedToken::alone(','),
        ],
    )?);
    tokens.push(GeneratedToken::alone(','));
    Ok(tokens)
}

/// The decode refusal one shape's surface declares.
///
/// Every member-bearing arm, then the whole-material arm, then the assembly arm
/// a CHECKED assembly road earns — and only that road earns it, so a total
/// assembly renders a refusal with nothing on it that cannot happen.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the declaration
/// outgrows the declared token magnitude.
pub fn refusal_declaration(shape: &CodecShape) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut variants: Vec<GeneratedToken> = Vec::new();
    for arm in DECODE_REFUSAL_ARMS {
        variants.extend(member_variant(arm)?);
    }
    variants.extend(doc_attribute(TRAILING_SENTENCE)?);
    variants.push(GeneratedToken::word(TRAILING_BYTES_ARM));
    variants.push(GeneratedToken::alone(','));
    if let AssemblyPosture::Checked { refusal } = shape.assembly().posture() {
        variants.extend(doc_attribute(ASSEMBLY_SENTENCE)?);
        variants.push(GeneratedToken::word(NOT_ASSEMBLED_ARM));
        variants.push(group(GeneratedDelimiter::Parenthesis, type_path(refusal))?);
        variants.push(GeneratedToken::alone(','));
    }
    let mut tokens = doc_attribute(REFUSAL_SENTENCE)?;
    tokens.extend(derive_attribute()?);
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("enum"));
    tokens.push(GeneratedToken::word(shape.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, variants)?);
    Ok(tokens)
}

/// The conversion a CHECKED assembly road earns: the owner's own refusal into
/// this surface's.
///
/// Rendered rather than billed. A checked assembly is the only posture that needs
/// it, and writing it here costs the address nothing — where the test-descriptor
/// crossing writes `?` and states a bill, this home owns both sides of the
/// conversion and simply declares it.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the implementation
/// outgrows the declared token magnitude.
pub fn refusal_conversion(shape: &CodecShape) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let AssemblyPosture::Checked { refusal } = shape.assembly().posture() else {
        return Ok(Vec::new());
    };
    let carried = type_path(refusal);
    let mut body = vec![GeneratedToken::word("Self")];
    body.extend(associated(NOT_ASSEMBLED_ARM));
    body.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    let mut parameters = vec![
        GeneratedToken::word(CARRIED_BINDING),
        GeneratedToken::alone(':'),
    ];
    parameters.extend(carried.clone());
    let road = vec![
        GeneratedToken::word("fn"),
        GeneratedToken::word("from"),
        group(GeneratedDelimiter::Parenthesis, parameters)?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
        GeneratedToken::word("Self"),
        group(GeneratedDelimiter::Brace, body)?,
    ];
    let mut tokens = vec![GeneratedToken::word("impl")];
    tokens.extend(language_path(&["core", "convert", "From"]));
    tokens.extend(generics(carried));
    tokens.push(GeneratedToken::word("for"));
    tokens.push(GeneratedToken::word(shape.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, road)?);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The encode road.
// ---------------------------------------------------------------------------

/// `::std::vec::Vec<u8>` — the sink the encode road appends to.
#[must_use]
pub fn byte_sink() -> Vec<GeneratedToken> {
    let mut tokens = language_path(&["std", "vec", "Vec"]);
    tokens.extend(generics(vec![GeneratedToken::word("u8")]));
    tokens
}

/// The byte slice type a decode road reads: `[u8]`.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the group outgrows
/// the declared token magnitude.
pub fn byte_slice() -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    Ok(vec![group(
        GeneratedDelimiter::Bracket,
        vec![GeneratedToken::word("u8")],
    )?])
}

/// One `let <binding> = <expression>;` statement.
#[must_use]
pub fn bound(binding: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word(binding),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    statement(tokens)
}

/// One `let mut <binding> = <expression>;` statement.
#[must_use]
pub fn bound_mutable(binding: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("mut"),
        GeneratedToken::word(binding),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    statement(tokens)
}

/// One `<binding> = <expression>;` reassignment.
#[must_use]
pub fn reassigned(binding: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(binding), GeneratedToken::alone('=')];
    tokens.extend(expression);
    statement(tokens)
}

/// `::std::vec::Vec::new()` — one empty gathering.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn empty_vector() -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = language_path(&["std", "vec", "Vec"]);
    tokens.extend(associated("new"));
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    Ok(tokens)
}

/// One member's write, over the subject the cardinality handed it.
///
/// The subject always stands for a REFERENCE to one occurrence, whichever
/// cardinality supplied it, so the five shape roads never learn how many of the
/// member there were.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the write outgrows
/// the declared token magnitude.
pub fn write_member(
    member: &CodecMember,
    subject: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    match member.shape() {
        CodecMemberShape::Count => write_count(subject),
        CodecMemberShape::Bytes => write_framed(member, subject, CodecMemberShape::Bytes),
        CodecMemberShape::Text => write_framed(member, subject, CodecMemberShape::Text),
        CodecMemberShape::ClosedChoice => write_slot(subject),
        CodecMemberShape::Nested => write_nested(subject),
    }
}

/// A count, at the framing width.
fn write_count(subject: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut dereferenced = vec![GeneratedToken::alone('*')];
    dereferenced.extend(subject);
    let mut widened = vec![GeneratedToken::word("u64")];
    widened.extend(associated("from"));
    widened.push(group(GeneratedDelimiter::Parenthesis, dereferenced)?);
    let bytes = call(widened, "to_be_bytes", Vec::new())?;
    let mut borrowed = vec![GeneratedToken::alone('&')];
    borrowed.extend(bytes);
    appended(borrowed)
}

/// The type the `AsRef` road a framed member is read through hands back.
fn framed_target(shape: CodecMemberShape) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    match shape {
        CodecMemberShape::Text => Ok(vec![GeneratedToken::word("str")]),
        CodecMemberShape::Count
        | CodecMemberShape::Bytes
        | CodecMemberShape::ClosedChoice
        | CodecMemberShape::Nested => byte_slice(),
    }
}

/// Framed bytes, or framed text read as its UTF-8 bytes.
fn write_framed(
    member: &CodecMember,
    subject: Vec<GeneratedToken>,
    shape: CodecMemberShape,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut contract = language_path(&["core", "convert", "AsRef"]);
    contract.extend(generics(framed_target(shape)?));
    let mut expression = qualified(type_path(member.held_as()), contract, "as_ref");
    expression.push(group(GeneratedDelimiter::Parenthesis, subject)?);
    if shape == CodecMemberShape::Text {
        expression = call(expression, "as_bytes", Vec::new())?;
    }
    let mut tokens = bound(MATERIAL_PARAMETER, expression);
    let material = vec![GeneratedToken::word(MATERIAL_PARAMETER)];
    tokens.extend(appended(framed_length(material.clone())?)?);
    tokens.extend(appended(material)?);
    Ok(tokens)
}

/// One arm of a closed roster, as its own declared slot.
fn write_slot(subject: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let slot = call(subject, SLOT_ROAD, Vec::new())?;
    let pushed = call(vec![GeneratedToken::word(INTO_PARAMETER)], "push", slot)?;
    Ok(statement(pushed))
}

/// A nested value, written by its own codec and then framed at its own length.
fn write_nested(subject: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = bound_mutable(NESTED_BINDING, empty_vector()?);
    let sink = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
        GeneratedToken::word(NESTED_BINDING),
    ];
    tokens.extend(statement(call(subject, ENCODE_ROAD, sink)?));
    let nested = vec![GeneratedToken::word(NESTED_BINDING)];
    tokens.extend(appended(framed_length(nested)?)?);
    tokens.extend(appended(vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word(NESTED_BINDING),
    ])?);
    Ok(tokens)
}

/// One member's complete contribution to the encode road, under its declared
/// cardinality.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the contribution
/// outgrows the declared token magnitude.
pub fn encode_member(member: &CodecMember) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    match member.cardinality() {
        FieldCardinality::Required => {
            let subject = borrowed_self_member(member.spelling())?;
            let written = write_member(member, subject)?;
            Ok(vec![group(GeneratedDelimiter::Brace, written)?])
        }
        FieldCardinality::Optional => encode_optional(member),
        FieldCardinality::Repeated => encode_repeated(member),
    }
}

/// An optional member: its presence byte, then its value where there is one.
///
/// The presence byte is `u8::from(…)` over the member's own answer rather than a
/// numeric literal, and the decode road reads it back through the very same road
/// — one spelling, read from both ends.
fn encode_optional(member: &CodecMember) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let asked = call(self_member(member.spelling()), "is_some", Vec::new())?;
    let mut presence = vec![GeneratedToken::word("u8")];
    presence.extend(associated("from"));
    presence.push(group(GeneratedDelimiter::Parenthesis, asked)?);
    let pushed = call(vec![GeneratedToken::word(INTO_PARAMETER)], "push", presence)?;
    let mut tokens = statement(pushed);
    let written = write_member(member, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    tokens.push(GeneratedToken::word("if"));
    tokens.push(GeneratedToken::word("let"));
    tokens.extend(language_path(&["core", "option", "Option", "Some"]));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    tokens.push(GeneratedToken::alone('='));
    tokens.push(GeneratedToken::alone('&'));
    tokens.extend(self_member(member.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, written)?);
    Ok(tokens)
}

/// A repeated member: its framed count, then each occurrence in order.
fn encode_repeated(member: &CodecMember) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let counted = framed_length(self_member(member.spelling()))?;
    let mut tokens = appended(counted)?;
    let written = write_member(member, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    tokens.push(GeneratedToken::word("for"));
    tokens.push(GeneratedToken::word(CARRIED_BINDING));
    tokens.push(GeneratedToken::word("in"));
    tokens.push(GeneratedToken::alone('&'));
    tokens.extend(self_member(member.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, written)?);
    Ok(tokens)
}

/// The encode road: one member at a time, in the order the shape declares them.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the road outgrows
/// the declared token magnitude.
pub fn encode_road(shape: &CodecShape) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut body: Vec<GeneratedToken> = Vec::new();
    for member in shape.members() {
        body.extend(encode_member(member)?);
    }
    let mut parameters = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word("self"),
        GeneratedToken::alone(','),
        GeneratedToken::word(INTO_PARAMETER),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
    ];
    parameters.extend(byte_sink());
    let mut tokens = doc_attribute(ENCODE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(ENCODE_ROAD));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The decode road.
// ---------------------------------------------------------------------------

/// `remaining.get(..<binding>)` — the run one read stands over.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn taken(binding: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "get",
        vec![
            GeneratedToken::joint('.'),
            GeneratedToken::alone('.'),
            GeneratedToken::word(binding),
        ],
    )
}

/// `remaining.get(<binding>..)` — what a read leaves behind it.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn left(binding: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "get",
        vec![
            GeneratedToken::word(binding),
            GeneratedToken::joint('.'),
            GeneratedToken::alone('.'),
        ],
    )
}

/// The framing read: the width, the run it covers, what it leaves, and the
/// length those bytes spell.
///
/// A block EXPRESSION rather than a run of statements, so its own bindings die
/// at its brace and the read that follows binds the same spellings without
/// shadowing anything.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the read outgrows
/// the declared token magnitude.
pub fn framing_read(refusal: &str, member: &str) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut width = framing_width();
    width.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    let mut body = bound(WIDTH_BINDING, width);
    let mut carried = taken(WIDTH_BINDING)?;
    carried.extend(absent(member_refusal(refusal, TRUNCATED_ARM, member)?)?);
    body.extend(bound(CARRIED_BINDING, carried));
    let mut rest = left(WIDTH_BINDING)?;
    rest.extend(absent(member_refusal(refusal, TRUNCATED_ARM, member)?)?);
    body.extend(reassigned(REMAINING_BINDING, rest));
    let mut converted = language_path(&["core", "convert", "TryInto", "try_into"]);
    converted.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    converted.extend(mapped(member_refusal(refusal, TRUNCATED_ARM, member)?)?);
    let mut spelled = vec![GeneratedToken::word("u64")];
    spelled.extend(associated("from_be_bytes"));
    spelled.push(group(GeneratedDelimiter::Parenthesis, converted)?);
    body.extend(spelled);
    group(GeneratedDelimiter::Brace, body)
}

/// The framed prelude every variable-length read shares: the declared length, the
/// addressable width it narrows to, the run it covers, and what it leaves.
fn framed_prelude(refusal: &str, member: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut body = bound(LENGTH_BINDING, vec![framing_read(refusal, member)?]);
    let mut narrowed = vec![GeneratedToken::word("usize")];
    narrowed.extend(associated("try_from"));
    narrowed.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(LENGTH_BINDING)],
    )?);
    narrowed.extend(mapped(member_refusal(
        refusal,
        LENGTH_PAST_WIDTH_ARM,
        member,
    )?)?);
    body.extend(bound(WIDTH_BINDING, narrowed));
    let mut carried = taken(WIDTH_BINDING)?;
    carried.extend(absent(member_refusal(
        refusal,
        LENGTH_PAST_REMAINING_ARM,
        member,
    )?)?);
    body.extend(bound(CARRIED_BINDING, carried));
    let mut rest = left(WIDTH_BINDING)?;
    rest.extend(absent(member_refusal(
        refusal,
        LENGTH_PAST_REMAINING_ARM,
        member,
    )?)?);
    body.extend(reassigned(REMAINING_BINDING, rest));
    Ok(body)
}

/// One occurrence of one member, read back off the material.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the read outgrows
/// the declared token magnitude.
pub fn read_occurrence(
    refusal: &str,
    member: &CodecMember,
) -> Result<GeneratedToken, CodecSurfaceIssue> {
    match member.shape() {
        CodecMemberShape::Count => read_count(refusal, member),
        CodecMemberShape::Bytes => read_bytes(refusal, member),
        CodecMemberShape::Text => read_text(refusal, member),
        CodecMemberShape::ClosedChoice => read_choice(refusal, member),
        CodecMemberShape::Nested => read_nested(refusal, member),
    }
}

/// A count, narrowed back to the width the member is held at.
fn read_count(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut body = bound(
        LENGTH_BINDING,
        vec![framing_read(refusal, member.spelling())?],
    );
    let mut contract = language_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(vec![GeneratedToken::word("u64")]));
    let mut narrowed = qualified(type_path(member.held_as()), contract, "try_from");
    narrowed.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(LENGTH_BINDING)],
    )?);
    narrowed.extend(mapped(member_refusal(
        refusal,
        COUNT_PAST_WIDTH_ARM,
        member.spelling(),
    )?)?);
    body.extend(narrowed);
    group(GeneratedDelimiter::Brace, body)
}

/// Framed bytes, handed to the member's own type.
fn read_bytes(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let owned = call(
        vec![GeneratedToken::word(CARRIED_BINDING)],
        "to_vec",
        Vec::new(),
    )?;
    let mut contract = language_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(byte_sink()));
    let mut built = qualified(type_path(member.held_as()), contract, "try_from");
    built.push(group(GeneratedDelimiter::Parenthesis, owned)?);
    built.extend(mapped(member_refusal(
        refusal,
        MEMBER_NOT_ADMITTED_ARM,
        member.spelling(),
    )?)?);
    body.extend(built);
    group(GeneratedDelimiter::Brace, body)
}

/// Framed text, checked for UTF-8 and handed to the member's own type.
fn read_text(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let mut checked = language_path(&["core", "str", "from_utf8"]);
    checked.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    checked.extend(mapped(member_refusal(
        refusal,
        TEXT_NOT_UTF8_ARM,
        member.spelling(),
    )?)?);
    body.extend(bound(CHOSEN_BINDING, checked));
    let owned = call(
        vec![GeneratedToken::word(CHOSEN_BINDING)],
        "to_owned",
        Vec::new(),
    )?;
    let mut contract = language_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(language_path(&["std", "string", "String"])));
    let mut built = qualified(type_path(member.held_as()), contract, "try_from");
    built.push(group(GeneratedDelimiter::Parenthesis, owned)?);
    built.extend(mapped(member_refusal(
        refusal,
        MEMBER_NOT_ADMITTED_ARM,
        member.spelling(),
    )?)?);
    body.extend(built);
    group(GeneratedDelimiter::Brace, body)
}

/// A framed nested value, read by the nested type's own codec.
fn read_nested(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let mut nested = type_path(member.held_as());
    nested.extend(associated(DECODE_ROAD));
    nested.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    nested.extend(mapped(member_refusal(
        refusal,
        NESTED_REFUSED_ARM,
        member.spelling(),
    )?)?);
    body.extend(nested);
    group(GeneratedDelimiter::Brace, body)
}

/// One arm of a closed roster, elected by walking the roster the OWNER declared.
///
/// This home writes no table of slots. The walk compares the byte it read against
/// each candidate's own `slot()`, so a roster that gained an arm gains it here
/// too — and a slot no arm answers to refuses rather than electing a neighbour.
fn read_choice(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut width = byte_width();
    width.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    let mut body = bound(WIDTH_BINDING, width);
    body.extend(read_one_byte(refusal, member.spelling(), CHOSEN_BINDING)?);
    let mut empty = language_path(&["core", "option", "Option"]);
    empty.extend(associated("None"));
    body.extend(bound_mutable(ELECTED_BINDING, empty));
    let mut some = language_path(&["core", "option", "Option"]);
    some.extend(associated("Some"));
    some.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CANDIDATE_BINDING)],
    )?);
    let elected = reassigned(ELECTED_BINDING, some);
    let mut comparison = call(
        vec![GeneratedToken::word(CANDIDATE_BINDING)],
        SLOT_ROAD,
        Vec::new(),
    )?;
    comparison.push(GeneratedToken::joint('='));
    comparison.push(GeneratedToken::alone('='));
    comparison.push(GeneratedToken::word(CARRIED_BINDING));
    let mut walk = vec![
        GeneratedToken::word("for"),
        GeneratedToken::word(CANDIDATE_BINDING),
        GeneratedToken::word("in"),
    ];
    walk.extend(type_path(member.held_as()));
    walk.extend(associated(ROSTER_CONSTANT));
    let mut test = vec![GeneratedToken::word("if")];
    test.extend(comparison);
    test.push(group(GeneratedDelimiter::Brace, elected)?);
    walk.push(group(GeneratedDelimiter::Brace, test)?);
    body.extend(walk);
    body.push(GeneratedToken::word(ELECTED_BINDING));
    body.extend(absent(member_refusal(
        refusal,
        SLOT_NOT_ADMITTED_ARM,
        member.spelling(),
    )?)?);
    group(GeneratedDelimiter::Brace, body)
}

/// One byte read off the material: borrowed under the named binding, copied into
/// [`CARRIED_BINDING`], and stepped over.
///
/// The step is `width` rather than a literal, and the caller bound `width` to one
/// byte's own size — so the two reads that need a single byte spell the step the
/// same way.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the read outgrows
/// the declared token magnitude.
pub fn read_one_byte(
    refusal: &str,
    member: &str,
    borrowed: &str,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut first = call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "first",
        Vec::new(),
    )?;
    first.extend(absent(member_refusal(refusal, TRUNCATED_ARM, member)?)?);
    let mut body = bound(borrowed, first);
    body.extend(bound(
        CARRIED_BINDING,
        vec![
            GeneratedToken::alone('*'),
            GeneratedToken::word(borrowed),
        ],
    ));
    let mut rest = left(WIDTH_BINDING)?;
    rest.extend(absent(member_refusal(refusal, TRUNCATED_ARM, member)?)?);
    body.extend(reassigned(REMAINING_BINDING, rest));
    Ok(body)
}

/// One member's complete contribution to the decode road, under its declared
/// cardinality.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the contribution
/// outgrows the declared token magnitude.
pub fn decode_member(
    refusal: &str,
    member: &CodecMember,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let read = match member.cardinality() {
        FieldCardinality::Required => read_occurrence(refusal, member)?,
        FieldCardinality::Optional => decode_optional(refusal, member)?,
        FieldCardinality::Repeated => decode_repeated(refusal, member)?,
    };
    Ok(bound(member.spelling(), vec![read]))
}

/// An optional member: the presence byte the encode road wrote, read back through
/// the same road, and the occurrence where there is one.
fn decode_optional(
    refusal: &str,
    member: &CodecMember,
) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut width = byte_width();
    width.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    let mut body = bound(WIDTH_BINDING, width);
    body.extend(read_one_byte(refusal, member.spelling(), CHOSEN_BINDING)?);
    body.extend(bound(
        PRESENT_BINDING,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    ));
    let mut none = language_path(&["core", "option", "Option"]);
    none.extend(associated("None"));
    let mut some = language_path(&["core", "option", "Option"]);
    some.extend(associated("Some"));
    some.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![read_occurrence(refusal, member)?],
    )?);
    let mut refused = vec![GeneratedToken::word("return")];
    refused.extend(language_path(&["core", "result", "Result", "Err"]));
    refused.push(group(
        GeneratedDelimiter::Parenthesis,
        member_refusal(refusal, PRESENCE_NOT_ADMITTED_ARM, member.spelling())?,
    )?);
    body.extend(presence_choice(none, some, statement(refused))?);
    group(GeneratedDelimiter::Brace, body)
}

/// The three-way choice an optional member's presence byte decides.
///
/// The two admitted bytes are `u8::from(false)` and `u8::from(true)` — the exact
/// road the encode surface wrote them by — so neither end carries a numeric
/// literal and neither can drift from the other.
fn presence_choice(
    none: Vec<GeneratedToken>,
    some: Vec<GeneratedToken>,
    refused: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = vec![GeneratedToken::word("if")];
    tokens.extend(presence_test("false")?);
    tokens.push(group(GeneratedDelimiter::Brace, none)?);
    tokens.push(GeneratedToken::word("else"));
    tokens.push(GeneratedToken::word("if"));
    tokens.extend(presence_test("true")?);
    tokens.push(group(GeneratedDelimiter::Brace, some)?);
    tokens.push(GeneratedToken::word("else"));
    tokens.push(group(GeneratedDelimiter::Brace, refused)?);
    Ok(tokens)
}

/// `present == u8::from(<answer>)`.
fn presence_test(answer: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut tokens = vec![
        GeneratedToken::word(PRESENT_BINDING),
        GeneratedToken::joint('='),
        GeneratedToken::alone('='),
        GeneratedToken::word("u8"),
    ];
    tokens.extend(associated("from"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(answer)],
    )?);
    Ok(tokens)
}

/// A repeated member: the framed count, then that many occurrences.
///
/// The loop stops on a length comparison rather than on a counted range, so no
/// numeric literal is written — and a count larger than the material admits runs
/// out of bytes on its next read and refuses there.
fn decode_repeated(
    refusal: &str,
    member: &CodecMember,
) -> Result<GeneratedToken, CodecSurfaceIssue> {
    let mut body = bound(
        LENGTH_BINDING,
        vec![framing_read(refusal, member.spelling())?],
    );
    let mut narrowed = vec![GeneratedToken::word("usize")];
    narrowed.extend(associated("try_from"));
    narrowed.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(LENGTH_BINDING)],
    )?);
    narrowed.extend(mapped(member_refusal(
        refusal,
        COUNT_PAST_WIDTH_ARM,
        member.spelling(),
    )?)?);
    body.extend(bound(WIDTH_BINDING, narrowed));
    body.extend(bound_mutable(COLLECTED_BINDING, empty_vector()?));
    let gathered = call(
        vec![GeneratedToken::word(COLLECTED_BINDING)],
        "push",
        vec![read_occurrence(refusal, member)?],
    )?;
    let mut test = call(
        vec![GeneratedToken::word(COLLECTED_BINDING)],
        "len",
        Vec::new(),
    )?;
    test.push(GeneratedToken::alone('<'));
    test.push(GeneratedToken::word(WIDTH_BINDING));
    let mut walk = vec![GeneratedToken::word("while")];
    walk.extend(test);
    walk.push(group(GeneratedDelimiter::Brace, statement(gathered))?);
    body.extend(walk);
    body.push(GeneratedToken::word(COLLECTED_BINDING));
    group(GeneratedDelimiter::Brace, body)
}

/// The assembly call the decode road ends on, under the posture the caller
/// stated.
fn assembly_call(shape: &CodecShape) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut arguments: Vec<GeneratedToken> = Vec::new();
    for member in shape.members() {
        arguments.push(GeneratedToken::word(member.spelling()));
        arguments.push(GeneratedToken::alone(','));
    }
    let mut tokens = vec![GeneratedToken::word("Self")];
    tokens.extend(associated(shape.assembly().road()));
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    match shape.assembly().posture() {
        AssemblyPosture::Total => {}
        AssemblyPosture::Checked { .. } => tokens.push(GeneratedToken::alone('?')),
    }
    let mut answered = language_path(&["core", "result", "Result", "Ok"]);
    answered.push(group(GeneratedDelimiter::Parenthesis, tokens)?);
    Ok(answered)
}

/// The trailing check: material after the last declared member is itself a
/// refusal, because a canonical encoding is the whole of what a value writes.
fn trailing_check(refusal: &str) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let asked = call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "is_empty",
        Vec::new(),
    )?;
    let mut refused = vec![GeneratedToken::word("return")];
    refused.extend(language_path(&["core", "result", "Result", "Err"]));
    refused.push(group(
        GeneratedDelimiter::Parenthesis,
        sole_refusal(refusal, TRAILING_BYTES_ARM),
    )?);
    let mut tokens = vec![GeneratedToken::word("if"), GeneratedToken::alone('!')];
    tokens.extend(asked);
    tokens.push(group(GeneratedDelimiter::Brace, statement(refused))?);
    Ok(tokens)
}

/// The decode road: one member at a time in declared order, then the trailing
/// check, then the assembly.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the road outgrows
/// the declared token magnitude.
pub fn decode_road(shape: &CodecShape) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let refusal = shape.refusal();
    let mut cursor = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("mut"),
        GeneratedToken::word(REMAINING_BINDING),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
    ];
    cursor.extend(byte_slice()?);
    cursor.push(GeneratedToken::alone('='));
    cursor.push(GeneratedToken::word(MATERIAL_PARAMETER));
    let mut body = statement(cursor);
    for member in shape.members() {
        body.extend(decode_member(refusal, member)?);
    }
    body.extend(trailing_check(refusal)?);
    body.extend(assembly_call(shape)?);
    let mut parameters = vec![
        GeneratedToken::word(MATERIAL_PARAMETER),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
    ];
    parameters.extend(byte_slice()?);
    let mut answer = language_path(&["core", "result", "Result"]);
    answer.extend(generics(vec![
        GeneratedToken::word("Self"),
        GeneratedToken::alone(','),
        GeneratedToken::word(refusal),
    ]));
    let mut tokens = doc_attribute(DECODE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(DECODE_ROAD));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(answer);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The placement, and the whole surface.
// ---------------------------------------------------------------------------

/// One visibly published module carrying a rendered surface.
///
/// Its head writes the one import a wrapped surface needs, because the shape's
/// own names live in the scope the module sits IN.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the module outgrows
/// the declared token magnitude.
pub fn published_module(
    spelling: &str,
    surface: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, CodecSurfaceIssue> {
    let mut body = vec![
        GeneratedToken::word("use"),
        GeneratedToken::word(MODULE_PRELUDE_ROOT),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('*'),
        GeneratedToken::alone(';'),
    ];
    body.extend(surface);
    let mut tokens = doc_attribute(MODULE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(spelling));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The whole codec surface: the refusal the decode road answers with, the
/// conversion a checked assembly earns, and the roads the declared direction
/// covers, under the placement the caller stated.
///
/// The refusal and the conversion are rendered only where the direction covers
/// the decode road, so an encode-only surface declares nothing that cannot
/// happen — and carries no validator, which is what an encode-only direction
/// means.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::SurfaceTreeUnbounded`] where the surface outgrows
/// the declared token magnitude.
pub fn codec_surface(
    shape: &CodecShape,
    placement: &CodecPlacement,
    direction: CodecDirection,
) -> Result<GeneratedTree, CodecSurfaceIssue> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    let reads = covers(direction, CodecRoad::Decode);
    if reads {
        tokens.extend(refusal_declaration(shape)?);
        tokens.extend(refusal_conversion(shape)?);
    }
    let mut roads: Vec<GeneratedToken> = Vec::new();
    if covers(direction, CodecRoad::Encode) {
        roads.extend(encode_road(shape)?);
    }
    if reads {
        roads.extend(decode_road(shape)?);
    }
    tokens.push(GeneratedToken::word("impl"));
    tokens.extend(type_path(shape.owner()));
    tokens.push(group(GeneratedDelimiter::Brace, roads)?);
    let placed = match placement {
        CodecPlacement::AtDeclarationSite => tokens,
        CodecPlacement::PublishedModule { spelling } => {
            published_module(spelling.spelling(), tokens)?
        }
    };
    GeneratedTree::assembled(placed).map_err(|_| unbounded())
}

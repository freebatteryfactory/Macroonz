//! The token half: the decode refusal this home declares for the caller, the encode road, the decode road, and the placement that carries them.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments and every brace is a group; nothing here composes Rust source.
//! The Rust a person reads is [`GeneratedTree::inspected`](crate::token::GeneratedTree::inspected), a projection of what is emitted rather than the thing itself.
//!
//! # The framing, stated once
//!
//! Every variable-length member is written as the framing width's worth of big-endian length followed by the bytes, so two members can never be re-cut at another boundary and produce one byte string.
//! A nested member is framed on exactly those terms rather than run to the end of the input, because a nested value that consumed the remainder would make the member after it unreadable.
//!
//! # No numeric literal is written anywhere here
//!
//! The generated-token roster carries a numeric arm and this home writes nothing through it.
//! The framing width is `::core::mem::size_of::<u64>()`, a presence byte is `u8::from(false)` and `u8::from(true)`, a repeated member stops on a length comparison, and a closed choice's admitted slots are the owner's own roster walked and compared.
//!
//! That last one is the road worth reading twice: this home never writes a table of slots.
//! A roster that gained an arm gains it in the decode road too, without this home ever learning what the arms are.

use super::{
    AssemblyPosture, Cardinality, CodecContent, CodecMember, CodecMemberShape, CodecPlacement,
    CodecProjection, CodecShape, CodecTypePath, DECODE_ROAD, DecodeRefusal, ENCODE_ROAD,
    PathRooting, ROSTER_CONSTANT, SLOT_ROAD,
};
use crate::bounded::Overflow;
use crate::kind::SoleRole;
use crate::plan::Plan;
use crate::render::{Output, RenderError};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, absolute_path, attribute, bound_local,
    bound_path, call, documentation, equality, group, method_call, result_type,
};

/// The decode road's one parameter: the material read.
pub(super) const MATERIAL_BINDING: &str = "material";

/// The decode road's running cursor over what it has not yet read.
pub(super) const REMAINING_BINDING: &str = "remaining";

/// The encode road's one parameter: the sink the bytes are appended to.
pub(super) const INTO_BINDING: &str = "into";

/// The binding a nested member's own encoding stands under before it is framed.
pub(super) const NESTED_BINDING: &str = "nested";

/// The binding a repeated member's occurrences are gathered into.
pub(super) const COLLECTED_BINDING: &str = "collected";

/// The binding one arm of a closed roster stands under while the walk compares it.
pub(super) const CANDIDATE_BINDING: &str = "candidate";

/// The binding a borrowed single byte — a slot, a presence — stands under.
pub(super) const CHOSEN_BINDING: &str = "chosen";

/// The binding the elected arm of a closed roster stands under.
pub(super) const ELECTED_BINDING: &str = "elected";

/// The binding an optional member's presence byte stands under.
pub(super) const PRESENT_BINDING: &str = "present";

/// The binding one member's own value stands under while it is written or read.
pub(super) const CARRIED_BINDING: &str = "carried";

/// The binding a framed member's declared length stands under.
pub(super) const LENGTH_BINDING: &str = "length";

/// The binding a framed member's length stands under once it is an addressable width.
pub(super) const WIDTH_BINDING: &str = "width";

/// The seat every member-bearing refusal arm names its member through.
const MEMBER_SEAT: &str = "member";

/// The one import a published module's head writes.
///
/// A wrapped surface names the owner's type and every member's type in the scope the module sits IN rather than in its own, so the head brings that scope with it.
/// One import and no more: a module that reached further would be deciding what else a caller's generated module can see.
const MODULE_PRELUDE_ROOT: &str = "super";

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

/// The sentence a published module documents itself with.
const MODULE_SENTENCE: &str = "The canonical encode and decode roads for one declared shape, \
     published here rather than spliced beside the declaration. Its head imports the scope the \
     module sits in, which is where the shape's own names live.";

/// Render the one unit a codec request produces.
///
/// Naming the seat is the whole call: everything else the unit answers to is that seat's planned member, read by the sink.
///
/// # Errors
///
/// Returns [`RenderError::SeatUnplanned`] where the plan declares no member under the kind's one seat, [`RenderError::BytesUnbounded`] where the surface passes the rendered-byte magnitude, and [`RenderError::TokensUnbounded`] where a level of it passes the per-level one.
pub fn render_codec(
    plan: &Plan<CodecProjection>,
    out: &mut Output<'_, CodecProjection>,
) -> Result<(), RenderError> {
    let tree = codec_surface(plan.content())?;
    out.unit(SoleRole::Sole, tree)
}

/// The whole surface: the refusal the decode road answers with, the conversion a checked assembly earns, the roads the direction covers, and the placement carrying them.
///
/// The refusal and the conversion are rendered only where the direction covers the decode road, so an encode-only surface declares nothing that cannot happen — and carries no reader, which is what an encode-only direction means.
///
/// # Errors
///
/// Returns [`Overflow`] where a level of the surface passes the declared per-level token magnitude.
pub fn codec_surface(content: &CodecContent) -> Result<GeneratedTree, Overflow> {
    let shape = &content.shape;
    let reads = content.direction.reads();
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    if reads {
        tokens.extend(refusal_declaration(shape)?);
        tokens.extend(refusal_conversion(shape)?);
    }
    let mut inherent: Vec<GeneratedToken> = Vec::new();
    if content.direction.writes() {
        inherent.extend(encode_road(shape)?);
    }
    if reads {
        inherent.extend(decode_road(shape)?);
    }
    tokens.push(GeneratedToken::word("impl"));
    tokens.extend(type_path(shape.owner()));
    tokens.push(group(GeneratedDelimiter::Brace, inherent)?);
    let placed = match &content.placement {
        CodecPlacement::AtDeclarationSite => tokens,
        CodecPlacement::PublishedModule { spelling } => {
            published_module(spelling.spelling(), tokens)?
        }
    };
    GeneratedTree::assembled(placed)
}

// ---------------------------------------------------------------------------
// The token primitives.
// ---------------------------------------------------------------------------

/// `::spelling` — the road from a path to an associated item on it.
fn associated(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(spelling),
    ]
}

/// The tokens a caller spelled, closed with a semicolon.
fn statement(mut tokens: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One generic argument list.
fn generics(arguments: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('<')];
    tokens.extend(arguments);
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// One qualified road, `<Subject as Contract>::road`.
///
/// Qualified rather than plain, so the call names the exact trait the member contract bills for and never resolves onto an inherent road that happened to share a spelling.
fn qualified(
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

/// One path a caller declared, spelled from the rooting it stated.
///
/// The crate-absolute rooting writes the language's own `crate` qualifier, because that rooting's claim is the caller's own crate — a leading `::` would address the extern prelude, which is a different crate entirely.
fn type_path(path: &CodecTypePath) -> Vec<GeneratedToken> {
    let segments: Vec<&str> = path.segments().collect();
    match path.rooting() {
        PathRooting::CrateAbsolute => bound_path("crate", &segments),
        PathRooting::SelfScoped => bound_path("self", &segments),
        PathRooting::ParentScoped => bound_path("super", &segments),
        PathRooting::InScope => match segments.split_first() {
            Some((root, rest)) => bound_path(root, rest),
            None => Vec::new(),
        },
    }
}

/// One `let mut name = expression;` statement.
fn bound_mutable(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("mut"),
        GeneratedToken::word(name),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    statement(tokens)
}

/// One `name = expression;` reassignment.
fn reassigned(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(name), GeneratedToken::alone('=')];
    tokens.extend(expression);
    statement(tokens)
}

/// The framing width, as the language's own road to it rather than as a number.
fn framing_width() -> Vec<GeneratedToken> {
    sized_width("u64")
}

/// One byte's width, on the same terms.
fn byte_width() -> Vec<GeneratedToken> {
    sized_width("u8")
}

/// `::core::mem::size_of::<name>` — the width of one named type.
fn sized_width(name: &str) -> Vec<GeneratedToken> {
    let mut tokens = absolute_path(&["core", "mem", "size_of"]);
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.extend(generics(vec![GeneratedToken::word(name)]));
    tokens
}

/// `::std::vec::Vec<u8>` — the sink the encode road appends to.
fn byte_sink() -> Vec<GeneratedToken> {
    let mut tokens = absolute_path(&["std", "vec", "Vec"]);
    tokens.extend(generics(vec![GeneratedToken::word("u8")]));
    tokens
}

/// `[u8]` — the slice a decode road reads.
fn byte_slice() -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![group(
        GeneratedDelimiter::Bracket,
        vec![GeneratedToken::word("u8")],
    )?])
}

/// `::std::vec::Vec::new()` — one empty gathering.
fn empty_vector() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = absolute_path(&["std", "vec", "Vec"]);
    tokens.extend(associated("new"));
    call(tokens, Vec::new())
}

/// `&u64::try_from(material.len()).unwrap_or(u64::MAX).to_be_bytes()` — one framed length, written without a numeric literal.
fn framed_length(material: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let counted = method_call(material, "len", Vec::new())?;
    let mut narrowing = vec![GeneratedToken::word("u64")];
    narrowing.extend(associated("try_from"));
    let narrowed = call(narrowing, counted)?;
    let mut ceiling = vec![GeneratedToken::word("u64")];
    ceiling.extend(associated("MAX"));
    let held = method_call(narrowed, "unwrap_or", ceiling)?;
    let bytes = method_call(held, "to_be_bytes", Vec::new())?;
    let mut tokens = vec![GeneratedToken::alone('&')];
    tokens.extend(bytes);
    Ok(tokens)
}

/// `into.extend_from_slice(material);` — one appended run.
fn appended(material: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let called = method_call(
        vec![GeneratedToken::word(INTO_BINDING)],
        "extend_from_slice",
        material,
    )?;
    Ok(statement(called))
}

/// `.map_err(|_| refusal)?` — the road a fallible step takes to this surface's own refusal.
fn mapped(refusal: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut closure = vec![
        GeneratedToken::alone('|'),
        GeneratedToken::word("_"),
        GeneratedToken::alone('|'),
    ];
    closure.extend(refusal);
    let mut tokens = call(
        vec![GeneratedToken::alone('.'), GeneratedToken::word("map_err")],
        closure,
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// `.ok_or(refusal)?` — the road an absent read takes to the same place.
fn absent(refusal: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = call(
        vec![GeneratedToken::alone('.'), GeneratedToken::word("ok_or")],
        refusal,
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// `Refusal::Arm` — one payload-free refusal construction.
fn sole_refusal(refusal: &str, arm: DecodeRefusal) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(refusal)];
    tokens.extend(associated(arm.name()));
    tokens
}

/// `Refusal::Arm { member: "spelling" }` — one member-bearing refusal construction.
///
/// The spelling is a text literal, so a refusal this home renders always names the member the read was standing at.
fn member_refusal(
    refusal: &str,
    arm: DecodeRefusal,
    member: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = sole_refusal(refusal, arm);
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

/// One member read off `self`.
fn self_member(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(spelling),
    ]
}

/// One member read off `self`, borrowed and parenthesized so a wire road always stands over a reference whatever the cardinality supplied it.
fn borrowed_self_member(spelling: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut inner = vec![GeneratedToken::alone('&')];
    inner.extend(self_member(spelling));
    Ok(vec![group(GeneratedDelimiter::Parenthesis, inner)?])
}

// ---------------------------------------------------------------------------
// The rendered decode refusal.
// ---------------------------------------------------------------------------

/// `#[derive(Debug, Clone, PartialEq, Eq)]`.
///
/// Four and no more: a refusal is shown in a report, cloned into one, and compared against an expectation, and nothing about a decode refusal needs ordering or hashing.
/// `Copy` is absent because the assembly arm may carry a refusal the owner declared, and this home does not decide whether that one copies.
fn derive_attribute() -> Result<Vec<GeneratedToken>, Overflow> {
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

/// One variant of the rendered refusal: its sentence, its spelling, and the payload it carries.
fn variant(
    arm: DecodeRefusal,
    payload: Option<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = documentation(arm.sentence())?;
    tokens.push(GeneratedToken::word(arm.name()));
    if let Some(seat) = payload {
        tokens.push(seat);
    }
    tokens.push(GeneratedToken::alone(','));
    Ok(tokens)
}

/// `{ member: &'static str, }` — the seat a member-bearing arm names its member through.
fn member_seat() -> Result<GeneratedToken, Overflow> {
    group(
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
    )
}

/// The decode refusal one shape's surface declares.
///
/// Every member-bearing arm, then the whole-material arm, then the assembly arm a CHECKED assembly earns — and only that posture earns it, so a total assembly renders a refusal with nothing on it that cannot happen.
fn refusal_declaration(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut variants: Vec<GeneratedToken> = Vec::new();
    for arm in DecodeRefusal::ALL.iter().copied() {
        if arm.carries_member() {
            variants.extend(variant(arm, Some(member_seat()?))?);
        }
    }
    variants.extend(variant(DecodeRefusal::TrailingBytes, None)?);
    if let AssemblyPosture::Checked { refusal } = shape.assembly().posture() {
        let carried = group(GeneratedDelimiter::Parenthesis, type_path(refusal))?;
        variants.extend(variant(DecodeRefusal::NotAssembled, Some(carried))?);
    }
    let mut tokens = documentation(REFUSAL_SENTENCE)?;
    tokens.extend(derive_attribute()?);
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("enum"));
    tokens.push(GeneratedToken::word(shape.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, variants)?);
    Ok(tokens)
}

/// The conversion a CHECKED assembly earns: the owner's own refusal into this surface's.
///
/// Rendered rather than billed, so a checked assembly costs the address nothing.
fn refusal_conversion(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let AssemblyPosture::Checked { refusal } = shape.assembly().posture() else {
        return Ok(Vec::new());
    };
    let carried = type_path(refusal);
    let mut body = vec![GeneratedToken::word("Self")];
    body.extend(associated(DecodeRefusal::NotAssembled.name()));
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
    tokens.extend(absolute_path(&["core", "convert", "From"]));
    tokens.extend(generics(carried));
    tokens.push(GeneratedToken::word("for"));
    tokens.push(GeneratedToken::word(shape.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, road)?);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The encode road.
// ---------------------------------------------------------------------------

/// One member's write, over the subject its cardinality handed it.
///
/// The subject always stands for a REFERENCE to one occurrence, so the five wire roads never learn how many of the member there were.
fn write_member(
    member: &CodecMember,
    subject: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    match member.shape() {
        CodecMemberShape::Count => write_count(subject),
        CodecMemberShape::Bytes => write_framed(member, subject, CodecMemberShape::Bytes),
        CodecMemberShape::Text => write_framed(member, subject, CodecMemberShape::Text),
        CodecMemberShape::ClosedChoice => write_slot(subject),
        CodecMemberShape::Nested => write_nested(subject),
    }
}

/// A count, at the framing width.
fn write_count(subject: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut dereferenced = vec![GeneratedToken::alone('*')];
    dereferenced.extend(subject);
    let mut widening = vec![GeneratedToken::word("u64")];
    widening.extend(associated("from"));
    let widened = call(widening, dereferenced)?;
    let bytes = method_call(widened, "to_be_bytes", Vec::new())?;
    let mut borrowed = vec![GeneratedToken::alone('&')];
    borrowed.extend(bytes);
    appended(borrowed)
}

/// The type the `AsRef` road a framed member is read through hands back.
fn framed_target(shape: CodecMemberShape) -> Result<Vec<GeneratedToken>, Overflow> {
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
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut contract = absolute_path(&["core", "convert", "AsRef"]);
    contract.extend(generics(framed_target(shape)?));
    let road = qualified(type_path(member.held_as()), contract, "as_ref");
    let mut expression = call(road, subject)?;
    if shape == CodecMemberShape::Text {
        expression = method_call(expression, "as_bytes", Vec::new())?;
    }
    let mut tokens = bound_local(MATERIAL_BINDING, expression);
    let material = vec![GeneratedToken::word(MATERIAL_BINDING)];
    tokens.extend(appended(framed_length(material.clone())?)?);
    tokens.extend(appended(material)?);
    Ok(tokens)
}

/// One arm of a closed roster, as its own declared position.
fn write_slot(subject: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let slot = method_call(subject, SLOT_ROAD, Vec::new())?;
    let pushed = method_call(vec![GeneratedToken::word(INTO_BINDING)], "push", slot)?;
    Ok(statement(pushed))
}

/// A nested value, written by its own codec and then framed at its own length.
fn write_nested(subject: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = bound_mutable(NESTED_BINDING, empty_vector()?);
    let sink = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
        GeneratedToken::word(NESTED_BINDING),
    ];
    tokens.extend(statement(method_call(subject, ENCODE_ROAD, sink)?));
    let nested = vec![GeneratedToken::word(NESTED_BINDING)];
    tokens.extend(appended(framed_length(nested)?)?);
    tokens.extend(appended(vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word(NESTED_BINDING),
    ])?);
    Ok(tokens)
}

/// One member's complete contribution to the encode road, under its declared cardinality.
fn encode_member(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    match member.cardinality() {
        Cardinality::Required => {
            let subject = borrowed_self_member(member.spelling())?;
            let written = write_member(member, subject)?;
            Ok(vec![group(GeneratedDelimiter::Brace, written)?])
        }
        Cardinality::Optional => encode_optional(member),
        Cardinality::Repeated => encode_repeated(member),
    }
}

/// An optional member: its presence byte, then its value where there is one.
///
/// The presence byte is `u8::from(…)` over the member's own answer rather than a numeric literal, and the decode road reads it back through the very same road.
fn encode_optional(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    let asked = method_call(self_member(member.spelling()), "is_some", Vec::new())?;
    let mut presence = vec![GeneratedToken::word("u8")];
    presence.extend(associated("from"));
    let presence = call(presence, asked)?;
    let pushed = method_call(vec![GeneratedToken::word(INTO_BINDING)], "push", presence)?;
    let mut tokens = statement(pushed);
    let written = write_member(member, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    tokens.push(GeneratedToken::word("if"));
    tokens.push(GeneratedToken::word("let"));
    tokens.extend(absolute_path(&["core", "option", "Option", "Some"]));
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
fn encode_repeated(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
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
fn encode_road(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body: Vec<GeneratedToken> = Vec::new();
    for member in shape.members() {
        body.extend(encode_member(member)?);
    }
    let mut parameters = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word("self"),
        GeneratedToken::alone(','),
        GeneratedToken::word(INTO_BINDING),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
    ];
    parameters.extend(byte_sink());
    let mut tokens = documentation(ENCODE_SENTENCE)?;
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

/// `remaining.get(..binding)` — the run one read stands over.
fn taken(binding: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    method_call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "get",
        vec![
            GeneratedToken::joint('.'),
            GeneratedToken::alone('.'),
            GeneratedToken::word(binding),
        ],
    )
}

/// `remaining.get(binding..)` — what a read leaves behind it.
fn left(binding: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    method_call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "get",
        vec![
            GeneratedToken::word(binding),
            GeneratedToken::joint('.'),
            GeneratedToken::alone('.'),
        ],
    )
}

/// The framing read: the width, the run it covers, what it leaves, and the length those bytes spell.
///
/// A block EXPRESSION rather than a run of statements, so its own bindings die at its brace and the read that follows binds the same spellings without shadowing anything.
fn framing_read(refusal: &str, member: &str) -> Result<GeneratedToken, Overflow> {
    let width = call(framing_width(), Vec::new())?;
    let mut body = bound_local(WIDTH_BINDING, width);
    let mut carried = taken(WIDTH_BINDING)?;
    carried.extend(absent(member_refusal(
        refusal,
        DecodeRefusal::Truncated,
        member,
    )?)?);
    body.extend(bound_local(CARRIED_BINDING, carried));
    body.extend(stepped_over(refusal, member, DecodeRefusal::Truncated)?);
    let widening = absolute_path(&["core", "convert", "TryInto", "try_into"]);
    let mut widened = call(widening, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    widened.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::Truncated,
        member,
    )?)?);
    let mut reading = vec![GeneratedToken::word("u64")];
    reading.extend(associated("from_be_bytes"));
    body.extend(call(reading, widened)?);
    group(GeneratedDelimiter::Brace, body)
}

/// `remaining = remaining.get(width..).ok_or(…)?;` — the step past what was just read.
fn stepped_over(
    refusal: &str,
    member: &str,
    arm: DecodeRefusal,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut rest = left(WIDTH_BINDING)?;
    rest.extend(absent(member_refusal(refusal, arm, member)?)?);
    Ok(reassigned(REMAINING_BINDING, rest))
}

/// The prelude every variable-length read shares: the declared length, the addressable width it narrows to, the run it covers, and what it leaves.
fn framed_prelude(refusal: &str, member: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = bound_local(LENGTH_BINDING, vec![framing_read(refusal, member)?]);
    let mut narrowing = vec![GeneratedToken::word("usize")];
    narrowing.extend(associated("try_from"));
    let mut narrowed = call(narrowing, vec![GeneratedToken::word(LENGTH_BINDING)])?;
    narrowed.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::LengthPastAddressableWidth,
        member,
    )?)?);
    body.extend(bound_local(WIDTH_BINDING, narrowed));
    let mut carried = taken(WIDTH_BINDING)?;
    carried.extend(absent(member_refusal(
        refusal,
        DecodeRefusal::LengthPastRemaining,
        member,
    )?)?);
    body.extend(bound_local(CARRIED_BINDING, carried));
    body.extend(stepped_over(
        refusal,
        member,
        DecodeRefusal::LengthPastRemaining,
    )?);
    Ok(body)
}

/// One occurrence of one member, read back off the material.
fn read_occurrence(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    match member.shape() {
        CodecMemberShape::Count => read_count(refusal, member),
        CodecMemberShape::Bytes => read_bytes(refusal, member),
        CodecMemberShape::Text => read_text(refusal, member),
        CodecMemberShape::ClosedChoice => read_choice(refusal, member),
        CodecMemberShape::Nested => read_nested(refusal, member),
    }
}

/// A count, narrowed back to the width the member is held at.
fn read_count(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let mut body = bound_local(
        LENGTH_BINDING,
        vec![framing_read(refusal, member.spelling())?],
    );
    let mut contract = absolute_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(vec![GeneratedToken::word("u64")]));
    let road = qualified(type_path(member.held_as()), contract, "try_from");
    let mut narrowed = call(road, vec![GeneratedToken::word(LENGTH_BINDING)])?;
    narrowed.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::CountPastDeclaredWidth,
        member.spelling(),
    )?)?);
    body.extend(narrowed);
    group(GeneratedDelimiter::Brace, body)
}

/// Framed bytes, handed to the member's own type.
fn read_bytes(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let owned = method_call(
        vec![GeneratedToken::word(CARRIED_BINDING)],
        "to_vec",
        Vec::new(),
    )?;
    let mut contract = absolute_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(byte_sink()));
    body.extend(admitted(refusal, member, contract, owned)?);
    group(GeneratedDelimiter::Brace, body)
}

/// Framed text, checked for UTF-8 and handed to the member's own type.
fn read_text(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let checking = absolute_path(&["core", "str", "from_utf8"]);
    let mut checked = call(checking, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    checked.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::TextNotUtf8,
        member.spelling(),
    )?)?);
    body.extend(bound_local(CHOSEN_BINDING, checked));
    let owned = method_call(
        vec![GeneratedToken::word(CHOSEN_BINDING)],
        "to_owned",
        Vec::new(),
    )?;
    let mut contract = absolute_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(absolute_path(&["std", "string", "String"])));
    body.extend(admitted(refusal, member, contract, owned)?);
    group(GeneratedDelimiter::Brace, body)
}

/// The member's own type asked to admit what was read for it.
fn admitted(
    refusal: &str,
    member: &CodecMember,
    contract: Vec<GeneratedToken>,
    material: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let road = qualified(type_path(member.held_as()), contract, "try_from");
    let mut built = call(road, material)?;
    built.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::MemberNotAdmitted,
        member.spelling(),
    )?)?);
    Ok(built)
}

/// A framed nested value, read by the nested type's own codec.
fn read_nested(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let mut road = type_path(member.held_as());
    road.extend(associated(DECODE_ROAD));
    let mut nested = call(road, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    nested.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::NestedMemberRefused,
        member.spelling(),
    )?)?);
    body.extend(nested);
    group(GeneratedDelimiter::Brace, body)
}

/// One byte read off the material: borrowed under the named binding, copied into the carried one, and stepped over.
///
/// The step is `width`, and the caller bound `width` to one byte's own size, so the two reads that need a single byte spell the step the same way.
fn read_one_byte(
    refusal: &str,
    member: &str,
    borrowed: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut first = method_call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "first",
        Vec::new(),
    )?;
    first.extend(absent(member_refusal(
        refusal,
        DecodeRefusal::Truncated,
        member,
    )?)?);
    let mut body = bound_local(borrowed, first);
    body.extend(bound_local(
        CARRIED_BINDING,
        vec![GeneratedToken::alone('*'), GeneratedToken::word(borrowed)],
    ));
    body.extend(stepped_over(refusal, member, DecodeRefusal::Truncated)?);
    Ok(body)
}

/// One arm of a closed roster, elected by walking the roster the OWNER declared.
///
/// This home writes no table of slots: the walk compares the byte it read against each candidate's own position, so a roster that gained an arm gains it here too, and a slot no arm answers to refuses rather than electing a neighbour.
fn read_choice(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let width = call(byte_width(), Vec::new())?;
    let mut body = bound_local(WIDTH_BINDING, width);
    body.extend(read_one_byte(refusal, member.spelling(), CHOSEN_BINDING)?);
    let mut empty = absolute_path(&["core", "option", "Option"]);
    empty.extend(associated("None"));
    body.extend(bound_mutable(ELECTED_BINDING, empty));
    body.extend(roster_walk(member)?);
    body.push(GeneratedToken::word(ELECTED_BINDING));
    body.extend(absent(member_refusal(
        refusal,
        DecodeRefusal::SlotNotAdmitted,
        member.spelling(),
    )?)?);
    group(GeneratedDelimiter::Brace, body)
}

/// The walk over the owner's own roster, electing the arm whose position is the byte that was read.
fn roster_walk(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut some = absolute_path(&["core", "option", "Option"]);
    some.extend(associated("Some"));
    let some = call(some, vec![GeneratedToken::word(CANDIDATE_BINDING)])?;
    let elected = reassigned(ELECTED_BINDING, some);
    let position = method_call(
        vec![GeneratedToken::word(CANDIDATE_BINDING)],
        SLOT_ROAD,
        Vec::new(),
    )?;
    let compared = equality(position, vec![GeneratedToken::word(CARRIED_BINDING)]);
    let mut test = vec![GeneratedToken::word("if")];
    test.extend(compared);
    test.push(group(GeneratedDelimiter::Brace, elected)?);
    let mut walk = vec![
        GeneratedToken::word("for"),
        GeneratedToken::word(CANDIDATE_BINDING),
        GeneratedToken::word("in"),
    ];
    walk.extend(type_path(member.held_as()));
    walk.extend(associated(ROSTER_CONSTANT));
    walk.push(group(GeneratedDelimiter::Brace, test)?);
    Ok(walk)
}

/// One member's complete contribution to the decode road, under its declared cardinality.
fn decode_member(refusal: &str, member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    let read = match member.cardinality() {
        Cardinality::Required => read_occurrence(refusal, member)?,
        Cardinality::Optional => decode_optional(refusal, member)?,
        Cardinality::Repeated => decode_repeated(refusal, member)?,
    };
    Ok(bound_local(member.spelling(), vec![read]))
}

/// An optional member: the presence byte the encode road wrote, read back through the same road, and the occurrence where there is one.
fn decode_optional(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let width = call(byte_width(), Vec::new())?;
    let mut body = bound_local(WIDTH_BINDING, width);
    body.extend(read_one_byte(refusal, member.spelling(), CHOSEN_BINDING)?);
    body.extend(bound_local(
        PRESENT_BINDING,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    ));
    let mut none = absolute_path(&["core", "option", "Option"]);
    none.extend(associated("None"));
    let mut some = absolute_path(&["core", "option", "Option"]);
    some.extend(associated("Some"));
    let some = call(some, vec![read_occurrence(refusal, member)?])?;
    let mut refused = vec![GeneratedToken::word("return")];
    let error = absolute_path(&["core", "result", "Result", "Err"]);
    refused.extend(call(
        error,
        member_refusal(
            refusal,
            DecodeRefusal::PresenceNotAdmitted,
            member.spelling(),
        )?,
    )?);
    body.extend(presence_choice(none, some, statement(refused))?);
    group(GeneratedDelimiter::Brace, body)
}

/// The three-way choice an optional member's presence byte decides.
///
/// The two admitted bytes are `u8::from(false)` and `u8::from(true)` — the exact road the encode surface wrote them by — so neither end carries a numeric literal and neither can drift from the other.
fn presence_choice(
    none: Vec<GeneratedToken>,
    some: Vec<GeneratedToken>,
    refused: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
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

/// `present == u8::from(answer)`.
fn presence_test(answer: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut road = vec![GeneratedToken::word("u8")];
    road.extend(associated("from"));
    let written = call(road, vec![GeneratedToken::word(answer)])?;
    Ok(equality(
        vec![GeneratedToken::word(PRESENT_BINDING)],
        written,
    ))
}

/// A repeated member: the framed count, then that many occurrences.
///
/// The loop stops on a length comparison rather than on a counted range, so no numeric literal is written — and a count larger than the material admits runs out of bytes on its next read and refuses there.
fn decode_repeated(refusal: &str, member: &CodecMember) -> Result<GeneratedToken, Overflow> {
    let mut body = bound_local(
        LENGTH_BINDING,
        vec![framing_read(refusal, member.spelling())?],
    );
    let mut narrowing = vec![GeneratedToken::word("usize")];
    narrowing.extend(associated("try_from"));
    let mut narrowed = call(narrowing, vec![GeneratedToken::word(LENGTH_BINDING)])?;
    narrowed.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::CountPastDeclaredWidth,
        member.spelling(),
    )?)?);
    body.extend(bound_local(WIDTH_BINDING, narrowed));
    body.extend(bound_mutable(COLLECTED_BINDING, empty_vector()?));
    let gathered = method_call(
        vec![GeneratedToken::word(COLLECTED_BINDING)],
        "push",
        vec![read_occurrence(refusal, member)?],
    )?;
    let mut test = method_call(
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

/// The assembly call the decode road ends on, under the posture the caller stated.
fn assembly_call(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut arguments: Vec<GeneratedToken> = Vec::new();
    for member in shape.members() {
        arguments.push(GeneratedToken::word(member.spelling()));
        arguments.push(GeneratedToken::alone(','));
    }
    let mut road = vec![GeneratedToken::word("Self")];
    road.extend(associated(shape.assembly().road()));
    let mut assembled = call(road, arguments)?;
    match shape.assembly().posture() {
        AssemblyPosture::Total => {}
        AssemblyPosture::Checked { .. } => assembled.push(GeneratedToken::alone('?')),
    }
    call(
        absolute_path(&["core", "result", "Result", "Ok"]),
        assembled,
    )
}

/// The trailing check: material after the last declared member is itself a refusal, because a canonical encoding is the whole of what a value writes.
fn trailing_check(refusal: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let asked = method_call(
        vec![GeneratedToken::word(REMAINING_BINDING)],
        "is_empty",
        Vec::new(),
    )?;
    let mut refused = vec![GeneratedToken::word("return")];
    let error = absolute_path(&["core", "result", "Result", "Err"]);
    refused.extend(call(
        error,
        sole_refusal(refusal, DecodeRefusal::TrailingBytes),
    )?);
    let mut tokens = vec![GeneratedToken::word("if"), GeneratedToken::alone('!')];
    tokens.extend(asked);
    tokens.push(group(GeneratedDelimiter::Brace, statement(refused))?);
    Ok(tokens)
}

/// The decode road: one member at a time in declared order, then the trailing check, then the assembly.
fn decode_road(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let refusal = shape.refusal();
    let mut body = statement(cursor()?);
    for member in shape.members() {
        body.extend(decode_member(refusal, member)?);
    }
    body.extend(trailing_check(refusal)?);
    body.extend(assembly_call(shape)?);
    let mut parameters = vec![
        GeneratedToken::word(MATERIAL_BINDING),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
    ];
    parameters.extend(byte_slice()?);
    let answer = result_type(
        vec![GeneratedToken::word("Self")],
        vec![GeneratedToken::word(refusal)],
    );
    let mut tokens = documentation(DECODE_SENTENCE)?;
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

/// `let mut remaining: &[u8] = material` — the cursor every read moves.
fn cursor() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("mut"),
        GeneratedToken::word(REMAINING_BINDING),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
    ];
    tokens.extend(byte_slice()?);
    tokens.push(GeneratedToken::alone('='));
    tokens.push(GeneratedToken::word(MATERIAL_BINDING));
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The placement.
// ---------------------------------------------------------------------------

/// One visibly published module carrying a rendered surface.
///
/// Its head writes the one import a wrapped surface needs, because the shape's own names live in the scope the module sits IN.
fn published_module(
    spelling: &str,
    surface: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = vec![
        GeneratedToken::word("use"),
        GeneratedToken::word(MODULE_PRELUDE_ROOT),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('*'),
        GeneratedToken::alone(';'),
    ];
    body.extend(surface);
    let mut tokens = documentation(MODULE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(spelling));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

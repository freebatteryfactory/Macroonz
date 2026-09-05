//! The Rust-token spellings shared by the codec home's declaration, write, read, and placement operations.

use super::{CodecTypePath, DecodeRefusal, PathRooting};
use crate::bounded::Overflow;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, absolute_path, bound_path, call, group, method_call, vector,
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
pub(super) const MEMBER_SEAT: &str = "member";

/// `::spelling` — the road from a path to an associated item on it.
pub(super) fn associated(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(spelling),
    ]
}

/// The tokens a caller spelled, closed with a semicolon.
pub(super) fn statement(mut tokens: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One generic argument list.
pub(super) fn generics(arguments: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('<')];
    tokens.extend(arguments);
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// One qualified road, `<Subject as Contract>::road`.
///
/// Qualified rather than plain, so the call names the exact trait the member contract bills for and never resolves onto an inherent road that happened to share a spelling.
pub(super) fn qualified(
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
pub(super) fn type_path(path: &CodecTypePath) -> Vec<GeneratedToken> {
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
pub(super) fn bound_mutable(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
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
pub(super) fn reassigned(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(name), GeneratedToken::alone('=')];
    tokens.extend(expression);
    statement(tokens)
}

/// The framing width, as the language's own road to it rather than as a number.
pub(super) fn framing_width() -> Vec<GeneratedToken> {
    sized_width("u64")
}

/// One byte's width, on the same terms.
pub(super) fn byte_width() -> Vec<GeneratedToken> {
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
pub(super) fn byte_sink() -> Vec<GeneratedToken> {
    let mut tokens = absolute_path(&["std", "vec", "Vec"]);
    tokens.extend(generics(vec![GeneratedToken::word("u8")]));
    tokens
}

/// `[u8]` — the slice a decode road reads.
pub(super) fn byte_slice() -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![group(
        GeneratedDelimiter::Bracket,
        vec![GeneratedToken::word("u8")],
    )?])
}

/// `::std::vec::Vec::new()` — one empty gathering.
pub(super) fn empty_vector() -> Result<Vec<GeneratedToken>, Overflow> {
    vector(Vec::new())
}

/// `&u64::try_from(material.len()).unwrap_or(u64::MAX).to_be_bytes()` — one framed length, written without a numeric literal.
pub(super) fn framed_length(
    material: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
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
pub(super) fn appended(material: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let called = method_call(
        vec![GeneratedToken::word(INTO_BINDING)],
        "extend_from_slice",
        material,
    )?;
    Ok(statement(called))
}

/// `.map_err(|_| refusal)?` — the road a fallible step takes to this surface's own refusal.
pub(super) fn mapped(refusal: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
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
pub(super) fn absent(refusal: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = call(
        vec![GeneratedToken::alone('.'), GeneratedToken::word("ok_or")],
        refusal,
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// `Refusal::Arm` — one payload-free refusal construction.
pub(super) fn sole_refusal(refusal: &str, arm: DecodeRefusal) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(refusal)];
    tokens.extend(associated(arm.name()));
    tokens
}

/// `Refusal::Arm { member: "spelling" }` — one member-bearing refusal construction.
///
/// The spelling is a text literal, so a refusal this home renders always names the member the read was standing at.
pub(super) fn member_refusal(
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
pub(super) fn self_member(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(spelling),
    ]
}

/// One member read off `self`, borrowed and parenthesized so a wire road always stands over a reference whatever the cardinality supplied it.
pub(super) fn borrowed_self_member(spelling: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut inner = vec![GeneratedToken::alone('&')];
    inner.extend(self_member(spelling));
    Ok(vec![group(GeneratedDelimiter::Parenthesis, inner)?])
}

/// The callable spelling at the end of one publicly stated road.
pub(super) fn road_spelling(stated: &'static str) -> &'static str {
    stated.rsplit("::").next().unwrap_or(stated)
}

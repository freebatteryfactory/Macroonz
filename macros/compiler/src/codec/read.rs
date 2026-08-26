//! The generated road that reads one declared shape back from its canonical bytes.

use super::spell::{
    CANDIDATE_BINDING, CARRIED_BINDING, CHOSEN_BINDING, COLLECTED_BINDING, ELECTED_BINDING,
    LENGTH_BINDING, MATERIAL_BINDING, PRESENT_BINDING, REMAINING_BINDING, WIDTH_BINDING, absent,
    associated, byte_sink, byte_slice, byte_width, empty_vector, framing_width, generics, mapped,
    member_refusal, qualified, reassigned, road_spelling, sole_refusal, statement, type_path,
};
use super::type_contract::{ReadRoad, rendering_contract};
use super::{AssemblyPosture, Cardinality, CodecMember, CodecShape, DecodeRefusal};
use crate::bounded::Overflow;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, absolute_path, bound_local, call, documentation, equality,
    group, method_call, result_type,
};

/// The sentence the rendered decode road documents itself with.
const DECODE_SENTENCE: &str = "Read one value back from its canonical bytes, refusing where the \
     material is not this shape's. A refusal names the member the read was standing at, and \
     material remaining after the last declared member is itself a refusal.";

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
/// A block expression rather than a run of statements, so its own bindings die at its brace and the read that follows binds the same spellings without shadowing anything.
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
    let contract = rendering_contract(member.shape());
    let road = road_spelling(contract.bill.decode_road);
    match contract.read {
        ReadRoad::Count => read_count(refusal, member, road),
        ReadRoad::Bytes => read_bytes(refusal, member, road),
        ReadRoad::Text => read_text(refusal, member, road),
        ReadRoad::ClosedChoice => read_choice(
            refusal,
            member,
            road,
            road_spelling(contract.bill.encode_road),
        ),
        ReadRoad::Nested => read_nested(refusal, member, road),
    }
}

/// A count, narrowed back to the width the member is held at.
fn read_count(
    refusal: &str,
    member: &CodecMember,
    road_name: &str,
) -> Result<GeneratedToken, Overflow> {
    let mut body = bound_local(
        LENGTH_BINDING,
        vec![framing_read(refusal, member.spelling())?],
    );
    let mut contract = absolute_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(vec![GeneratedToken::word("u64")]));
    let road = qualified(type_path(member.held_as()), contract, road_name);
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
fn read_bytes(
    refusal: &str,
    member: &CodecMember,
    road_name: &str,
) -> Result<GeneratedToken, Overflow> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let owned = method_call(
        vec![GeneratedToken::word(CARRIED_BINDING)],
        "to_vec",
        Vec::new(),
    )?;
    let mut contract = absolute_path(&["core", "convert", "TryFrom"]);
    contract.extend(generics(byte_sink()));
    body.extend(admitted(refusal, member, contract, owned, road_name)?);
    group(GeneratedDelimiter::Brace, body)
}

/// Framed text, checked for UTF-8 and handed to the member's own type.
fn read_text(
    refusal: &str,
    member: &CodecMember,
    road_name: &str,
) -> Result<GeneratedToken, Overflow> {
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
    body.extend(admitted(refusal, member, contract, owned, road_name)?);
    group(GeneratedDelimiter::Brace, body)
}

/// The member's own type asked to admit what was read for it.
fn admitted(
    refusal: &str,
    member: &CodecMember,
    contract: Vec<GeneratedToken>,
    material: Vec<GeneratedToken>,
    road_name: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let road = qualified(type_path(member.held_as()), contract, road_name);
    let mut built = call(road, material)?;
    built.extend(mapped(member_refusal(
        refusal,
        DecodeRefusal::MemberNotAdmitted,
        member.spelling(),
    )?)?);
    Ok(built)
}

/// A framed nested value, read by the nested type's own codec.
fn read_nested(
    refusal: &str,
    member: &CodecMember,
    road_name: &str,
) -> Result<GeneratedToken, Overflow> {
    let mut body = framed_prelude(refusal, member.spelling())?;
    let mut road = type_path(member.held_as());
    road.extend(associated(road_name));
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

/// One arm of a closed roster, elected by walking the roster the owner declared.
///
/// This home writes no table of slots: the walk compares the byte it read against each candidate's own position, so a roster that gained an arm gains it here too, and a slot no arm answers to refuses rather than electing a neighbour.
fn read_choice(
    refusal: &str,
    member: &CodecMember,
    roster: &str,
    slot_road: &str,
) -> Result<GeneratedToken, Overflow> {
    let width = call(byte_width(), Vec::new())?;
    let mut body = bound_local(WIDTH_BINDING, width);
    body.extend(read_one_byte(refusal, member.spelling(), CHOSEN_BINDING)?);
    let mut empty = absolute_path(&["core", "option", "Option"]);
    empty.extend(associated("None"));
    body.extend(super::spell::bound_mutable(ELECTED_BINDING, empty));
    body.extend(roster_walk(member, roster, slot_road)?);
    body.push(GeneratedToken::word(ELECTED_BINDING));
    body.extend(absent(member_refusal(
        refusal,
        DecodeRefusal::SlotNotAdmitted,
        member.spelling(),
    )?)?);
    group(GeneratedDelimiter::Brace, body)
}

/// The walk over the owner's own roster, electing the arm whose position is the byte that was read.
fn roster_walk(
    member: &CodecMember,
    roster: &str,
    slot_road: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut some = absolute_path(&["core", "option", "Option"]);
    some.extend(associated("Some"));
    let some = call(some, vec![GeneratedToken::word(CANDIDATE_BINDING)])?;
    let elected = reassigned(ELECTED_BINDING, some);
    let position = method_call(
        vec![GeneratedToken::word(CANDIDATE_BINDING)],
        slot_road,
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
    walk.extend(associated(roster));
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
    body.extend(super::spell::bound_mutable(
        COLLECTED_BINDING,
        empty_vector()?,
    ));
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
pub(super) fn decode_road(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
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
    tokens.push(GeneratedToken::word(super::DECODE_ROAD));
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

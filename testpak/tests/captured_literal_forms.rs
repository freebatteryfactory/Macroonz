//! The literal-forms lane: what a captured literal is, observed from outside the
//! crate that decides it.
//!
//! A producer hands the services a spelling a compiler already lexed. What the
//! services then hold is supposed to be the VALUE that spelling names, under the
//! form it was written in — and both halves of that are observable from here
//! without reading a line of the reader.
//!
//! # Reversals
//!
//! A lane that only ever asked "does `"x"` read as text?" would pass against a
//! reader that answers every quoted spelling with text and every other spelling
//! with a number, which is the reader this grammar exists instead of. So the
//! separations are required rather than assumed: four spellings that once
//! reached two rows must reach four, one value written two ways must reach one
//! row, and one spelling written under two forms must reach two.

use threadpak_macroc::{
    CapturedInput, CapturedPayload, CapturedTokenTree, LiteralReadCause, SpanHandle, TokenPath,
    capture_literal,
};

/// The payload one spelling reads to, or nothing where reading it refused.
fn read(spelling: &str) -> Option<CapturedPayload> {
    capture_literal(spelling).ok()
}

/// The canonical bytes of a one-token declaration carrying one payload.
///
/// The encoding is what a captured declaration's identity is derived over, so
/// two payloads reaching one byte string are two declarations reaching one name.
fn encoded(payload: CapturedPayload) -> Option<Vec<u8>> {
    let tree = CapturedTokenTree::captured(payload, TokenPath::root(), SpanHandle::at(0));
    CapturedInput::taken(vec![tree], 1)
        .ok()
        .map(|input| input.canonical_bytes())
}

/// Each literal form reads to the value it names, under its own row.
///
/// The escape is the case worth stating plainly: a text carrying `\n` is three
/// characters, and the two characters a backslash and an `n` are written with
/// are how the line break was spelled rather than what the declaration says.
#[test]
fn every_literal_form_reads_to_the_value_it_names() {
    assert_eq!(
        read(r#""a\nb""#),
        Some(CapturedPayload::Text(String::from("a\nb")))
    );
    assert_eq!(
        read(r####"r#"a"b"#"####),
        Some(CapturedPayload::Text(String::from("a\"b")))
    );
    assert_eq!(
        read(r#"b"ab""#),
        Some(CapturedPayload::ByteText(b"ab".to_vec()))
    );
    assert_eq!(
        read(r#"br"a\nb""#),
        Some(CapturedPayload::ByteText(br"a\nb".to_vec()))
    );
    assert_eq!(
        read(r#"c"ab""#),
        Some(CapturedPayload::NulTerminatedText(b"ab".to_vec()))
    );
    assert_eq!(read("'x'"), Some(CapturedPayload::Character('x')));
    assert_eq!(read(r"'\u{1F600}'"), Some(CapturedPayload::Character('😀')));
    assert_eq!(read("b'x'"), Some(CapturedPayload::Byte(b'x')));
    assert_eq!(read(r"b'\xff'"), Some(CapturedPayload::Byte(0xFF)));
    assert_eq!(
        read("1_000u32"),
        Some(CapturedPayload::Number(String::from("1_000u32")))
    );
    assert_eq!(
        read("0xFF"),
        Some(CapturedPayload::Number(String::from("0xFF")))
    );
}

/// A raw text carries its body and reads no escape out of it.
///
/// The pair is the whole of what raw means: the same six characters are three
/// characters under one form and six under the other, and a reader that decided
/// by the quotes at the ends would have no way to tell them apart.
#[test]
fn a_raw_text_carries_what_a_quoted_one_reads() {
    assert_eq!(
        read(r#""a\nb""#),
        Some(CapturedPayload::Text(String::from("a\nb")))
    );
    assert_eq!(
        read(r##"r"a\nb""##),
        Some(CapturedPayload::Text(String::from(r"a\nb")))
    );
}

/// The four spellings that once shared two rows reach four rows.
///
/// This is the reversal. Under a reader that asks only whether a spelling is
/// quoted at both ends, `b"x"`, `r"x"` and `'x'` are every one of them the same
/// answer as `1` — so a lane requiring all four to differ fails against that
/// reader and passes only against a grammar that reads the form.
#[test]
fn four_literal_forms_are_four_answers() {
    let forms = [
        read(r#""x""#),
        read(r#"b"x""#),
        read(r#"c"x""#),
        read("'x'"),
        read("b'x'"),
        read("1"),
    ];
    assert!(forms.iter().all(Option::is_some));
    for (position, form) in forms.iter().enumerate() {
        assert!(
            forms
                .iter()
                .skip(position.saturating_add(1))
                .all(|other| other != form)
        );
    }
}

/// One value written two ways is one captured value, and one spelling written
/// under two forms is two.
///
/// The first is why a form decides the value rather than the characters: `"x"`
/// and `r"x"` say the same thing about the declaration. The second is why the
/// form has to be carried: `"x"` and `b"x"` say different things with the same
/// characters, and a seat declared to take one does not take the other.
#[test]
fn a_value_survives_its_spelling_and_a_form_survives_its_characters() {
    assert_eq!(read(r#""x""#), read(r##"r"x""##));
    assert_ne!(read(r#""x""#), read(r#"b"x""#));
}

/// Two forms carrying the same characters encode to different canonical bytes.
///
/// The rows are separated in the encoding and not only in the type, which is
/// what keeps two declarations that say different things from deriving one
/// name.
#[test]
fn two_forms_carrying_one_body_encode_apart() {
    let text = encoded(CapturedPayload::Text(String::from("x")));
    let bytes = encoded(CapturedPayload::ByteText(b"x".to_vec()));
    let terminated = encoded(CapturedPayload::NulTerminatedText(b"x".to_vec()));
    assert!(text.is_some() && bytes.is_some() && terminated.is_some());
    assert_ne!(text, bytes);
    assert_ne!(bytes, terminated);
    assert_ne!(text, terminated);
}

/// A spelling written in no form this grammar has a row for refuses, and is not
/// filed under a neighbouring row.
///
/// The refusal is the point. A literal form this grammar has not learned is
/// visible the day it arrives, rather than arriving as a number whose spelling
/// happens to start with a letter.
#[test]
fn a_spelling_in_no_known_form_refuses() {
    assert_eq!(
        capture_literal("identifier").err(),
        Some(LiteralReadCause::NotAKnownForm)
    );
    assert_eq!(
        capture_literal("").err(),
        Some(LiteralReadCause::NotAKnownForm)
    );
}

/// A known form whose body this grammar could not read refuses under its own
/// cause.
///
/// Two causes rather than one: a form with no row is a row this seam owes, and a
/// body a known form could not read is a reader that does not reach as far as
/// the form does, and a caller holding one of them can tell which.
#[test]
fn a_known_form_this_grammar_cannot_read_refuses_under_its_own_cause() {
    assert_eq!(
        capture_literal(r#""a\qb""#).err(),
        Some(LiteralReadCause::NotReadable)
    );
    assert_eq!(
        capture_literal(r"'\u{110000}'").err(),
        Some(LiteralReadCause::NotReadable)
    );
    assert_eq!(
        capture_literal(r#"b"é""#).err(),
        Some(LiteralReadCause::NotReadable)
    );
}

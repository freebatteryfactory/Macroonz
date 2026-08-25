//! Reading one literal token's spelling into the value it names.
//!
//! Every producer hands this seam a spelling a compiler already lexed, so what arrives is a well-formed Rust literal and the question is never whether it is one.
//! The question is which form it is, and a form is decided by the opening the spelling was written with — never by whether some other form's characters happen to be at the ends of it.
//!
//! That distinction is the whole of this file.
//! A reader that asks "does it start and end with a quote?" answers `b"x"`, `r"x"`, `'x'`, and `1u32` with one word, and the three that are not numbers are then carried as numbers through everything downstream, including the canonical bytes an identity is derived over.
//!
//! What a form decides is which value the body names: `"x"` and `r"x"` are one text, and `"a\nb"` is three characters and not four.
//! The quoting, the raw-string hashes, and the escapes are how a declaration was written down, and a producer's job is to stop carrying them.

use super::{CapturedAtom, LiteralReadCause};

/// Read one literal token's spelling into the atom it names.
///
/// # Errors
///
/// Returns [`LiteralReadCause::NotAKnownForm`] where the spelling opens with no form this grammar has a row for, and [`LiteralReadCause::NotReadable`] where a known form's body carries material this grammar could not read.
/// A form with no row refuses instead of falling through to a neighbouring row, so a literal Rust grows and this grammar has not learned is visible the day it arrives rather than silently misfiled.
pub fn capture_literal(spelling: &str) -> Result<CapturedAtom, LiteralReadCause> {
    if opens_raw(spelling, "br") {
        return Ok(CapturedAtom::ByteText(
            raw_body(spelling, "br")?.as_bytes().to_vec(),
        ));
    }
    if opens_raw(spelling, "cr") {
        return Ok(CapturedAtom::NulTerminatedText(
            raw_body(spelling, "cr")?.as_bytes().to_vec(),
        ));
    }
    if opens_raw(spelling, "r") {
        return Ok(CapturedAtom::Text(raw_body(spelling, "r")?.to_owned()));
    }
    if let Some(body) = quoted(spelling, "b") {
        return byte_material(body).map(CapturedAtom::ByteText);
    }
    if let Some(body) = quoted(spelling, "c") {
        return nul_terminated_material(body).map(CapturedAtom::NulTerminatedText);
    }
    if let Some(body) = quoted(spelling, "") {
        return text_material(body).map(CapturedAtom::Text);
    }
    if let Some(body) = charred(spelling, "b") {
        return one_byte(body).map(CapturedAtom::Byte);
    }
    if let Some(body) = charred(spelling, "") {
        return one_character(body).map(CapturedAtom::Character);
    }
    if opens_number(spelling) {
        return Ok(CapturedAtom::Number(spelling.to_owned()));
    }
    Err(LiteralReadCause::NotAKnownForm)
}

// ---------------------------------------------------------------------------
// Which form the spelling opens with.
// ---------------------------------------------------------------------------

/// Whether the spelling opens a raw literal under one prefix.
///
/// A raw opening is the prefix, then the hashes, then the quote, so what says it is raw is the character after the prefix and a prefix alone decides nothing.
fn opens_raw(spelling: &str, opening: &str) -> bool {
    spelling
        .strip_prefix(opening)
        .is_some_and(|rest| rest.starts_with('#') || rest.starts_with('"'))
}

/// The body of one quoted literal written under one prefix.
fn quoted<'spelling>(spelling: &'spelling str, opening: &str) -> Option<&'spelling str> {
    spelling
        .strip_prefix(opening)?
        .strip_prefix('"')?
        .strip_suffix('"')
}

/// The body of one single-quoted literal written under one prefix.
fn charred<'spelling>(spelling: &'spelling str, opening: &str) -> Option<&'spelling str> {
    spelling
        .strip_prefix(opening)?
        .strip_prefix('\'')?
        .strip_suffix('\'')
}

/// Whether the spelling opens a numeric literal.
///
/// The leading minus is admitted because a producer can hand one over: a token stream a macro composed may carry a suffixed negative integer as one literal token, where source text spells the same value as a punctuation mark beside an unsigned one.
fn opens_number(spelling: &str) -> bool {
    spelling
        .strip_prefix('-')
        .unwrap_or(spelling)
        .starts_with(|character: char| character.is_ascii_digit())
}

/// The body of one raw literal, between the opening quote and the closing quote that carries the same hash count.
fn raw_body<'spelling>(
    spelling: &'spelling str,
    opening: &str,
) -> Result<&'spelling str, LiteralReadCause> {
    let rest = spelling
        .strip_prefix(opening)
        .ok_or(LiteralReadCause::NotReadable)?;
    let hashes = rest
        .chars()
        .take_while(|character| *character == '#')
        .count();
    let mut body = rest
        .get(hashes..)
        .ok_or(LiteralReadCause::NotReadable)?
        .strip_prefix('"')
        .ok_or(LiteralReadCause::NotReadable)?;
    for _ in 0..hashes {
        body = body
            .strip_suffix('#')
            .ok_or(LiteralReadCause::NotReadable)?;
    }
    body.strip_suffix('"').ok_or(LiteralReadCause::NotReadable)
}

// ---------------------------------------------------------------------------
// Reading one body into the value its form names.
// ---------------------------------------------------------------------------

/// One unit of a read body: either a character the body names, or a byte an escape names directly.
///
/// Two and not one, because `\x80` names a byte that is no character of the text a `\u{…}` escape names, and a form decides which of the two it can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReadUnit {
    /// One character, written plainly or named by an escape.
    Character(char),
    /// One byte, named by a hexadecimal escape.
    Byte(u8),
}

/// One cooked body's units, in the order they were written.
fn units(body: &str) -> Result<Vec<ReadUnit>, LiteralReadCause> {
    let mut units = Vec::new();
    let mut characters = body.chars().peekable();
    while let Some(character) = characters.next() {
        if character != '\\' {
            units.push(ReadUnit::Character(character));
            continue;
        }
        if let Some(unit) = escaped(&mut characters)? {
            units.push(unit);
        }
    }
    Ok(units)
}

/// The unit one escape names, with the backslash already read.
///
/// The line continuation names no unit at all: it is how a written line break is spelled out of the value, so it eats the break and the indentation behind it and contributes nothing.
fn escaped(
    characters: &mut core::iter::Peekable<core::str::Chars<'_>>,
) -> Result<Option<ReadUnit>, LiteralReadCause> {
    let marker = characters.next().ok_or(LiteralReadCause::NotReadable)?;
    let unit = match marker {
        'n' => ReadUnit::Character('\n'),
        'r' => ReadUnit::Character('\r'),
        't' => ReadUnit::Character('\t'),
        '0' => ReadUnit::Character('\0'),
        '\\' => ReadUnit::Character('\\'),
        '\'' => ReadUnit::Character('\''),
        '"' => ReadUnit::Character('"'),
        'x' => ReadUnit::Byte(hexadecimal_byte(characters)?),
        'u' => ReadUnit::Character(scalar_value(characters)?),
        '\n' => {
            while characters.next_if(|next| next.is_whitespace()).is_some() {}
            return Ok(None);
        }
        _ => return Err(LiteralReadCause::NotReadable),
    };
    Ok(Some(unit))
}

/// The byte two hexadecimal digits name.
fn hexadecimal_byte(
    characters: &mut core::iter::Peekable<core::str::Chars<'_>>,
) -> Result<u8, LiteralReadCause> {
    let high = hexadecimal_digit(characters.next())?;
    let low = hexadecimal_digit(characters.next())?;
    Ok((high << 4) | low)
}

/// The value one hexadecimal digit names.
fn hexadecimal_digit(character: Option<char>) -> Result<u8, LiteralReadCause> {
    character
        .and_then(|character| character.to_digit(16))
        .and_then(|value| u8::try_from(value).ok())
        .ok_or(LiteralReadCause::NotReadable)
}

/// The character one braced scalar value names, with `u` already read.
fn scalar_value(
    characters: &mut core::iter::Peekable<core::str::Chars<'_>>,
) -> Result<char, LiteralReadCause> {
    if characters.next() != Some('{') {
        return Err(LiteralReadCause::NotReadable);
    }
    let mut value: u32 = 0;
    let mut read = false;
    loop {
        let character = characters.next().ok_or(LiteralReadCause::NotReadable)?;
        if character == '}' {
            break;
        }
        if character == '_' {
            continue;
        }
        let digit = character
            .to_digit(16)
            .ok_or(LiteralReadCause::NotReadable)?;
        value = value
            .checked_mul(16)
            .and_then(|shifted| shifted.checked_add(digit))
            .ok_or(LiteralReadCause::NotReadable)?;
        read = true;
    }
    if !read {
        return Err(LiteralReadCause::NotReadable);
    }
    char::from_u32(value).ok_or(LiteralReadCause::NotReadable)
}

/// One text literal's text.
fn text_material(body: &str) -> Result<String, LiteralReadCause> {
    let mut text = String::new();
    for unit in units(body)? {
        match unit {
            ReadUnit::Character(character) => text.push(character),
            ReadUnit::Byte(byte) => {
                if !byte.is_ascii() {
                    return Err(LiteralReadCause::NotReadable);
                }
                text.push(char::from(byte));
            }
        }
    }
    Ok(text)
}

/// One byte-string literal's material.
fn byte_material(body: &str) -> Result<Vec<u8>, LiteralReadCause> {
    let mut material = Vec::new();
    for unit in units(body)? {
        match unit {
            ReadUnit::Character(character) => material.push(ascii_byte(character)?),
            ReadUnit::Byte(byte) => material.push(byte),
        }
    }
    Ok(material)
}

/// One C string literal's material, without its terminator.
///
/// A character contributes the bytes it is encoded as rather than one byte, because this form admits the whole scalar range and a character outside ASCII is material the value carries.
fn nul_terminated_material(body: &str) -> Result<Vec<u8>, LiteralReadCause> {
    let mut material = Vec::new();
    let mut buffer = [0u8; 4];
    for unit in units(body)? {
        match unit {
            ReadUnit::Character(character) => {
                material.extend_from_slice(character.encode_utf8(&mut buffer).as_bytes());
            }
            ReadUnit::Byte(byte) => material.push(byte),
        }
    }
    Ok(material)
}

/// The one character a character literal's body names.
fn one_character(body: &str) -> Result<char, LiteralReadCause> {
    match units(body)?.as_slice() {
        [ReadUnit::Character(character)] => Ok(*character),
        [ReadUnit::Byte(byte)] if byte.is_ascii() => Ok(char::from(*byte)),
        _ => Err(LiteralReadCause::NotReadable),
    }
}

/// The one byte a byte literal's body names.
fn one_byte(body: &str) -> Result<u8, LiteralReadCause> {
    match units(body)?.as_slice() {
        [ReadUnit::Byte(byte)] => Ok(*byte),
        [ReadUnit::Character(character)] => ascii_byte(*character),
        _ => Err(LiteralReadCause::NotReadable),
    }
}

/// The byte one character names, where the form admits ASCII alone.
fn ascii_byte(character: char) -> Result<u8, LiteralReadCause> {
    if !character.is_ascii() {
        return Err(LiteralReadCause::NotReadable);
    }
    u8::try_from(character).map_err(|_| LiteralReadCause::NotReadable)
}

//! The Rust source text a generated tree projects, for a person to read.
//!
//! One-way.
//! Nothing reads a projection back, derives an identity from one, or decides anything by one: the artifact is the tree.

use super::{GeneratedDelimiter, GeneratedLiteralForm, GeneratedSpacing, GeneratedToken};

/// Project one generated token into Rust source text.
pub(super) fn inspect_token(token: &GeneratedToken, into: &mut String) {
    match token {
        GeneratedToken::Word(word) => {
            into.push_str(word);
            into.push(' ');
        }
        GeneratedToken::RawIdentifier(name) => {
            into.push_str("r#");
            into.push_str(name);
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
                inspect_string_character(character, into);
            }
            into.push('"');
            into.push(' ');
        }
        GeneratedToken::Group { delimiter, tokens } => {
            let written = match delimiter {
                GeneratedDelimiter::Parenthesis => Some(('(', ')')),
                GeneratedDelimiter::Brace => Some(('{', '}')),
                GeneratedDelimiter::Bracket => Some(('[', ']')),
                GeneratedDelimiter::Bare => None,
            };
            if let Some((open, _)) = written {
                into.push(open);
                into.push(' ');
            }
            for inner in tokens.as_slice() {
                inspect_token(inner, into);
            }
            if let Some((_, close)) = written {
                into.push(close);
                into.push(' ');
            }
        }
        GeneratedToken::ByteText(material) => {
            into.push('b');
            into.push('"');
            for byte in material {
                inspect_byte(*byte, into);
            }
            into.push('"');
            into.push(' ');
        }
        GeneratedToken::Number(value) => {
            into.push_str(&value.to_string());
            into.push(' ');
        }
        GeneratedToken::Literal(literal) => inspect_literal(literal.form(), into),
    }
}

/// Write one admitted exact literal as readable Rust source.
fn inspect_literal(literal: GeneratedLiteralForm<'_>, into: &mut String) {
    match literal {
        GeneratedLiteralForm::Number(spelling) => into.push_str(spelling),
        GeneratedLiteralForm::Character(character) => {
            into.push('\'');
            inspect_quoted_character(character, into);
            into.push('\'');
        }
        GeneratedLiteralForm::Byte(byte) => {
            into.push('b');
            into.push('\'');
            inspect_quoted_byte(byte, into);
            into.push('\'');
        }
        GeneratedLiteralForm::NulTerminatedText(material) => {
            into.push('c');
            into.push('"');
            for byte in material {
                inspect_byte(*byte, into);
            }
            into.push('"');
        }
    }
    into.push(' ');
}

/// Write one character inside a string literal.
fn inspect_string_character(character: char, into: &mut String) {
    for escaped in character.escape_default() {
        into.push(escaped);
    }
}

/// Write one character inside a character literal.
fn inspect_quoted_character(character: char, into: &mut String) {
    if character == '\'' {
        into.push('\\');
        into.push('\'');
    } else {
        inspect_string_character(character, into);
    }
}

/// Write one byte inside a byte-character literal.
fn inspect_quoted_byte(byte: u8, into: &mut String) {
    if byte == b'\'' || byte == b'\\' {
        into.push('\\');
        into.push(char::from(byte));
    } else if byte.is_ascii_graphic() || byte == b' ' {
        into.push(char::from(byte));
    } else {
        inspect_hex_byte(byte, into);
    }
}

/// Project one byte of a byte-string literal, as the escape Rust's own grammar admits for it.
///
/// One rule and not a table of shorthands: the quote and the backslash go behind a backslash, a printable ASCII byte writes itself, and every other byte writes as `\xHH`.
/// That spells all two hundred and fifty-six values, so the projection is lawful Rust for any material a renderer holds — and a person reading a byte string is reading bytes, where `\x0A` says which byte is there and `\n` says which character somebody hoped was.
fn inspect_byte(byte: u8, into: &mut String) {
    match byte {
        b'"' | b'\\' => {
            into.push('\\');
            into.push(char::from(byte));
        }
        0x20..=0x7E => into.push(char::from(byte)),
        _ => {
            inspect_hex_byte(byte, into);
        }
    }
}

/// Write one byte as an uppercase hexadecimal escape.
fn inspect_hex_byte(byte: u8, into: &mut String) {
    into.push('\\');
    into.push('x');
    into.push(hex_digit(byte >> 4));
    into.push(hex_digit(byte & 0x0F));
}

/// One uppercase hex digit for one nibble.
///
/// The last arm is fifteen's; both callers mask to the low four bits, so no higher value arrives.
const fn hex_digit(nibble: u8) -> char {
    match nibble {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        _ => 'F',
    }
}

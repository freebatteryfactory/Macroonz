//! The Rust source text a generated tree projects, for a person to read.
//!
//! One-way.
//! Nothing reads a projection back, derives an identity from one, or decides anything by one: the artifact is the tree.

use super::{GeneratedDelimiter, GeneratedSpacing, GeneratedToken};

/// Project one generated token into Rust source text.
pub(super) fn inspect_token(token: &GeneratedToken, into: &mut String) {
    match token {
        GeneratedToken::Word(word) => {
            into.push_str(word);
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
                if character == '"' || character == '\\' {
                    into.push('\\');
                }
                into.push(character);
            }
            into.push('"');
            into.push(' ');
        }
        GeneratedToken::Group { delimiter, tokens } => {
            let (open, close) = match delimiter {
                GeneratedDelimiter::Parenthesis => ('(', ')'),
                GeneratedDelimiter::Brace => ('{', '}'),
                GeneratedDelimiter::Bracket => ('[', ']'),
            };
            into.push(open);
            into.push(' ');
            for inner in tokens.as_slice() {
                inspect_token(inner, into);
            }
            into.push(close);
            into.push(' ');
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
            into.push('\\');
            into.push('x');
            into.push(hex_digit(byte >> 4));
            into.push(hex_digit(byte & 0x0F));
        }
    }
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

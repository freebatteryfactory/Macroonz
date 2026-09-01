//! The canonical byte form of a generated token tree.
//!
//! Existing occupied slots never move; new token distinctions append at the end.

use super::{GeneratedDelimiter, GeneratedLiteralForm, GeneratedSpacing, GeneratedToken};
use crate::identity::{encode_bytes, encode_length};

/// Encode one generated token into the canonical byte form.
pub(super) fn encode_generated(token: &GeneratedToken, into: &mut Vec<u8>) {
    match token {
        GeneratedToken::Word(word) => {
            into.push(1);
            encode_text(word, into);
        }
        GeneratedToken::Punct { mark, spacing } => {
            into.push(2);
            into.push(match spacing {
                GeneratedSpacing::Joint => 0,
                GeneratedSpacing::Alone => 1,
            });
            let mut buffer = [0u8; 4];
            encode_text(mark.encode_utf8(&mut buffer), into);
        }
        GeneratedToken::Text(text) => {
            into.push(3);
            encode_text(text, into);
        }
        GeneratedToken::Group { delimiter, tokens } => {
            into.push(4);
            into.push(match delimiter {
                GeneratedDelimiter::Parenthesis => 0,
                GeneratedDelimiter::Brace => 1,
                GeneratedDelimiter::Bracket => 2,
                GeneratedDelimiter::Bare => 3,
            });
            encode_length(tokens.len(), into);
            for inner in tokens.as_slice() {
                encode_generated(inner, into);
            }
        }
        GeneratedToken::ByteText(material) => {
            into.push(5);
            encode_bytes(material, into);
        }
        GeneratedToken::Number(value) => {
            into.push(6);
            into.extend_from_slice(&value.to_be_bytes());
        }
        GeneratedToken::RawIdentifier(name) => {
            into.push(7);
            encode_text(name, into);
        }
        GeneratedToken::Literal(literal) => {
            into.push(8);
            encode_literal(literal.form(), into);
        }
    }
}

/// Encode one admitted exact literal beneath the generated-literal slot.
fn encode_literal(literal: GeneratedLiteralForm<'_>, into: &mut Vec<u8>) {
    match literal {
        GeneratedLiteralForm::Number(spelling) => {
            into.push(0);
            encode_text(spelling, into);
        }
        GeneratedLiteralForm::Character(character) => {
            into.push(1);
            let mut buffer = [0u8; 4];
            encode_text(character.encode_utf8(&mut buffer), into);
        }
        GeneratedLiteralForm::Byte(byte) => {
            into.push(2);
            into.push(byte);
        }
        GeneratedLiteralForm::NulTerminatedText(material) => {
            into.push(3);
            encode_bytes(material, into);
        }
    }
}

fn encode_text(text: &str, into: &mut Vec<u8>) {
    encode_bytes(text.as_bytes(), into);
}

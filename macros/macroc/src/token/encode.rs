//! The canonical byte form of a token tree, in both directions.
//!
//! Spans are excluded on the reading side on purpose: a capture's identity is
//! about the declaration, and two producers reading one declaration issue
//! different handles for it. Every variable-length member is written through the
//! plane's one length framing, so no two token sequences can be cut at another
//! boundary and produce one byte string.

use super::{CapturedDelimiter, CapturedPayload, CapturedTokenTree, GeneratedDelimiter};
use super::{GeneratedSpacing, GeneratedToken};
use crate::plane::{encode_bytes, encode_length};

/// Encode one captured tree into the canonical byte form, spans excluded.
pub(super) fn encode_captured(tree: &CapturedTokenTree, into: &mut Vec<u8>) {
    match tree.payload() {
        CapturedPayload::Word(word) => {
            into.push(1);
            encode_text(word, into);
        }
        CapturedPayload::Punct(mark) => {
            into.push(2);
            let mut buffer = [0u8; 4];
            encode_text(mark.encode_utf8(&mut buffer), into);
        }
        CapturedPayload::Text(text) => {
            into.push(3);
            encode_text(text, into);
        }
        CapturedPayload::Number(number) => {
            into.push(4);
            encode_text(number, into);
        }
        CapturedPayload::Group { delimiter, trees } => {
            into.push(5);
            into.push(match delimiter {
                CapturedDelimiter::Parenthesis => 0,
                CapturedDelimiter::Brace => 1,
                CapturedDelimiter::Bracket => 2,
                CapturedDelimiter::Bare => 3,
            });
            encode_length(trees.len(), into);
            for inner in trees.iter() {
                encode_captured(inner, into);
            }
        }
    }
}

/// Encode one length-prefixed text under the plane's one length framing.
fn encode_text(text: &str, into: &mut Vec<u8>) {
    encode_bytes(text.as_bytes(), into);
}

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
            });
            encode_length(tokens.len(), into);
            for inner in tokens.iter() {
                encode_generated(inner, into);
            }
        }
    }
}

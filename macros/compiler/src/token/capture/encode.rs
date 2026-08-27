//! The canonical byte form of a captured token tree.
//!
//! Spans are excluded on the reading side on purpose: a capture's identity is about the declaration, and two producers reading one declaration issue different handles for it.
//! Every variable-length member is written through the one length framing, so no two token sequences can be cut at another boundary and produce one byte string.
//! A member of fixed width is written at that width and is not framed, because framing exists to stop two members being cut at another boundary and a field that is always the same width has exactly one.
//!
//! # The slot tables
//!
//! The first byte written for a token is that arm's slot, and a slot is content: these bytes are what an identity over a captured declaration is derived from.
//! So a table grows at its end and is never renumbered.
//! Renumbering an occupied slot re-encodes trees that were already encoded, which renames every identity derived from them; appending re-encodes nothing, because a tree that could not carry the new arm encodes exactly as it did before.

#[cfg(feature = "host")]
use super::TokenPath;
use super::{CapturedDelimiter, CapturedPayload, CapturedTokenTree};
use crate::identity::{encode_bytes, encode_length};

/// Encode one declaration-local token path as its step count followed by fixed-width steps.
///
/// The count uses the compiler's one length encoding, and every step is the `u32` value the path owns in big-endian order.
/// Producer-local spans never reach this grammar.
#[cfg(feature = "host")]
pub(crate) fn encode_token_path(path: &TokenPath, into: &mut Vec<u8>) {
    encode_length(path.steps().len(), into);
    for step in path.steps() {
        into.extend_from_slice(&step.to_be_bytes());
    }
}

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
            for inner in trees.as_slice() {
                encode_captured(inner, into);
            }
        }
        // Framed as raw bytes rather than as text: a byte string is not text, and there is no decode on the road to its encoding.
        CapturedPayload::ByteText(material) => {
            into.push(6);
            encode_bytes(material, into);
        }
        CapturedPayload::Character(character) => {
            into.push(7);
            let mut buffer = [0u8; 4];
            encode_text(character.encode_utf8(&mut buffer), into);
        }
        // One byte, unframed: the width is fixed, so there is no boundary a reader could cut at differently.
        CapturedPayload::Byte(byte) => {
            into.push(8);
            into.push(*byte);
        }
        // The material without its terminator: the terminating NUL is the literal form's and never the value's, and the slot already says which form this is.
        CapturedPayload::NulTerminatedText(material) => {
            into.push(9);
            encode_bytes(material, into);
        }
        CapturedPayload::RawIdentifier(name) => {
            into.push(10);
            encode_text(name, into);
        }
        CapturedPayload::JointPunct(mark) => {
            into.push(11);
            let mut buffer = [0u8; 4];
            encode_text(mark.encode_utf8(&mut buffer), into);
        }
    }
}

/// Encode one length-prefixed text under the one length framing.
fn encode_text(text: &str, into: &mut Vec<u8>) {
    encode_bytes(text.as_bytes(), into);
}

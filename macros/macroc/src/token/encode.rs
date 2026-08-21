//! The canonical byte form of a token tree, in both directions.
//!
//! Spans are excluded on the reading side on purpose: a capture's identity is
//! about the declaration, and two producers reading one declaration issue
//! different handles for it.
//! Every variable-length member is written through the plane's one length
//! framing, so no two token sequences can be cut at another boundary and
//! produce one byte string.
//! A member of FIXED width is written at that width and is not framed: framing
//! exists to stop two members being cut at another boundary, and a field that is
//! always eight bytes has exactly one boundary. Eight bytes stating the constant
//! eight would be a length nobody can read differently.
//!
//! # The slot tables
//!
//! The first byte this file writes for a token is that arm's SLOT, and a slot is
//! transcript content: the bytes written here are
//! [`super::GeneratedTree::canonical_bytes`] and
//! [`super::CapturedInput::canonical_bytes`], which are what the plane's
//! identities over a rendered unit and a captured declaration are derived over.
//!
//! So a slot table grows at its END and is never renumbered. Renumbering an
//! occupied slot re-encodes trees that were already encoded, which renames every
//! identity derived from them; appending re-encodes nothing, because a tree that
//! could not carry the new arm encodes exactly as it did before.
//!
//! The captured table stands at nine rows: five as first declared, and the four
//! literal forms appended at six through nine behind them.
//! The generated table stands at six: four as first declared, and the
//! byte-string and numeric arms appended at five and six behind the four already
//! in use.
//!
//! # Which family a slot table moves, and whether appending moves it
//!
//! The two tables here feed two different preimage grammars, so they answer to
//! two different version ladders. The generated table is the content a rendered
//! unit's identity and its output-bytes digest are derived over, which is
//! [`crate::plane::RENDERED_UNIT_IDENTITY_PROFILE`]; the captured table is the
//! content a captured declaration's identity is derived over, which is
//! [`crate::plane::CAPTURED_DECLARATION_IDENTITY_PROFILE`]. An arm added to one
//! table is a question about that table's family alone, and no identity outside
//! it is reachable from the answer.
//!
//! [`crate::plane::IdentityProfileVersion`] states that a change to what a
//! transcript CONTAINS — the members, their order, or the content a mint site
//! composes — is a bump of that family, and that a bump renames every identity
//! the family derives. The two generated arms pulled that law both ways when
//! they were appended, and the honest reading of each is:
//!
//! - **Nothing already derived moves.** Word, punct, text, and group keep slots
//!   one through four, the framing is unchanged, and no group's inner encoding
//!   shifts. Every generated tree that was spellable before the two arms existed
//!   encodes byte for byte as it did, so every identity derived under the
//!   current position re-derives to the same value. There is no renaming for a
//!   bump to perform, and a bump would rename identities to distinguish them
//!   from trees that could not have existed.
//! - **The grammar did grow.** What a generated tree's canonical bytes may
//!   contain now has two more rows, reachable only by trees no renderer could
//!   build before. An independent reader holding the previous grammar and a tree
//!   carrying a byte-string literal cannot read it, and "a reader handed two
//!   identities of one family under one position may assume both were derived
//!   the same way" is the sentence the version's own declaration makes.
//!
//! **The rendered-unit family settles it by standing over the table as it is.**
//! Its position one was declared against the six-row generated grammar, arms
//! included, so the promise that position makes is true of exactly what this
//! file writes and there is no earlier position for a reader to be holding. The
//! next arm appended is the next reader's question, at this seat, against that
//! family's position and no other.
//!
//! **The captured table reaches the opposite answer, and the difference is what
//! the two readings above are for.** Its four appended rows are not reachable
//! only by declarations that could not exist before. A declaration carrying
//! `b"x"`, `r"x"`, or `'x'` was always lawful and was always captured — it was
//! captured under the numeric row, whose framed content was the spelling — and a
//! text carrying an escape was captured with the escape's own characters in it.
//! Those declarations encode to different bytes now, so identities already
//! derived over them re-derive to different values.
//!
//! That is the case [`crate::plane::IdentityProfileVersion`] describes rather
//! than the case the two arms above argued their way out of, so the
//! captured-declaration family is bumped and the renaming is the bump doing
//! exactly what it is for. Its position is stated on
//! [`crate::plane::CAPTURED_DECLARATION_IDENTITY_PROFILE`], where the reason
//! sits beside the number.

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
        // Appended at six: the material is written through the one framing as
        // raw bytes rather than through `encode_text`, exactly as the generated
        // side's byte string is — a byte string is not text and there is no
        // decode on the road to its encoding.
        CapturedPayload::ByteText(material) => {
            into.push(6);
            encode_bytes(material, into);
        }
        // Appended at seven: the character's own UTF-8, framed. A character
        // literal and a one-character text are different tokens at the seat they
        // are written to, and the slot is what says so — the framed bytes behind
        // it are the same either way.
        CapturedPayload::Character(character) => {
            into.push(7);
            let mut buffer = [0u8; 4];
            encode_text(character.encode_utf8(&mut buffer), into);
        }
        // Appended at eight: one byte, unframed. The width is fixed, so there is
        // no boundary a reader could cut at differently.
        CapturedPayload::Byte(byte) => {
            into.push(8);
            into.push(*byte);
        }
        // Appended at nine: the material without its terminator, framed. The
        // terminating NUL is the literal form's and never the value's, and the
        // slot already says which form this is.
        CapturedPayload::NulTerminatedText(material) => {
            into.push(9);
            encode_bytes(material, into);
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
        // Appended at five: the material is variable-length, so it is written
        // through the one framing, as raw bytes rather than through
        // `encode_text` — a byte string is not text and there is no decode on
        // the road to its encoding.
        GeneratedToken::ByteText(material) => {
            into.push(5);
            encode_bytes(material, into);
        }
        // Appended at six: eight big-endian bytes, unframed. The width is fixed,
        // so there is no boundary a reader could cut at differently and nothing
        // for a length prefix to settle. Written from the VALUE and never from
        // its digits: two spellings of one number would be two preimages for one
        // token.
        GeneratedToken::Number(value) => {
            into.push(6);
            into.extend_from_slice(&value.to_be_bytes());
        }
    }
}

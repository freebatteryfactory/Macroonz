//! The plane's one canonical framing, and the citation encoding written with
//! it.
//!
//! Every canonical encoding anywhere in the services — a captured tree, a
//! planned membership, a rendered unit, a transcript — is written through the
//! two primitives here.
//! One framing rather than one per home is what keeps the concatenation
//! collision from being reintroduced locally: a home that invented its own
//! length spelling would admit two byte strings for one value without anything
//! else in the crate noticing.

use super::OwnerFactRef;

/// Append one length as eight big-endian bytes.
///
/// A fixed width rather than a varint, because a canonical encoding that
/// admitted two spellings of one length would admit two preimages for one
/// value.
pub fn encode_length(length: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(length).unwrap_or(u64::MAX).to_be_bytes());
}

/// Append one length-prefixed byte string: the eight-byte length, then the
/// bytes.
///
/// Without the prefix, two members could be split at a different boundary and
/// encode identically — the concatenation collision the prefix removes
/// outright.
pub fn encode_bytes(material: &[u8], into: &mut Vec<u8>) {
    encode_length(material.len(), into);
    into.extend_from_slice(material);
}

impl OwnerFactRef {
    /// The canonical bytes of this citation, for a transcript to be taken over.
    #[must_use]
    pub fn citation_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Self::Minted { home, fact } => {
                bytes.push(0);
                bytes.extend_from_slice(home.as_bytes());
                bytes.extend_from_slice(fact.as_bytes());
            }
            Self::Declared(named) => {
                bytes.push(1);
                bytes.extend_from_slice(named.home.as_bytes());
                bytes.push(b'.');
                bytes.extend_from_slice(named.fact.as_bytes());
            }
        }
        bytes
    }
}

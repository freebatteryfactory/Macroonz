//! The one canonical framing, and the two citation encodings written with it.
//!
//! Every canonical encoding anywhere in this crate — a captured tree, a planned membership, a rendered unit, a transcript — is written through the two primitives here.
//! One framing rather than one per home is what keeps the concatenation collision from being reintroduced locally: a home that invented its own length spelling would admit two byte strings for one value without anything else noticing.

use super::{OwnerFact, OwnerIdentity, Profile};

/// Append one length as eight big-endian bytes.
///
/// A fixed width rather than a varint, because a canonical encoding that admitted two spellings of one length would admit two preimages for one value.
pub fn encode_length(length: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(length).unwrap_or(u64::MAX).to_be_bytes());
}

/// Append one length-prefixed byte string: the eight-byte length, then the bytes.
///
/// Without the prefix, two members could be split at a different boundary and encode identically — the concatenation collision the prefix removes outright.
pub fn encode_bytes(material: &[u8], into: &mut Vec<u8>) {
    encode_length(material.len(), into);
    into.extend_from_slice(material);
}

impl Profile {
    /// Appends this profile's canonical bytes: the stem of whoever owns the grammar, its declared name, then its version position in four big-endian bytes.
    ///
    /// Seated with the type on purpose: every identity home that commits to a profile writes it through this one road, so a lawful grammar edit moves every identity family at once rather than splitting the homes that restated the spelling from the homes that did not.
    pub fn encode_into(self, into: &mut Vec<u8>) {
        encode_bytes(self.stem().as_bytes(), into);
        encode_bytes(self.name().as_bytes(), into);
        into.extend_from_slice(&self.version().position().to_be_bytes());
    }
}

impl OwnerFact {
    /// The canonical bytes of this citation, for a transcript to be taken over.
    #[must_use]
    pub fn citation_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_bytes(self.home.as_bytes(), &mut bytes);
        encode_bytes(self.name.as_bytes(), &mut bytes);
        bytes
    }
}

impl OwnerIdentity {
    /// The canonical bytes of this citation, for a transcript to be taken over.
    ///
    /// The subject is framed ahead of the identity, so one consumer's thirty-two bytes cited under two subjects are two citations rather than one.
    #[must_use]
    pub fn citation_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_bytes(self.subject.as_bytes(), &mut bytes);
        encode_bytes(&self.bytes, &mut bytes);
        bytes
    }
}

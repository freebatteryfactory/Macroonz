//! Canonical framing shared by every harness identity preimage.

/// Append one length as eight big-endian bytes.
///
/// A fixed width rather than a varint, because an encoding that admitted two spellings of one length would admit two preimages for one value.
/// Seated with the substrate: one framing for the whole crate is what keeps a concatenation collision from being reintroduced locally, in a home that invented its own length spelling.
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

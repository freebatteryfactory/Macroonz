//! The invariant nucleus for context spelling and independent preimage composition.

use super::{
    ContextRefusal, DerivedIdentity, SpecifiedContext, TranscriptDerivation,
    TranscriptDisagreement, TranscriptMember, TranscriptVerdict,
};

impl SpecifiedContext {
    /// Spell one derivation context from the segments a caller writes out.
    ///
    /// The segments are joined by `/`, the separator a published profile grammar states, so a stem published as a multi-segment path is handed in as its own segments and this lane never invents a join.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, an empty segment, or a segment containing `/` before a context exists.
    pub fn spelled(segments: &[&str]) -> Result<Self, ContextRefusal> {
        if segments.is_empty() {
            return Err(ContextRefusal::NoSegments);
        }
        let mut spelling = String::new();
        for (at, segment) in segments.iter().enumerate() {
            if segment.is_empty() {
                return Err(ContextRefusal::EmptySegment { at });
            }
            if segment.contains('/') {
                return Err(ContextRefusal::EmbeddedSeparator { at });
            }
            if !spelling.is_empty() {
                spelling.push('/');
            }
            spelling.push_str(segment);
        }
        Ok(Self(spelling))
    }

    /// Spell one derivation context under a profile version: the stem's segments, then `v` and the version, then the rest.
    ///
    /// The version is written here from the number the specification publishes, because a lane that imported it would agree with a producer that silently changed it.
    ///
    /// # Errors
    ///
    /// Refuses exactly what [`SpecifiedContext::spelled`] refuses, over the assembled roster: a position a refusal carries is a position in that roster and not in the caller's stem.
    pub fn under_version(
        stem: &[&str],
        version: u32,
        tail: &[&str],
    ) -> Result<Self, ContextRefusal> {
        let versioned = format!("v{version}");
        let mut segments: Vec<&str> = Vec::new();
        segments.extend_from_slice(stem);
        segments.push(&versioned);
        segments.extend_from_slice(tail);
        Self::spelled(&segments)
    }

    /// The context as a derivation reads it.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl TranscriptDerivation {
    /// An empty preimage, before its first member.
    #[must_use]
    pub const fn opened() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    /// Append one length-prefixed byte string.
    #[must_use]
    pub fn framed(mut self, material: &[u8]) -> Self {
        self.members
            .push(TranscriptMember::Framed(material.to_vec()));
        self
    }

    /// Append one length-prefixed byte string, over text framed exactly as bytes are.
    #[must_use]
    pub fn framed_text(self, text: &str) -> Self {
        self.framed(text.as_bytes())
    }

    /// Append one bare discriminant byte.
    #[must_use]
    pub fn discriminant(mut self, slot: u8) -> Self {
        self.members.push(TranscriptMember::Discriminant(slot));
        self
    }

    /// Append one 32-bit number, four big-endian bytes, unframed.
    #[must_use]
    pub fn fixed32(mut self, value: u32) -> Self {
        self.members.push(TranscriptMember::Fixed32(value));
        self
    }

    /// Append one 64-bit number, eight big-endian bytes, unframed.
    #[must_use]
    pub fn fixed64(mut self, value: u64) -> Self {
        self.members.push(TranscriptMember::Fixed64(value));
        self
    }

    /// The members written so far, in order, as typed decisions rather than as accumulated bytes.
    #[must_use]
    pub fn members(&self) -> &[TranscriptMember] {
        &self.members
    }

    /// Compose the preimage: every member, in order, by this lane's own framing.
    ///
    /// A framed member is eight big-endian length bytes then its bytes, a discriminant is one bare byte, and a fixed number is its own width in big-endian bytes.
    /// No separators, no padding, and nothing between members.
    #[must_use]
    pub fn preimage(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for member in &self.members {
            match member {
                TranscriptMember::Framed(material) => {
                    let width = u64::try_from(material.len()).unwrap_or(u64::MAX);
                    bytes.extend_from_slice(&width.to_be_bytes());
                    bytes.extend_from_slice(material);
                }
                TranscriptMember::Discriminant(slot) => bytes.push(*slot),
                TranscriptMember::Fixed32(value) => bytes.extend_from_slice(&value.to_be_bytes()),
                TranscriptMember::Fixed64(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            }
        }
        bytes
    }

    /// Derive the identity this preimage names under one context, by BLAKE3's `derive_key` over the context's spelling and these bytes.
    #[must_use]
    pub fn derived(&self, context: &SpecifiedContext) -> DerivedIdentity {
        DerivedIdentity(blake3::derive_key(context.spelling(), &self.preimage()))
    }
}

impl DerivedIdentity {
    /// The thirty-two bytes, borrowed for comparison and for rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Compare this re-derivation against the identity a producer published.
    ///
    /// The claim it supports is this lane's alone: the published specification, read and encoded independently, names the identity the producer minted.
    pub fn compared(&self, published: &[u8; 32]) -> TranscriptVerdict {
        if self.0 == *published {
            TranscriptVerdict::Agrees
        } else {
            TranscriptVerdict::Disagrees(TranscriptDisagreement {
                rederived: self.0,
                published: *published,
            })
        }
    }
}

impl TranscriptDisagreement {
    /// What this lane derived from the specification alone.
    #[must_use]
    pub const fn rederived(&self) -> &[u8; 32] {
        &self.rederived
    }

    /// What the producer published.
    #[must_use]
    pub const fn published(&self) -> &[u8; 32] {
        &self.published
    }
}

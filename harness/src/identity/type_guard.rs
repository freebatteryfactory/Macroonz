//! Constructors and readers for the identity substrate.

use super::{ContentAddress, DomainTag, IdentityProfile, IdentityProfileVersion};
use crate::identity::HARNESS_IDENTITY_PROFILE;

impl IdentityProfileVersion {
    /// The position the family's authority assigned.
    #[must_use]
    pub const fn declared(position: u32) -> Self {
        Self(position)
    }

    /// The assigned position.
    #[must_use]
    pub const fn position(self) -> u32 {
        self.0
    }
}

impl DomainTag {
    /// The tag its owning home declared, at that family's own position.
    ///
    /// The position travels with the tag, so moving one family's order cannot rename addresses under a family whose grammar never changed.
    ///
    /// # Panics
    ///
    /// Panics if the spelling contains a byte other than a lowercase ASCII letter, an ASCII digit, or `-`.
    #[must_use]
    pub const fn declared(spelling: &'static str, version: IdentityProfileVersion) -> Self {
        let mut remaining = spelling.as_bytes();
        while let [byte, tail @ ..] = remaining {
            assert!(
                matches!(*byte, b'a'..=b'z' | b'0'..=b'9' | b'-'),
                "a domain tag contains a byte outside its declared character grammar"
            );
            remaining = tail;
        }
        Self { spelling, version }
    }

    /// The declared spelling, as it appears in a derivation context.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.spelling
    }

    /// This family's own position.
    #[must_use]
    pub const fn version(self) -> IdentityProfileVersion {
        self.version
    }
}

impl IdentityProfile {
    /// The profile at one stem.
    #[must_use]
    pub const fn declared(stem: &'static str) -> Self {
        Self { stem }
    }

    /// The declared stem — everything of the context ahead of the family.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        self.stem
    }

    /// The derivation context for one domain tag.
    ///
    /// The version comes off the tag, so a caller cannot spell one family's context while naming another family's position.
    #[must_use]
    pub fn context_for(self, tag: DomainTag) -> String {
        let stem = self.stem;
        let family = tag.spelling();
        let version = tag.version().position();
        format!("{stem}/{family}/v{version}")
    }
}

impl ContentAddress {
    /// Derive the address of one preimage under one kind's domain tag.
    ///
    /// The bytes handed in are the preimage: an address is never a digest of source text, and a preimage is never "the id".
    #[must_use]
    pub fn derived(tag: DomainTag, preimage: &[u8]) -> Self {
        Self(blake3::derive_key(
            &HARNESS_IDENTITY_PROFILE.context_for(tag),
            preimage,
        ))
    }

    /// The address's thirty-two bytes, borrowed for comparison and for rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

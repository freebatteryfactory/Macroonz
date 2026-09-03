//! Constructors and readers for the identity substrate.

use super::{BodyReader, ContentAddress, DomainTag, IdentityProfile, IdentityProfileVersion};
use crate::identity::HARNESS_IDENTITY_PROFILE;

/// The declaration and reader every typed content-address wrapper shares.
///
/// A semantic home still owns how its wrapper is earned; this stamp owns only the one-field representation and whether its established public reader returns the address by value or by reference.
macro_rules! content_address_reference {
    (
        $(#[$reference_meta:meta])*
        $visibility:vis struct $reference:ident;
    ) => {
        $(#[$reference_meta])*
        $visibility struct $reference($crate::identity::ContentAddress);
    };
    (
        $(#[$reader_meta:meta])*
        value $reference:ident;
    ) => {
        impl $reference {
            $(#[$reader_meta])*
            #[must_use]
            pub const fn address(self) -> $crate::identity::ContentAddress {
                self.0
            }
        }
    };
    (
        $(#[$reader_meta:meta])*
        borrowed $reference:ident;
    ) => {
        impl $reference {
            $(#[$reader_meta])*
            #[must_use]
            pub const fn address(&self) -> &$crate::identity::ContentAddress {
                &self.0
            }
        }
    };
}

pub(crate) use content_address_reference;

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
    /// The fixed width of every content address.
    pub(crate) const WIDTH: usize = size_of::<[u8; 32]>();

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

impl<'body, Refusal: Copy> BodyReader<'body, Refusal> {
    /// Open at the first body byte under the calling home's refusal vocabulary.
    pub(crate) const fn over(
        body: &'body [u8],
        truncated: Refusal,
        length_outside_platform: fn(u64) -> Refusal,
    ) -> Self {
        Self {
            body,
            at: 0,
            truncated,
            length_outside_platform,
        }
    }

    /// Read one fixed-width 32-bit integer.
    pub(crate) fn u32(&mut self) -> Result<u32, Refusal> {
        self.fixed::<4>().map(u32::from_be_bytes)
    }

    /// Read one fixed-width 64-bit integer.
    pub(crate) fn u64(&mut self) -> Result<u64, Refusal> {
        self.fixed::<8>().map(u64::from_be_bytes)
    }

    /// Read one declared count, refused where the platform cannot index it.
    pub(crate) fn count(&mut self) -> Result<usize, Refusal> {
        let declared = self.u64()?;
        usize::try_from(declared)
            .map_err(|_beyond_platform| (self.length_outside_platform)(declared))
    }

    /// Read one length-prefixed byte string.
    pub(crate) fn bytes(&mut self) -> Result<&'body [u8], Refusal> {
        let length = self.count()?;
        self.take(length)
    }

    /// How many bytes remain unread.
    pub(crate) const fn remaining(&self) -> usize {
        self.body.len().saturating_sub(self.at)
    }

    /// Read one fixed-width byte array.
    fn fixed<const WIDTH: usize>(&mut self) -> Result<[u8; WIDTH], Refusal> {
        let bytes = self.take(WIDTH)?;
        <[u8; WIDTH]>::try_from(bytes).map_err(|_unexpected_width| self.truncated)
    }

    /// Advance over exactly this many bytes, or refuse the body as truncated.
    fn take(&mut self, width: usize) -> Result<&'body [u8], Refusal> {
        let Some(end) = self.at.checked_add(width) else {
            return Err(self.truncated);
        };
        let Some(bytes) = self.body.get(self.at..end) else {
            return Err(self.truncated);
        };
        self.at = end;
        Ok(bytes)
    }
}

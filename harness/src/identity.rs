//! One derivation mechanism for every identity this crate mints.
//!
//! An address is `blake3::derive_key` over a preimage, under a context assembled from one stem, the minting home's [`DomainTag`], and the position that tag carries.
//! Separation is by derivation context rather than by a prefix inside the message, so two addresses over one preimage under different tags are unrelated values.
//! Nothing here knows what is being named: a tag arrives as an argument from the home that owns the kind, so the substrate carries no semantic noun.
//!
//! An address commits to the preimage its minting home wrote and to nothing else.

/// One position in one preimage family's own order.
///
/// A position is a real segment of every derivation context, and it belongs to the family whose [`DomainTag`] it is declared beside.
/// It moves when that family's preimage changes and somebody outside this repository already holds an address derived under the old one; until then the change is an edit to position one in place.
/// There is no `Ord`, because positions are matched rather than ranked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfileVersion(u32);

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

/// One kind's declared derivation domain.
///
/// The tag is a segment of the derive-key context, so two kinds over identical preimages never share an address.
/// Its spelling is lowercase ASCII letters, digits, and `-`, written as a literal by the owning home, so the grammar is a declaration discipline and no road here reads a tag from data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DomainTag {
    spelling: &'static str,
    version: IdentityProfileVersion,
}

impl DomainTag {
    /// The tag its owning home declared, at that family's own position.
    ///
    /// The position travels with the tag, so moving one family's order cannot rename addresses under a family whose grammar never changed.
    #[must_use]
    pub const fn declared(spelling: &'static str, version: IdentityProfileVersion) -> Self {
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

/// The stem an address is derived under, and the grammar that joins it to a family.
///
/// ```text
/// <stem>/<tag>/v<version>
/// ```
///
/// The tag sits ahead of the version, so a position reads as its family's rather than as the stem's: position one of `trial-key` and position one of `seed-pack` are two key spaces, and a family that moves moves alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfile {
    stem: &'static str,
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

/// The stem every content address in this crate is derived under.
pub const HARNESS_IDENTITY_PROFILE: IdentityProfile =
    IdentityProfile::declared("macroonz/harness-identity");

/// One thirty-two byte content address.
///
/// Collision resistance is claimed as BLAKE3's, over the preimage the minting home wrote, under the [`DomainTag`] that home declared, at the position that tag carries — and nothing broader.
/// [`ContentAddress::derived`] is the only road: no seam wraps arbitrary bytes, so an address always came from a preimage under a tag.
/// The byte-lexicographic order exists so addresses can key an ordered map deterministically, and means nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentAddress([u8; 32]);

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

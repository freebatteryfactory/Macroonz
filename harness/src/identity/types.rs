//! The declared identity domains, profile, and content address.

#[path = "type_guard.rs"]
mod guard;

pub(crate) use guard::content_address_reference;

/// One position in one preimage family's own order.
///
/// A position is a real segment of every derivation context, and it belongs to the family whose [`DomainTag`] it is declared beside.
/// It moves when that family's preimage changes and somebody outside this repository already holds an address derived under the old one; until then the change is an edit to position one in place.
/// There is no `Ord`, because positions are matched rather than ranked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfileVersion(pub(super) u32);

/// One kind's declared derivation domain.
///
/// The tag is a segment of the derive-key context, so two kinds over identical preimages never share an address.
/// Its spelling contains only lowercase ASCII letters, digits, and `-`; the declaration refuses every other byte, and no road here reads a tag from data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DomainTag {
    pub(super) spelling: &'static str,
    pub(super) version: IdentityProfileVersion,
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
    pub(super) stem: &'static str,
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
pub struct ContentAddress(pub(super) [u8; 32]);

/// A bounded cursor over one canonical body, parameterized by its owning home's refusal vocabulary.
pub(crate) struct BodyReader<'body, Refusal> {
    body: &'body [u8],
    at: usize,
    truncated: Refusal,
    length_outside_platform: fn(u64) -> Refusal,
}

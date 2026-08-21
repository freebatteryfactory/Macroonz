//! The family identity substrate: one content address, and the one versioned,
//! domain-separated profile every identity in this crate is derived under.
//!
//! Every identity the harness mints is a [`ContentAddress`] derived here, so
//! the harness is a citizen of the workspace identity family rather than an
//! identity island holding a second mechanism. Every checksum, deep compare,
//! and second naming scheme the harness would otherwise invent dies against
//! this file.
//!
//! # Authority
//!
//! Nothing here knows what is being named. A kind's [`DomainTag`] is declared
//! by the home that owns the kind and arrives as an argument, so the substrate
//! carries no semantic noun and gains no opinion when a home adds one.
//!
//! The mechanism is BLAKE3's `derive_key`, at the feature cut the workspace
//! dependency table decides: separation is by derivation CONTEXT, never by a
//! prefix inside the message, so two addresses over identical preimages under
//! different tags are unrelated values rather than neighbouring ones.
//!
//! # Nonclaims
//!
//! An address commits to the preimage its minting home wrote and to nothing
//! else. Whether that preimage is complete — whether two things the harness
//! considers different always encode differently — is the minting home's own
//! claim, stated where the preimage is written.

/// One kind's declared derivation domain.
///
/// The tag is a segment of the derive-key context, so two kinds derived over
/// identical preimages never share an address. Changing a kind's spelling
/// renames every address that kind ever derived, which is a profile version
/// bump and never an edit.
///
/// # Bounds
///
/// The declared grammar is lowercase ASCII letters, digits, and `-`. The
/// spelling is a compile-time literal written by the owning home, so the
/// grammar is a declaration discipline rather than a runtime check: there is no
/// road here that reads a tag from data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DomainTag(&'static str);

impl DomainTag {
    /// The tag its owning home declared.
    #[must_use]
    pub const fn declared(spelling: &'static str) -> Self {
        Self(spelling)
    }

    /// The declared spelling, as it appears in a derivation context.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.0
    }
}

/// One version of the harness identity profile: a position in that profile's
/// own order.
///
/// The version is a real segment of every derivation context, not a comment
/// about one. Changing what any preimage in this crate contains, what order its
/// members are written in, or what the context grammar spells is a bump, and a
/// bump renames every address the profile derives — which is exactly what it is
/// for.
///
/// There is no `Ord`: versions are not ranked, they are matched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfileVersion(u32);

impl IdentityProfileVersion {
    /// The version the profile's authority assigned.
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

/// The versioned, domain-separated profile a content address is derived under.
///
/// One derivation context per domain tag, spelled exactly:
///
/// ```text
/// <stem>/v<version>/<tag>
/// ```
///
/// with `<stem>` the profile's declared stem, `<version>` the decimal
/// [`IdentityProfileVersion::position`], and `<tag>` the
/// [`DomainTag::spelling`]. Every segment is lowercase ASCII letters, digits,
/// and `-`, joined by `/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfile {
    stem: &'static str,
    version: IdentityProfileVersion,
}

impl IdentityProfile {
    /// The profile at one stem and one version.
    #[must_use]
    pub const fn declared(stem: &'static str, version: IdentityProfileVersion) -> Self {
        Self { stem, version }
    }

    /// The declared stem — everything of the context ahead of the version.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        self.stem
    }

    /// The declared version.
    #[must_use]
    pub const fn version(self) -> IdentityProfileVersion {
        self.version
    }

    /// The derivation context for one domain tag, spelled by the grammar
    /// [`IdentityProfile`] states.
    #[must_use]
    pub fn context_for(self, tag: DomainTag) -> String {
        let stem = self.stem;
        let version = self.version.position();
        let tag = tag.spelling();
        format!("{stem}/v{version}/{tag}")
    }
}

/// The profile every content address in this crate is derived under.
///
/// # Bounds
///
/// The position moves when an address derived under it is held by somebody this
/// repository cannot edit. Nothing is: no preimage here reaches a published
/// artifact, a stored report, or a consumer, so a grammar that moves renames
/// names nobody holds and the position stands.
pub const HARNESS_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    "threadpak/testpak/harness-identity",
    IdentityProfileVersion::declared(1),
);

/// One thirty-two byte content address.
///
/// # Authority
///
/// Collision resistance is claimed AS BLAKE3's, over the preimage the minting
/// home wrote, under the [`DomainTag`] that home declared, and at the version
/// [`HARNESS_IDENTITY_PROFILE`] pins — and nothing broader. Finding two
/// different preimages under one tag that derive one address is as hard as
/// finding a BLAKE3 collision.
///
/// # Construction
///
/// [`ContentAddress::derived`] is the only road. No seam wraps arbitrary bytes,
/// so an address always came from a preimage under a tag, and the bytes handed
/// back are one-way by the absence of an inverse rather than by a runtime
/// check.
///
/// # Ordering
///
/// The order is byte-lexicographic over the derived bytes. It exists so
/// addresses can key an ordered map deterministically and carries no other
/// meaning: two addresses are not related because their bytes sort together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentAddress([u8; 32]);

impl ContentAddress {
    /// Derive the address of one preimage under one kind's domain tag.
    ///
    /// The bytes handed in are the PREIMAGE and the address is derived from
    /// them: an address is never a digest of source text, and a preimage is
    /// never "the id". Deterministic and total — every preimage names an
    /// address, on any machine, with no ambient fact anywhere in the
    /// derivation.
    #[must_use]
    pub fn derived(tag: DomainTag, preimage: &[u8]) -> Self {
        Self(blake3::derive_key(
            &HARNESS_IDENTITY_PROFILE.context_for(tag),
            preimage,
        ))
    }

    /// The address's thirty-two bytes, borrowed for comparison and for
    /// rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

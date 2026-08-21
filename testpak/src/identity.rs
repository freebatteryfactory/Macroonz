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
pub struct DomainTag {
    spelling: &'static str,
    version: IdentityProfileVersion,
}

impl DomainTag {
    /// The tag its owning home declared, at that family's own position.
    ///
    /// The position travels WITH the tag, because a position belongs to one
    /// preimage family and to no other. A tag declared here and a position
    /// declared somewhere else would be two facts that agree until one of them
    /// is moved, and moving the wrong one renames addresses under a grammar
    /// that never changed.
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

/// One position in ONE preimage family's own order.
///
/// The version is a real segment of every derivation context, not a comment
/// about one. It belongs to the family whose [`DomainTag`] it is declared
/// beside: a position under one tag and a position under another are two
/// orders, and moving one leaves every address under the other with its name.
///
/// # When a position moves
///
/// **Not when a grammar changes. When a grammar changes AND somebody holds an
/// address derived under the old one.** A position exists so a reader holding
/// two addresses of one family can assume both were derived the same way; where
/// no reader holds one, there is nothing for a second position to distinguish
/// and a move renames names nobody has.
///
/// So while no address under a family is held outside this repository —
/// persisted, published, promised, or reached by an adopter — a change to that
/// family's preimage is an edit to position one IN PLACE. The first externally
/// held address is what activates compatibility history for its family, and
/// from that point a change to what the family's transcript contains moves that
/// family's position and no other's.
///
/// There is no `Ord`: positions are not ranked, they are matched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// The domain-separated profile a content address is derived under: the one
/// stem every preimage family in this crate shares.
///
/// One derivation context per domain tag, spelled exactly:
///
/// ```text
/// <stem>/<tag>/v<version>
/// ```
///
/// with `<stem>` the profile's declared stem, `<tag>` the
/// [`DomainTag::spelling`], and `<version>` the decimal
/// [`IdentityProfileVersion::position`] THAT TAG carries. Every segment is
/// lowercase ASCII letters, digits, and `-`, joined by `/`.
///
/// # Authority
///
/// **The tag sits ahead of the version, so a position belongs to the family it
/// is written beside.** Position one of `trial-identity` and position one of
/// `row-revision` are two key spaces rather than one reached twice, and a family
/// that moves moves alone. A version segment ahead of the tag would read as the
/// stem's, which is the shape this crate carried when one number stood over
/// every family and a change to any preimage renamed them all.
///
/// The profile itself carries no version, because there is no fact left for one
/// to be about: what varies between two families is which family, and the tag
/// says that and carries its own order.
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

    /// The derivation context for one domain tag, spelled by the grammar
    /// [`IdentityProfile`] states.
    ///
    /// The version comes off the TAG, so a caller cannot assemble one family's
    /// context while naming another family's position.
    #[must_use]
    pub fn context_for(self, tag: DomainTag) -> String {
        let stem = self.stem;
        let family = tag.spelling();
        let version = tag.version().position();
        format!("{stem}/{family}/v{version}")
    }
}

/// The stem every content address in this crate is derived under.
///
/// One stem for every family, and the family segment beside it is what
/// separates them — never a stem a family chose for itself. Each family's
/// position rides its own [`DomainTag`], and the terms one moves on are stated
/// on [`IdentityProfileVersion`].
pub const HARNESS_IDENTITY_PROFILE: IdentityProfile =
    IdentityProfile::declared("threadpak/testpak/harness-identity");

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

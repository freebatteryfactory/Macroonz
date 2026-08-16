//! The byte plane's shared primitives: the one frame grammar, the domain-tag
//! register's shape, the digest-family law, width conventions, the text-form
//! scheme, commitment roles, content regions, and the bounded-reader maxima.
//! Role-specific frame *profiles* (`EventFrame`, image components, cursors,
//! receipts) are their owner homes' rows, citing these primitives.
//!
//! # Canonical encoding law
//!
//! One selected profile has one canonical byte encoding for one semantic value.
//! Canonical bytes never depend on hash-map or allocation iteration, pointer or
//! process identity, host endianness or padding, locale/timezone/debug
//! formatting, compiler layout, nondeterministic traversal, wall-clock timing,
//! or checkout paths. Every collection with canonical meaning has an explicit
//! order. A reader may reject noncanonical bytes but may not silently normalize
//! them into authority. Private Rust representation is noncanonical unless an
//! admitted profile explicitly says otherwise: raw memory, enum discriminants,
//! derive behavior, debug output, and serializer defaults never define durable
//! bytes. All multi-byte integers are uniform big-endian (network order); no
//! varints; Rust layout is never wire format.
//!
//! # The two-layer preimage model
//!
//! The logical preimage (algorithm-independent: the domain tag plus the
//! canonical fields — what the identity commits to) stays distinct from the
//! digest transcript (the algorithm-specific realization — a family may consume
//! the tag through its own domain-separation mode rather than as a literal
//! message prefix). Inspection names the registered context from the register
//! row either way: self-explanation is a register guarantee, not a hex-dump
//! guarantee. Every commitment declares its role, preimage version, exact
//! field order, length framing, optional-field and extension participation,
//! algorithm and output width, and the exact semantic claim it proves AND does
//! not prove — binding a preimage supports an integrity or identity claim; it
//! does not independently prove the underlying assertion true.
//!
//! # The digest-family law
//!
//! The digest family lives in the identity: the family id sits inside the
//! preimage's domain-separation tag, so every id names its family by
//! construction and no ambient "which algorithm?" question exists. Cross-family
//! comparison refuses at the scope guard; family migration is a named re-digest
//! morphism over the immutable store producing old→new receipts; generation
//! policy selects the families admissible for new writes; non-32-byte outputs
//! are distinct registered roles with their own tags, never ad-hoc truncations —
//! truncation changes the collision claim, so it must change the name. The
//! day-one family is blake3-256, admitted (hash = plain commitments, keyed hash =
//! keyed-when-protected under `KeyScope`, derive-key context = the domain-tag
//! register): its three modes map natively onto settled law, so the family
//! needed no bending to fit. Admission is not qualification and qualification is
//! not a support promise. The family stays swappable behind the machine-owned
//! digest role contract, which is why security-plane evidence widening the
//! roster changes a register row rather than this law.

use crate::identity::{ByteIdentity, CreationLaw, IdentityClass, IdentityRole};
use crate::refusal::{FamilyShape, RefusalFamily};
use crate::types::EvidenceRef;

/// The one magic for every binary `ThreadPak` artifact. The registered role
/// distinguishes kinds.
pub const FRAME_MAGIC: [u8; 4] = *b"TPAK";

/// Header bytes before the payload: magic 4 + role 2 + profile-version 2 +
/// flags 2 + length 4.
pub const FRAME_HEADER_BYTES: usize = 14;

/// Trailer bytes after the payload: the 32-byte frame digest.
pub const FRAME_TRAILER_BYTES: usize = 32;

/// A registered frame-role identity (u16, from the domain-tag register).
/// Unknown roles refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameRoleId(u16);

impl FrameRoleId {
    /// In-crate mint for laws. Test-gated until the register emitter exists.
    #[cfg(test)]
    pub(crate) const fn registered(id: u16) -> Self {
        Self(id)
    }

    /// The registered identity.
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// A registered digest-family identity (u16). The family id lives inside the
/// domain-separation tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigestFamilyId(u16);

impl DigestFamilyId {
    /// The registered identity.
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// The shared frame header — the grammar every binary role profiles over:
/// `magic | role u16 | profile-version u16 | flags u16 (reserved-zero) |
/// length u32 | payload | frame digest 32 B`. Each role keeps its own container
/// identity, version line, and recovery law; a payload codec never versions the
/// enclosing frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameHeader {
    /// The registered role.
    pub role: FrameRoleId,
    /// The role profile's version. Unknown versions refuse.
    pub profile_version: u16,
    /// Reserved-zero flags. Any nonzero reserved flag refuses.
    pub flags: u16,
    /// The physical bound: the payload byte length. Anything larger than this
    /// field can carry is a content region by construction — the tiering law
    /// expressed as a field width.
    pub length: u32,
}

/// The frame digest trailer. Family bound by the role profile's own
/// identity/generation law: store generation governs store-owned rows only; a
/// standalone image, receipt, or cursor binds its family through its own
/// profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameDigest(pub [u8; 32]);

/// The semantic bound checked BEFORE decode — distinct from the physical u32
/// length (the two-bound law). Authored v1 shape; profiles declare it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapacityProfile {
    /// The admitted payload bytes for this role under this profile.
    pub admitted_bytes: u32,
}

/// Frame decode refusal: dependent checks in declared order. Inside an
/// accepted-history frame a digest mismatch is classified under the recovery
/// law instead of refusing plainly — that classification is the history
/// home's.
#[must_use = "a decode refusal carries the lawful reason the frame was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameDecode {
    /// The role is not registered.
    UnknownRole,
    /// The role is known but the profile version is not.
    UnknownVersion,
    /// A reserved flag bit is nonzero.
    NonzeroReservedFlag,
    /// The frame digest does not verify.
    DigestMismatch,
}

impl RefusalFamily for FrameDecode {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "UnknownRole",
        "UnknownVersion",
        "NonzeroReservedFlag",
        "DigestMismatch",
    ];
}

/// One domain tag: `threadpak/<tag-version>/<family>/<role>/<schema-version>`.
/// Preimages are domain-separated so identical bytes in two roles cannot create
/// interchangeable claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DomainTag {
    /// The tag scheme version.
    pub tag_version: u16,
    /// The digest family.
    pub family: DigestFamilyId,
    /// The registered role.
    pub role: FrameRoleId,
    /// The bound schema version.
    pub schema_version: u64,
}

/// The four projections every domain-tag register row emits — all four derived
/// from ONE row, so wire id, human prefix, and hash domain cannot drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagProjection {
    /// The derive-key context string.
    DeriveKeyContext,
    /// The text-form prefix.
    TextFormPrefix,
    /// The frame role id.
    FrameRole,
    /// The docs table row.
    DocsTable,
}

/// The eight commitment roles — THE one neutral-inspection sum; other homes
/// reference it, never redefine it. Non-authoritative: authority lives in
/// separate typed evidence families with typed verifiers. These remain
/// different evidence and never substitute: a checksum does not authenticate; a
/// MAC is not publicly verifiable; a signature does not prove freshness;
/// inclusion does not prove completeness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommitmentRole {
    /// Corruption triage.
    Checksum,
    /// Exact-byte binding.
    ContentDigest,
    /// Domain-separated semantic preimage commitment.
    SemanticCommitment,
    /// Shared-key authenticity in one trust domain.
    Mac,
    /// Signer authenticity under one key policy.
    Signature,
    /// Membership in one committed structure.
    InclusionProof,
    /// A freshness witness.
    FreshnessWitness,
    /// A rollback anchor.
    RollbackAnchor,
}

/// The text-form scheme's refusal (dependent ladder): every identity with a
/// human rendering uses a closed role prefix + case-insensitive base32 payload
/// under a strict role-covering checksum whose domain includes the prefix — a
/// string minted for one role cannot validate as another (wrong-role refusal at
/// the copy-paste layer, before any parser runs). Mixed case refuses: a free
/// mangled-paste detector. The checker rides an admitted checksum mechanism
/// behind a machine-owned role contract.
#[must_use = "a decode refusal carries the lawful reason the text form was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextFormDecode {
    /// The prefix is not in the closed role-prefix register.
    PrefixUnknown,
    /// The string mixes cases.
    MixedCase,
    /// The role-covering checksum does not verify.
    ChecksumInvalid,
}

impl RefusalFamily for TextFormDecode {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] =
        &["PrefixUnknown", "MixedCase", "ChecksumInvalid"];
}

/// The identity role marker for content regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentRegionRole;

/// The identity of one immutable byte region — Class B: the identity IS the
/// digest of the exact bytes; it answers storage, addressing, and dedup
/// identity and only those. In the sealed-extent case it digests exact
/// ciphertext (revealing nothing about protected meaning) and never
/// substitutes for the keyed payload binding — same ciphertext is never proof
/// of same protected meaning. (The general unprotected case is this home's
/// extension of the sealed-extent row under the tiering law.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentRegionId(ByteIdentity<ContentRegionRole>);

impl ContentRegionId {
    /// In-crate mint for laws. Test-gated until digest derivation exists.
    #[cfg(test)]
    pub(crate) const fn of(digest: ByteIdentity<ContentRegionRole>) -> Self {
        Self(digest)
    }

    /// The region digest.
    #[must_use]
    pub fn digest(&self) -> &ByteIdentity<ContentRegionRole> {
        &self.0
    }
}

impl IdentityRole for ContentRegionId {
    const CLASS: IdentityClass = IdentityClass::ByteDigest;
    const CREATION: CreationLaw = CreationLaw::DigestOfExactBytes;
}

/// The claim marker for payload-binding references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PayloadBindingClaim;

/// A reference to payload bytes living as a content region: `{extent id,
/// length u64, binding ref}`. The binding reference points at the keyed
/// commitment answering the *meaning* question the region digest cannot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PayloadReference {
    /// The region holding the bytes.
    pub extent: ContentRegionId,
    /// The byte length.
    pub length: u64,
    /// The keyed payload-binding commitment reference.
    pub binding: EvidenceRef<PayloadBindingClaim>,
}

/// The sixteen declared decode maxima every durable or interoperable format
/// names — the roster is law; each format declares its values. Before each
/// allocation or traversal a reader checks every gating fact, and a failure
/// leaves nothing partially admitted, initialized, published, or advanced.
pub const DECODE_MAXIMA: [&str; 16] = [
    "total-length",
    "components",
    "fields",
    "collections",
    "nesting",
    "strings",
    "extents",
    "offsets",
    "decoded-expansion",
    "decompression",
    "imports",
    "definitions",
    "program-nodes",
    "results",
    "artifacts",
    "effects",
];

/// The standing width conventions, machine-readable: every register row obeys
/// them.
pub const WIDTH_CONVENTIONS: [(&str, &str); 8] = [
    ("digests", "32 bytes, family per the identity's domain tag"),
    (
        "order-scalars",
        "u64 with scope binding in the bytes, never bare",
    ),
    ("fresh-occurrence-ids", "16 bytes, no structure parsed"),
    ("registered-ids", "u16 from the domain-tag register"),
    ("enums", "registered u16, refuse on unknown"),
    (
        "vectors",
        "u32 count prefix + elements in declared canonical order",
    ),
    ("counts-and-lengths", "u32"),
    ("coordinates", "u64"),
];

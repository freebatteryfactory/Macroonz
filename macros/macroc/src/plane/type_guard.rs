//! The plane's invariant nucleus: every road that reaches a private field.
//!
//! This file is declared inside `types.rs` as its own child, so it sees the
//! fields the declarations keep private and nothing else in the crate does. That
//! is what makes the plane's walls structural rather than reviewed: there is no
//! public raw-byte constructor for either identity family, no way to re-wrap an
//! identity's bytes under another subject, and no road to a human projection
//! that truncates — because the roads that could do any of those would have to
//! be written here, and they are not.

use super::{
    GeneratorIdentity, GeneratorProfileId, GeneratorSchemaVersion, HumanProjection,
    IdentityProfile, IdentityProfileVersion, IdentitySubject, MACROC_GENERATOR, OwnerFactName,
    OwnerFactRef, OwnerIdentityRef, PROJECTION_IDENTITY_PROFILE, ProfileVersion,
    ProjectionIdentity, ProjectionProvenance, ProjectionRole, ProjectionTranscript, RefusalReason,
    RenderedRoleSeal, SubjectSeal, TranscriptAnchoring,
};
use core::marker::PhantomData;
use threadpak::identity::Commitment;
use threadpak::refusal::ReasonId;
use threadpak::types::{Bounded, BoundedConstruction, ConstLimit, Limit};

impl<Subject> OwnerIdentityRef<Subject> {
    /// The production road: project one machine commitment into the plane. The
    /// commitment's domain is the reference's subject, so a commitment over one
    /// domain cannot become a reference naming another. Nothing is adapted —
    /// the bytes cross unchanged.
    #[must_use]
    pub fn of_commitment(commitment: &Commitment<Subject>) -> Self {
        Self {
            bytes: *commitment.as_bytes(),
            _subject: PhantomData,
        }
    }

    /// The decode-route seam, awaiting a real decoder.
    ///
    /// It is crate-internal on purpose: an identity that arrived already in its
    /// declared byte order comes from an artifact somebody decoded, and the
    /// decoder that will own this route does not exist yet. Until it does, this
    /// is the single byte road in the plane and no caller outside the services
    /// can reach it. It mints nothing and admits nothing; the machine never
    /// accepts a plane reference as an identity mint.
    #[must_use]
    pub(crate) const fn decoded(bytes: [u8; 32]) -> Self {
        Self {
            bytes,
            _subject: PhantomData,
        }
    }

    /// The identity's declared raw-byte storage order, borrowed for comparison
    /// and for rendering.
    ///
    /// This is not a subject-erasing conversion. Reading the bytes out and
    /// re-wrapping them under a different subject is unrepresentable outside
    /// this crate because no public byte constructor exists to wrap them with —
    /// the accessor is one-way by the absence of its inverse, not by a runtime
    /// check.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl OwnerIdentityRef<RefusalReason> {
    /// Project one registered refusal reason into the plane. A diagnostic names
    /// the reason the machine registered; it never registers one.
    #[must_use]
    pub fn of_reason(reason: ReasonId) -> Self {
        Self::decoded(*reason.as_bytes())
    }
}

impl OwnerFactRef {
    /// Cite one owner fact by the declared names its home wrote down.
    #[must_use]
    pub const fn named(home: &'static str, fact: &'static str) -> Self {
        Self::Declared(OwnerFactName { home, fact })
    }
}

impl ProfileVersion {
    /// The version the profile's authority assigned.
    #[must_use]
    pub const fn declared(position: u64) -> Self {
        Self(position)
    }

    /// The assigned position.
    #[must_use]
    pub const fn position(self) -> u64 {
        self.0
    }
}

impl<L: ConstLimit> HumanProjection<L> {
    /// Render one bounded human projection.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the rendering exceeds the
    /// family's declared byte maximum. A projection that does not fit refuses
    /// rather than truncating: a silently cut explanation is a false one.
    pub fn projected(text: &str) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(text.as_bytes().to_vec()).map(|text| Self { text })
    }

    /// The seam behind [`human_projection!`], which is the only road to it.
    ///
    /// # There is no length to check here, so there is no branch to fall down
    ///
    /// The rendering arrives as a fixed-width byte array, and the width is the
    /// array's own TYPE. So this road carries no runtime count, returns no
    /// refusal, and has no branch where a rendering that did not fit becomes an
    /// empty one — the earlier seam had exactly that branch, and an oversized
    /// explanation silently became a blank one.
    ///
    /// The width cannot be chosen independently of the material either: the
    /// caller does not pass a length, it passes the array, and
    /// [`human_projection!`] builds that array in a `const` item out of the
    /// rendering itself. A rendering the width does not cover stops the
    /// compiler during that const evaluation.
    #[must_use]
    pub(crate) fn proven<const N: usize>(rendered: [u8; N]) -> Self {
        Self {
            text: Bounded::from_array(rendered),
        }
    }
}

/// One static rendering's bytes, at the fixed width the caller declared.
///
/// Written for the `const` item [`human_projection!`] builds. Evaluated at
/// compile time, where a width the rendering does not reach is a compile error
/// rather than a padded or cut projection handed to a reader.
#[expect(
    clippy::indexing_slicing,
    reason = "the walk is a const evaluation over the declared width, so an index past the rendering stops the compiler instead of reading at runtime"
)]
#[must_use]
pub(crate) const fn static_bytes<const N: usize>(text: &str) -> [u8; N] {
    let source = text.as_bytes();
    let mut rendered = [0u8; N];
    let mut at = 0usize;
    while at < N {
        rendered[at] = source[at];
        at = at.saturating_add(1);
    }
    rendered
}

/// Projects one STATIC rendering, proving at COMPILE TIME that it fits the named
/// limit family.
///
/// This is the total road, and it is the only road to
/// [`HumanProjection::proven`]. `HumanProjection::projected` reads a runtime
/// length and may refuse, and a caller that swallowed that refusal with an empty
/// fallback would be silently deleting an explanation — which is exactly the
/// defect this macro exists to make unrepresentable. Where the material is
/// static, the length is a compile-time fact: the `const` block below settles
/// the bound, the `const` item below carries the rendering at its own width, and
/// no refusal road appears anywhere between them.
macro_rules! human_projection {
    ($limit:ty, $text:literal) => {{
        const RENDERED: [u8; $text.len()] = $crate::plane::static_bytes($text);
        const {
            ::core::assert!(
                $text.len() <= <$limit as ::threadpak::types::ConstLimit>::MAX,
                "a static human projection longer than its limit family admits",
            );
        }
        $crate::plane::HumanProjection::<$limit>::proven(RENDERED)
    }};
}

pub(crate) use human_projection;

impl<L: Limit> HumanProjection<L> {
    /// The empty rendering. Total: nothing exceeds any bound, and a caller with
    /// nothing to say for a person still owes a value rather than a hole.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            text: Bounded::empty(),
        }
    }

    /// The rendering's byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Whether the rendering carries no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// The rendering, for a caller to SHOW a person.
    ///
    /// This is the one lawful use of the bytes and it is a one-way road out of
    /// the plane. Nothing inside the plane calls it: no decision, no identity,
    /// and no refusal consults a human projection, and none ever will. A
    /// frontend that must put a sentence in front of somebody calls this, and
    /// that is what the type exists for.
    #[must_use]
    pub fn shown(&self) -> String {
        let bytes: Vec<u8> = self.text.iter().copied().collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

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

    /// The derive-key context for one subject under one role, spelled by the
    /// grammar above.
    #[must_use]
    pub fn context_for(self, subject: &str, role: ProjectionRole) -> String {
        let version = self.version.position();
        let role = role.context_name();
        format!("{}/v{version}/{subject}/{role}", self.stem)
    }
}

impl GeneratorProfileId {
    /// The generator under its declared stable name.
    #[must_use]
    pub const fn declared(spelling: &'static str) -> Self {
        Self(spelling)
    }

    /// The declared name.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.0
    }
}

impl GeneratorSchemaVersion {
    /// The schema version the generator's authority assigned.
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

impl GeneratorIdentity {
    /// The generator under its declared name, rendered shape, and recorded
    /// package version.
    #[must_use]
    pub const fn declared(
        profile: GeneratorProfileId,
        schema: GeneratorSchemaVersion,
        package: &'static str,
    ) -> Self {
        Self {
            profile,
            schema,
            package,
        }
    }

    /// The generator's stable name. Load-bearing: it is in every transcript.
    #[must_use]
    pub const fn profile(self) -> GeneratorProfileId {
        self.profile
    }

    /// The rendered shape's version. Load-bearing: it is in every transcript.
    #[must_use]
    pub const fn schema(self) -> GeneratorSchemaVersion {
        self.schema
    }

    /// The package version, recorded for a reader and load-bearing nowhere.
    #[must_use]
    pub const fn package_version(self) -> &'static str {
        self.package
    }
}

impl<'material> ProjectionTranscript<'material> {
    /// Derive under an identity the MACHINE minted.
    #[must_use]
    pub fn under_owner<Subject>(
        role: ProjectionRole,
        anchor: &OwnerIdentityRef<Subject>,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self::anchored(
            role,
            TranscriptAnchoring::UnderOwnerIdentity(*anchor.as_bytes()),
            content,
            position,
        )
    }

    /// Derive under another identity the PLANE owns.
    #[must_use]
    pub fn under_projection<Subject>(
        role: ProjectionRole,
        anchor: &ProjectionIdentity<Subject>,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self::anchored(
            role,
            TranscriptAnchoring::UnderProjectionIdentity(*anchor.as_bytes()),
            content,
            position,
        )
    }

    /// Derive under no anchor at all — the root of one derivation chain.
    #[must_use]
    pub fn rooted(role: ProjectionRole, content: &'material [u8], position: u32) -> Self {
        Self::anchored(role, TranscriptAnchoring::Rooted, content, position)
    }

    /// Derive under an anchoring the caller already decided.
    ///
    /// The road for a mint site whose anchor depends on a typed posture rather
    /// than on which of two identity families it holds — a plan hangs off
    /// whatever caused it, and what caused it is a sum type.
    #[must_use]
    pub fn under(
        role: ProjectionRole,
        anchoring: TranscriptAnchoring,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self::anchored(role, anchoring, content, position)
    }

    /// The shared constructor: every transcript names the one declared profile
    /// and the one declared generator, so neither can be varied per call site.
    #[must_use]
    fn anchored(
        role: ProjectionRole,
        anchoring: TranscriptAnchoring,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self {
            profile: PROJECTION_IDENTITY_PROFILE,
            generator: MACROC_GENERATOR,
            role,
            anchoring,
            content,
            position,
        }
    }

    /// The profile this transcript is written under.
    #[must_use]
    pub const fn profile(&self) -> IdentityProfile {
        self.profile
    }

    /// The generator this transcript names.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// The role this transcript stands for.
    #[must_use]
    pub const fn role(&self) -> ProjectionRole {
        self.role
    }

    /// What this transcript is anchored under.
    #[must_use]
    pub const fn anchoring(&self) -> TranscriptAnchoring {
        self.anchoring
    }

    /// The varying material, at full length.
    #[must_use]
    pub const fn content(&self) -> &'material [u8] {
        self.content
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The derivation record this transcript leaves for one identity subject.
    #[must_use]
    pub fn provenance(&self, subject: &'static str) -> ProjectionProvenance {
        ProjectionProvenance {
            subject,
            role: self.role,
            profile: self.profile,
            generator: self.generator,
            anchoring: self.anchoring,
            content_length: u64::try_from(self.content.len()).unwrap_or(u64::MAX),
            position: self.position,
        }
    }
}

impl ProjectionProvenance {
    /// The identity subject this derivation named.
    #[must_use]
    pub const fn subject(&self) -> &'static str {
        self.subject
    }

    /// The role it stood for.
    #[must_use]
    pub const fn role(&self) -> ProjectionRole {
        self.role
    }

    /// The profile and version it was derived under.
    #[must_use]
    pub const fn profile(&self) -> IdentityProfile {
        self.profile
    }

    /// The generator that derived it.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// What it was anchored under, anchor commitment included.
    #[must_use]
    pub const fn anchoring(&self) -> TranscriptAnchoring {
        self.anchoring
    }

    /// How many bytes of content went into the transcript.
    #[must_use]
    pub const fn content_length(&self) -> u64 {
        self.content_length
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The derive-key context this derivation ran under, rendered by the domain
    /// grammar.
    #[must_use]
    pub fn context(&self) -> String {
        self.profile.context_for(self.subject, self.role)
    }
}

impl<Subject: IdentitySubject> ProjectionIdentity<Subject> {
    /// Derive one plane identity from its complete transcript. Deterministic
    /// and total: every transcript names an identity.
    #[must_use]
    pub fn derived(transcript: ProjectionTranscript<'_>) -> Self {
        let context = transcript
            .profile()
            .context_for(Subject::SUBJECT_NAME, transcript.role());
        Self {
            bytes: blake3::derive_key(&context, &transcript.encoded(Subject::SUBJECT_NAME)),
            _subject: PhantomData,
        }
    }

    /// Derive one plane identity and the record of how it was derived.
    ///
    /// The record is for whoever is going to keep it. Three values keep theirs:
    /// a plan, a proved closure, and a closed expansion — the three whose
    /// identity a reader is most likely to be handed on its own and asked to
    /// account for. A caller with nowhere to put one takes
    /// [`ProjectionIdentity::derived`] instead, and the record is simply not
    /// made rather than made and carried by everything.
    #[must_use]
    pub fn derived_with_provenance(
        transcript: ProjectionTranscript<'_>,
    ) -> (Self, ProjectionProvenance) {
        (
            Self::derived(transcript),
            transcript.provenance(Subject::SUBJECT_NAME),
        )
    }
}

impl<Subject> ProjectionIdentity<Subject> {
    /// The identity's thirty-two bytes, borrowed for comparison and for
    /// rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl RenderedRoleSeal {
    /// The seal, admitted only within the services.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
}

impl SubjectSeal {
    /// The seal, admitted only within the services. The `subjects!` declaration
    /// is the one caller, which is what makes the subject roster closed.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
}

/// One plane identity minted for the proof surface alone.
///
/// Test-gated on purpose. The laws need distinguishable identities without
/// having a captured declaration to derive them from, and this road exists
/// nowhere else: a production caller derives from a real transcript or has no
/// identity at all.
#[cfg(test)]
pub(crate) fn for_laws<Subject: IdentitySubject>(tag: u8) -> ProjectionIdentity<Subject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        &[tag],
        u32::from(tag),
    ))
}

//! The plane's invariant nucleus: every road that reaches a private field.
//!
//! This file is declared inside `types.rs` as its own child, so it sees the
//! fields the declarations keep private and nothing else in the crate does.
//! That is what makes the plane's walls structural rather than reviewed: a road
//! around one of them would have to be written here, and none is.

use super::{
    AuthoringLimitProfile, GeneratorIdentity, GeneratorProfileId, GeneratorSchemaVersion,
    HumanProjection, IDENTITY_PROFILE_STEM, IdentityProfile, IdentityProfileVersion,
    IdentitySubject, MACROC_GENERATOR, OwnerFactName, OwnerFactRef, OwnerIdentityRef,
    PreimageFamily, ProfileVersion, ProjectionIdentity, ProjectionProvenance, ProjectionRole,
    ProjectionTranscript, RefusalReason, RenderedRoleSeal, SubjectSeal, TranscriptAnchoring,
};
use core::marker::PhantomData;
use macroonz::{
    AdmittedLimit, Bounded, BoundedConstruction, Commitment, ConstLimit, Limit, ReasonId,
};

impl<Subject> OwnerIdentityRef<Subject> {
    /// The production road: project one machine commitment into the plane.
    ///
    /// The commitment's domain is the reference's subject, so a commitment over
    /// one domain cannot become a reference naming another.
    /// Nothing is adapted — the bytes cross unchanged.
    #[must_use]
    pub fn of_commitment(commitment: &Commitment<Subject>) -> Self {
        Self {
            bytes: *commitment.as_bytes(),
            _subject: PhantomData,
        }
    }

    /// The plane's single byte road: one identity that arrived already in its
    /// declared byte order.
    ///
    /// It mints nothing and admits nothing — the machine never accepts a plane
    /// reference as an identity mint — and it is crate-internal until a decoder
    /// owns the route.
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
    /// One-way by the absence of its inverse rather than by a runtime check:
    /// no public byte constructor exists to re-wrap what it hands back.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl OwnerIdentityRef<RefusalReason> {
    /// Project one registered refusal reason into the plane.
    /// A diagnostic names the reason the machine registered; it never registers
    /// one.
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
    /// family's declared byte maximum.
    /// A projection that does not fit refuses rather than truncating: a silently
    /// cut explanation is a false one.
    pub fn projected(text: &str) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(
            text.as_bytes().to_vec(),
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|text| Self { text })
    }

    /// The seam behind [`human_projection!`], which is the only road to it.
    ///
    /// The rendering arrives as a fixed-width byte array, and the width is the
    /// array's own TYPE, so this road carries no runtime count, returns no
    /// refusal, and has no branch where a rendering that did not fit becomes an
    /// empty one.
    /// The width cannot be chosen independently of the material either: the
    /// caller passes the array rather than a length, [`human_projection!`]
    /// builds that array in a `const` item out of the rendering itself, and a
    /// rendering the width does not cover stops the compiler during that const
    /// evaluation.
    #[must_use]
    pub(crate) fn proven<const N: usize>(rendered: [u8; N]) -> Self {
        Self {
            text: Bounded::from_array(rendered),
        }
    }
}

/// One static rendering's bytes, at the fixed width the caller declared.
///
/// Written for the `const` item [`human_projection!`] builds, and evaluated at
/// compile time: a width the rendering does not reach is a compile error rather
/// than a padded or cut projection handed to a reader.
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
/// This is the total road, and the only road to [`HumanProjection::proven`].
/// [`HumanProjection::projected`] reads a runtime length and may refuse, and a
/// caller that swallowed that refusal with an empty fallback would be silently
/// deleting an explanation.
/// Where the material is static, the length is a compile-time fact instead: the
/// `const` block settles the bound, the `const` item carries the rendering at
/// its own width, and no refusal road appears anywhere between them.
macro_rules! human_projection {
    ($limit:ty, $text:literal) => {{
        const RENDERED: [u8; $text.len()] = $crate::plane::static_bytes($text);
        const {
            ::core::assert!(
                $text.len() <= <$limit as ::macroonz::ConstLimit>::MAX,
                "a static human projection longer than its limit family admits",
            );
        }
        $crate::plane::HumanProjection::<$limit>::proven(RENDERED)
    }};
}

pub(crate) use human_projection;

impl<L: Limit> HumanProjection<L> {
    /// The empty rendering.
    /// Total: nothing exceeds any bound, and a caller with nothing to say for a
    /// person still owes a value rather than a hole.
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
    /// The one lawful use of the bytes, and a one-way road out of the plane:
    /// a frontend that must put a sentence in front of somebody calls this, and
    /// nothing inside the plane calls it at all.
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
    /// One preimage family's profile at one version.
    ///
    /// The family arrives where a stem used to, and that is the whole of the
    /// collision argument: a stem is a literal a declaration could repeat, and a
    /// family is a row of a closed roster that cannot be.
    #[must_use]
    pub const fn declared(family: PreimageFamily, version: IdentityProfileVersion) -> Self {
        Self { family, version }
    }

    /// The preimage family this profile versions.
    #[must_use]
    pub const fn family(self) -> PreimageFamily {
        self.family
    }

    /// The declared version — this family's own position, and no other
    /// family's.
    #[must_use]
    pub const fn version(self) -> IdentityProfileVersion {
        self.version
    }

    /// The derive-key context one SUBJECT derives under, at one role, spelled by
    /// the domain grammar [`IdentityProfile`] states.
    ///
    /// The family segment sits ahead of the version, so one family's position
    /// one and another's are two key spaces and a bump under either reaches
    /// nothing under the other.
    ///
    /// # Authority
    ///
    /// **The subject is the TYPE's and never an argument.** A road that took the
    /// subject as text would let a caller derive under any name space it could
    /// spell — including one another subject already occupies — and the typed
    /// identity above it would be a promise the encoder never had to keep. Here
    /// the subject arrives as a parameter of the sealed [`IdentitySubject`]
    /// roster, so asking for the wrong name space is unwritable rather than
    /// discouraged.
    #[must_use]
    pub fn context_for<Subject: IdentitySubject>(self, role: ProjectionRole) -> String {
        self.context_over(Subject::SUBJECT_NAME, role)
    }

    /// The same context, over a subject NAME a derivation record already
    /// carries.
    ///
    /// Crate-internal, with one caller: [`ProjectionProvenance::context`], which
    /// renders what a derivation recorded rather than performing one. The record
    /// stores the declared name because that is what it observed; nothing here
    /// derives anything, and no public road reaches this one.
    pub(crate) fn context_over(self, subject: &str, role: ProjectionRole) -> String {
        let family = self.family.stable_name();
        let version = self.version.position();
        let role = role.stable_name();
        format!("{IDENTITY_PROFILE_STEM}/{family}/v{version}/{subject}/{role}")
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

    /// The generator's stable name. One of the two facts a staleness comparison
    /// reads.
    #[must_use]
    pub const fn profile(self) -> GeneratorProfileId {
        self.profile
    }

    /// The rendered shape's version. The other fact a staleness comparison
    /// reads.
    #[must_use]
    pub const fn schema(self) -> GeneratorSchemaVersion {
        self.schema
    }

    /// The package version, recorded for a reader and compared by nothing.
    #[must_use]
    pub const fn package_version(self) -> &'static str {
        self.package
    }

    /// Whether two generator identities name the same generator rendering the
    /// same shape.
    ///
    /// The comparison a staleness reading wants, and the reason it is a named
    /// road rather than `==`: equality compares the package version too, and the
    /// package version moves for reasons no output noticed. A reader asking
    /// "would this generator render what I am holding?" is asking about the
    /// declared name and the schema position, which are the two facts that
    /// answer it.
    #[must_use]
    pub fn same_rendered_shape(self, other: Self) -> bool {
        self.profile == other.profile && self.schema == other.schema
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

    /// The shared constructor: the profile is READ OFF the role's own preimage
    /// family and the generator is the one declared generator, so neither can be
    /// varied per call site.
    ///
    /// A mint site names a role and gets the family's ladder with it. Nothing
    /// here takes a profile, so a rendered unit cannot be derived under the plan
    /// family's version by a caller that passed the wrong constant, and a family
    /// added to the roster reaches every mint site of its role at once.
    #[must_use]
    fn anchored(
        role: ProjectionRole,
        anchoring: TranscriptAnchoring,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self {
            profile: role.family().profile(),
            generator: MACROC_GENERATOR,
            role,
            anchoring,
            content,
            position,
        }
    }

    /// The family profile this transcript is written under.
    #[must_use]
    pub const fn profile(&self) -> IdentityProfile {
        self.profile
    }

    /// The generator this transcript records.
    ///
    /// Recorded and never written: [`ProjectionTranscript::encoded`] carries no
    /// member for it, and this road exists so the derivation record can.
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
    ///
    /// # Authority
    ///
    /// The subject is the TYPE's, on exactly
    /// [`IdentityProfile::context_for`]'s terms: a record that could be handed a
    /// subject name would be a record of a derivation that did not happen.
    #[must_use]
    pub fn provenance<Subject: IdentitySubject>(&self) -> ProjectionProvenance {
        ProjectionProvenance {
            subject: Subject::SUBJECT_NAME,
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

    /// The family's profile and version it was derived under.
    #[must_use]
    pub const fn profile(&self) -> IdentityProfile {
        self.profile
    }

    /// The generator that derived it.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// Whether this derivation was recorded under the generator and rendered
    /// shape these services declare today.
    ///
    /// The one reading the generator is FOR, now that it names nothing: a reader
    /// holding an old record asks whether the producer has moved, and gets an
    /// answer about the producer alone.
    ///
    /// # Nonclaims
    ///
    /// A `false` here says a different generator shape produced the record. It
    /// says nothing about whether the material moved, whether the identity would
    /// re-derive the same, or whether anything needs redoing: the identity is a
    /// fact about the preimage, and the preimage does not contain this.
    #[must_use]
    pub fn under_current_shape(&self) -> bool {
        self.generator.same_rendered_shape(MACROC_GENERATOR)
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
        self.profile.context_over(self.subject, self.role)
    }
}

impl<Subject: IdentitySubject> ProjectionIdentity<Subject> {
    /// Derive one plane identity from its complete transcript.
    /// Deterministic and total: every transcript names an identity.
    #[must_use]
    pub fn derived(transcript: ProjectionTranscript<'_>) -> Self {
        let context = transcript
            .profile()
            .context_for::<Subject>(transcript.role());
        Self {
            bytes: blake3::derive_key(&context, &transcript.encoded::<Subject>()),
            _subject: PhantomData,
        }
    }

    /// Derive one plane identity and the record of how it was derived.
    ///
    /// The record is for a value that is going to keep it — one whose identity a
    /// reader may be handed on its own and asked to account for.
    /// A caller with nowhere to put one takes [`ProjectionIdentity::derived`]
    /// instead, and the record is simply not made rather than made and carried
    /// by everything.
    #[must_use]
    pub fn derived_with_provenance(
        transcript: ProjectionTranscript<'_>,
    ) -> (Self, ProjectionProvenance) {
        (
            Self::derived(transcript),
            transcript.provenance::<Subject>(),
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
    /// The seal, admitted only within the services; the `subjects!` declaration
    /// is the one caller.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
}

/// Whether a roster of declared names really separates: every name inside the
/// context grammar, and no name declared twice.
///
/// Written for the `const` block [`subjects!`] emits, and evaluated at compile
/// time for the same reason the projection width is — a name that would collapse
/// two derive-key name spaces is a compile error rather than a defect a reader
/// has to notice.
///
/// The grammar is the closed one [`IdentitySubject::SUBJECT_NAME`] declares:
/// lowercase ASCII letters and digits in `-`-joined segments, with no leading,
/// trailing, or doubled separator.
#[expect(
    clippy::indexing_slicing,
    reason = "the walk is a const evaluation over the declared roster, so an index past it stops the compiler instead of reading at runtime"
)]
#[must_use]
pub(crate) const fn names_are_separating(names: &[&str]) -> bool {
    let mut at = 0usize;
    while at < names.len() {
        if !name_is_grammatical(names[at]) {
            return false;
        }
        let mut earlier = 0usize;
        while earlier < at {
            if same_bytes(names[earlier].as_bytes(), names[at].as_bytes()) {
                return false;
            }
            earlier = earlier.saturating_add(1);
        }
        at = at.saturating_add(1);
    }
    true
}

/// Whether one declared name stands inside the closed context grammar.
#[expect(
    clippy::indexing_slicing,
    reason = "the walk is a const evaluation over one declared name, so an index past it stops the compiler instead of reading at runtime"
)]
const fn name_is_grammatical(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let last = bytes.len().saturating_sub(1);
    let mut at = 0usize;
    while at < bytes.len() {
        let byte = bytes[at];
        if byte == b'-' {
            // No leading, trailing, or doubled separator.
            if at == 0 || at == last || bytes[at.saturating_sub(1)] == b'-' {
                return false;
            }
        } else if !byte.is_ascii_lowercase() && !byte.is_ascii_digit() {
            return false;
        }
        at = at.saturating_add(1);
    }
    true
}

/// Whether two declared names are the same bytes.
#[expect(
    clippy::indexing_slicing,
    reason = "the walk is a const evaluation over two declared names, so an index past either stops the compiler instead of reading at runtime"
)]
const fn same_bytes(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut at = 0usize;
    while at < left.len() {
        if left[at] != right[at] {
            return false;
        }
        at = at.saturating_add(1);
    }
    true
}

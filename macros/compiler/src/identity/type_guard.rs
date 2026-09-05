//! The identity home's invariant nucleus: every road that reaches a private field.
//!
//! This file is declared inside `types.rs` as its own child, so it sees the fields the declarations keep private and nothing else in the crate does.
//! That is what makes the walls structural rather than reviewed: a road around one of them would have to be written here, and none is.

use super::{
    Anchoring, GENERATOR, GeneratorIdentity, HumanProjection, Identity, OwnerIdentity, Profile,
    Provenance, Role, ShapeVersion, Subject, Transcript, Version,
};
use crate::bounded::{Bounded, Overflow};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

impl<S: Subject> Identity<S> {
    /// Derive one identity from its complete transcript.
    /// Deterministic and total: every transcript names an identity.
    #[must_use]
    pub fn derived(transcript: Transcript<'_>) -> Self {
        let context = transcript.profile().context_for::<S>(transcript.role());
        Self(
            blake3::derive_key(&context, &transcript.encoded::<S>()),
            PhantomData,
        )
    }

    /// Derive one identity and the record of how it was derived.
    ///
    /// The record is for a value that is going to keep it — one whose identity a reader may be handed on its own and asked to account for.
    /// A caller with nowhere to put one takes [`Identity::derived`], and the record is simply not made rather than made and carried by everything.
    #[must_use]
    pub fn derived_with_provenance(transcript: Transcript<'_>) -> (Self, Provenance) {
        (Self::derived(transcript), transcript.provenance::<S>())
    }

    /// The identity's thirty-two bytes, borrowed for comparison and for rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl<S: Subject> PartialEq for Identity<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: Subject> Eq for Identity<S> {}

impl<S: Subject> Hash for Identity<S> {
    fn hash<H: Hasher>(&self, into: &mut H) {
        self.0.hash(into);
    }
}

impl<S: Subject> fmt::Debug for Identity<S> {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        into.debug_tuple(S::NAME).field(&self.0).finish()
    }
}

impl Version {
    /// The position the grammar's owner assigned.
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

impl Profile {
    /// One grammar, under the stem of whoever owns it, at one version.
    #[must_use]
    pub const fn declared(stem: &'static str, name: &'static str, version: Version) -> Self {
        Self {
            stem,
            name,
            version,
        }
    }

    /// The stem of whoever owns this grammar.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        self.stem
    }

    /// The grammar's declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The declared version — this grammar's own position, and no other grammar's.
    #[must_use]
    pub const fn version(self) -> Version {
        self.version
    }

    /// The derive-key context one SUBJECT derives under, at one role.
    ///
    /// # Authority
    ///
    /// **The subject is the TYPE's and never an argument.** A road that took the subject as text would let a caller derive under any key space it could spell — including one another subject already occupies — and the typed identity above it would be a promise the encoder never had to keep.
    #[must_use]
    pub fn context_for<S: Subject>(self, role: Role) -> String {
        self.context_over(S::STEM, S::NAME, role)
    }

    /// The same context, over a subject a derivation record already carries.
    ///
    /// Crate-internal, with one caller: [`Provenance::context`], which renders what a derivation recorded rather than performing one.
    pub(crate) fn context_over(self, subject_stem: &str, subject: &str, role: Role) -> String {
        let Self {
            stem,
            name,
            version,
        } = self;
        let position = version.position();
        let seat = role.name();
        format!("{stem}/{name}/v{position}/{subject_stem}/{subject}/{seat}")
    }
}

impl ShapeVersion {
    /// The shape position the generator's owner assigned.
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
    /// The generator under its declared name, rendered shape, and recorded package version.
    #[must_use]
    pub const fn declared(name: &'static str, shape: ShapeVersion, package: &'static str) -> Self {
        Self {
            name,
            shape,
            package,
        }
    }

    /// The generator's stable name. One of the two facts a staleness comparison reads.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The rendered shape's version. The other fact a staleness comparison reads.
    #[must_use]
    pub const fn shape(self) -> ShapeVersion {
        self.shape
    }

    /// The package version, recorded for a reader and compared by nothing.
    #[must_use]
    pub const fn package_version(self) -> &'static str {
        self.package
    }

    /// Whether two generator identities name the same generator rendering the same shape.
    ///
    /// The comparison a staleness reading wants, and the reason it is a named road rather than `==`: equality compares the package version too, and the package version moves for reasons no output noticed.
    #[must_use]
    pub fn same_shape(self, other: Self) -> bool {
        self.name == other.name && self.shape == other.shape
    }
}

impl<'material> Transcript<'material> {
    /// Write a transcript under a grammar the caller names.
    ///
    /// The road for a preimage whose grammar is the caller's own; the four roads below name a role and take this compiler's grammar for it.
    #[must_use]
    pub const fn under_profile(
        profile: Profile,
        role: Role,
        anchoring: Anchoring,
        material: &'material [u8],
        position: u32,
    ) -> Self {
        Self {
            profile,
            generator: GENERATOR,
            role,
            anchoring,
            material,
            position,
        }
    }

    /// Write a transcript under an anchoring the caller already decided.
    ///
    /// The road for a mint site whose anchor depends on a typed posture rather than on which family of identity it holds — a plan hangs off whatever caused it, and what caused it is a sum type.
    #[must_use]
    pub const fn under(
        role: Role,
        anchoring: Anchoring,
        material: &'material [u8],
        position: u32,
    ) -> Self {
        Self::under_profile(role.profile(), role, anchoring, material, position)
    }

    /// Derive under no anchor at all — the root of one derivation chain.
    #[must_use]
    pub const fn rooted(role: Role, material: &'material [u8], position: u32) -> Self {
        Self::under(role, Anchoring::Rooted, material, position)
    }

    /// Derive under an identity a CONSUMER minted.
    #[must_use]
    pub const fn under_owner(
        role: Role,
        anchor: &OwnerIdentity,
        material: &'material [u8],
        position: u32,
    ) -> Self {
        Self::under(
            role,
            Anchoring::UnderOwner(anchor.bytes),
            material,
            position,
        )
    }

    /// Derive under another identity this compiler derived.
    #[must_use]
    pub fn under_projection<S: Subject>(
        role: Role,
        anchor: &Identity<S>,
        material: &'material [u8],
        position: u32,
    ) -> Self {
        Self::under(
            role,
            Anchoring::UnderProjection(*anchor.as_bytes()),
            material,
            position,
        )
    }

    /// The grammar this transcript is written under.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// The generator this transcript records.
    ///
    /// Recorded and never written: the encoding carries no member for it, and this road exists so the derivation record can.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// The seat this transcript stands in.
    #[must_use]
    pub const fn role(&self) -> Role {
        self.role
    }

    /// What this transcript hangs off.
    #[must_use]
    pub const fn anchoring(&self) -> Anchoring {
        self.anchoring
    }

    /// The varying material, at full length.
    #[must_use]
    pub const fn material(&self) -> &'material [u8] {
        self.material
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The derivation record this transcript leaves for one identity subject.
    #[must_use]
    pub fn provenance<S: Subject>(&self) -> Provenance {
        Provenance {
            subject_stem: S::STEM,
            subject: S::NAME,
            role: self.role,
            profile: self.profile,
            generator: self.generator,
            anchoring: self.anchoring,
            material_length: u64::try_from(self.material.len()).unwrap_or(u64::MAX),
            position: self.position,
        }
    }
}

impl Provenance {
    /// The stem of whoever owns the subject this derivation named.
    #[must_use]
    pub const fn subject_stem(&self) -> &'static str {
        self.subject_stem
    }

    /// The identity subject this derivation named.
    #[must_use]
    pub const fn subject(&self) -> &'static str {
        self.subject
    }

    /// The seat it stood in.
    #[must_use]
    pub const fn role(&self) -> Role {
        self.role
    }

    /// The grammar and version it was derived under.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// The generator that derived it.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// What it hung off, anchor included.
    #[must_use]
    pub const fn anchoring(&self) -> Anchoring {
        self.anchoring
    }

    /// How many bytes of material went into the transcript.
    #[must_use]
    pub const fn material_length(&self) -> u64 {
        self.material_length
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The derive-key context this derivation ran under.
    #[must_use]
    pub fn context(&self) -> String {
        self.profile
            .context_over(self.subject_stem, self.subject, self.role)
    }

    /// Whether this derivation was recorded under the generator and rendered shape this crate declares today.
    ///
    /// # Nonclaims
    ///
    /// A `false` says a different generator shape produced the record.
    /// It says nothing about whether the material moved, whether the identity would re-derive the same, or whether anything needs redoing.
    #[must_use]
    pub fn under_current_shape(&self) -> bool {
        self.generator.same_shape(GENERATOR)
    }
}

impl HumanProjection {
    /// Render one bounded human projection.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] when the rendering runs past [`HUMAN_TEXT_LIMIT`](super::HUMAN_TEXT_LIMIT).
    /// A projection that does not fit refuses rather than truncating: a silently cut explanation is a false one.
    pub fn projected(text: &str) -> Result<Self, Overflow> {
        Bounded::new(text.as_bytes().to_vec()).map(Self)
    }

    /// The seam behind [`human_projection!`](crate::identity::human_projection), which is the only road to it.
    ///
    /// The rendering arrives as a fixed-width array, and the width is the array's own TYPE, so this road carries no runtime count, returns no refusal, and has no branch where a rendering that did not fit becomes an empty one.
    #[must_use]
    pub(crate) fn proven<const N: usize>(rendered: [u8; N]) -> Self {
        Self(Bounded::from_array(rendered))
    }

    /// The empty rendering.
    /// Total: a caller with nothing to say for a person still owes a value rather than a hole.
    #[must_use]
    pub fn empty() -> Self {
        Self(Bounded::empty())
    }

    /// The rendering's byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the rendering carries no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The rendering, for a caller to SHOW a person.
    ///
    /// The one lawful use of the bytes and a one-way road out: a frontend that must put a sentence in front of somebody calls this, and nothing inside the compiler calls it at all.
    #[must_use]
    pub fn shown(&self) -> String {
        String::from_utf8_lossy(self.0.as_slice()).into_owned()
    }
}

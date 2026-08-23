//! The annex's invariant nucleus: every road that reaches a private field.
//!
//! This file is declared inside `types.rs` as its own child, so it sees the
//! fields the declarations keep private and no sibling module does. Two laws
//! are held here and nowhere else: a golden vector exists only by being read
//! out of a pack, so no road admits one exported from a producer; and a
//! transcript preimage exists only by being composed here, member by member,
//! from typed arguments a caller took off a published specification.

use super::{
    ByteDifference, ContextRefusal, DerivedIdentity, SpecifiedContext, TranscriptDerivation,
    TranscriptDisagreement, TranscriptMember, TranscriptVerdict, VECTOR_PACK_MAGIC,
    VECTOR_PACK_VERSION, VectorDisagreement, VectorEntry, VectorPack, VectorPackRefusal,
    VectorSubject, VectorVerdict,
};

/// The width of every number a pack or a preimage frames, in bytes.
///
/// One width for every number rather than one per member: a framing that
/// admitted two spellings of one length would admit two encodings of one value.
const WIDE_WIDTH: usize = 8;

// ---------------------------------------------------------------------------
// The golden-vector lane.
// ---------------------------------------------------------------------------

impl<'pack> VectorSubject<'pack> {
    /// The owner that declares the subject.
    #[must_use]
    pub const fn namespace(self) -> &'pack str {
        self.namespace
    }

    /// The subject's own spelling inside that owner.
    #[must_use]
    pub const fn stem(self) -> &'pack str {
        self.stem
    }
}

impl<'pack> VectorEntry<'pack> {
    /// The subject this vector is about.
    #[must_use]
    pub const fn subject(self) -> VectorSubject<'pack> {
        self.subject
    }

    /// The input the specification states.
    #[must_use]
    pub const fn input(self) -> &'pack [u8] {
        self.input
    }

    /// The bytes the specification says a producer renders from that input.
    #[must_use]
    pub const fn expected(self) -> &'pack [u8] {
        self.expected
    }

    /// Compare what a producer rendered against what this vector states.
    ///
    /// **The claim this supports** is the golden-vector lane's and only it: the
    /// producer rendered exactly the bytes the specification states for this
    /// input. It says nothing about any other input, and nothing about whether
    /// the specification is the right one.
    pub fn compared(self, produced: &[u8]) -> VectorVerdict {
        if produced == self.expected {
            return VectorVerdict::Agrees;
        }
        VectorVerdict::Disagrees(VectorDisagreement {
            expected: self.expected.to_vec(),
            produced: produced.to_vec(),
            difference: first_difference(self.expected, produced),
        })
    }
}

impl<'pack> VectorPack<'pack> {
    /// Read one length-prefixed vector pack.
    ///
    /// # The complete grammar
    ///
    /// Two primitives:
    ///
    /// - `u64be(n)` — the number in eight big-endian bytes.
    /// - `framed(x)` — `u64be(x.len())` followed by the bytes of `x`.
    ///
    /// A pack is, with no separators and no padding:
    ///
    /// | # | member | encoding |
    /// | - | ------ | -------- |
    /// | 1 | magic | the eight bytes of [`VECTOR_PACK_MAGIC`] |
    /// | 2 | version | `u64be`, and it must be [`VECTOR_PACK_VERSION`] |
    /// | 3 | count | `u64be` — how many vectors follow |
    /// | 4 | vectors | `count` of them, back to back |
    ///
    /// and one vector is four framed members in exactly this order: the
    /// subject's namespace as UTF-8, the subject's stem as UTF-8, the input
    /// bytes, and the expected bytes. Nothing follows the last vector.
    ///
    /// That is everything an adopter needs to write a pack for their own types:
    /// the depot ships the data, this instrument ships the tool that reads it.
    ///
    /// # Errors
    ///
    /// Refuses bytes that do not open with the magic, a version this
    /// instrument does not read, a pack that ends inside a member, a declared
    /// length this platform cannot address, a subject part that is not UTF-8 or
    /// that is empty, and any bytes left over after the declared count.
    pub fn read(pack: &'pack [u8]) -> Result<Self, VectorPackRefusal> {
        let mut reading = PackReading { pack, at: 0 };
        reading.magic()?;
        let version = reading.wide()?;
        if version != VECTOR_PACK_VERSION {
            return Err(VectorPackRefusal::UnsupportedVersion { declared: version });
        }
        let count = reading.wide()?;
        let mut entries: Vec<VectorEntry<'pack>> = Vec::new();
        for _ in 0..count {
            entries.push(reading.entry()?);
        }
        if reading.at != pack.len() {
            return Err(VectorPackRefusal::TrailingBytes { at: reading.at });
        }
        Ok(Self { entries })
    }

    /// The vectors the pack carries, in the order it states them.
    #[must_use]
    pub fn entries(&self) -> &[VectorEntry<'pack>] {
        &self.entries
    }
}

impl VectorDisagreement {
    /// The bytes the specification states.
    #[must_use]
    pub fn expected(&self) -> &[u8] {
        &self.expected
    }

    /// The bytes the producer rendered.
    #[must_use]
    pub fn produced(&self) -> &[u8] {
        &self.produced
    }

    /// Where the two part company.
    #[must_use]
    pub const fn difference(&self) -> ByteDifference {
        self.difference
    }
}

/// Where two byte strings first part company.
///
/// A shared prefix followed by different bytes is reported at that offset; two
/// strings where one is a prefix of the other part only at the end, and the
/// reading says so with both lengths rather than pointing past one of them.
fn first_difference(expected: &[u8], produced: &[u8]) -> ByteDifference {
    for (at, (left, right)) in expected.iter().zip(produced.iter()).enumerate() {
        if left != right {
            return ByteDifference::AtByte { at };
        }
    }
    ByteDifference::Length {
        expected: expected.len(),
        produced: produced.len(),
    }
}

/// One read in progress over one pack's bytes.
///
/// It never indexes and never adds without checking, so a pack authored by
/// anybody at all reaches a refusal rather than a panic.
struct PackReading<'pack> {
    /// The bytes being read.
    pack: &'pack [u8],
    /// How far the read has got.
    at: usize,
}

impl<'pack> PackReading<'pack> {
    /// Take one fixed run of bytes and step past it.
    fn take(&mut self, width: usize) -> Result<&'pack [u8], VectorPackRefusal> {
        let pack = self.pack;
        let at = self.at;
        let end = at
            .checked_add(width)
            .ok_or(VectorPackRefusal::Truncated { at })?;
        let taken = pack
            .get(at..end)
            .ok_or(VectorPackRefusal::Truncated { at })?;
        self.at = end;
        Ok(taken)
    }

    /// Read the opening magic, or refuse the bytes as no pack at all.
    fn magic(&mut self) -> Result<(), VectorPackRefusal> {
        let taken = self
            .take(VECTOR_PACK_MAGIC.len())
            .map_err(|_| VectorPackRefusal::NotAVectorPack)?;
        if taken == VECTOR_PACK_MAGIC.as_slice() {
            Ok(())
        } else {
            Err(VectorPackRefusal::NotAVectorPack)
        }
    }

    /// Read one eight-byte big-endian number.
    fn wide(&mut self) -> Result<u64, VectorPackRefusal> {
        let at = self.at;
        let taken = self.take(WIDE_WIDTH)?;
        let wide = taken
            .first_chunk::<WIDE_WIDTH>()
            .ok_or(VectorPackRefusal::Truncated { at })?;
        Ok(u64::from_be_bytes(*wide))
    }

    /// Read one framed member: its length, then its bytes.
    fn framed(&mut self) -> Result<&'pack [u8], VectorPackRefusal> {
        let at = self.at;
        let declared = self.wide()?;
        let width = usize::try_from(declared)
            .map_err(|_| VectorPackRefusal::LengthUnrepresentable { at, declared })?;
        self.take(width)
    }

    /// Read one vector: its subject's two parts, its input, and its expected
    /// bytes.
    fn entry(&mut self) -> Result<VectorEntry<'pack>, VectorPackRefusal> {
        let at = self.at;
        let namespace_bytes = self.framed()?;
        let stem_bytes = self.framed()?;
        let input = self.framed()?;
        let expected = self.framed()?;
        let namespace = core::str::from_utf8(namespace_bytes)
            .map_err(|_| VectorPackRefusal::SubjectNotText { at })?;
        let stem = core::str::from_utf8(stem_bytes)
            .map_err(|_| VectorPackRefusal::SubjectNotText { at })?;
        if namespace.is_empty() {
            return Err(VectorPackRefusal::EmptyNamespace { at });
        }
        if stem.is_empty() {
            return Err(VectorPackRefusal::EmptyStem { at });
        }
        Ok(VectorEntry {
            subject: VectorSubject { namespace, stem },
            input,
            expected,
        })
    }
}

// ---------------------------------------------------------------------------
// The independent transcript lane.
// ---------------------------------------------------------------------------

impl SpecifiedContext {
    /// Spell one derivation context from the segments a caller writes out.
    ///
    /// The segments are joined by `/`, which is the separator every published
    /// profile grammar in this workspace states. A stem the specification
    /// publishes as a multi-segment path is handed in as its own segments, so
    /// this lane never invents a join.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, and refuses an empty segment: an empty segment
    /// would spell a doubled separator and let two different rosters name one
    /// context.
    pub fn spelled(segments: &[&str]) -> Result<Self, ContextRefusal> {
        if segments.is_empty() {
            return Err(ContextRefusal::NoSegments);
        }
        let mut spelling = String::new();
        for (at, segment) in segments.iter().enumerate() {
            if segment.is_empty() {
                return Err(ContextRefusal::EmptySegment { at });
            }
            if !spelling.is_empty() {
                spelling.push('/');
            }
            spelling.push_str(segment);
        }
        Ok(Self(spelling))
    }

    /// Spell one derivation context under a profile version: the stem's
    /// segments, then `v` and the version, then the rest.
    ///
    /// The version segment is written here from the number the specification
    /// publishes, rather than read from the producer — a lane that imported the
    /// version would agree with a producer that silently changed it.
    ///
    /// # Errors
    ///
    /// Refuses exactly what [`SpecifiedContext::spelled`] refuses, over the
    /// assembled roster: a position a refusal carries is a position in that
    /// roster and not in the caller's stem.
    pub fn under_version(
        stem: &[&str],
        version: u32,
        tail: &[&str],
    ) -> Result<Self, ContextRefusal> {
        let versioned = format!("v{version}");
        let mut segments: Vec<&str> = Vec::new();
        segments.extend_from_slice(stem);
        segments.push(&versioned);
        segments.extend_from_slice(tail);
        Self::spelled(&segments)
    }

    /// The context as a derivation reads it.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl TranscriptDerivation {
    /// An empty preimage, before its first member.
    #[must_use]
    pub const fn opened() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    /// Append one length-prefixed byte string.
    #[must_use]
    pub fn framed(mut self, material: &[u8]) -> Self {
        self.members
            .push(TranscriptMember::Framed(material.to_vec()));
        self
    }

    /// Append one length-prefixed byte string, over text.
    ///
    /// Text is framed exactly as bytes are, because a specification that framed
    /// its text differently from its bytes would carry two rules where one
    /// does.
    #[must_use]
    pub fn framed_text(self, text: &str) -> Self {
        self.framed(text.as_bytes())
    }

    /// Append one bare discriminant byte.
    #[must_use]
    pub fn discriminant(mut self, slot: u8) -> Self {
        self.members.push(TranscriptMember::Discriminant(slot));
        self
    }

    /// Append one 32-bit number, four big-endian bytes, unframed.
    #[must_use]
    pub fn fixed32(mut self, value: u32) -> Self {
        self.members.push(TranscriptMember::Fixed32(value));
        self
    }

    /// Append one 64-bit number, eight big-endian bytes, unframed.
    #[must_use]
    pub fn fixed64(mut self, value: u64) -> Self {
        self.members.push(TranscriptMember::Fixed64(value));
        self
    }

    /// The members written so far, in order.
    ///
    /// The roster is kept as typed members rather than as accumulated bytes, so
    /// a disagreement can be read as the encoding decisions that produced it
    /// rather than as one opaque string.
    #[must_use]
    pub fn members(&self) -> &[TranscriptMember] {
        &self.members
    }

    /// Compose the preimage: every member, in order, by this lane's own
    /// framing.
    ///
    /// A framed member is eight big-endian length bytes then its bytes; a
    /// discriminant is one bare byte; a fixed number is its own width in
    /// big-endian bytes. No separators, no padding, and nothing between
    /// members.
    #[must_use]
    pub fn preimage(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for member in &self.members {
            match member {
                TranscriptMember::Framed(material) => {
                    let width = u64::try_from(material.len()).unwrap_or(u64::MAX);
                    bytes.extend_from_slice(&width.to_be_bytes());
                    bytes.extend_from_slice(material);
                }
                TranscriptMember::Discriminant(slot) => bytes.push(*slot),
                TranscriptMember::Fixed32(value) => bytes.extend_from_slice(&value.to_be_bytes()),
                TranscriptMember::Fixed64(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            }
        }
        bytes
    }

    /// Derive the identity this preimage names under one context.
    ///
    /// The digest is BLAKE3's `derive_key`, over the context's spelling and
    /// this lane's own preimage. Deterministic and total: every preimage names
    /// an identity, on any machine, with no ambient fact in the derivation.
    #[must_use]
    pub fn derived(&self, context: &SpecifiedContext) -> DerivedIdentity {
        DerivedIdentity(blake3::derive_key(context.spelling(), &self.preimage()))
    }
}

impl DerivedIdentity {
    /// The thirty-two bytes, borrowed for comparison and for rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Compare this re-derivation against the identity a producer published.
    ///
    /// **The claim this supports** is the transcript lane's and only it: the
    /// published specification, read independently and encoded independently,
    /// names the identity the producer minted. A disagreement says the two
    /// readings differ; which of them is right is a person's ruling.
    pub fn compared(&self, published: &[u8; 32]) -> TranscriptVerdict {
        if self.0 == *published {
            TranscriptVerdict::Agrees
        } else {
            TranscriptVerdict::Disagrees(TranscriptDisagreement {
                rederived: self.0,
                published: *published,
            })
        }
    }
}

impl TranscriptDisagreement {
    /// What this lane derived from the specification alone.
    #[must_use]
    pub const fn rederived(&self) -> &[u8; 32] {
        &self.rederived
    }

    /// What the producer published.
    #[must_use]
    pub const fn published(&self) -> &[u8; 32] {
        &self.published
    }
}

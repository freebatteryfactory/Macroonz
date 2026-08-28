//! The invariant nucleus for golden-vector packs and comparisons.

use super::{
    ByteDifference, VECTOR_PACK_MAGIC, VECTOR_PACK_VERSION, VectorDisagreement, VectorEntry,
    VectorPack, VectorPackRefusal, VectorSubject, VectorVerdict,
};

/// The width of every number a pack or a preimage frames, in bytes.
const WIDE_WIDTH: usize = 8;

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
    /// The claim it supports is this lane's alone: the producer rendered exactly the bytes the specification states for this input, and nothing about any other input.
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
    /// One vector is four framed members in exactly this order: the subject's namespace as UTF-8, the subject's stem as UTF-8, the input bytes, and the expected bytes.
    /// Nothing follows the last vector.
    ///
    /// # Errors
    ///
    /// Refuses bytes that do not open with the magic, a version this home does not read, a pack that ends inside a member, a declared length this platform cannot address, a subject part that is not UTF-8 or that is empty, and any bytes left over after the declared count.
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
/// Two strings where one is a prefix of the other part only at the end, and the reading says so with both lengths rather than pointing past one of them.
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
/// It never indexes and never adds without checking, so a pack authored by anybody at all reaches a refusal rather than a panic.
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

    /// Read one vector: its subject's two parts, its input, and its expected bytes.
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

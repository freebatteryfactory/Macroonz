//! The golden-vector reading and verdict vocabulary.

#[path = "type_guard.rs"]
mod guard;

/// The eight bytes every vector pack opens with.
pub const VECTOR_PACK_MAGIC: [u8; 8] = *b"macroonz";

/// The pack format version this home reads.
///
/// A pack declaring any other version is refused rather than decoded under the wrong grammar.
pub const VECTOR_PACK_VERSION: u64 = 1;

/// The subject one golden vector is about: the owner that declares the subject, and the subject's own spelling.
///
/// Both parts are refused empty, so two owners never collide by spelling alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorSubject<'pack> {
    namespace: &'pack str,
    stem: &'pack str,
}

/// One golden vector: the subject it is about, the input the specification states, and the bytes the specification says a producer renders from that input.
///
/// [`VectorPack::read`] is the only road to one, so nothing exported from a producer reaches a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorEntry<'pack> {
    subject: VectorSubject<'pack>,
    input: &'pack [u8],
    expected: &'pack [u8],
}

/// One vector pack, read: every vector it carries, in the order the pack states them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorPack<'pack> {
    entries: Vec<VectorEntry<'pack>>,
}

/// Why one vector pack was refused.
///
/// Every arm but the first two carries where the read stopped.
#[must_use = "a refusal is the reason a pack was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorPackRefusal {
    /// The bytes do not open with [`VECTOR_PACK_MAGIC`], so they are not a pack at all.
    ///
    /// Bytes too short to carry the magic are this rather than a truncation, because nothing established that a pack was ever there.
    NotAVectorPack,
    /// The pack declares a format version this home does not read.
    UnsupportedVersion {
        /// The version the pack declares.
        declared: u64,
    },
    /// The pack ended where a member's declared bytes should have stood.
    Truncated {
        /// The offset the read stopped at.
        at: usize,
    },
    /// A declared length is larger than this platform can address, so the member it frames could never be read.
    LengthUnrepresentable {
        /// The offset the length was read at.
        at: usize,
        /// The length the pack declares.
        declared: u64,
    },
    /// Bytes remain after the last vector the pack's own count admits.
    TrailingBytes {
        /// The offset the surplus begins at.
        at: usize,
    },
    /// A subject part is not valid UTF-8, so the vector names no subject a reader could match.
    SubjectNotText {
        /// The offset the vector begins at.
        at: usize,
    },
    /// A subject's namespace is empty, so the vector states no owner.
    EmptyNamespace {
        /// The offset the vector begins at.
        at: usize,
    },
    /// A subject's stem is empty, so the vector states no spelling.
    EmptyStem {
        /// The offset the vector begins at.
        at: usize,
    },
}

/// Where two byte strings first part company.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteDifference {
    /// The two share a prefix and carry different bytes at this offset.
    AtByte {
        /// The offset of the first differing byte.
        at: usize,
    },
    /// One is a prefix of the other, so they part only at the end.
    Length {
        /// How many bytes the specification states.
        expected: usize,
        /// How many bytes the producer rendered.
        produced: usize,
    },
}

/// One golden-vector disagreement: both renderings at full length, and where they part.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorDisagreement {
    expected: Vec<u8>,
    produced: Vec<u8>,
    difference: ByteDifference,
}

/// What one golden-vector comparison concluded.
#[must_use = "a verdict is what the comparison concluded about the produced bytes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VectorVerdict {
    /// The producer rendered exactly the bytes the vector states, byte for byte.
    Agrees,
    /// The producer and the vector disagree, this way.
    Disagrees(VectorDisagreement),
}

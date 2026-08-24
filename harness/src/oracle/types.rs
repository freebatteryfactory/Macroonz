//! Every reading this home can produce and every verdict it can state.
//!
//! Declarations only.
//! The roads that reach a private field are in this module's own child `type_guard.rs`, the parse is `parse.rs`, the two comparisons are `structural.rs` and `compiled.rs`, and the causes are `conclude.rs`.

#[path = "type_guard.rs"]
mod guard;

/// The cause family every finding raised here is cited under, with the lane and the finding in the local key.
pub const ORACLE_CAUSE_FAMILY: &str = "macroonz.oracle";

// ---------------------------------------------------------------------------
// The golden-vector lane.
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// The independent transcript lane.
// ---------------------------------------------------------------------------

/// One member of a preimage, as this lane writes it.
///
/// The roster is the closed set of encoding decisions the lane makes on its own, from what a published specification states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptMember {
    /// A length-prefixed byte string: eight big-endian length bytes, then the bytes themselves.
    Framed(Vec<u8>),
    /// One bare byte — the slot a specification assigns to a variant.
    Discriminant(u8),
    /// One 32-bit number, four big-endian bytes, unframed because its width is fixed.
    Fixed32(u32),
    /// One 64-bit number, eight big-endian bytes, unframed for the same reason.
    Fixed64(u64),
}

/// One preimage this lane composes for itself, member by member.
///
/// Every byte is written here from typed arguments a caller took off a published specification: not a framing, not a field order, not a spelling is imported from the producer whose identity is under judgement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranscriptDerivation {
    members: Vec<TranscriptMember>,
}

/// One derivation context, spelled by this lane from a published grammar.
///
/// [`SpecifiedContext::spelled`] joins segments a caller writes out, and [`SpecifiedContext::under_version`] adds the `v<n>` segment a versioned profile grammar states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecifiedContext(String);

/// Why one derivation context was refused.
#[must_use = "a refusal is the reason a context was not spelled"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextRefusal {
    /// No segments were offered, so the context names no domain.
    NoSegments,
    /// One segment is empty, which would spell a doubled separator and let two rosters name one context.
    EmptySegment {
        /// The segment's position in the assembled roster.
        at: usize,
    },
}

/// The thirty-two bytes this lane derived from its own preimage.
///
/// The digest is BLAKE3's `derive_key`, and it is the one mechanism deliberately shared with the producer: the two sides differ in what they encode and in nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedIdentity([u8; 32]);

/// One transcript disagreement: what this lane derived, and what the producer published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TranscriptDisagreement {
    rederived: [u8; 32],
    published: [u8; 32],
}

/// What one independent re-derivation concluded.
#[must_use = "a verdict is what the re-derivation concluded about the published identity"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptVerdict {
    /// The specification, read independently, names the identity the producer published.
    Agrees,
    /// The two namings disagree, this way.
    Disagrees(TranscriptDisagreement),
}

// ---------------------------------------------------------------------------
// The structural read: what an artifact declares.
// ---------------------------------------------------------------------------

/// One way an implementation may be written beyond the plain form.
///
/// Each of the four changes what the artifact declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplPosture {
    /// `unsafe impl` — a contract with an obligation attached.
    Unsafely,
    /// `impl !Trait for Type` — the opposite of the contract the declaration named.
    Negative,
    /// `default impl` — a realization other implementations may replace.
    Defaulted,
    /// Generic parameters or a `where` clause: a family of implementations rather than the one the declaration named.
    Generic,
}

/// What one associated constant's value expression says, read shallowly.
///
/// This is syntax and never meaning: whether a spelling resolves, typechecks, or evaluates to what it suggests is the compiled read-back's question.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstantReading {
    /// A path expression, spelled back with its segments and its leading `::` where it carries one.
    Path(String),
    /// A string literal's value.
    Text(String),
    /// An integer literal, in its base-ten digits.
    Number(String),
    /// A boolean literal.
    Truth(bool),
    /// The elements of a borrowed array — `&[…]` — in order.
    BorrowedArray(Vec<ConstantReading>),
    /// The elements of an array written without a borrow — `[…]` — in order.
    Array(Vec<ConstantReading>),
    /// A call: the path called, and the readings of its arguments in order.
    Call {
        /// The path the call calls.
        path: String,
        /// The readings of the call's arguments, in order.
        arguments: Vec<ConstantReading>,
    },
}

/// One member an implementation carries, as the structural read found it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplementationMember {
    /// An associated constant: the name it states, and the reading of its value.
    Constant {
        /// The constant's name.
        name: String,
        /// The reading of its value, or `None` where the value's shape is one this lane does not name.
        reading: Option<ConstantReading>,
    },
    /// A member that is not an associated constant, described by what it is.
    Other {
        /// What the member is, in this lane's own words.
        described: &'static str,
    },
}

/// One implementation the artifact declares, as the structural read recovered it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationStructure {
    /// The type the implementation targets, spelled as its path.
    pub target: String,
    /// The trait path the implementation realizes, or no path for an inherent implementation.
    pub trait_path: Option<String>,
    /// The postures the implementation is written under, in roster order.
    pub postures: Vec<ImplPosture>,
    /// Every attribute that decides something, on the implementation or on one of its members, by path; doc comments decide nothing and never appear.
    pub meaning_bearing_attributes: Vec<String>,
    /// The members the implementation carries, in order, with a member stated twice appearing twice rather than written over.
    pub members: Vec<ImplementationMember>,
}

/// Everything the structural read recovered from one rendered artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactStructure {
    /// The implementations the artifact declares, in declaration order.
    pub implementations: Vec<ImplementationStructure>,
    /// How many items the artifact declares that are not implementations of a named type.
    ///
    /// Nothing an artifact renders lawfully is one, so any count above zero is a finding rather than a detail.
    pub other_items: usize,
}

/// One member a caller states an implementation will carry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredMember<'spec> {
    /// The name the member states.
    pub name: &'spec str,
    /// The reading its value must produce.
    pub reading: ConstantReading,
}

/// One implementation a caller states an artifact will declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredImplementation<'spec> {
    /// The type this implementation targets.
    pub target: &'spec str,
    /// The trait path this implementation realizes, or no path for an inherent implementation.
    pub trait_path: Option<&'spec str>,
    /// The postures this implementation is written under, so a plain implementation names none and any posture at all is a finding.
    pub postures: &'spec [ImplPosture],
    /// The attributes the declaration admits on this implementation or on one of its members, by path.
    pub attributes: &'spec [&'spec str],
    /// The members this implementation carries, and what each states.
    pub members: &'spec [DeclaredMember<'spec>],
}

/// What a caller states one artifact will declare, written independently of the thing under judgement.
///
/// Nothing here is obtained by asking the producer what it did, which is why its agreement is worth anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredArtifact<'spec> {
    /// The implementations the artifact declares, in the order it declares them.
    pub implementations: &'spec [DeclaredImplementation<'spec>],
}

/// Which structural fact the artifact and the declaration disagree about, and where.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructuralDisagreement {
    /// The artifact declares an item that is not an implementation of a named type.
    UnexpectedItem,
    /// The artifact declares a different number of implementations than the declaration names.
    OutputCardinality {
        /// How many the declaration names.
        declared: usize,
        /// How many the artifact declares.
        read: usize,
    },
    /// One trait-and-target pair is implemented more than once.
    DuplicateImplementation {
        /// The position of the second implementation of the pair.
        at: usize,
    },
    /// An implementation targets a type the declaration did not name.
    ImplementationTarget {
        /// The implementation's position.
        at: usize,
    },
    /// An implementation's trait or inherent posture differs from the declaration.
    TraitPath {
        /// The implementation's position.
        at: usize,
    },
    /// An implementation is written `unsafe`, negative, `default`, or generic where the declaration names another posture roster.
    ImplPosture {
        /// The implementation's position.
        at: usize,
    },
    /// An implementation or one of its members carries an attribute that decides something and that the declaration did not name.
    MeaningBearingAttribute {
        /// The implementation's position.
        at: usize,
        /// The attribute's path.
        attribute: String,
    },
    /// An implementation carries a member the declaration did not name.
    UnexpectedImplMember {
        /// The implementation's position.
        at: usize,
        /// The member's name, or what the member is where it is not an associated constant.
        member: String,
    },
    /// An implementation states one member more than once.
    DuplicateMember {
        /// The implementation's position.
        at: usize,
        /// The member's name.
        member: String,
    },
    /// An implementation does not state a member the declaration names.
    MissingImplMember {
        /// The implementation's position.
        at: usize,
        /// The member's name.
        member: String,
    },
    /// A member's value is of a shape this lane does not name, so nothing was compared.
    MemberValueUnread {
        /// The implementation's position.
        at: usize,
        /// The member's name.
        member: String,
    },
    /// A member's value is read, and is not the declared reading.
    MemberValue {
        /// The implementation's position.
        at: usize,
        /// The member's name.
        member: String,
    },
}

/// What one structural reading concluded.
///
/// [`StructuralVerdict::Unparsable`] is a failure class of its own: never a skip, never a softer [`StructuralVerdict::Deviates`], and never folded into [`StructuralVerdict::Conforms`], which would assert over a reading that never happened.
#[must_use = "a verdict is what the structural reading concluded"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructuralVerdict {
    /// The artifact declares exactly what the caller declared it would.
    Conforms,
    /// The artifact and the declaration disagree, about this.
    Deviates(StructuralDisagreement),
    /// The text is not parseable Rust, so nothing structural was read at all.
    Unparsable,
}

// ---------------------------------------------------------------------------
// The compiled read-back: what a compiled artifact hands back.
// ---------------------------------------------------------------------------

/// One value a compiled artifact handed back, as the reader that ran it observed it.
///
/// This is a value and not syntax, so it carries no constructor path: by the time a compiler hands a constant back, the path it was built through has been resolved away.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObservedValue {
    /// One word a typed value states — the variant a constant reads back as.
    Word(String),
    /// One text value.
    Text(String),
    /// One whole number.
    Count(u64),
    /// One truth value.
    Truth(bool),
    /// Several values, in the order the compiled artifact hands them back.
    Series(Vec<ObservedValue>),
}

/// One member a compiled artifact handed back: the name the reader asked for, and the value it got.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObservedMember {
    /// The member's name.
    pub name: String,
    /// The value it read back as.
    pub value: ObservedValue,
}

/// What a caller reports observing after presenting one artifact to a compiler.
///
/// Holding one does not establish that a compiler ran; the effectful challenge that constructs it owns that provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledObservation {
    /// The compiler refused the artifact, so nothing was read back.
    RefusedByCompiler,
    /// The artifact compiled, and these are the members the caller read back, in read order.
    ReadBack(Vec<ObservedMember>),
}

/// One member a caller states a compiled artifact will hand back.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredReadBack<'spec> {
    /// The member's name.
    pub name: &'spec str,
    /// The value it must read back as.
    pub value: ObservedValue,
}

/// What a caller states a compiler will do with one artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclaredBehavior<'spec> {
    /// The compiler must refuse this artifact.
    RefusedByCompiler,
    /// The artifact must compile and hand back exactly these members.
    ReadsBack(&'spec [DeclaredReadBack<'spec>]),
}

/// Which supplied observation and declared behavior disagree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledDisagreement {
    /// The caller reported acceptance where the declaration requires refusal.
    AcceptedWhereRefusalDeclared,
    /// The caller reported refusal where the declaration requires acceptance.
    RefusedWhereAcceptanceDeclared,
    /// The supplied read-back carries a member the declaration did not name.
    UnexpectedMember {
        /// The member's name.
        member: String,
    },
    /// The supplied read-back carries one member more than once.
    DuplicateMember {
        /// The member's name.
        member: String,
    },
    /// The supplied read-back omits a member the declaration names.
    MissingMember {
        /// The member's name.
        member: String,
    },
    /// A supplied member value differs from the declaration.
    MemberValue {
        /// The member's name.
        member: String,
    },
}

/// What one compiled-observation comparison concluded.
#[must_use = "a verdict is what the compiled-observation comparison concluded"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledVerdict {
    /// The supplied observation agrees with the declared behavior.
    Conforms,
    /// The supplied observation and declared behavior disagree, about this.
    Deviates(CompiledDisagreement),
}

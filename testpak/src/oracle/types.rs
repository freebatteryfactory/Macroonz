//! The annex's declarations: what a golden vector is, what an independent
//! re-derivation writes, what an artifact was read to declare, what a caller
//! states an artifact will declare, what a compiled artifact handed back, and
//! what each of these readings may conclude.
//!
//! Declarations only. Every road that reaches a private field is in this
//! module's own child `type_guard.rs`; the closed cause tables and the roads
//! into the record vocabulary are `type_contract.rs`; the structural read and
//! the compiled read-back are their own pure-function modules. Nothing here
//! decides anything, so a reader of this file learns exactly what the annex can
//! say and never how it says it.

#[path = "type_guard.rs"]
mod guard;

/// The cause family every finding this annex raises is cited under.
///
/// One family for the whole annex, with the lane and the finding in the local
/// key, so a reader grouping failures by family sees "the oracle disagreed"
/// once rather than four times under four spellings.
pub const ORACLE_CAUSE_FAMILY: &str = "testpak.oracle";

// ---------------------------------------------------------------------------
// The golden-vector lane.
// ---------------------------------------------------------------------------

/// The eight bytes every vector pack opens with.
pub const VECTOR_PACK_MAGIC: [u8; 8] = *b"tpak-vec";

/// The pack format version this instrument reads.
///
/// A pack declaring any other version is refused rather than read at a guess:
/// a reader that skipped the version would decode one grammar's bytes under
/// another's rules and report the difference as a producer's fault.
pub const VECTOR_PACK_VERSION: u64 = 1;

/// The subject one golden vector is about: the owner that declares the subject,
/// and the subject's own spelling.
///
/// # Authority
///
/// Both parts are read out of a pack and both are refused empty, so a vector
/// always states whose specification it stands for and two owners never collide
/// by spelling alone.
///
/// # Nonclaims
///
/// It is deliberately not a
/// [`NamespacedName`](crate::descriptor::NamespacedName). That vocabulary is
/// AUTHORED — by a hand, by a stamp, or by a depot const — and is never minted
/// from data a reader parsed; a pack is data a reader parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorSubject<'pack> {
    namespace: &'pack str,
    stem: &'pack str,
}

/// One golden vector: the subject it is about, the input the specification
/// states, and the bytes the specification says a producer renders from that
/// input.
///
/// # Authority
///
/// A vector is BORN FROM THE SPECIFICATION and reaches this type only by being
/// read out of a pack, which is why [`VectorPack::read`] is the only road to
/// one. A vector exported from a producer would make every comparison a mirror
/// of the thing under judgement.
///
/// # Nonclaims
///
/// The expected bytes are the specification's word, not a proof of it: holding
/// a vector says the pack states these bytes for this input, and never that the
/// specification is right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorEntry<'pack> {
    subject: VectorSubject<'pack>,
    input: &'pack [u8],
    expected: &'pack [u8],
}

/// One vector pack, read: every vector it carries, in the order it states them.
///
/// # Construction
///
/// [`VectorPack::read`] is the only road, and it states the complete pack
/// grammar — an adopter writing vectors for their own types writes that grammar
/// and gets this same instrument.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorPack<'pack> {
    entries: Vec<VectorEntry<'pack>>,
}

/// Why one vector pack was refused.
///
/// Every arm but the first two carries where the read stopped, so a refusal
/// names a place in the bytes rather than leaving a reader to find it.
///
/// # Nonclaims
///
/// A refusal is a fact about the PACK and never about a producer: nothing was
/// compared, so nothing is claimed about anything the pack was going to judge.
#[must_use = "a refusal is the reason a pack was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorPackRefusal {
    /// The bytes do not open with [`VECTOR_PACK_MAGIC`], so they are not a
    /// vector pack at all. Bytes too short to carry the magic are this and not
    /// a truncation: nothing established that a pack was ever there.
    NotAVectorPack,
    /// The pack declares a format version this instrument does not read.
    UnsupportedVersion {
        /// The version the pack declares.
        declared: u64,
    },
    /// The pack ended where a member's declared bytes should have stood.
    Truncated {
        /// The offset the read stopped at.
        at: usize,
    },
    /// A declared length is larger than this platform can address, so the
    /// member it frames could never be read.
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
    /// A subject part is not valid UTF-8, so the vector names no subject a
    /// reader could match.
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
///
/// A disagreement that only said "not equal" would leave a person diffing two
/// blobs by eye, so the reading states the first place they differ.
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

/// One golden-vector disagreement: both renderings, and where they part.
///
/// Both are carried at full length. A disagreement that showed only a fold, a
/// prefix, or a count would send a reader back to re-run the comparison to see
/// what actually happened.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorDisagreement {
    expected: Vec<u8>,
    produced: Vec<u8>,
    difference: ByteDifference,
}

/// What one golden-vector comparison concluded.
///
/// Two answers, and neither is silence: there is no third arm for "no vector
/// was found", because a comparison with no vector is a pack that was never
/// read, and that is a [`VectorPackRefusal`].
#[must_use = "a verdict is what the comparison concluded about the produced bytes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VectorVerdict {
    /// The producer rendered exactly the bytes the vector states, byte for
    /// byte.
    Agrees,
    /// The producer and the vector disagree, this way.
    Disagrees(VectorDisagreement),
}

// ---------------------------------------------------------------------------
// The independent transcript lane.
// ---------------------------------------------------------------------------

/// One member of a preimage, as this lane writes it.
///
/// The roster is the closed set of encoding decisions the annex makes on its
/// own. A published specification states which members a preimage carries and
/// in which order; this lane writes each one out from that statement, importing
/// no encoder from the producer whose identity is under judgement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptMember {
    /// A length-prefixed byte string: eight big-endian length bytes, then the
    /// bytes themselves.
    ///
    /// A member written without its length would let two different member
    /// sequences cut into one byte string, which is exactly the collision the
    /// framing exists to refuse.
    Framed(Vec<u8>),
    /// One bare byte — the slot a specification assigns to a variant.
    Discriminant(u8),
    /// One 32-bit number, four big-endian bytes, written without a frame
    /// because its width is fixed.
    Fixed32(u32),
    /// One 64-bit number, eight big-endian bytes, written without a frame for
    /// the same reason.
    Fixed64(u64),
}

/// One preimage this lane composes for itself, member by member.
///
/// # Authority
///
/// Every byte the derivation hands out was written here, from typed arguments
/// the caller took off a published specification. Nothing is imported from the
/// producer that mints the identity under judgement — not a framing, not a
/// field order, not a spelling — because whether the specification says enough
/// for somebody else to re-derive the value is the thing being judged.
///
/// # Nonclaims
///
/// It does not claim the specification is complete. It claims that THIS reading
/// of the specification produces these bytes; a disagreement says the two
/// readings differ and never which one is right.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranscriptDerivation {
    members: Vec<TranscriptMember>,
}

/// One derivation context, spelled by this lane from a published grammar.
///
/// # Construction
///
/// [`SpecifiedContext::spelled`] joins segments a caller writes out;
/// [`SpecifiedContext::under_version`] adds the `v<n>` segment two published
/// profiles in this workspace state. Both are the lane's own assembly of a
/// grammar the specification publishes — never a call into the producer that
/// assembles it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecifiedContext(String);

/// Why one derivation context was refused.
///
/// Both arms are the same concern: a context that could be spelled two ways
/// would separate two domains that a reader believes are separate and a
/// derivation treats as one.
#[must_use = "a refusal is the reason a context was not spelled"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextRefusal {
    /// No segments were offered, so the context names no domain.
    NoSegments,
    /// One segment is empty, which would spell a doubled separator and let two
    /// different segment rosters name one context.
    EmptySegment {
        /// The segment's position in the assembled roster.
        at: usize,
    },
}

/// The thirty-two bytes this lane derived from its own preimage.
///
/// # Authority
///
/// The digest is BLAKE3's `derive_key`, and sharing it with the producer is
/// deliberate: a lane that reimplemented the hash would be judging an
/// arithmetic exercise rather than a specification. The two sides differ in
/// what they ENCODE and in nothing else.
///
/// # Nonclaims
///
/// It is deliberately not a [`ContentAddress`](crate::identity::ContentAddress).
/// That substrate derives under the harness's OWN profile, and a lane that
/// re-derives another home's published identity has to spell that home's
/// published context itself — reaching for the harness's profile would name a
/// different value and call the difference a producer's fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedIdentity([u8; 32]);

/// One transcript disagreement: what this lane derived, and what the producer
/// published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TranscriptDisagreement {
    rederived: [u8; 32],
    published: [u8; 32],
}

/// What one independent re-derivation concluded.
#[must_use = "a verdict is what the re-derivation concluded about the published identity"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptVerdict {
    /// The specification, read independently, names the identity the producer
    /// published.
    Agrees,
    /// The two namings disagree, this way.
    Disagrees(TranscriptDisagreement),
}

// ---------------------------------------------------------------------------
// The structural read: what an artifact declares.
// ---------------------------------------------------------------------------

/// One way an implementation may be WRITTEN beyond the plain form.
///
/// An implementation carries no visibility in Rust — there is no seat for one
/// on the item — so these four are the postures a reader can be lied to about,
/// and each changes what the artifact declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplPosture {
    /// `unsafe impl` — a contract with an obligation attached.
    Unsafely,
    /// `impl !Trait for Type` — the opposite of the contract the declaration
    /// named.
    Negative,
    /// `default impl` — a realization other implementations may replace.
    Defaulted,
    /// Generic parameters or a `where` clause: a family of implementations
    /// rather than the one the declaration named.
    Generic,
}

/// What one associated constant's value expression SAYS, read shallowly.
///
/// # Authority
///
/// This is syntax and never meaning. A path is read as the path it spells; a
/// call is read as the path called and the readings of its arguments. Whether
/// any of it resolves, typechecks, or evaluates to the value its spelling
/// suggests is the compiled read-back's, where a compiler parses by its own
/// rules and hands back values.
///
/// # Bounds
///
/// The roster is exactly the shapes this reading can name. A value of any other
/// shape is not read at all — the member's reading is `None` — because a
/// reading that silently flattened an unnameable shape into a nameable one
/// would let two different declarations compare equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstantReading {
    /// A path expression, spelled back with its segments and its leading `::`
    /// where it carries one.
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
    ///
    /// The path is a column of the reading and not decoration. A value carried
    /// through some other constructor declares something else entirely, and a
    /// reading that kept only the arguments would have called it equal.
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
    /// An associated constant: the name it states, and the reading of its
    /// value where this lane can name the value's shape.
    Constant {
        /// The constant's name.
        name: String,
        /// The reading of its value, or `None` where the value's shape is one
        /// this lane does not name.
        reading: Option<ConstantReading>,
    },
    /// A member that is not an associated constant, described by what it is.
    Other {
        /// What the member is, in this lane's own words.
        described: &'static str,
    },
}

/// One trait implementation the artifact declares, as the structural read
/// recovered it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationStructure {
    /// The type the implementation targets, spelled as its path.
    pub target: String,
    /// The trait path the implementation realizes, spelled with its leading
    /// `::` where it carries one.
    pub trait_path: String,
    /// The postures the implementation is written under, in roster order.
    pub postures: Vec<ImplPosture>,
    /// The attributes that decide something — every attribute on the
    /// implementation or on one of its members that is not a doc comment, by
    /// path.
    ///
    /// One roster for the item and its members together, because an attribute
    /// anywhere inside an implementation decides something about that
    /// implementation.
    pub meaning_bearing_attributes: Vec<String>,
    /// The members the implementation carries, in the order it states them.
    ///
    /// Every member, and nothing stepped over: a reader that recovered the
    /// members a declaration named and ignored the rest has a blind spot
    /// exactly the size of everything the declaration did not name. A member
    /// stated twice appears twice here and is never written over.
    pub members: Vec<ImplementationMember>,
}

/// Everything the structural read recovered from one rendered artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactStructure {
    /// The trait implementations the artifact declares, in the order it
    /// declares them.
    pub implementations: Vec<ImplementationStructure>,
    /// How many items the artifact declares that are not trait implementations
    /// of a named type. Nothing an artifact renders lawfully is one, so any
    /// count above zero is a finding rather than a detail.
    pub other_items: usize,
}

/// One member a caller states an implementation will carry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredMember<'spec> {
    /// The name the member states.
    pub name: &'spec str,
    /// The reading its value must produce, as a typed value the caller writes
    /// beside the declaration it handed to the producer.
    pub reading: ConstantReading,
}

/// One implementation a caller states an artifact will declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredImplementation<'spec> {
    /// The type this implementation targets.
    pub target: &'spec str,
    /// The trait path this implementation realizes.
    pub trait_path: &'spec str,
    /// The postures this implementation is written under. A plain
    /// implementation declares none, so any posture at all is a finding.
    pub postures: &'spec [ImplPosture],
    /// The attributes the declaration admits on this implementation or on one
    /// of its members, by path. Doc comments are not attributes for this
    /// purpose and never appear here.
    pub attributes: &'spec [&'spec str],
    /// The members this implementation carries, and what each states.
    pub members: &'spec [DeclaredMember<'spec>],
}

/// What a caller states one artifact will declare, written independently of the
/// thing under judgement.
///
/// # Authority
///
/// Every roster here is authored by the caller beside the declaration it handed
/// to the producer. Nothing in this structure is obtained by asking the
/// producer what it did, which is the whole reason its agreement is worth
/// anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredArtifact<'spec> {
    /// The implementations the artifact declares, in the order it declares
    /// them.
    pub implementations: &'spec [DeclaredImplementation<'spec>],
}

/// Which structural fact the artifact and the declaration disagree about, and
/// where.
///
/// One finding, named, and placed. A verdict that only said "no" would leave a
/// caller guessing which of a dozen questions came back wrong, and a finding
/// without a position would leave it guessing which implementation asked it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructuralDisagreement {
    /// The artifact declares an item that is not a trait implementation of a
    /// named type.
    UnexpectedItem,
    /// The artifact declares a different number of implementations than the
    /// declaration names.
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
    /// An implementation realizes a trait path the declaration did not name, or
    /// names them in another order.
    TraitPath {
        /// The implementation's position.
        at: usize,
    },
    /// An implementation is written `unsafe`, negative, `default`, or generic
    /// where the declaration names another posture roster.
    ImplPosture {
        /// The implementation's position.
        at: usize,
    },
    /// An implementation or one of its members carries an attribute that
    /// decides something and that the declaration did not name.
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
        /// The member's name, or what the member is where it is not an
        /// associated constant.
        member: String,
    },
    /// An implementation states one member more than once. The second reading
    /// is a finding and never an overwrite of the first.
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
    /// A member's value is of a shape this lane does not name, so nothing was
    /// compared. A failure class of its own, never a quiet pass.
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
/// Three answers, and none of them is silence.
///
/// # Nonclaims
///
/// [`StructuralVerdict::Unparsable`] is a failure class of its own: never a
/// skip, never a softer [`StructuralVerdict::Deviates`], and never foldable
/// into [`StructuralVerdict::Conforms`]. A caller that folded it would be
/// asserting over a reading that never happened.
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

/// One value a compiled artifact handed back, as the reader that ran it
/// observed it.
///
/// # Nonclaims
///
/// This is a VALUE and not syntax, which is why it carries no constructor path:
/// by the time a compiler has handed a constant back, the path it was built
/// through has been resolved away. A claim about which path a value was carried
/// through is the structural read's, over [`ConstantReading`].
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

/// One member a compiled artifact handed back: the name the reader asked for,
/// and the value it got.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObservedMember {
    /// The member's name.
    pub name: String,
    /// The value it read back as.
    pub value: ObservedValue,
}

/// What a compiler did with one artifact, as a reader that ran it brings the
/// observation back.
///
/// # Authority
///
/// The two arms make the wrong move unrepresentable: values without a
/// compilation cannot be built, because a refused artifact has no constants for
/// anybody to read.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledObservation {
    /// The compiler refused the artifact, so nothing was read back.
    RefusedByCompiler,
    /// The artifact compiled, and these are the members the reader read back,
    /// in the order it read them.
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
pub enum DeclaredBehaviour<'spec> {
    /// The compiler must refuse this artifact.
    RefusedByCompiler,
    /// The artifact must compile and hand back exactly these members.
    ReadsBack(&'spec [DeclaredReadBack<'spec>]),
}

/// Which observable fact the compiled artifact and the declaration disagree
/// about.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledDisagreement {
    /// The compiler accepted an artifact the declaration says it must refuse.
    AcceptedWhereRefusalDeclared,
    /// The compiler refused an artifact the declaration says it must accept.
    RefusedWhereAcceptanceDeclared,
    /// The artifact handed back a member the declaration did not name.
    UnexpectedMember {
        /// The member's name.
        member: String,
    },
    /// The artifact handed one member back more than once.
    DuplicateMember {
        /// The member's name.
        member: String,
    },
    /// The artifact did not hand back a member the declaration names.
    MissingMember {
        /// The member's name.
        member: String,
    },
    /// A member read back as a value the declaration did not state.
    MemberValue {
        /// The member's name.
        member: String,
    },
}

/// What one compiled read-back concluded.
#[must_use = "a verdict is what the compiled read-back concluded"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledVerdict {
    /// The compiled artifact behaves exactly as the caller declared it would.
    Conforms,
    /// The compiled artifact and the declaration disagree, about this.
    Deviates(CompiledDisagreement),
}

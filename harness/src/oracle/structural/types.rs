//! The structural path, declaration, observation, and verdict vocabulary.

#[path = "type_guard.rs"]
mod guard;

/// Whether a structural path carries a leading separator or begins in its surrounding scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralPathRoot {
    /// The path carries no leading `::`.
    Relative,
    /// The path carries a leading `::`.
    Absolute,
}

/// One indivisible segment of a structural path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructuralPathSegment(String);

/// One complete path as the structural method reads or declares it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructuralPath {
    root: StructuralPathRoot,
    segments: Vec<StructuralPathSegment>,
}

/// Why a structural path was refused.
#[must_use = "a refusal is the reason a structural path was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralPathRefusal {
    /// No segments were offered, so the path names nothing.
    NoSegments,
    /// One segment is empty.
    EmptySegment {
        /// The segment's position in the offered roster.
        at: usize,
    },
    /// One segment already contains `::`, so it is not one segment.
    EmbeddedSeparator {
        /// The segment's position in the offered roster.
        at: usize,
    },
}

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
/// This is syntax and never meaning: whether a path resolves, typechecks, or evaluates to what it suggests is the compiled read-back's question.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstantReading {
    /// A complete path expression.
    Path(StructuralPath),
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
    /// A call: the complete path called, and the readings of its arguments in order.
    Call {
        /// The path the call calls.
        path: StructuralPath,
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
    /// The complete path of the type the implementation targets.
    pub target: StructuralPath,
    /// The complete trait path the implementation realizes, or no path for an inherent implementation.
    pub trait_path: Option<StructuralPath>,
    /// The postures the implementation is written under, in roster order.
    pub postures: Vec<ImplPosture>,
    /// Every attribute that decides something, on the implementation or on one of its members, by complete path; doc comments decide nothing and never appear.
    pub meaning_bearing_attributes: Vec<StructuralPath>,
    /// The members the implementation carries, in order, with a member stated twice appearing twice rather than written over.
    pub members: Vec<ImplementationMember>,
}

/// Everything the structural read recovered from one rendered artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactStructure {
    /// The implementations the artifact declares, in declaration order.
    pub implementations: Vec<ImplementationStructure>,
    /// How many items the artifact declares that are not complete implementations of a named type.
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

/// A duplicate-free roster of members a caller states one implementation will carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredMemberRoster<'spec> {
    members: &'spec [DeclaredMember<'spec>],
}

/// Why a declared structural member roster was refused.
#[must_use = "a refusal is the reason a declared member roster was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclaredMemberRosterRefusal {
    /// One member name appears more than once.
    DuplicateMember {
        /// The second member's position in the offered roster.
        at: usize,
    },
}

/// One implementation a caller states an artifact will declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredImplementation<'spec> {
    /// The complete path of the type this implementation targets.
    pub target: &'spec StructuralPath,
    /// The complete trait path this implementation realizes, or no path for an inherent implementation.
    pub trait_path: Option<&'spec StructuralPath>,
    /// The postures this implementation is written under, so a plain implementation names none and any posture at all is a finding.
    pub postures: &'spec [ImplPosture],
    /// The attributes the declaration admits on this implementation or on one of its members, by complete path.
    pub attributes: &'spec [StructuralPath],
    /// The duplicate-free members this implementation carries, and what each states.
    pub members: DeclaredMemberRoster<'spec>,
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
    /// The artifact declares an item that is not a complete implementation of a named type.
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
        /// The attribute's complete path spelling.
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

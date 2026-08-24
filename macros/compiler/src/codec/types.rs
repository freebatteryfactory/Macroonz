//! The codec home's declarations: the kind, the shape a codec is written for, the wire vocabulary its members stand under, where the surface lands, and how a declaration of any of it refuses.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes this home's walls structural: a shape is seated by one road that refuses an empty roster, a doubled spelling, and a spelling the decode road has already taken, and there is no second road that seats one.

use crate::bounded::{Bounded, Capped, NonEmpty};
use crate::explanation::ASSUMPTION_LIMIT;
use crate::identity::{OwnerFact, OwnerIdentity};

#[path = "type_guard.rs"]
mod guard;

/// Members one declared shape may carry.
///
/// Every member is one framed run in the encode road and one bound local in the decode road, so a shape past this has stopped being one value's spelling; the repair is a nested member carrying its own codec rather than a longer roster here.
pub const CODEC_MEMBER_LIMIT: usize = 64;

/// Segments one rendered type path may carry.
///
/// A path reaching deeper has stopped naming an item and started describing a tree, and the repair is a re-export at the address rather than a longer spelling at this end.
pub const CODEC_PATH_SEGMENT_LIMIT: usize = 8;

/// Issues one codec refusal carries before it begins counting the rest.
///
/// One per member seat, because the widest pass asks one question of every declared member and every member can answer it at once — and a caller repairing a shape one member per attempt is a caller this home failed.
pub const CODEC_ISSUE_LIMIT: usize = 64;

/// The road a rendered surface writes one value's canonical bytes by, and the road a nested member's own type is billed for.
pub const ENCODE_ROAD: &str = "encode_canonical";

/// The road a rendered surface reads those bytes back by.
pub const DECODE_ROAD: &str = "decode_canonical";

/// The roster constant a closed choice's admitted arms are walked through.
pub const ROSTER_CONSTANT: &str = "ALL";

/// The road one arm of a closed roster answers its declared position through, one byte wide because a choice is written as one byte.
pub const SLOT_ROAD: &str = "slot";

crate::roster! {
    /// How many of one member there are.
    ///
    /// Three rows and no fourth: the wire roads below stand over ONE occurrence whatever supplied it, so a cardinality decides how many times a road runs and never what the road writes.
    pub enum Cardinality {
        /// Exactly one, written where the shape declares it.
        Required = "required",
        /// One or none, written behind a presence byte.
        Optional = "optional",
        /// As many as the value holds, written behind a framed count.
        Repeated = "repeated",
    }
}

crate::roster! {
    /// The wire shape one member is written under.
    ///
    /// Five rows, every one a shape the rendering can write end to end: there is no opaque arm, because a member the rendering could not write would be a member whose bytes nobody could re-read.
    pub enum CodecMemberShape {
        /// A count, carried at the framing width and narrowed back at the member's own type on the way in.
        Count = "count",
        /// Variable-length bytes, written length-prefixed.
        Bytes = "bytes",
        /// Variable-length text, written length-prefixed as its UTF-8 bytes and read back through a check that refuses.
        Text = "text",
        /// One arm of a closed roster, written as that arm's own declared position and read back by walking the roster the owner declared.
        ClosedChoice = "closed-choice",
        /// A nested value carrying its own codec, framed at its own length so the member after it stays readable.
        Nested = "nested",
    }
}

crate::roster! {
    /// Which of the two roads a request covers.
    pub enum CodecDirection {
        /// Typed value to canonical bytes.
        Encode = "encode",
        /// Canonical bytes to typed value.
        Decode = "decode",
        /// Both, so neither can drift from the other.
        RoundTrip = "round-trip",
    }
}

crate::roster! {
    /// Where one rendered type path is rooted.
    ///
    /// Two rows and neither is a default: a path spelled from the caller's crate root and a path resolved wherever the surface lands are two different claims, and a rendering that guessed would put the wrong one in somebody else's crate.
    pub enum PathRooting {
        /// Rooted at the caller's own crate, so the path resolves the same wherever the surface lands.
        CrateAbsolute = "crate-absolute",
        /// Resolved in the scope the surface lands in, exactly as the caller spelled it.
        InScope = "in-scope",
    }
}

crate::roster! {
    /// One arm of the decode refusal this home renders, by the spelling it is rendered under.
    ///
    /// The declared name is the arm's own Rust spelling, because this roster IS the rendered type's variant list.
    /// Which arms name the member a read was standing at is said once, at `carries_member`.
    pub enum DecodeRefusal {
        /// The material ended inside a member.
        Truncated = "Truncated",
        /// A declared length runs past the material that remains.
        LengthPastRemaining = "LengthPastRemaining",
        /// A declared length does not fit an addressable width.
        LengthPastAddressableWidth = "LengthPastAddressableWidth",
        /// A declared count does not fit the width its member is held at.
        CountPastDeclaredWidth = "CountPastDeclaredWidth",
        /// Framed bytes that were to be text are not UTF-8.
        TextNotUtf8 = "TextNotUtf8",
        /// The member's own type refused what was read for it.
        MemberNotAdmitted = "MemberNotAdmitted",
        /// A slot names no arm of the roster it was declared over.
        SlotNotAdmitted = "SlotNotAdmitted",
        /// A nested codec refused the framed material.
        NestedMemberRefused = "NestedMemberRefused",
        /// A presence byte is neither of the two the encode road writes.
        PresenceNotAdmitted = "PresenceNotAdmitted",
        /// Material remains after the last declared member.
        TrailingBytes = "TrailingBytes",
        /// Every member was read and the road that assembles them refused.
        NotAssembled = "NotAssembled",
    }
}

/// One wire shape's bill: the roads the rendered surface calls on a member's own type.
///
/// A road named through a trait is written qualified, with `T` standing for the member's own type; a road named on that type itself is written bare.
///
/// # Authority
///
/// **The bill is stated and never worked around.**
/// A member the rendering could not write end to end would be a member whose bytes nobody could re-read, so the rendering does not degrade: it calls the roads named here, and where one is absent the failure lands at the caller's site as an ordinary unresolved method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberContract {
    /// The wire shape this row is about.
    pub shape: CodecMemberShape,
    /// The road the encode surface calls to read the member out.
    pub encode_road: &'static str,
    /// The road the decode surface calls to build the member back.
    pub decode_road: &'static str,
}

/// One type path a rendered expression names.
///
/// # Bounds
///
/// The segments are structurally non-empty: a path naming no segment names nothing, and a rendering that wrote one would emit a bare separator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecTypePath {
    rooting: PathRooting,
    segments: NonEmpty<String, CODEC_PATH_SEGMENT_LIMIT>,
}

/// One member of a declared shape: what the owner calls it, the type it is held at, how it is written, and how many of it there are.
///
/// # Bounds
///
/// The spelling is one Rust identifier by construction, because the decode road binds a local under it and the encode road reads a field under it — and a spelling that is not one renders tokens the caller's compiler reads as something else.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecMember {
    spelling: String,
    held_as: CodecTypePath,
    shape: CodecMemberShape,
    cardinality: Cardinality,
}

/// What the decode road does with the members once it has read them all.
///
/// Not an option and not a default: a total constructor and a checked one are called differently, and a rendering that guessed would either drop a refusal the owner declared or write a `?` on a value that is not a `Result`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssemblyPosture {
    /// The road is total: every member the decode road read is an argument, and there is nothing left to refuse.
    Total,
    /// The road is checked, and this is the refusal it answers with.
    Checked {
        /// The refusal the assembly road answers with, carried into the rendered one by a conversion this home writes.
        refusal: CodecTypePath,
    },
}

/// The road one decoded value is assembled by, and the posture it stands under.
///
/// # Bounds
///
/// The road is an associated road on the owner's own type, so a free function is unwritable here rather than refused: that is the shape a decode road can call without learning where the owner's module sits.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecAssembly {
    road: String,
    posture: AssemblyPosture,
}

/// The complete declared shape one codec is rendered for.
///
/// # Bounds
///
/// The member set is structurally non-empty: a codec over no member writes no byte and reads none, so it could refuse for one reason and would admit every other input — and a codec that cannot refuse is not the validator this home says a codec is.
///
/// The rendered refusal's spelling is carried rather than derived, because it is a type declared in the caller's own scope and this home may not choose a name there.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecShape {
    owner: CodecTypePath,
    refusal: String,
    assembly: CodecAssembly,
    members: NonEmpty<CodecMember, CODEC_MEMBER_LIMIT>,
}

/// The spelling one visibly published module is declared under.
///
/// # Bounds
///
/// One Rust identifier by construction: the module lands in the caller's own scope and shares a namespace with every other item there.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleSpelling {
    spelling: String,
}

/// What shape the rendered surface lands in.
///
/// # Bounds
///
/// Both rows are declaration-site deliveries, so the delivery is the seat's constant answer under either and what this decides is the surface's shape alone.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodecPlacement {
    /// Spliced beside the owner's own item, in the scope the declaration sits in.
    AtDeclarationSite,
    /// Wrapped in a visibly published module, whose head imports the scope the module sits in.
    PublishedModule {
        /// The module's declared spelling.
        spelling: ModuleSpelling,
    },
}

/// What one codec request carries beyond its captured tokens.
///
/// # Nonclaims
///
/// The schema and the byte role reach no token of the rendered surface: the schema is what the codec is projected FROM and the shape is what it is written FOR, and the framing is this home's whatever the bytes are called.
/// They travel so an explanation can name them and a caller can join the surface back to the declaration it answers to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecContent {
    /// The declared shape the two roads are written over.
    pub shape: CodecShape,
    /// Which of the two roads the surface carries.
    pub direction: CodecDirection,
    /// The shape the rendered surface lands in.
    pub placement: CodecPlacement,
    /// The schema the codec is projected from, where the caller minted one.
    pub schema: Option<OwnerIdentity>,
    /// The byte role naming which bytes these are, where the caller minted one.
    pub byte_role: Option<OwnerIdentity>,
    /// The owner facts this projection assumes.
    pub assumptions: Bounded<OwnerFact, ASSUMPTION_LIMIT>,
}

/// Projects a declared shape into the codec that reads and writes its canonical bytes.
///
/// One rendered unit, at the declaration site: the surface is Rust the caller's normal build compiles, whether it is spliced beside the owner's item or wrapped in a published module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodecProjection;

/// How one declaration of this home's vocabulary refuses.
///
/// No row is payload-free where a payload would tell a caller what to repair: a row names the spelling it refused at, or the two counts a magnitude was passed by.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodecIssue {
    /// A rendered type path names no segment, so it names nothing.
    PathSegmentsAbsent,
    /// A path segment cannot name a rendered item — not one Rust identifier, or a keyword the language already took — so the rendering would write tokens the caller's compiler reads as something else.
    SegmentNotAnIdentifier {
        /// The segment as it was spelled.
        segment: String,
    },
    /// A path carries more segments than the declared magnitude.
    PathSegmentsUnbounded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
    /// A member states no spelling, so nothing names it in either road.
    MemberSpellingAbsent,
    /// A member's spelling cannot name a rendered item: not one Rust identifier, or a keyword the language already took.
    MemberSpellingNotAnIdentifier {
        /// The spelling as it was stated.
        spelling: String,
    },
    /// Two members of one shape carry one spelling, so the decode road would bind one local twice and the assembly would be handed the second.
    MemberSpellingDoubled {
        /// The spelling two members share.
        spelling: String,
    },
    /// A member's spelling is one of the locals the decode road declares for itself, so the member's binding would shadow the rendering's and the road would go on reading a value nobody meant.
    MemberShadowsBinding {
        /// The member that collided.
        spelling: String,
        /// The binding it collided with.
        binding: &'static str,
    },
    /// An assembly road states no spelling.
    AssemblyRoadAbsent,
    /// An assembly road's spelling cannot name a rendered item: not one Rust identifier, or a keyword the language already took.
    AssemblyRoadNotAnIdentifier {
        /// The spelling as it was stated.
        spelling: String,
    },
    /// A rendered decode refusal's spelling cannot name a rendered item: not one Rust identifier, or a keyword the language already took.
    RefusalSpellingNotAnIdentifier {
        /// The spelling as it was stated.
        spelling: String,
    },
    /// A published module's spelling cannot name a rendered item: not one Rust identifier, or a keyword the language already took.
    ModuleSpellingNotAnIdentifier {
        /// The spelling as it was stated.
        spelling: String,
    },
    /// A shape declares no member at all.
    MembersAbsent,
    /// A shape declares more members than the declared magnitude.
    MembersUnbounded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// How declaring a codec says no.
///
/// The passes that reach it are dependent in a stated order, so exactly one cause is true of a refused path, spelling, or road — while the pass over a shape's members co-establishes freely, because a caller told about one colliding member and not the next is a caller who repairs a shape one attempt at a time.
#[must_use = "a codec refusal names the exact seat the declaration did not fill"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecError {
    body: Capped<CodecIssue, CODEC_ISSUE_LIMIT>,
}

/// The one alphabet every spelling this home renders as a Rust identifier is admitted by, published from the nucleus every road here already reads it through.
pub use guard::{rendered_identifier, rendered_name};

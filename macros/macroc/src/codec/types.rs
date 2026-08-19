//! The codec home's declarations: the type paths a rendered expression names,
//! the declared shape a codec is written for, the wire shape and cardinality
//! each member is written under, where the surface lands, the composed surface
//! itself, and the magnitudes and refusal families this home answers through.
//!
//! Declarations only.
//! Every road that reaches a private field — a path's segments and rooting, a
//! member's spelling, a shape's members and its assembly road, a placement's
//! module spelling, the surface's composition, and the refusal body's one seat —
//! lives in `type_guard.rs`, this file's own child.
//!
//! # Nothing here decides how a value is written
//!
//! The wire shape of a member, the cardinality it stands under, and the road the
//! members are assembled by all arrive from the caller. The plan names a schema,
//! a byte role, and a direction; a generator that decided a member was bytes
//! rather than text would be declaring how somebody else's value is written down
//! and then encoding it that way.
//!
//! The CARDINALITY roster is the machine's ([`FieldCardinality`]), imported
//! rather than restated, on the charter's terms: every roster these services
//! speak is the machine's.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    AssumptionLimit, ByteRoleSubject, GeneratedUnitSubject, GeneratorVersionSubject, OwnerFactRef,
    OwnerIdentityRef, ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, SchemaSubject,
    SoleRenderedUnit,
};
use crate::planning::{CauseAnchoring, CodecDirection};
use crate::token::GeneratedTree;
use threadpak::schema::FieldCardinality;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many members one declared codec shape may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Sixty-four. Every member is one framed run in the encode road and one
    /// bound local in the decode road, so a shape past sixty-four has stopped
    /// being one value's spelling and started being a record nobody reads in one
    /// sitting — and the repair is a NESTED member carrying its own codec, not a
    /// longer roster here.
    ///
    /// # Nonclaims
    ///
    /// It is this home's own family because a codec shape is this home's
    /// capacity. The plane's rows are the magnitudes more than one home asks
    /// about; this one nobody else asks, so it is declared beside what it
    /// governs.
    CodecMemberLimit = 64,
    /// The magnitude governing how many segments one rendered type path may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Eight. A path reaching deeper than eight segments has stopped naming an
    /// item and started describing a tree, and the repair is a re-export at the
    /// address rather than a longer spelling at this end.
    ///
    /// Declared here rather than borrowed from the test-descriptor home's own
    /// path family: that one is rooted at a rename twin and crosses the wall,
    /// and a path this home writes is rooted in the consumer's own crate. One
    /// family standing for both would be one authority answering two questions.
    CodecPathSegmentLimit = 8,
    /// The magnitude governing how many issues one codec-composition refusal
    /// body may carry.
    ///
    /// # Bounds
    ///
    /// Sixty-four — one per member seat, because the widest pass is the binding
    /// pass and that pass asks one question of every declared member: whether
    /// the local the decode road would bind for it collides with a binding the
    /// rendering declares itself. Every member can collide at once, and a caller
    /// repairing a shape one member per attempt is a caller this home failed.
    ///
    /// Written as the number rather than as the member magnitude read a second
    /// time: a magnitude derived from another magnitude reads as a fact when it
    /// is a choice, and this home would still owe the same number if the member
    /// magnitude moved for its own reasons.
    CodecSurfaceIssueLimit = 64,
}

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a path's segments are read before a member's
    /// spelling, a member's spelling before a shape's members, and a shape's
    /// members before the placement it lands under.
    /// Every one of them refuses before a partial value exists — a shape holding
    /// some of its members is a codec for a value nobody declared.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum CodecDeclarationRefusal {
        /// The path names no segment at all, so it names nothing.
        PathSegmentsAbsent = "path-segments-absent",
            "a rendered type path names no segment";
        /// The path carries more segments than the declared magnitude.
        PathSegmentsUnbounded = "path-segments-unbounded",
            "a rendered type path carries more segments than the declared magnitude";
        /// A path segment is not one Rust identifier, so the rendering would
        /// write tokens the consumer's compiler reads as something else.
        SegmentNotAnIdentifier = "segment-not-an-identifier",
            "a rendered path segment is not one Rust identifier";
        /// The member states no spelling, so nothing names it in either road.
        EmptyMemberSpelling = "empty-member-spelling",
            "a codec member states no spelling";
        /// The member's spelling is not one Rust identifier.
        MemberSpellingNotAnIdentifier = "member-spelling-not-an-identifier",
            "a codec member spelling is not one Rust identifier";
        /// The assembly road states no spelling.
        EmptyAssemblyRoad = "empty-assembly-road",
            "a codec assembly road states no spelling";
        /// The assembly road's spelling is not one Rust identifier.
        AssemblyRoadNotAnIdentifier = "assembly-road-not-an-identifier",
            "a codec assembly road is not one Rust identifier";
        /// The rendered refusal's spelling is not one Rust identifier.
        RefusalSpellingNotAnIdentifier = "refusal-spelling-not-an-identifier",
            "a rendered decode refusal spelling is not one Rust identifier";
        /// The shape declares no member at all.
        ///
        /// A codec over no member writes no byte and reads none, so its decode
        /// road can refuse for exactly one reason and admits every other input —
        /// and a codec that cannot refuse is not the validator this home says a
        /// codec is.
        MembersAbsent = "members-absent",
            "a codec shape declares no member";
        /// The shape declares more members than the declared magnitude.
        MembersUnbounded = "members-unbounded",
            "a codec shape declares more members than the declared magnitude";
        /// Two members of one shape carry one spelling, so the decode road would
        /// bind one local twice and the assembly would be handed the second.
        MemberSpellingDoubled = "member-spelling-doubled",
            "two members of one codec shape carry one spelling";
        /// The published module's spelling is not one Rust identifier.
        ModuleSpellingNotAnIdentifier = "module-spelling-not-an-identifier",
            "a published codec module spelling is not one Rust identifier";
    }
}

// ---------------------------------------------------------------------------
// The rendered vocabulary.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// Where one rendered type path is rooted.
    ///
    /// A closed roster of exactly two, and neither is a default: a path spelled
    /// from the consumer's crate root and a path resolved in whatever scope the
    /// surface lands in are two different claims about where a name comes from,
    /// and a rendering that guessed would put the wrong one in somebody else's
    /// crate.
    pub enum PathRooting {
        /// Rooted at the consumer's own crate: the rendering writes a leading
        /// `::`, so the path resolves the same wherever the surface lands.
        CrateAbsolute = "crate-absolute",
            "rooted at the consumer's crate, written with a leading path separator";
        /// Resolved in the scope the surface lands in, exactly as the caller
        /// spelled it.
        InScope = "in-scope",
            "resolved in the scope the rendered surface lands in";
    }
}

threadpak::closed_register! {
    /// The wire shape one codec member is written under.
    ///
    /// A closed roster of exactly five, and every arm is a shape the rendering
    /// can actually write end to end. There is no "opaque" arm and no escape
    /// hatch: a member the rendering could not write would be a member whose
    /// bytes nobody could re-read, which is the one thing a canonical encoding
    /// may not admit.
    ///
    /// What each arm demands of the member's own type is stated once, as
    /// `MEMBER_CONTRACT` in `type_contract.rs`, rather than as a sentence each
    /// reader re-derives.
    pub enum CodecMemberShape {
        /// A count, carried at the framing width and narrowed back at the
        /// member's own type on the way in.
        Count = "count",
            "a count, carried at the framing width";
        /// Variable-length bytes, written length-prefixed.
        Bytes = "bytes",
            "variable-length bytes, written length-prefixed";
        /// Variable-length text, written length-prefixed as its UTF-8 bytes and
        /// read back through a UTF-8 check that refuses.
        Text = "text",
            "variable-length text, written length-prefixed as UTF-8";
        /// One arm of a closed roster, written as that arm's own declared slot
        /// and read back by walking the roster the owner declared.
        ClosedChoice = "closed-choice",
            "one arm of a closed roster, written as its declared slot";
        /// A nested value carrying its own codec, framed at its own length so the
        /// member after it stays readable.
        Nested = "nested",
            "a nested value carrying its own codec, framed at its own length";
    }
}

threadpak::closed_register! {
    /// One of the two roads a codec surface renders.
    ///
    /// The roster is what a direction is asked about: `covers` in
    /// `type_contract.rs` is a constant table over this roster and
    /// [`CodecDirection`], so "an encode-only codec renders no reader" is a value
    /// a reader can read back and a match the compiler keeps exhaustive.
    pub enum CodecRoad {
        /// The road that writes one declared shape's canonical bytes.
        Encode = "encode", "the road that writes one shape's canonical bytes";
        /// The road that reads them back, and refuses where they are not the
        /// shape's.
        Decode = "decode", "the road that reads canonical bytes back, and refuses";
    }
}

/// One type path a rendered expression names.
///
/// # Bounds
///
/// The segments are structurally non-empty: a path naming no segment names
/// nothing, and a rendering that wrote one would emit a bare `::`.
///
/// The parts are OWNED text, where a `'static` roster would be this crate's own:
/// a path here is cut from the token material one expansion was handed, and it
/// becomes static text only once it is spliced into the consumer's own item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecTypePath {
    rooting: PathRooting,
    segments: NonEmptyBounded<String, CodecPathSegmentLimit>,
}

/// One member of a declared codec shape: what the owner calls it, the type it is
/// held at, how it is written, and how many of it there are.
///
/// # Authority
///
/// **Every seat arrives from the caller and none is derived here.** Which
/// members a value has, what each one is held at, and how each is written down
/// are the owner's declarations; a generator that decided any of them would be
/// producing its own facts and then encoding them.
///
/// # Bounds
///
/// The spelling is one Rust identifier by construction, because the decode road
/// binds a local under it and the encode road reads a field under it. A spelling
/// that is not an identifier renders tokens the consumer's compiler reads as
/// something else, and the place that failure would surface is a consumer's
/// build with no idea where the name came from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecMember {
    spelling: String,
    /// The type ONE OCCURRENCE of this member is held at — never the collection
    /// or the option a cardinality wraps it in. The five shape roads stand over
    /// one occurrence whatever the cardinality supplied it, so a member's type
    /// path names the same thing under all three.
    held_as: CodecTypePath,
    shape: CodecMemberShape,
    cardinality: FieldCardinality,
}

/// What the decode road does with the members once it has read them all.
///
/// Not an option and not a default: a total constructor and a checked one are
/// called differently — plain, and with the language's own `?` — and a rendering
/// that guessed would either drop a refusal the owner declared or write a `?` on
/// a value that is not a `Result`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssemblyPosture {
    /// The assembly road is total: every member the decode road read is an
    /// argument, and there is nothing left to refuse.
    Total,
    /// The assembly road is checked, and this is the refusal it answers with.
    ///
    /// The rendered decode refusal carries that refusal beside a `From`
    /// implementation this home writes, so a checked assembly costs the address
    /// nothing: the conversion is rendered rather than billed.
    Checked {
        /// The refusal the assembly road answers with.
        refusal: CodecTypePath,
    },
}

/// The road one decoded value is assembled by, and the posture it stands under.
///
/// # Bounds
///
/// The road is an associated road on the owner's own type — the rendering writes
/// `<Owner>::<road>(…)` — so a free function is unwritable here rather than
/// refused. That is the shape a decode road can call without learning where the
/// owner's module sits.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecAssembly {
    road: String,
    posture: AssemblyPosture,
}

/// The complete declared shape one codec is rendered for.
///
/// # Bounds
///
/// The member set is structurally non-empty, for the reason
/// [`CodecDeclarationRefusal::MembersAbsent`] states: a codec over no member
/// cannot refuse, and a codec that cannot refuse is not the validator this home
/// says a codec is.
///
/// The rendered refusal's spelling is carried rather than derived, because it is
/// a type declared in the consumer's own scope and this home may not choose a
/// name there. It is one Rust identifier by construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecShape {
    owner: CodecTypePath,
    refusal: String,
    assembly: CodecAssembly,
    members: NonEmptyBounded<CodecMember, CodecMemberLimit>,
}

/// Where one rendered codec surface lands.
///
/// # Bounds
///
/// Both arms are EXPANSION deliveries, so the plan's destination is the
/// declaration site under either: what the placement decides is the surface's
/// SHAPE, never a second destination. A planned member written as a standalone
/// artifact is a different delivery and is refused before a surface exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodecPlacement {
    /// Spliced beside the owner's own item, in the scope the declaration sits
    /// in.
    AtDeclarationSite,
    /// Wrapped in a visibly published module of this spelling, whose head writes
    /// the one import a wrapped surface needs.
    PublishedModule {
        /// The module's declared spelling — one Rust identifier by construction.
        spelling: ModuleSpelling,
    },
}

/// The spelling one visibly published codec module is declared under.
///
/// # Bounds
///
/// One Rust identifier by construction. The module lands in the consumer's own
/// scope and shares a namespace with every other item there, so a spelling that
/// is not an identifier is a compile error in somebody else's crate with no sign
/// of where it came from.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleSpelling {
    spelling: String,
}

/// What a codec plan decided, read off the plan's own public surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or its direction would be an account that sometimes
/// says less than it knows. There is no private field here and this home's
/// invariant nucleus holds nothing of it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its kind's
/// one rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecPlan {
    /// The rendered role the surface stands for.
    pub role: SoleRenderedUnit,
    /// The planned member's semantic key, exactly as the plan declared it.
    pub semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    /// The profile the plan expects to render it.
    pub profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// The member's origin trail, walked back to authored material.
    pub origin: OriginTrail,
    /// The ONE address the entry account walked in the door carrying.
    pub declaration: CauseAnchoring,
    /// The rendering engine the surface is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// The schema the codec is projected from.
    ///
    /// # Bounds
    ///
    /// It reaches no token of the rendered surface. The schema is what the codec
    /// is projected FROM and the shape is what the codec is written FOR, and the
    /// two are separate facts: this one travels for the explanation station and
    /// for a caller joining the surface back to the declaration it answers to.
    pub schema: OwnerIdentityRef<SchemaSubject>,
    /// The byte role the codec reads or writes.
    ///
    /// # Bounds
    ///
    /// It reaches no token either, on the same terms. The role names which bytes
    /// these are; the framing names how they are cut, and the framing is this
    /// home's.
    pub byte_role: OwnerIdentityRef<ByteRoleSubject>,
    /// The direction covered — which of this home's two roads are rendered.
    pub direction: CodecDirection,
    /// The owner facts this projection assumes.
    pub assumptions: Bounded<OwnerFactRef, AssumptionLimit>,
}

/// The rendered codec surface's typed description.
///
/// # Bounds
///
/// There is no destination seat: both admitted placements are expansion
/// deliveries, so the answer is a constant ([`CodecSurface::DESTINATION`]) rather
/// than a seat that could say something else.
///
/// The remaining seats are exactly what a rendered unit is rebuilt from — role,
/// semantic key, profile at its version, origin trail, and the tree — plus the
/// placement, which is a fact about THIS rendering and is therefore read back
/// rather than recomputed by a caller.
#[must_use = "a codec surface is the encode and decode roads one declared shape is written by"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecSurface {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    placement: CodecPlacement,
    covered: CodecDirection,
    tree: GeneratedTree,
}

// ---------------------------------------------------------------------------
// The composition refusal family.
// ---------------------------------------------------------------------------

/// How composing a codec surface disagrees with the plan, with the declared
/// shape, or with what the token magnitude admits.
///
/// No issue is payload-free: an issue names the role, the member, or the bound
/// it is about, because a caller told only that composition failed has nothing to
/// repair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodecSurfaceIssue {
    /// The plan declares no member under its kind's one rendered role, so there
    /// is no surface to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands somewhere other than the declaration site.
    ///
    /// Both admitted placements are expansion deliveries — spliced beside the
    /// owner's item, or wrapped in a visibly published module — so a codec
    /// surface belongs in the tokens the consumer's normal build compiles.
    /// The destination roster names four deliveries, and a member that is not at
    /// the declaration site declared one of the other three: a standalone
    /// artifact a publication writes to its own address, the deferred cargo a
    /// test target invokes, or the deferred cargo a bench target invokes. Each
    /// of the three is a different delivery and each establishes this issue.
    DestinationNotDeclarationSite {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
    /// A member's spelling is one of the locals the decode road declares for
    /// itself, so the member's own binding would shadow the rendering's and the
    /// road would go on reading a value nobody meant.
    ///
    /// The roster it collided with is `RESERVED_BINDINGS` in `type_contract.rs`,
    /// stated once so a caller reads which names are taken rather than
    /// discovering them one refusal at a time.
    MemberShadowsRenderedBinding {
        /// The member that collided.
        member: String,
        /// The binding it collided with.
        binding: &'static str,
    },
    /// The rendered surface outgrows the declared token magnitude.
    /// A surface carrying both roads over every member is the widest tree this
    /// home writes, and it refuses rather than materializing part of one.
    SurfaceTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

/// The codec-composition refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared beside the rest of this home's declarations would put all of them
/// inside the same wall.
pub use guard::CodecComposition;

/// The one alphabet every spelling this home renders as a Rust identifier is
/// admitted by, published from the nucleus every road here already reads it
/// through.
pub use guard::is_codec_identifier;

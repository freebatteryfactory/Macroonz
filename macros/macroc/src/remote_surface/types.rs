//! The remote-surface home's declarations: the type paths a rendered expression
//! names, the codec pairing a surface rides, the signature the rendered road
//! stands at, the declared shape a surface is written for, where a surface
//! lands, what a plan decided, what this home is available FOR, the composed
//! surface itself, and the magnitude and refusal families this home answers
//! through.
//!
//! Declarations only.
//! Every road that reaches a private field — a path's segments and rooting, a
//! pairing's two roads, a signature's three paths, a shape's port road and entry
//! spelling, the landing's byte role, and the surface's composition — lives in
//! `type_guard.rs`, this file's own child.
//!
//! # What the plan actually carries, and nothing beside it
//!
//! [`RemoteSurfaceContent`](crate::planning::RemoteSurfaceContent) names exactly
//! three facts: the PORT declaration projected, the WIRE CONTRACT spoken, and
//! which way the surface FACES. It names no type, no road, no signature, and — in
//! particular — **no codec**. So the pairing a surface rides arrives from the
//! CALLER as [`CodecPairing`], and this home derives none of it: the codec that
//! reads and writes a wire contract's bytes is its own projection over its own
//! plan, and a generator that elected one here would be pairing somebody else's
//! surface with a reader nobody asked for.
//!
//! # The facing is the one seat the rendering turns on
//!
//! [`SurfaceDirection`](crate::planning::SurfaceDirection) decides which of the
//! pairing's two roads opens the rendered road and which closes it, and that is
//! the whole of what a facing changes: an inbound surface reads the wire and
//! answers with wire material, an outbound surface writes the wire and answers
//! with a value. The table is stated once as `facing` in `type_contract.rs`.
//!
//! # The outside road is not open, and the vocabulary says so
//!
//! [`SurfaceAvailability`] is a typed reading of what this home is available for,
//! and [`SurfaceContractMint`] is the standing of the mint that would open it.
//! Neither is a crippled surface that answers anyway.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ByteRoleSubject, GeneratedUnitSubject, GeneratorVersionSubject, OwnerIdentityRef,
    PortSubject, ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit,
    WireContractSubject,
};
use crate::planning::{CauseAnchoring, SurfaceDirection};
use crate::token::GeneratedTree;
use threadpak::declaration::types::ProjectionTargetDomain;
use threadpak::types::NonEmptyBounded;

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitude.
// ---------------------------------------------------------------------------

/// The magnitude governing how many segments one rendered type path may carry.
///
/// # Bounds
///
/// Eight. A path reaching deeper than eight segments has stopped naming an item
/// and started describing a tree, and the repair is a re-export at the address
/// rather than a longer spelling at this end.
///
/// The authority and the number are written together in `type_contract.rs`, one
/// row per family, so a family cannot stand on the compile-time ladder while
/// wearing another road's authority.
///
/// # Nonclaims
///
/// It is this home's own family. Three rendering homes now declare a path
/// magnitude of their own — the test-descriptor home's is rooted at a rename twin
/// and crosses the wall, the codec home's is rooted in the consumer's own crate
/// beside an owner's item, and this one is rooted in an INTEGRATION TARGET that
/// names both a port realization and a codec from outside the crate that declared
/// either. One family standing for all three would be one authority answering
/// three questions, and the day one road has to reach deeper than the others is
/// the day that would show.
///
/// The promotion is real and it is not an edit: a single path magnitude on the
/// plane's own roster, read by every rendering home, is a decision about where the
/// capacity lives, and it is owed the same argument the plane's other magnitudes
/// carry. Until it is made, three families that agree on a number are three
/// authorities that happen to agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfacePathSegmentLimit;

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a path's segments are read before a pairing's roads,
    /// a pairing's roads before the port's road, and the port's road before the
    /// entry spelling.
    /// Every one of them refuses before a partial value exists — a pairing
    /// holding one of its two roads is a codec nobody can both write and read
    /// through.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum RemoteSurfaceDeclarationRefusal {
        /// The path names no segment at all, so it names nothing.
        PathSegmentsAbsent = "path-segments-absent",
            "a rendered type path names no segment";
        /// The path carries more segments than the declared magnitude.
        PathSegmentsUnbounded = "path-segments-unbounded",
            "a rendered type path carries more segments than the declared magnitude";
        /// A path segment is not one Rust identifier, so the rendering would
        /// write tokens the integration target's compiler reads as something
        /// else.
        SegmentNotAnIdentifier = "segment-not-an-identifier",
            "a rendered path segment is not one Rust identifier";
        /// One of the pairing's two roads states no spelling.
        EmptyPairingRoad = "empty-pairing-road",
            "a codec pairing road states no spelling";
        /// One of the pairing's two roads is not one Rust identifier.
        PairingRoadNotAnIdentifier = "pairing-road-not-an-identifier",
            "a codec pairing road is not one Rust identifier";
        /// The pairing's two roads carry one spelling.
        ///
        /// A surface calls one of them on the way in and the other on the way
        /// out, so a pairing that spelled them alike would write the wire with
        /// the road it then read the wire with — and the rendered road would
        /// compile while meaning something nobody declared.
        PairingRoadsDoubled = "pairing-roads-doubled",
            "a codec pairing spells its two roads alike";
        /// The port states no road, so nothing is called between the two codec
        /// roads.
        EmptyPortRoad = "empty-port-road",
            "a remote surface states no port road";
        /// The port's road is not one Rust identifier.
        PortRoadNotAnIdentifier = "port-road-not-an-identifier",
            "a remote surface port road is not one Rust identifier";
        /// The entry states no spelling, so the rendered surface has no name.
        EmptyEntrySpelling = "empty-entry-spelling",
            "a remote surface entry states no spelling";
        /// The entry's spelling is not one Rust identifier.
        EntrySpellingNotAnIdentifier = "entry-spelling-not-an-identifier",
            "a remote surface entry spelling is not one Rust identifier";
    }
}

// ---------------------------------------------------------------------------
// The rendered vocabulary.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// Where one rendered type path is rooted.
    ///
    /// A closed roster of exactly two, and neither is a default: a path spelled
    /// from a crate root and a path resolved in whatever scope the artifact lands
    /// in are two different claims about where a name comes from, and a rendering
    /// that guessed would put the wrong one in an integration target nobody here
    /// can see.
    pub enum SurfacePathRooting {
        /// Rooted absolutely: the rendering writes a leading path separator, so
        /// the path resolves the same wherever in the integration target it
        /// lands.
        CrateAbsolute = "crate-absolute",
            "rooted absolutely, written with a leading path separator";
        /// Resolved in the scope the artifact lands in, exactly as the caller
        /// spelled it.
        InScope = "in-scope",
            "resolved in the scope the rendered artifact lands in";
    }
}

threadpak::closed_register! {
    /// One of the two roads a codec pairing carries.
    ///
    /// The roster is what a FACING is asked about: `facing` in
    /// `type_contract.rs` is a constant table over this roster and
    /// [`SurfaceDirection`](crate::planning::SurfaceDirection), so "an inbound
    /// surface opens by reading the wire" is a value a reader can read back and a
    /// match the compiler keeps exhaustive.
    ///
    /// It is this home's own roster and not the codec home's road roster, and the
    /// two are about different things: that one names which of a codec's two
    /// roads a DIRECTION renders at all, and this one names which of a supplied
    /// pairing's two roads a FACING calls first. A shared roster would join a
    /// rendering decision inside one home to a rendering decision inside another.
    pub enum PairedCodecRoad {
        /// The road that writes a value's canonical bytes.
        Encode = "encode", "the pairing's road from a value to wire material";
        /// The road that reads them back, and refuses where they are not the
        /// value's.
        Decode = "decode", "the pairing's road from wire material to a value";
    }
}

/// One type path a rendered expression names.
///
/// # Bounds
///
/// The segments are structurally non-empty: a path naming no segment names
/// nothing, and a rendering that wrote one would emit a bare separator.
///
/// The parts are OWNED text, where a `'static` roster would be this crate's own:
/// a path here is the caller's spelling of a type in an integration target, and
/// it becomes static text only once it is written into that target's own file.
///
/// There is no ordering. Nothing here ranks paths, and the roster a path's
/// rooting stands in declares no order either — so a derived one would be an
/// order over a rooting roster's declaration sequence, which is a spelling
/// accident rather than a fact about where a name comes from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceTypePath {
    rooting: SurfacePathRooting,
    segments: NonEmptyBounded<String, SurfacePathSegmentLimit>,
}

/// The codec a remote surface rides: the type whose two roads carry the wire
/// contract's bytes, and the spelling of each road.
///
/// # Authority
///
/// **The pairing is the CALLER's and this home derives none of it.** The plan
/// names a wire contract, which says WHICH bytes these are; it names no codec,
/// because the codec that reads and writes those bytes is its own projection over
/// its own plan. A surface that elected a codec would be pairing somebody else's
/// declaration with a reader nobody asked for, and the pairing would then be a
/// fact the plan never recorded.
///
/// # Bounds
///
/// Both roads are associated roads on the codec's own type — the rendering writes
/// `<Codec>::<road>(…)` — so a free function is unwritable here rather than
/// refused, and the two are spelled differently by construction. What each road
/// is called with and hands back is stated once, as `PAIRING_CONTRACT` in
/// `type_contract.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecPairing {
    codec: SurfaceTypePath,
    encode: String,
    decode: String,
}

/// The signature one rendered surface road stands at: what it accepts, what it
/// answers with, and the refusal every checked call is converted into.
///
/// # Bounds
///
/// What is accepted and what is answered are TWO paths and not one, because a
/// facing decides whether they are the same thing. An inbound surface takes wire
/// material and answers with wire material; an outbound surface takes a request
/// and answers with an answer, and those are two of the owner's types. A single
/// path would be a claim about the owner's declaration that only one of the two
/// facings makes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceSignature {
    accepts: SurfaceTypePath,
    answers: SurfaceTypePath,
    refusal: SurfaceTypePath,
}

/// The complete declared shape one remote surface is rendered for.
///
/// # Bounds
///
/// The port is a TYPE PATH and the plan's port seat is an IDENTITY, and the two
/// are not the same fact: the plan names which declaration is projected, and this
/// names the type that realizes it in the integration target. This home neither
/// derives one from the other nor checks that they correspond — a correspondence
/// nobody declared is not a correspondence these services may assert.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemoteSurfaceShape {
    port: SurfaceTypePath,
    call: String,
    pairing: CodecPairing,
    signature: SurfaceSignature,
    entry: String,
}

/// Where one remote surface lands: in the integration target, under the byte role
/// the plan declared for it.
///
/// # Authority
///
/// **The landing is read off the plan and never chosen here.** The delivery
/// matrix spells this projection's delivery as **the remote surface in its
/// integration target**, which is a different FILE than the declaration the plan
/// was derived from — so the planned member is written as a standalone artifact
/// and the byte role that artifact is written under is the plan's own seat.
///
/// # Bounds
///
/// There is no constant destination here, for the reason the host-wrapper home's
/// landing states: the destination is fixed by a seat only the plan holds, so it
/// is composed from the plan rather than stated ahead of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegrationTargetLanding {
    byte_role: OwnerIdentityRef<ByteRoleSubject>,
}

// ---------------------------------------------------------------------------
// What the plan decided.
// ---------------------------------------------------------------------------

/// What a remote-surface plan decided, read off the plan's own public surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or the facing it renders would be an account that
/// sometimes says less than it knows. There is no private field here and this
/// home's invariant nucleus holds nothing of it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its kind's
/// one rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemoteSurfacePlan {
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
    /// The host contract this surface is bound to.
    ///
    /// # Bounds
    ///
    /// It reaches no token of the rendered surface, and it is read off the plan's
    /// CONTEXT — the binding
    /// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned)
    /// refused a target-free plan over. This kind's content names no contract at
    /// all, so unlike the host-wrapper home there is nothing here to read twice.
    pub host_contract: OwnerIdentityRef<ProjectionTargetDomain>,
    /// The port declaration projected.
    ///
    /// # Bounds
    ///
    /// It reaches no token either. The port identity says WHICH declaration is
    /// projected and the shape's port path says what realizes it at the address;
    /// this one travels for the explanation station and for a caller joining the
    /// artifact back to the declaration it answers to.
    pub port: OwnerIdentityRef<PortSubject>,
    /// The wire contract spoken.
    ///
    /// # Bounds
    ///
    /// It reaches no token either, on the same terms. The contract names which
    /// bytes travel; the pairing names the roads that write and read them, and
    /// the pairing is the caller's.
    pub wire_contract: OwnerIdentityRef<WireContractSubject>,
    /// Which way the surface faces — the one content seat the rendering turns on.
    pub direction: SurfaceDirection,
    /// Where the surface lands.
    pub landing: IntegrationTargetLanding,
}

// ---------------------------------------------------------------------------
// What this home is available for.
// ---------------------------------------------------------------------------

/// Whether a caller can be handed the machine's identity for a host contract, and
/// on whose mint that turns.
///
/// # Authority
///
/// **Not a boolean, and never a fabricated identity.** A road that cannot be
/// walked is unwalkable for a stated reason that names the seat closing it; a
/// bare `false` would say a caller could not bind a contract without saying whose
/// declaration would let it.
///
/// # Bounds
///
/// This home declares its own standing rather than reading the host-wrapper
/// home's, and the two stand on ONE machine fact — the absent mint for a
/// domain-tagged commitment over a declaration target. That duplication is
/// stated rather than hidden: a shared standing belongs on the plane, beside the
/// binding it is about, and putting it there is a decision about where the fact
/// lives rather than an edit either home may make on its own. Until it is made,
/// two homes hold two readings of one fact and say so.
///
/// [`SurfaceContractMint::Minted`] has no inhabitant in this crate today, and
/// declaring it anyway is deliberate on exactly the terms
/// [`VerifiedDerived`](crate::planning::VerifiedDerived) is declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceContractMint {
    /// Callers hold the machine's own minted identity for host contracts, so the
    /// road below is reachable from outside these services.
    Minted,
    /// No mint exists yet, and this is the seat that opens the road.
    AwaitingOwnerMint {
        /// The home that owes the mint.
        home: &'static str,
        /// The exact seat that would open it.
        seat: &'static str,
    },
}

/// What this home is available for, read from the binding a caller actually
/// holds.
///
/// # Authority
///
/// **Absence is a typed disposition and never a crippled fake surface.** A remote
/// surface is available exactly when the context binds one named host contract;
/// every other state names itself and names what would open it. This is the
/// honest-absence shape the interpreted mutation lane states for its own
/// unavailable road, applied to a road whose OUTSIDE entrance does not exist yet.
///
/// # Nonclaims
///
/// [`SurfaceAvailability::Bound`] claims that the CALLER holds the identity, and
/// nothing about whether an outside caller could obtain one — that is the mint's
/// question and [`SurfaceContractMint`] is where it is answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceAvailability {
    /// The context binds one named host contract, so a plan of this kind stands
    /// and the surface renders against it.
    Bound {
        /// The contract the surface would be bound to.
        contract: OwnerIdentityRef<ProjectionTargetDomain>,
    },
    /// The context binds no host contract at all, so no plan of this kind can be
    /// made — [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned)
    /// refuses a target-free plan for a kind that requires a bound contract.
    NoHostContract {
        /// What would open the road.
        opening: SurfaceContractMint,
    },
}

// ---------------------------------------------------------------------------
// The composed surface.
// ---------------------------------------------------------------------------

/// The rendered remote surface's typed description.
///
/// The seats are exactly what a rendered unit is rebuilt from — role, semantic
/// key, profile at its version, origin trail, and the tree — plus the landing,
/// which carries the byte role this artifact is written under, and the facing,
/// which is a fact about THIS rendering and is therefore read back rather than
/// recomputed by a caller.
///
/// # Nonclaims
///
/// The tree is the surface's own item run and never the port's declaration or the
/// codec's. A projection that emitted either would be a second declaration of
/// something its owner already declared once.
#[must_use = "a remote surface is the road one declared port speaks a wire contract over"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemoteSurface {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    landing: IntegrationTargetLanding,
    faces: SurfaceDirection,
    tree: GeneratedTree,
}

// ---------------------------------------------------------------------------
// The composition refusal family.
// ---------------------------------------------------------------------------

/// How composing a remote surface disagrees with the plan or with what the token
/// magnitude admits.
///
/// # Authority
///
/// **A single-cause family, and the shape is structural rather than chosen.**
/// Every check on this road is DEPENDENT on the one before it — there is no
/// destination to read until a member was found, no binding to read until the
/// member lands where a surface lands, and nothing to render until the binding is
/// there — so exactly one cause is true of any refused composition and there is
/// no set for a body to collect. The neighbouring host-wrapper home refuses with a
/// collection because its component roster gives it a pass whose issues
/// co-establish; this home has no such roster, and declaring a collection anyway
/// would be a body shape claiming a pass that does not exist.
///
/// No issue is payload-free: an issue names the role, the kind, or the bound it is
/// about, because a caller told only that composition failed has nothing to
/// repair. The declared selection order is stated in `type_contract.rs`, in the
/// order the checks establish them.
#[must_use = "a composition refusal names the exact disagreement the road established"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemoteSurfaceIssue {
    /// The plan declares no member under its kind's one rendered role, so there is
    /// no surface to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member is spliced at the declaration site rather than written
    /// as a standalone artifact.
    ///
    /// A remote surface lands in its INTEGRATION target, which is a different file
    /// than the declaration the plan was derived from; a member spliced beside
    /// that declaration is a surface inside the library that declared the port,
    /// and that is a different delivery.
    DestinationNotIntegrationTarget {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
    /// The plan's context binds no host contract, so there is nothing to serve
    /// over.
    ///
    /// Foreclosed on this seam's own route:
    /// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned)
    /// refuses a target-free plan for a kind whose target requirement is a bound
    /// host contract, so a plan of this kind that reached this reading is bound.
    /// The issue exists so the reading has a truthful road for the posture the
    /// TYPE still admits rather than a fabricated one.
    TargetBindingFree {
        /// The kind whose plans are meaningless without a contract, by its own
        /// declared stable name.
        kind: &'static str,
    },
    /// The rendered surface outgrows the declared token magnitude.
    SurfaceTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

/// The one alphabet every spelling this home renders as a Rust identifier is
/// admitted by, published from the nucleus every road here already reads it
/// through.
pub use guard::is_surface_identifier;

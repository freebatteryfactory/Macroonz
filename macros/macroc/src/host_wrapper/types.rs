//! The host-wrapper home's declarations: the type paths a rendered expression
//! names, the stage each composed component contributes, the declared shape a
//! wrapper is written for, where a wrapper lands, what a plan decided, what this
//! home is available FOR, the composed surface itself, and the magnitudes and
//! refusal families this home answers through.
//!
//! Declarations only.
//! Every road that reaches a private field — a path's segments and rooting, a
//! stage's road, a shape's stages, the landing's byte role, the surface's
//! composition, and the refusal body's one seat — lives in `type_guard.rs`, this
//! file's own child.
//!
//! # Nothing here decides what a host contract is
//!
//! The plan names a CONTRACT, the COMPONENTS composed into the wrapper, and the
//! declared capability that selected them. It names no type, no road, no
//! signature and no landing spelling — so [`WrapperShape`] arrives from the
//! caller and `plan.rs` reads only what the plan actually decided. A generator
//! that decided which road a host answers admission on would be declaring
//! somebody else's calling convention and then calling it.
//!
//! # The component roster is the machine's
//!
//! [`WrapperComponent`] and
//! [`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS) are the plane's,
//! imported rather than restated, on the charter's terms. This home adds one
//! fact per component and one only: the local its stage's answer is bound under,
//! stated once as the stage contract in `type_contract.rs`.
//!
//! # The outside road is not open, and the vocabulary says so
//!
//! [`WrapperAvailability`] is a typed reading of what this home is available
//! for, and [`WrapperContractMint`] is the standing of the mint that would open
//! it. Neither is a crippled wrapper that answers anyway.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ByteRoleSubject, GeneratedUnitSubject, GeneratorVersionSubject, OwnerFactRef, OwnerIdentityRef,
    ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit,
    WrapperComponentLimit,
};
use crate::planning::{CauseAnchoring, WrapperComponent};
use crate::token::GeneratedTree;
use threadpak::declaration::types::ProjectionTargetDomain;
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
    /// The magnitude governing how many segments one rendered type path may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Eight. A path reaching deeper than eight segments has stopped naming an
    /// item and started describing a tree, and the repair is a re-export at the
    /// address rather than a longer spelling at this end.
    ///
    /// # Nonclaims
    ///
    /// It is this home's own family and not the codec home's path family, even
    /// though the two magnitudes agree today. That one bounds a path written
    /// beside an owner's own item inside one expansion; this one bounds a path
    /// written into a HOST TARGET, which is a different file than the
    /// declaration and resolves against a different root. One family standing
    /// for both would be one authority answering two questions, and the day one
    /// of the two roads has to reach deeper is the day that would show.
    WrapperPathSegmentLimit = 8,
    /// The magnitude governing how many stages one declared wrapper shape may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Eight — the wrapper-component roster's own cardinality, because a stage
    /// is earned by ONE component and a component earns at most one stage. It is
    /// not a number this home chose out of taste: a ninth stage would have to be
    /// earned by a ninth component, and the roster declares eight. The roster is
    /// the plane's, so this number moves when the roster grows a component and
    /// for no other reason.
    ///
    /// # Nonclaims
    ///
    /// It is not the plane's
    /// [`WrapperComponentLimit`](crate::plane::WrapperComponentLimit), which is
    /// sixteen and governs how many components a PLAN may name. The two numbers
    /// disagree on purpose and the disagreement is the reason to keep the
    /// families apart: the plan's seat is a bounded list that may name a
    /// component twice, while a shape's stages stand one per component. One
    /// family answering both questions would make a doubled selection look like
    /// a wider roster.
    WrapperStageLimit = 8,
    /// The magnitude governing how many issues one wrapper-composition refusal
    /// body may carry.
    ///
    /// # Bounds
    ///
    /// Eight — the wrapper-component roster's own cardinality, because the
    /// roster is the QUANTIFIER of the composition pass and each component
    /// establishes at most one issue. A component the plan selects is either
    /// unstaged or doubly staged and never both, and a component the plan does
    /// not select can only be staged when it should not have been; the three
    /// answers are mutually exclusive per component, so eight issues can hold at
    /// once and no more.
    ///
    /// # Nonclaims
    ///
    /// It is this home's own family: every rendering home sizes its own refusal
    /// body by its own widest pass, and this body's widest pass is this home's.
    WrapperCompositionIssueLimit = 8,
}

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a path's segments are read before a stage's road, a
    /// stage's road before the entry spelling, and the entry spelling before a
    /// shape's stages.
    /// Every one of them refuses before a partial value exists — a shape holding
    /// some of its stages is a wrapper for a demand nobody made.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum WrapperDeclarationRefusal {
        /// The path names no segment at all, so it names nothing.
        PathSegmentsAbsent = "path-segments-absent",
            "a rendered type path names no segment";
        /// The path carries more segments than the declared magnitude.
        PathSegmentsUnbounded = "path-segments-unbounded",
            "a rendered type path carries more segments than the declared magnitude";
        /// A path segment is not one Rust identifier, so the rendering would
        /// write tokens the host target's compiler reads as something else.
        SegmentNotAnIdentifier = "segment-not-an-identifier",
            "a rendered path segment is not one Rust identifier";
        /// The stage states no road, so nothing is called for that component.
        EmptyStageRoad = "empty-stage-road",
            "a wrapper stage states no road";
        /// The stage's road is not one Rust identifier.
        StageRoadNotAnIdentifier = "stage-road-not-an-identifier",
            "a wrapper stage road is not one Rust identifier";
        /// The entry states no spelling, so the rendered wrapper has no name.
        EmptyEntrySpelling = "empty-entry-spelling",
            "a wrapper entry states no spelling";
        /// The entry's spelling is not one Rust identifier.
        EntrySpellingNotAnIdentifier = "entry-spelling-not-an-identifier",
            "a wrapper entry spelling is not one Rust identifier";
        /// The shape declares no stage at all.
        ///
        /// A wrapper that composes nothing wraps nothing: its rendered road
        /// would hand back exactly what it was given, which is a function the
        /// host already has.
        StagesAbsent = "stages-absent",
            "a wrapper shape declares no stage";
        /// The shape declares more stages than the declared magnitude.
        StagesUnbounded = "stages-unbounded",
            "a wrapper shape declares more stages than the declared magnitude";
    }
}

// ---------------------------------------------------------------------------
// The rendered vocabulary.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// Where one rendered type path is rooted.
    ///
    /// A closed roster of exactly two, and neither is a default: a path spelled
    /// from a crate root and a path resolved in whatever scope the artifact
    /// lands in are two different claims about where a name comes from, and a
    /// rendering that guessed would put the wrong one in a host target nobody
    /// here can see.
    pub enum WrapperPathRooting {
        /// Rooted absolutely: the rendering writes a leading path separator, so
        /// the path resolves the same wherever in the host target it lands.
        CrateAbsolute = "crate-absolute",
            "rooted absolutely, written with a leading path separator";
        /// Resolved in the scope the artifact lands in, exactly as the caller
        /// spelled it.
        InScope = "in-scope",
            "resolved in the scope the rendered artifact lands in";
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
/// a path here is the caller's spelling of a type in a host target, and it
/// becomes static text only once it is written into that target's own file.
///
/// There is no ordering. Nothing here ranks paths, and the roster a path's
/// rooting stands in declares no order either — so a derived one would be an
/// order over a rooting roster's declaration sequence, which is a spelling
/// accident rather than a fact about where a name comes from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperTypePath {
    rooting: WrapperPathRooting,
    segments: NonEmptyBounded<String, WrapperPathSegmentLimit>,
}

/// One stage of a wrapper: the component it composes, and the road on the host
/// contract's own type the rendered wrapper calls for it.
///
/// # Authority
///
/// **Both seats are the caller's and neither is derived here.** Which components
/// a wrapper composes is the PLAN's decision, read off the plan's kind content;
/// which road answers each one is the HOST's declaration, and a generator that
/// chose it would be inventing somebody else's calling convention.
///
/// # Bounds
///
/// The road is an associated road on the host contract's own type — the
/// rendering writes `<Host>::<road>(…)` — so a free function is unwritable here
/// rather than refused. That is the shape a wrapper can call without learning
/// where the host's module sits.
///
/// The road is one Rust identifier by construction, because it is written into a
/// host target this home never sees and a spelling that is not an identifier
/// would surface as a compile error in somebody else's target with no sign of
/// where the name came from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperStage {
    component: WrapperComponent,
    road: String,
}

/// The complete declared shape one wrapper is rendered for.
///
/// # Bounds
///
/// The stages are structurally non-empty, for the reason
/// [`WrapperDeclarationRefusal::StagesAbsent`] states.
///
/// The carried type is ONE type and not two: every stage takes what the stage
/// before it handed back, so the wrapper's parameter, every intermediate
/// binding, and its answer stand at one type. A shape that named a different
/// type per stage would be describing a pipeline this home cannot check the
/// joins of, and the joins are exactly what the host's own compiler answers when
/// it reads the rendered call.
///
/// The refusal's spelling is a path rather than a name, because it is a type
/// declared in the host target's own scope and this home may not choose a name
/// there.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperShape {
    host: WrapperTypePath,
    carried: WrapperTypePath,
    refusal: WrapperTypePath,
    entry: String,
    stages: NonEmptyBounded<WrapperStage, WrapperStageLimit>,
}

/// Where one wrapper lands: in the host's own target, under the byte role the
/// plan declared for it.
///
/// # Authority
///
/// **The landing is read off the plan and never chosen here.** The delivery
/// matrix spells this projection's delivery as *host wrappers in host targets*,
/// which is a different FILE than the declaration the plan was derived from — so
/// the planned member is written as a standalone artifact and the byte role that
/// artifact is written under is the plan's own seat.
///
/// # Bounds
///
/// There is no constant destination here, and the absence is the honest shape.
/// A home whose delivery is decided by its ROLE alone — a codec surface, a
/// documentation run, a generated support shell at the declaration site, a
/// mutation-evaluation copy in the test carrier — can state its destination as a
/// constant, because the answer is the same for every plan that home ever reads.
/// This one lands under a byte role that only the plan holds, so the destination
/// is a value composed from the plan rather than a fact stated ahead of it.
///
/// A wrapper written at the declaration site would be a wrapper in the library
/// that declared the contract, which is the one place a host target is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostTargetLanding {
    byte_role: OwnerIdentityRef<ByteRoleSubject>,
}

// ---------------------------------------------------------------------------
// What the plan decided.
// ---------------------------------------------------------------------------

/// What a host-wrapper plan decided, read off the plan's own public surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or the contract it binds to would be an account that
/// sometimes says less than it knows. There is no private field here and this
/// home's invariant nucleus holds nothing of it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its kind's
/// one rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostWrapperPlan {
    /// The rendered role the wrapper stands for.
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
    /// The rendering engine the wrapper is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// The host contract this wrapper is bound to.
    ///
    /// # Bounds
    ///
    /// It reaches no token of the rendered wrapper. The contract is what the
    /// wrapper is bound TO and the shape is what the wrapper is written FOR, and
    /// the two are separate facts: this one travels for the explanation station
    /// and for a caller joining the artifact back to the contract it answers to.
    ///
    /// It is read off the plan's CONTEXT rather than off its kind content. The
    /// context's binding is what
    /// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned)
    /// refused a target-free plan over, so it is the one of the two that a plan
    /// of this kind cannot carry unfilled.
    pub host_contract: OwnerIdentityRef<ProjectionTargetDomain>,
    /// The contract the plan's kind content names.
    ///
    /// # Bounds
    ///
    /// Carried BESIDE [`HostWrapperPlan::host_contract`] rather than instead of
    /// it, and the two are not folded: the context's binding is what the plan
    /// was decided under and the content's is what the wrapper was planned for.
    /// Nothing in the plane requires them to agree, so a reading that carried one
    /// of them would be electing an answer to a question the plan states twice.
    pub content_contract: OwnerIdentityRef<ProjectionTargetDomain>,
    /// The components the plan composes into this wrapper.
    ///
    /// # Ordering
    ///
    /// This order is NOT meaning. The plan states a selection; the plane's own
    /// [`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS) roster states
    /// the order a wrapper composes them in, and the rendering walks that roster
    /// rather than this list. A caller that reordered its selection renders the
    /// same wrapper.
    pub components: NonEmptyBounded<WrapperComponent, WrapperComponentLimit>,
    /// The declared capability that selected them.
    pub capability_basis: OwnerFactRef,
    /// Where the wrapper lands.
    pub landing: HostTargetLanding,
}

// ---------------------------------------------------------------------------
// What this home is available for.
// ---------------------------------------------------------------------------

/// Whether a caller can be handed the machine's identity for a host contract,
/// and on whose mint that turns.
///
/// # Authority
///
/// **Not a boolean, and never a fabricated identity.** A road that cannot be
/// walked is unwalkable for a stated reason that names the seat closing it; a
/// bare `false` would say a caller could not bind a contract without saying
/// whose declaration would let it.
///
/// # Bounds
///
/// [`WrapperContractMint::Minted`] has no inhabitant in this crate today, and
/// declaring it anyway is deliberate on exactly the terms
/// [`VerifiedDerived`](crate::planning::VerifiedDerived) was declared on: an arm
/// written before it lands is a declaration of what will be true rather than a
/// claim that it already is, and the type that tells the two apart is what makes
/// the landing a change of type rather than an edit. That posture has since
/// landed, which is what the precedent is worth: the seat written ahead was the
/// seat the arrival went into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperContractMint {
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
/// **Absence is a typed disposition and never a crippled fake wrapper.** A
/// wrapper is available exactly when the context binds one named host contract;
/// every other state names itself and names what would open it. This is the
/// honest-absence shape the interpreted mutation lane states for its own
/// unavailable road, applied to a road whose OUTSIDE entrance does not exist
/// yet.
///
/// # Nonclaims
///
/// [`WrapperAvailability::Bound`] claims that the CALLER holds the identity, and
/// nothing about whether an outside caller could obtain one — that is the mint's
/// question and [`WrapperContractMint`] is where it is answered. The two are
/// separate readings on purpose: a caller inside the workspace that already holds
/// an identity is not evidence that the mint exists, and the reverse would be a
/// road that reported itself open because somebody in the same crate had a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperAvailability {
    /// The context binds one named host contract, so a plan of this kind stands
    /// and the wrapper renders against it.
    Bound {
        /// The contract the wrapper would be bound to.
        contract: OwnerIdentityRef<ProjectionTargetDomain>,
    },
    /// The context binds no host contract at all, so no plan of this kind can be
    /// made — [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned)
    /// refuses a target-free plan for a kind that requires a bound contract.
    NoHostContract {
        /// What would open the road.
        opening: WrapperContractMint,
    },
}

// ---------------------------------------------------------------------------
// The composed surface.
// ---------------------------------------------------------------------------

/// The rendered wrapper surface's typed description.
///
/// The seats are exactly what a rendered unit is rebuilt from — role, semantic
/// key, profile at its version, origin trail, and the tree — plus the landing,
/// which carries the byte role this artifact is written under, and the composed
/// roster, which is a fact about THIS rendering and is therefore read back rather
/// than recomputed by a caller.
///
/// # Nonclaims
///
/// The tree is the wrapper's own item run and never the host's declaration. A
/// projection that emitted the contract would be a second declaration of
/// something the host already declared once.
#[must_use = "a wrapper surface is the shell one bound host contract's demand composes to"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WrapperSurface {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    landing: HostTargetLanding,
    composed: Bounded<WrapperComponent, WrapperStageLimit>,
    tree: GeneratedTree,
}

// ---------------------------------------------------------------------------
// The composition refusal family.
// ---------------------------------------------------------------------------

/// How composing a wrapper surface disagrees with the plan, with the component
/// roster, or with what the token magnitude admits.
///
/// No issue is payload-free: an issue names the role, the component, or the bound
/// it is about, because a caller told only that composition failed has nothing to
/// repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperSurfaceIssue {
    /// The plan declares no member under its kind's one rendered role, so there
    /// is no wrapper to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands somewhere other than a standalone artifact.
    ///
    /// A wrapper lands in a FILE the host target owns, which is a different file
    /// than the declaration the plan was derived from.
    /// The destination roster names four deliveries, and a member that is not an
    /// artifact declared one of the other three: tokens spliced at the
    /// declaration site, the deferred cargo a test target invokes, or the
    /// deferred cargo a bench target invokes. The first is a wrapper in the
    /// library that declared the contract, and neither carrier is a file at all —
    /// a carrier is deferred cargo a consumption target expands — so each of the
    /// three establishes this issue.
    DestinationNotHostTarget {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
    /// The plan's context binds no host contract, so there is nothing to wrap.
    ///
    /// Foreclosed on this seam's own route:
    /// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned)
    /// refuses a target-free plan for a kind whose target requirement is a bound
    /// host contract, so a plan of this kind that reached this reading is bound.
    /// The issue exists so the reading has a truthful road for the posture the
    /// TYPE still admits rather than a fabricated one — and it is the seat that
    /// would carry the disagreement if the planning refusal were ever relaxed.
    TargetBindingFree {
        /// The kind whose plans are meaningless without a contract, by its own
        /// declared stable name.
        kind: &'static str,
    },
    /// The plan composes this component and the shape writes no stage for it.
    /// The component roster is the quantifier: a selected component with no road
    /// behind it is a composition claim the wrapper does not keep.
    SelectedComponentNotStaged {
        /// The component nobody staged.
        component: WrapperComponent,
    },
    /// The shape writes a stage for a component the plan does not compose, so the
    /// stage stands on a selection nobody planned.
    StageComponentNotSelected {
        /// The component the stage named.
        component: WrapperComponent,
    },
    /// Two stages of one shape are earned by one component, so the wrapper would
    /// call two roads under one selection and a reader cannot tell which was
    /// meant.
    ComponentStageDoubled {
        /// The doubled component.
        component: WrapperComponent,
    },
    /// The composed roster outran its declared magnitude.
    ///
    /// Foreclosed on this seam's own route: the roster is built by walking the
    /// plane's own component roster once, so it is never longer than that roster
    /// and never longer than the magnitude sized by it. The issue exists so the
    /// seat's construction has a truthful road rather than a fabricated one.
    ComposedSeatBoundExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
    /// The rendered wrapper outgrows the declared token magnitude.
    WrapperTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

/// The wrapper-composition refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared beside the rest of this home's declarations would put all of them
/// inside the same wall.
pub use guard::WrapperComposition;

/// The one alphabet every spelling this home renders as a Rust identifier is
/// admitted by, published from the nucleus every road here already reads it
/// through.
pub use guard::is_wrapper_identifier;

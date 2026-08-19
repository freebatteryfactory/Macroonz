//! The derive-implementation home's declarations: the two surfaces one
//! implementation meaning is delivered as, the mutation points the evaluation
//! copy carries, the control that is never absent from it, and the parity
//! between the two.
//!
//! Declarations only.
//! Every road that reaches a private field — a name's two parts, an operation's
//! tokens, a point's alternatives, the table's control seat, either surface's
//! rendering, the parity's seats, and the refusal body's one seat — lives in
//! `type_guard.rs`, this file's own child.
//! That is what makes the control's presence STRUCTURAL: the table's only road
//! puts the control in, so a surface without one is unwritable rather than
//! refused.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    GeneratedUnitSubject, GeneratorVersionSubject, MutationAlternativeLimit, MutationPointLimit,
    ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, RenderedUnitSubject,
};
use crate::planning::{CauseAnchoring, RenderedImplementation};
use crate::token::{GeneratedTree, TokenPath};
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a name's parts are read before an operation's
    /// tokens, and an operation's tokens before a point's alternatives.
    /// Every one of them refuses before a partial value exists — a point holding
    /// some of its alternatives is a point about a damage nobody admitted.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum SurfaceDeclarationRefusal {
        /// The name states no owner.
        EmptyNamespace = "empty-namespace",
            "a mutation-point name states no owner";
        /// The name states no spelling.
        EmptyStem = "empty-stem",
            "a mutation-point name states no spelling";
        /// The operation carries no tokens, so it names nothing a rendering
        /// could substitute at and nothing a walk could find.
        OperationEmpty = "operation-empty",
            "a mutation operation carries no tokens";
        /// The point admits no alternative at all, and a selection among one
        /// thing selects nothing.
        AlternativesAbsent = "alternatives-absent",
            "a mutation point admits no alternative to select";
        /// The point admits more alternatives than the declared magnitude.
        AlternativesUnbounded = "alternatives-unbounded",
            "a mutation point admits more alternatives than the declared magnitude";
        /// Two of one point's alternatives carry one spelling, so the point's
        /// own roster cannot say which damage a variant stands for.
        AlternativeSpellingDoubled = "alternative-spelling-doubled",
            "two alternatives of one mutation point carry one spelling";
        /// A binding spelling is not one Rust identifier, so the rendering would
        /// write tokens the consumer's compiler reads as something else.
        SpellingNotAnIdentifier = "spelling-not-an-identifier",
            "an evaluation binding spelling is not one Rust identifier";
    }
}

// ---------------------------------------------------------------------------
// The mutation-point vocabulary, in the harness's field shape.
// ---------------------------------------------------------------------------

/// A namespaced name: the owner that declares a spelling, and the spelling.
///
/// The FIELD SHAPE is the harness's mutation-point vocabulary, mirrored here as
/// data. Nothing of the harness is imported and no harness type is named: this
/// home writes letters to an address and does not own the mailbox, so what
/// crosses the wall is a conforming pair of parts rather than a borrowed type.
///
/// # Construction
///
/// Both parts are refused empty, so a name that names nothing is not a value
/// anybody can hold.
///
/// # Bounds
///
/// The parts are OWNED text, where the harness's own are `'static`. That
/// difference is the side of the wall each one is on: a name here is cut from
/// the token material one expansion was handed, and it becomes static text only
/// once the shell splices it into the consumer's own target.
///
/// # Ordering
///
/// The order is the storage order a set needs to iterate the same way every run,
/// over the namespace and then the stem. It ranks nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationPointName {
    namespace: String,
    stem: String,
}

/// The owner claim one mutation point stands under.
///
/// A point that killed nothing is a finding about the CLAIM behind it, which is
/// why the claim rides the point rather than being looked up afterwards: a
/// survivor's explanation walks survivor to owning claim to the missing oracle
/// class, and a point that named no claim would break that walk at its first
/// step.
///
/// # Nonclaims
///
/// It is a reference and never the claim itself. This home neither declares
/// claims nor checks that the named one exists.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationClaimRef(MutationPointName);

/// One operation the evaluation copy can stand at a mutation point: how it is
/// NAMED, and how it is WRITTEN.
///
/// Two seats because two sides read it. The spelling is what the harness reads —
/// it is the data that crosses the wall and the name a survivor is reported
/// under. The tokens are what the rendering engine substitutes, and a spelling
/// is not tokens: a renderer that re-parsed the spelling would be composing Rust
/// out of a string, which is exactly the round trip the token seam exists to
/// remove.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationOperation {
    spelling: MutationPointName,
    tree: GeneratedTree,
}

/// One mutation point on the evaluation surface.
///
/// It carries its own identity, the owner claim it stands under, the original
/// operation it is about, the alternatives admitted against that operation, and
/// the activation site the operation sits at in the captured declaration.
///
/// # Authority
///
/// **Every seat arrives from the caller and none is derived here.** Which
/// operation is worth damaging, which alternatives are admitted against it, and
/// which claim owns the site are the harness's declarations. A generator that
/// decided any of them would be producing its own facts and then proving them,
/// which is the one thing these services never do.
///
/// # Bounds
///
/// The alternative set is structurally non-empty, because a selection among one
/// thing selects nothing. The door reads a runtime count and refuses
/// ([`SurfaceDeclarationRefusal::AlternativesAbsent`]); the VALUE cannot be
/// empty at all.
///
/// The activation site is a route into the CAPTURED declaration and never into
/// the rendered tree: the walk that finds a point happens over the services'
/// own typed capture, and a route is stable under everything a span is not.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPoint {
    name: MutationPointName,
    claim: MutationClaimRef,
    original: MutationOperation,
    alternatives: NonEmptyBounded<MutationOperation, MutationAlternativeLimit>,
    activation: TokenPath,
}

/// The mandatory no-mutation control.
///
/// It is not a mutation and it damages nothing: it is the arm under which every
/// point renders its ORIGINAL operation, so the evaluation copy with this
/// selected emits exactly what the production surface emits.
///
/// It carries its declared name, because the harness reports a parity failure
/// against a named point exactly as it reports a survivor against one; a control
/// with no name would be the one row of the table nobody could cite.
///
/// # Construction
///
/// There is no public road to one. The only value of this type anybody can hold
/// is the one [`MutationPointTable::over`] seats at the table's first position,
/// which is what makes "every evaluation surface contains the control" a shape
/// rather than a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoMutationControl {
    name: MutationPointName,
}

/// The mutation-point table: the control at the first position, and the admitted
/// points after it.
///
/// # Authority
///
/// **The control's position is the type's, not a caller's.** The two seats are
/// separate rather than one collection with a reserved first element, because a
/// collection can be built short and a seat cannot: a table whose first element
/// was supposed to be the control but was not is a value this shape cannot
/// express.
///
/// # Bounds
///
/// The admitted set may be EMPTY, and an empty one is a stated fact rather than
/// a missing one: an implementation nobody admitted a damage against still has a
/// lawful evaluation copy, and that copy still owes the parity its control
/// proves.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPointTable {
    control: NoMutationControl,
    admitted: Bounded<MutationPoint, MutationPointLimit>,
}

/// The namespace the no-mutation control's name is declared under.
pub const NO_MUTATION_NAMESPACE: &str = "threadpak.macroc";

/// The spelling the no-mutation control's name is declared with.
///
/// An admitted point claiming it is refused: two points under one name make the
/// harness's own join elect one of them, and the one it must never elect is the
/// control.
pub const NO_MUTATION_STEM: &str = "no-mutation";

/// The Rust variant the no-mutation control is rendered as, at the first
/// position of the active-point enum.
pub const NO_MUTATION_VARIANT: &str = "NoMutation";

// ---------------------------------------------------------------------------
// The two surfaces.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// The two surfaces one implementation meaning is delivered as.
    ///
    /// A closed roster of exactly two, because the delivery matrix declares
    /// exactly two and a third would be a delivery nobody planned. What
    /// separates them is stated once, as a constant table in
    /// `type_contract.rs`, rather than as a sentence each reader re-derives.
    pub enum ImplementationSurface {
        /// The implementation the consumer's normal build compiles, at the
        /// declaration site.
        Production = "production",
            "the implementation the normal build compiles, at the declaration site";
        /// The evaluation copy, carrying every admitted mutation point, in the
        /// consumer's test target.
        MutationEvaluation = "mutation-evaluation",
            "the evaluation copy carrying every admitted mutation point";
    }
}

/// The rendered production implementation's typed description.
///
/// # Authority
///
/// **There is no selector seat here, and there never is one.** The absence is
/// the guarantee: a production rendering that consulted a mutation selector, a
/// test switch, or a configuration arm is not a value this type can hold, so
/// "production carries no selector" is checked by the compiler rather than by a
/// reader.
///
/// # Bounds
///
/// There is no destination seat either, for the opposite reason: where a member
/// under a rendered role LANDS is that roster's own constant answer
/// ([`RenderedImplementation::destination`]) rather than a seat here that could
/// say something else. A planned member landing anywhere but where the role says
/// is refused before a surface exists, against exactly that answer.
///
/// The remaining seats are exactly what a rendered unit is rebuilt from — role,
/// semantic key, profile at its version, origin trail, and the tree — so the
/// closure's reconstruction reads this surface's own answers rather than a
/// summary of them.
#[must_use = "a production surface is the implementation the normal build compiles"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductionSurface {
    role: RenderedImplementation,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    tree: GeneratedTree,
}

/// How the evaluation copy names the two spellings it cannot invent: the
/// active-point enum it declares, and the selector it reads.
///
/// # Bounds
///
/// The selector is a name the evaluation copy READS and never declares. Where
/// that name comes to be in scope at every activation site is the shell's
/// splice, not this home's rendering — a home that declared the seat would be
/// deciding the shape of an item it does not own. A selector not in scope is an
/// ordinary compile error at the consumption target, which is where the shell's
/// own contract is checked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvaluationBinding {
    active_enum: String,
    selector: String,
}

/// The evaluation copy's typed description: what it is, what it carries, and how
/// it names its selector.
///
/// # Authority
///
/// **The table is the whole of what is selectable.** Runtime picks one of the
/// table's variants and nothing else — there is no road from a selection to
/// arbitrary source, because a road like that would be a second authority over
/// what the implementation means.
///
/// # Bounds
///
/// The identity is derived over THIS copy's canonical bytes, anchored on the
/// copy's OWN planned semantic key at its own role's roster position — the same
/// derivation [`RenderedUnit::materialized`] performs for any planned member, so
/// the copy's identity is a fact about this rendering rather than a name
/// borrowed from the production member beside it.
///
/// The role seat is the EVALUATION role
/// ([`RenderedImplementation::twin`] of the production one), never the
/// production role: the copy is a planned member on exactly the terms the
/// production unit is, and a copy wearing the production role would be the
/// second member standing under a role the closure matches one member per.
///
/// [`RenderedUnit::materialized`]: crate::closure::RenderedUnit::materialized
#[must_use = "an evaluation surface is the copy every admitted mutation point is selected from"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationEvaluationSurface {
    role: RenderedImplementation,
    identity: ProjectionIdentity<RenderedUnitSubject>,
    binding: EvaluationBinding,
    table: MutationPointTable,
    tree: GeneratedTree,
}

/// The typed statement that the two surfaces stand on one declaration and one
/// rendering engine.
///
/// # Authority
///
/// It is DERIVED from seats that exist when it is made — the address the entry
/// account walked in with, the generator identity the plan's context names, the
/// production member's semantic key, and the evaluation copy's own identity —
/// and never asserted about a comparison nobody performed.
///
/// # Nonclaims
///
/// **It names what the two roads SHARE, and it is silent about both of them.**
/// A declaration that says the wrong thing says it to both surfaces; a rendering
/// engine that writes the wrong tokens writes them twice. Agreement across a
/// shared substrate is silence about that substrate, so holding this value
/// establishes that the evaluation copy is faithful to the RENDERED PRODUCTION
/// SURFACE — never that either surface matches the owner's intent, never that
/// the admitted alternatives are meaningful damages, and never that a mutant the
/// table carries will be observed to fire. Those are answered by running, and
/// running is the harness's.
#[must_use = "a parity statement names what the two surfaces share, and what it is silent about"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceParity {
    declaration: CauseAnchoring,
    engine: ProjectionIdentity<GeneratorVersionSubject>,
    production: ProjectionIdentity<GeneratedUnitSubject>,
    evaluation: ProjectionIdentity<RenderedUnitSubject>,
}

/// One implementation meaning, delivered: both surfaces and the parity between
/// them, bound together.
///
/// The three arrive together because they are one fact. A caller holding a
/// production surface and an evaluation surface separately could have been
/// handed two that were never rendered from one plan; there is no road here that
/// produces one without the others.
#[must_use = "the two surfaces and their parity are one delivery of one implementation meaning"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationSurfaces {
    production: ProductionSurface,
    evaluation: MutationEvaluationSurface,
    parity: SurfaceParity,
}

// ---------------------------------------------------------------------------
// The composition refusal family.
// ---------------------------------------------------------------------------

/// How composing the two surfaces disagrees with the plan, with the table, or
/// with the production tree.
///
/// No issue is payload-free: an issue names the point or the role it is about,
/// because a caller told only that composition failed has nothing to repair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplementationSurfaceIssue {
    /// The plan declares no member under one half of the requested pair — no
    /// production surface to render, or no planned evaluation copy to render it
    /// as. Both halves are planned members, so either absence is this issue.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands somewhere other than where its ROLE declares it
    /// lands ([`RenderedImplementation::destination`]).
    ///
    /// The roster answers differently for the two halves of every pair, and the
    /// difference is what the pair is for: the production implementation lands
    /// at the declaration site, and the evaluation copy lands in the test
    /// carrier. So the disagreement is with the ROLE's own answer and never with
    /// a landing the whole roster shares — a production member written as a
    /// standalone artifact and an evaluation copy written at the declaration
    /// site are both this issue, at the role each was planned under.
    DestinationNotRoleDeclared {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
    /// Two admitted points carry one name, so the harness's join over the
    /// surface would elect one of them and report the other's survival under
    /// the elected one's identity.
    PointNameDoubled {
        /// The doubled name.
        point: MutationPointName,
    },
    /// An admitted point claims the no-mutation control's reserved name.
    /// The control is the one variant parity is stated over, and a point wearing
    /// its name is a mutant that can be selected as the control.
    ControlNameClaimed {
        /// The point that claimed it.
        point: MutationPointName,
    },
    /// The admitted points outgrow the declared magnitude.
    PointsUnbounded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
    /// The point's original operation does not occur in the production tree at
    /// all, so the point is about an operation this surface does not contain.
    OriginalOperationAbsent {
        /// The point whose operation is absent.
        point: MutationPointName,
    },
    /// The point's original operation occurs more than once in the production
    /// tree, so the site the point names is ambiguous and a substitution would
    /// damage operations nobody admitted a point against.
    OriginalOperationNotUnique {
        /// The point whose operation is ambiguous.
        point: MutationPointName,
        /// How many occurrences the walk found.
        observed: u32,
    },
    /// The point's one occurrence sits INSIDE another point's operation, so the
    /// single walk consumed it before this point could be substituted at it.
    /// Two points overlapping at one site is two claims about one operation.
    OriginalOperationOverlapped {
        /// The point that was never substituted.
        point: MutationPointName,
    },
    /// The evaluation copy outgrows the declared token magnitude.
    /// A copy carrying every point's arms at every point is the widest tree this
    /// home writes, and it refuses rather than materializing part of one.
    EvaluationTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

/// The surface-composition refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared in this file would put every other declaration in this file inside
/// the same wall.
pub use guard::ImplementationSurfaceComposition;

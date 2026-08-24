//! The support home's declarations: the carrier's own vocabulary, the pin its gate is matched against, the three cargo axes, the verified assembly one carrier is rendered from, the carrier itself, and the three ways this home says no.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes the home's central claim structural: proved cargo is read off a terminal's own delivery, and no second seam can build an assembly out of anything else.

use crate::bounded::{Capped, NonEmpty};
use crate::identity::{self, ClosedExpansionId, Identity, OwnerFact};
use crate::kind::{Destination, Disposition};
use crate::token::GeneratedTree;

#[path = "type_guard.rs"]
mod guard;

/// Segments one rendered path may carry after the crate it is rooted at.
///
/// A path reaching deeper has stopped naming an item and started describing a tree, and the repair is a re-export at the address rather than a longer spelling at this end.
pub const PATH_SEGMENT_LIMIT: usize = 8;

/// Issues one assembly refusal carries before it begins counting the rest.
///
/// The issues are facts about a fixed set of axes and about the one declaration and the one pin they stand under, so their count is bounded by the roster rather than by how much cargo any axis holds.
pub const ASSEMBLY_ISSUE_LIMIT: usize = 8;

/// The fact this home declares, and the one its refusals cite as a repair.
pub const ASSEMBLY_FACT: OwnerFact = OwnerFact {
    home: "support",
    name: "one-carrier-delivers-one-declarations-proved-cargo",
};

/// The generated-support schema identity a carrier's gate is matched against.
///
/// One address rather than thirty-two loose bytes, so an expectation cannot be assembled at a call site out of whatever bytes were nearby.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId([u8; 32]);

/// These services' own expectation of the harness's generated-support schema identity.
///
/// The bytes were DERIVED from the harness's published schema declaration and copied here, in the decimal base the gate matches in.
/// This side cannot derive them: the declaration they name lives in a crate these services do not depend on, and the harness's own currency lane keeps the two copies equal.
///
/// # Nonclaims
///
/// It says the producer's copy and the published copy are coherent, and nothing about whether either is current.
pub const EXPECTED_SCHEMA_ID: SchemaId = SchemaId::pinned([
    185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5, 84, 120, 104, 25,
    150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
]);

crate::roster! {
    /// Which crate a rendered path is rooted at.
    ///
    /// Exactly two, because a carrier's invocation names exactly two crates a consumer may have renamed and a generated expression names no third.
    /// A callable living in the consumption target's own crate has no row and needs none: it arrives as an EXPRESSION at the invocation, where that target's own hygiene reaches its own items.
    ///
    /// A row's declared name is also the metavariable a carrier binds it under, so the matcher and every path the body renders take one answer from one road.
    pub enum CrateFacing {
        /// The crate the declaration this carrier was planned over sits in.
        Declaring = "declaring",
        /// The crate that owns the descriptor vocabulary and the gate.
        Harness = "harness",
    }
}

crate::roster! {
    /// Which coupled pair of seats the gate is invoked under.
    ///
    /// A row's declared name is the stamped seat's clause; the opaque seat's clause is beside it in `type_contract.rs`.
    pub enum DeliveryForm {
        /// The form a test target invokes.
        Trials = "trials",
        /// The form a bench target invokes.
        Benches = "benches",
    }
}

crate::roster! {
    /// The cargo axes one carrier composes.
    ///
    /// Three axes carrying three genuinely different materials, so one sum with one payload would force three vocabularies into one shape and leave "which material may fill which seat" a question nobody answers.
    pub enum CargoAxis {
        /// The declaration-grammar material the gate's stamped seat carries.
        Declared = "declared",
        /// The proved cargo a test target's opaque seat carries.
        Deferred = "deferred",
        /// The proved cargo a bench target's opaque seat carries.
        Bench = "bench",
    }
}

/// How this home's own declaration vocabulary refuses.
///
/// Dependent checks in a declared order, so exactly one cause is true of any refused declaration, and each refuses before a partial value exists.
#[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclarationError {
    /// The name states no owner.
    EmptyNamespace,
    /// The name states no spelling.
    EmptyStem,
    /// A spelling written into a consumer's target in identifier position is not one Rust identifier, so the emission would write tokens that compiler reads as something else.
    SpellingNotAnIdentifier,
    /// The path names no segment past the crate it is rooted at, so it names the crate and nothing in it.
    PathSegmentsAbsent,
    /// The path carries more segments than the declared magnitude.
    PathSegmentsUnbounded,
}

/// A namespaced name: the owner that declares a spelling, and the spelling.
///
/// The FIELD SHAPE is the address's, mirrored here as data — what crosses is a conforming pair of parts rather than a borrowed type, and the expressions a renderer writes reach the address through the caller-supplied binding.
/// The parts are OWNED text where the address's own are static, because a name here is cut from token material one expansion was handed and becomes static only once a carrier splices it into a consumer's target.
///
/// # Ordering
///
/// Over the namespace and then the stem, which is the storage order a set needs to iterate the same way every run. It ranks nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WallName {
    namespace: String,
    stem: String,
}

/// One path a rendered expression names, rooted at one of the two crates a consumer may have renamed.
///
/// A path here never spells a crate name: it states which crate it is rooted at, and the rendering writes the carrier's own metavariable for that row — so a consumer that renamed either dependency gets its own name back without this home ever learning the name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundPath {
    facing: CrateFacing,
    segments: NonEmpty<String, PATH_SEGMENT_LIMIT>,
}

/// The exported name a consumption target invokes one declaration's carrier by.
///
/// The author chooses it and the consumer's own compiler collision-checks it: the carrier itself is exported under a plan-keyed spelling nobody can know before expansion, so a delivery whose carrier nobody can address is a delivery nobody can run.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportName(String);

/// The mangled, collision-free name one carrier is exported under.
///
/// # Authority
///
/// **The spelling is derived from the PLAN's identity, at full width, and from nothing else.** The carrier lands at the root of whatever crate the declaration site sits in and shares one namespace with every other exported macro there, so a name a producer chose would collide the first time two declarations in one crate wanted a carrier.
///
/// The key is the plan's and not a planned member's, because a member's semantic key is a value the planning caller supplies while a plan identity is one these services derive.
/// Full width is what makes "collision-free" true as written rather than true of a prefix.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellName {
    spelling: String,
}

/// The proved tokens one consumption target receives in a gate's opaque seat.
///
/// A declaration value: holding one claims nothing about any terminal, and it becomes proved cargo only by being read against the delivery that proved it.
#[must_use = "deferred cargo is the token tree one opaque seat receives"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeferredCargo {
    tokens: GeneratedTree,
}

/// The declaration-grammar material one stamped seat carries, together with the matcher clauses that material consumes.
///
/// The two travel as one value because they are one delivery: the body spells metavariables and the matcher names them, so a body paired with another delivery's matcher expands to a carrier nothing can invoke.
#[must_use = "declared cargo is one stamped body and the clauses its invocation must supply"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredCargo {
    matched: GeneratedTree,
    stamped: GeneratedTree,
}

/// One terminal's proved cargo, read off the delivery that proved it, with the parentage that reading established.
///
/// # Authority
///
/// **There is no road to one that takes a token tree on its own.** The tokens are read off a terminal's own proved delivery and compared against what that delivery carries, so holding one means the cargo it carries is the cargo that terminal proved.
///
/// The source identity, the declaration it stands over, and the digest ride BESIDE the tokens because one act established all four: a value carrying tokens from one terminal and an identity naming another would be a carrier claiming a parentage it does not have.
#[must_use = "proved cargo is one terminal's own tokens and the parentage that reading established"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProvedCargo {
    source: ClosedExpansionId,
    root: Identity<identity::CapturedDeclaration>,
    destination: Destination,
    digest: Identity<identity::OutputBytes>,
    cargo: DeferredCargo,
}

/// What one axis carries, or what happened to whatever would have filled it.
///
/// An empty seat is a shape a reader cannot act on, while "nobody asked for it" and "it does not apply here" are answers to different questions — and the answer is the one the deciding road already gave.
/// The material is the axis's own type, which is what keeps the axes composed: declared material and one terminal's proved cargo are different values, and neither can be seated where the other belongs.
#[must_use = "an axis either carries its material or states what happened to whatever would have filled it"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AxisCargo<Material> {
    /// Nothing filled this axis, and this is what happened to whatever would have.
    Absent {
        /// What happened to the projection that would have filled this axis.
        because: Disposition,
    },
    /// The material this axis carries.
    Carried(Material),
}

/// The three axes one assembly is composed from.
///
/// One value rather than three arguments, so a caller that fills two and forgets the third stops compiling exactly where a missing field does.
#[must_use = "the axes are what one carrier is composed from, whole"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportAxes {
    /// The declaration-grammar material the gate's stamped seat carries.
    pub declared: AxisCargo<DeclaredCargo>,
    /// One terminal's proved test-carrier cargo.
    pub deferred: AxisCargo<ProvedCargo>,
    /// One terminal's proved bench-carrier cargo.
    pub bench: AxisCargo<ProvedCargo>,
}

/// The verified whole one exported carrier is rendered from.
///
/// **This is the only value the carrier's rendering accepts.** A caller holding one has walked the verification and a caller that has not has nothing to hand in, which is what makes "nothing unproved crosses" a shape rather than a rule.
///
/// # Nonclaims
///
/// It claims nothing about the consumption side: whether any target invokes the carrier, whether the pin matches at the consumer's site, and whether the harness's published copy is current are facts on the other side of the crossing.
#[must_use = "an assembly is the verified whole one exported carrier is rendered from"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportAssembly {
    root: Identity<identity::CapturedDeclaration>,
    expectation: SchemaId,
    address: Option<SupportName>,
    declared: AxisCargo<DeclaredCargo>,
    deferred: AxisCargo<ProvedCargo>,
    bench: AxisCargo<ProvedCargo>,
}

/// One way a set of closed outputs does not compose into one carrier.
///
/// No issue is payload-free: every one names the axis, the terminal, the delivery, or the form that disagreed, because "the assembly failed" is not an answer anybody can repair from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssemblyIssue {
    /// A carried axis's terminal stands over a declaration other than the assembly's.
    ///
    /// Both are named and neither is elected: which one the caller meant is the caller's own fact, and either reading is one exported name delivering material from two places.
    RootsDisagree {
        /// The axis whose terminal disagreed.
        axis: CargoAxis,
        /// The declaration the assembly stands over.
        stated: Identity<identity::CapturedDeclaration>,
        /// The declaration that terminal stands over.
        carried: Identity<identity::CapturedDeclaration>,
    },
    /// Two axes read one terminal's one delivery, so one proved cargo would be delivered twice into one target.
    CargoConsumedTwice {
        /// The terminal whose cargo was read twice.
        source: ClosedExpansionId,
        /// The delivery it was read from.
        destination: Destination,
    },
    /// An axis read a delivery other than the one its own row names, so those units would reach a second destination.
    CargoReachesASecondDestination {
        /// The axis that read the wrong delivery.
        axis: CargoAxis,
        /// The delivery it read.
        destination: Destination,
    },
    /// The cargo handed for an axis is not the cargo that terminal's named delivery proved.
    ///
    /// Established where the delivery carries nothing at all as well: a terminal that planned no member into a delivery proved no cargo there.
    CargoNotTheSourcesOwn {
        /// The terminal the cargo claimed to come from.
        source: ClosedExpansionId,
        /// The delivery it claimed to come from.
        destination: Destination,
    },
    /// Both proved axes are carried, and one carrier writes one gate invocation under one form.
    ///
    /// Two forms are two carriers, and the repair is to compose them as two rather than to elect one here.
    TwoFormsCarried,
    /// The form this assembly would be rendered under requires stamped material and none was declared.
    StampedCargoAbsent {
        /// The form whose stamped seat has no empty row.
        form: DeliveryForm,
    },
}

/// How assembly says no.
///
/// Assembly issues are independent and co-establishable — one set of outputs may stand over two declarations AND read one terminal's delivery twice in the same pass — so the body carries every issue the pass established rather than electing a primary one, and says so where it kept only what fits.
#[must_use = "an assembly refusal carries every way the outputs did not compose"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssemblyError {
    body: Capped<AssemblyIssue, ASSEMBLY_ISSUE_LIMIT>,
}

/// How the one road from a carrier plan and a verified assembly to a rendered carrier says no.
///
/// The two rows are DEPENDENT and in that order — a carrier rendered over the wrong declaration is not made lawful by fitting its bound — so exactly one is ever established.
#[must_use = "a shell refusal names why no carrier was rendered for this pair"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellError {
    /// The carrier plan and the assembly do not stand over one declaration.
    ///
    /// Not the axis-level disagreement under another name: that one compares a carried axis's TERMINAL while the assembly is built, and this one compares the VEHICLE's own plan against the finished assembly, which nothing before this road holds both halves of.
    NotOneDeclaration {
        /// The declaration the assembly stands over.
        stated: Identity<identity::CapturedDeclaration>,
        /// The declaration the carrier's own plan stands over.
        planned: Identity<identity::CapturedDeclaration>,
    },
    /// The composed carrier outgrows the declared token magnitude.
    TreeUnbounded {
        /// The declared bound.
        bound: usize,
        /// The observed count.
        observed: usize,
    },
}

/// The exported carrier: the one physical vehicle every crossing rides.
///
/// A rendered definition, exported hidden under its mangled name, whose body is one gate invocation, plus the forwarding address where a delivery has one.
/// It holds its cargo INERT — the expressions inside it name no address until a consumption target expands it — and it executes nothing in an ordinary build.
#[must_use = "a carrier is the exported definition a consumption target invokes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportShell {
    name: ShellName,
    tree: GeneratedTree,
}

/// The kind one carrier delivery is: a verified assembly in, the exported carrier at the declaration site out.
///
/// Its content is the [`SupportAssembly`] itself, so a carrier request cannot be stated over anything a verification pass did not already admit.
/// Its one seat is the declaration site, because an exported `macro_rules!` definition is the one shape the ordinary build may compile while every token inside it stays inert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SupportCarrier;

/// The one alphabet every spelling any crossing renders in identifier position is admitted by, published from the nucleus every road here already reads it through.
pub use guard::rendered_identifier;

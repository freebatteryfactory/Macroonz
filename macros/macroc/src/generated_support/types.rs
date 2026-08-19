//! The assembly home's declarations: the axis roster, what one axis carries or
//! why it carries nothing, one terminal's proved cargo, the closed issue set,
//! the assembly itself, and the joined value a door hands back.
//!
//! Declarations only.
//! Every road that reaches a private field — a proved cargo's provenance, the
//! assembly's seats, the refusal body's one seat, and the joined value's two
//! terminals — lives in `type_guard.rs`, this file's own child.
//!
//! # Nothing here is planned, and nothing here is owned
//!
//! Not one value below is a projection kind, a semantic noun, or a provider
//! declaration. Every token an assembly holds was rendered and PROVED somewhere
//! else and arrives inside a closed expansion; every spelling it carries belongs
//! to the home that rendered it. What this home adds is the physical statement
//! that a set of proved outputs composes into ONE carrier.

use crate::closure::ClosedExpansion;
use crate::plane::{ClosedExpansionId, OutputBytesSubject, ProjectionIdentity};
use crate::planning::{
    CauseAnchoring, EmissionPartition, ExpectedGeneratedSupportSchemaId, ProjectionDisposition,
    TestDescriptorProjection,
};
use crate::test_descriptor::{DeferredCargo, TrialTablePayload};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitude.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many issues one carrier-assembly refusal body
    /// may carry.
    ///
    /// # Bounds
    ///
    /// Eight. The assembly's issues are facts about a fixed set of AXES and about
    /// the one root and the one expectation they stand under, so their count is
    /// bounded by the roster rather than by how much cargo any axis holds — an
    /// assembly of one carried axis and an assembly of three establish from the
    /// same small set. Eight leaves room for every issue every axis can raise at
    /// once and states the number rather than deriving it from the axis roster
    /// beside it, because a magnitude derived from another magnitude reads as a
    /// fact when it is a choice.
    AssemblyIssueLimit = 8,
}

// ---------------------------------------------------------------------------
// The axes.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// The closed roster of cargo axes one carrier composes.
    ///
    /// # Authority
    ///
    /// **Three axes, three materials, and never one product.** The axes carry
    /// genuinely different things — a DECLARED row payload, one terminal's
    /// PROVED deferred cargo, and the bench crossing's material — so a single
    /// sum with one payload would force three vocabularies into one shape and
    /// make "which material may fill which seat" a question nobody answers.
    /// Composing them keeps each axis's own material typed and each axis's own
    /// absence stated.
    ///
    /// An axis's `slot` is the position a refusal names it by, and adding a row
    /// is a law change here — one the compiler collects, since every reading over
    /// the roster is a `match` that stops compiling until the new row says which
    /// partition it delivers from.
    ///
    /// # Nonclaims
    ///
    /// A row says which cargo an axis carries. It says nothing about whether the
    /// carrier has a seat rendered for it: the published grammar's seats are the
    /// RENDERING's fact, and the bench axis in particular has material typed and
    /// no seat of its own yet.
    pub enum CargoAxis {
        /// The descriptor rows the gate's trials seat carries.
        Trial = "trial", "the descriptor rows the gate's trials seat carries";
        /// The proved cargo the gate's deferred seat carries.
        Evaluation = "evaluation", "the proved cargo the gate's deferred seat carries";
        /// The bench material the carrier's reserved seat will carry.
        Bench = "bench", "the bench material the carrier's reserved seat will carry";
    }
}

/// What one axis carries, or what happened to the projection that would have
/// filled it.
///
/// Two postures, and they are different facts rather than one with a missing
/// half. An axis nothing filled carries the DISPOSITION of the projection that
/// would have — generated elsewhere, inapplicable under a cited owner fact,
/// refused with a planning refusal, unavailable under the selected profile, not
/// requested, or excluded by configuration — because "this seat is empty" is a
/// shape a reader cannot act on and "nobody asked for it" and "it refused" are
/// answers to different questions.
///
/// # Authority
///
/// **The reason is the PLANNING road's answer and never this home's.** Why a
/// projection produced nothing is decided where it was planned, so the absence
/// arm carries that home's own value rather than a local roster this home would
/// have to keep in agreement with it.
///
/// # Bounds
///
/// The material is the axis's own type, which is what keeps the axes composed
/// rather than collapsed: a declared payload and one terminal's proved cargo are
/// different values, and neither can be seated where the other belongs.
#[must_use = "an axis either carries its material or states what happened to the projection that would have filled it"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AxisCargo<Material> {
    /// Nothing filled this axis, and this is what happened to the projection
    /// that would have.
    Absent {
        /// What happened to the projection that would have filled this axis.
        because: ProjectionDisposition,
    },
    /// The material this axis carries.
    Carried(Material),
}

/// One closed expansion's proved carrier cargo, read off the terminal that
/// proved it, with the provenance that reading establishes.
///
/// # Authority
///
/// **There is no road to one that takes a token tree on its own.** The tokens
/// are read off a terminal's own proved partition and are compared against what
/// that partition carries, so holding one of these means the cargo it carries is
/// the cargo that closed expansion proved — which is the whole of "no raw or
/// unproved token crossing".
///
/// The source identity and the digest ride BESIDE the tokens because they are
/// established by the same act that read them. A value carrying tokens from one
/// terminal and an identity naming another would be a carrier claiming a
/// parentage it does not have, and every reading downstream would answer
/// correctly about the wrong expansion.
///
/// # Nonclaims
///
/// It claims nothing about whether the shell has been rendered, what it is
/// named, or whether any target invokes it. Those are the carrier's facts.
#[must_use = "proved cargo is one terminal's own tokens and the parentage that reading established"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvedCargo {
    source: ClosedExpansionId,
    root: CauseAnchoring,
    partition: EmissionPartition,
    digest: ProjectionIdentity<OutputBytesSubject>,
    cargo: DeferredCargo,
}

// ---------------------------------------------------------------------------
// The assembly refusal family.
// ---------------------------------------------------------------------------

/// How a set of closed outputs fails to compose into one carrier.
///
/// No issue is payload-free: every one names the axis, the terminal, the
/// partition, or the material that disagreed, because "the assembly failed" is
/// not an answer anybody can repair from.
///
/// # Authority
///
/// **Every issue here is a fact about COMPOSITION and never about meaning.** What
/// a row says, what a copy stands over, and which selection it reads were
/// established where they were rendered; what this family establishes is that a
/// set of already-proved outputs physically belongs in one exported shell.
#[must_use = "an assembly issue names exactly what did not compose"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssemblyIssue {
    /// A carried axis's source terminal stands under a different root than the
    /// assembly does.
    ///
    /// Both roots are named and neither is elected as the offender: which of the
    /// two a caller meant is the caller's own fact, and a carrier composing two
    /// declarations' cargo is one exported name delivering material from two
    /// places whichever one was intended.
    RootsDisagree {
        /// The axis whose source disagreed.
        axis: CargoAxis,
        /// The root the assembly stands under.
        stated: CauseAnchoring,
        /// The root that axis's source terminal stands under.
        carried: CauseAnchoring,
    },
    /// The expectation the carrier's gate would be pinned against is not the one
    /// these services publish.
    ///
    /// The observed bytes travel because the repair is a comparison: a reader
    /// holding only "the expectation is wrong" cannot tell a version-mixed
    /// expectation from one somebody minted beside the published constant.
    SchemaExpectationNotPublished {
        /// The thirty-two bytes the assembly was handed.
        stated: [u8; 32],
    },
    /// Two axes read one terminal's one partition, so one proved cargo would be
    /// delivered twice into one target.
    CargoConsumedTwice {
        /// The terminal whose cargo was read twice.
        source: ClosedExpansionId,
        /// The partition it was read from.
        partition: EmissionPartition,
    },
    /// An axis read a partition other than the carrier partition its own
    /// delivery names, so the units in it would reach a second destination.
    ///
    /// The declaration-site partition is the reachable case and the costly one:
    /// its units are already compiled by the consumer's normal build, and
    /// carrying them again into a consumption target is exactly the tax the
    /// wall's delivery vocabulary exists to refuse.
    CargoReachesASecondDestination {
        /// The axis that read the wrong partition.
        axis: CargoAxis,
        /// The partition it read.
        partition: EmissionPartition,
    },
    /// The cargo handed for an axis is not the cargo that terminal's named
    /// partition proved.
    ///
    /// Established where the partition carries nothing at all as well: a
    /// terminal that planned no member into a partition proved no cargo there,
    /// so nothing handed for it can be that partition's own.
    CargoNotTheSourcesOwn {
        /// The terminal the cargo claimed to come from.
        source: ClosedExpansionId,
        /// The partition it claimed to come from.
        partition: EmissionPartition,
    },
    /// The bench axis carries material and the carrier has no seat to render it
    /// into.
    ///
    /// The carried arm is honestly typed — bench material is real and the
    /// benchmark crossing renders it — and its vehicle is a stated OPENING
    /// CONDITION rather than a gap: the published grammar writes two cargo seats
    /// and neither is the bench seat, so this axis opens when that seat is
    /// declared and refuses until then instead of pretending the material had
    /// somewhere to go.
    BenchVehicleNotOpen,
}

/// The carrier-assembly refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared beside the rest of this home's declarations would put all of them
/// inside the same wall.
pub use guard::CarrierAssembly;

// ---------------------------------------------------------------------------
// The assembly.
// ---------------------------------------------------------------------------

/// The verified physical assembly one exported support shell is rendered from.
///
/// Holding one means a set of closed outputs was proved to compose: every
/// carried axis's cargo is the cargo its named terminal proved, in the partition
/// that axis delivers from; every terminal stands under the assembly's one root;
/// no terminal's partition was read twice; and the expectation the gate will be
/// pinned against is the one these services publish.
///
/// # Authority
///
/// **This is the only value the carrier's composition road accepts.** The shell's
/// own road is crate-internal and this home is its one caller, so a public
/// caller reaching an exported shell has walked the verification or has nothing
/// to hand in — which is what makes "no raw or unproved token crossing" a shape
/// rather than a rule.
///
/// # Nonclaims
///
/// It claims nothing about the consumption side. Whether any target invokes the
/// shell, whether the pin will match at the consumer's site, and whether the
/// harness's published literal is CURRENT are facts on the other side of the
/// wall; this value is the producer's end of them.
#[must_use = "an assembly is the verified whole one exported shell is rendered from"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportAssembly {
    root: CauseAnchoring,
    expectation: ExpectedGeneratedSupportSchemaId,
    trial: AxisCargo<TrialTablePayload>,
    evaluation: AxisCargo<ProvedCargo>,
    bench: AxisCargo<ProvedCargo>,
}

/// What one JOINED door road produced: the kind's own terminal, the carrier
/// terminal that delivers its cargo, and the assembly that joined them.
///
/// # Authority
///
/// **A door that renders a carrier ends at TWO terminals, and this value is
/// both.** The projected terminal is what the declaration IS — the
/// implementations, the codec, the documentation, whatever the kind projects —
/// and the carrier terminal is the exported shell that carries whatever of it
/// was planned into a consumption target. Neither stands for the other: the
/// carrier's cargo is the projected terminal's proved partition, and the
/// carrier's own definition is a member of its own plan.
///
/// Both are DECLARATION-SITE emissions, and a door that emitted one of them
/// would be a door whose carrier is never defined or whose declaration never
/// expands. So the two declaration-site cargos are exactly the two terminals'
/// declaration-site partitions, read off the terminals themselves rather than
/// joined into a third value nobody proved.
///
/// # Bounds
///
/// The projected half is a TYPE PARAMETER, because the coming door wave grows
/// this road to every kind and each kind's door hands back its own value: the
/// refusal-family door hands back its family view over a closed expansion, and
/// another kind's may hand back the terminal itself. Whatever it is, this value
/// carries it whole and reads it through nothing — a seat that restated the
/// projected terminal's cargo, identity, or plan would be a second answer to a
/// question that value already answers.
///
/// The carrier half is NOT a type parameter, and the asymmetry is the wall's:
/// the wall declares one physical carrier, so the terminal that renders it is
/// one kind for every door.
#[must_use = "a joined expansion is both terminals one door produced, and the assembly that joined them"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinedExpansion<Projected> {
    projected: Projected,
    carrier: ClosedExpansion<TestDescriptorProjection>,
    assembly: SupportAssembly,
}

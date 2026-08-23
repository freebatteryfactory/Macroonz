//! The assembly home's declarations: the axis roster, what one axis carries or
//! why it carries nothing, one terminal's proved cargo, the closed issue set,
//! the answer the one road to a rendered shell refuses with, the assembly
//! itself, the joined value a door hands back, and the complete account that
//! joined value stands inside.
//!
//! Declarations only.
//! Every road that reaches a private field — a proved cargo's provenance, the
//! assembly's seats, the refusal body's one seat, the joined value's two
//! terminals, and the account's roster of dispositions — lives in
//! `type_guard.rs`, this file's own child.
//!
//! # Nothing here is planned, and nothing here is owned
//!
//! Not one value below is a projection kind, a semantic noun, or a provider
//! declaration. Every token an assembly holds was rendered and PROVED somewhere
//! else and arrives inside a closed expansion; every spelling it carries belongs
//! to the home that rendered it. What this home adds is the physical statement
//! that a set of proved outputs composes into ONE carrier.

use crate::closure::ClosedExpansion;
use crate::plane::{
    CapturedDeclarationSubject, ClosedExpansionId, OutputBytesSubject, ProjectionIdentity,
};
use crate::planning::{
    ContentAddressing, EmissionPartition, ExpectedGeneratedSupportSchemaId, KindDispositions,
    ProjectionDisposition, TestDescriptorProjection,
};
use crate::test_descriptor::{
    DeferredCargo, ShellRendering, SupportMacroName, TrialTablePayload,
};

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
    ///
    /// The vehicle-level issue does not widen it. That one is established alone,
    /// by the road that holds a carrier plan and an assembly at once, so the body
    /// it stands in carries exactly one issue — the bound governs the composing
    /// pass's co-established set, which is what it was chosen for.
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
/// **Promotion to proved cargo belongs to the road that owns the source's
/// rendering vocabulary, and the promotion road is crate-internal.** What the
/// reading authenticates is the TREE. The envelope around it — the local subject
/// the deferred implementations stand over, and the selectors they read their
/// active points through — is a declaration of the home that rendered them, and
/// no terminal carries a copy of it for the reading to compare against. So the
/// road that hands the envelope in is the road that declared it, and no caller
/// can wrap proved tokens in an envelope of its own and hand back a value whose
/// whole claim is that its contents are one terminal's own.
///
/// The deferred cargo an envelope is built from stays PUBLIC and is untouched by
/// that: it is a declaration value, refused seat by seat at the carrier's own
/// door, and holding one claims nothing about any terminal. What is closed is
/// the promotion, not the declaration.
///
/// A generic deferred-envelope contract is a stated OPENING CONDITION. It opens
/// when a second projection family needs to transport independently declared
/// envelope metadata around proved carrier tokens — at which point the envelope
/// has an owner beside the rendering home, and admitting it is that owner's
/// statement to make rather than a seam somebody widens under pressure.
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
    addressing: ContentAddressing,
    partition: EmissionPartition,
    digest: ProjectionIdentity<OutputBytesSubject>,
    cargo: DeferredCargo,
}

/// One trial declaration's commitment and payload as one carrier-axis fact.
///
/// The commitment cannot be replaced beside the rows because no public field or
/// constructor exposes the pair. The derive road reads both from one captured
/// declaration and the carrier assembly consumes them together.
#[must_use = "declared trial cargo carries both its commitment and its exact row payload"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredTrialCargo {
    commitment: ProjectionIdentity<CapturedDeclarationSubject>,
    payload: TrialTablePayload,
}

// ---------------------------------------------------------------------------
// The assembly refusal family.
// ---------------------------------------------------------------------------

/// How a set of closed outputs fails to compose into one carrier, and how a
/// carrier plan fails to belong to the assembly it would close around.
///
/// No issue is payload-free: every one names the axis, the terminal, the
/// partition, the root, or the material that disagreed, because "the assembly
/// failed" is not an answer anybody can repair from.
///
/// # Authority
///
/// **Every issue here is a fact about COMPOSITION and never about meaning.** What
/// a row says, what a copy stands over, and which selection it reads were
/// established where they were rendered; what this family establishes is that a
/// set of already-proved outputs physically belongs in one exported shell — and
/// that the shell they belong in is the one this declaration planned.
///
/// # Bounds
///
/// The issues are established at TWO seams and they are one family, because they
/// answer one question. The composing pass establishes the axis-level ones while
/// an assembly is built; the road from a plan and an assembly to a shell
/// establishes the vehicle-level one, because that is the first moment both
/// values exist. Two families for one question would derive two related
/// identities for one law's findings.
#[must_use = "an assembly issue names exactly what did not compose"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        stated: ContentAddressing,
        /// The root that axis's source terminal stands under.
        carried: ContentAddressing,
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
    /// The CARRIER PLAN a shell would be rendered from stands under a different
    /// root than the assembly it would close around.
    ///
    /// Not the axis issue above under another name. That one compares a carried
    /// axis's SOURCE TERMINAL against the assembly, and it is established while
    /// the assembly is built, before any plan is in hand. This one compares the
    /// VEHICLE's own plan against the finished assembly, and nothing before the
    /// rendering seam holds both values: an assembly proves that its cargo is one
    /// declaration's, and a carrier plan for a second declaration agrees with
    /// every reading it performs, because the rendered unit is born wearing that
    /// plan's own metadata.
    ///
    /// Both roots are named and neither is elected, on the terms
    /// [`AssemblyIssue::RootsDisagree`] states: which of the two a caller meant
    /// is the caller's own fact, and either reading is one exported name
    /// delivering another declaration's proved cargo.
    CarrierRootIsNotTheAssemblys {
        /// The root the assembly stands under.
        stated: ContentAddressing,
        /// The root the carrier's own plan declares.
        planned: ContentAddressing,
    },
}

/// The carrier-assembly refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared beside the rest of this home's declarations would put all of them
/// inside the same wall.
pub use guard::CarrierAssembly;

/// How the ONE road from a carrier plan and a verified assembly to a rendered
/// shell answers when it does not render.
///
/// Two questions are asked at that seam and they belong to two homes. Whether
/// the plan and the assembly are ONE DECLARATION's is a COMPOSITION fact and it
/// is this home's; whether the tokens fit the carrier's declared magnitude is
/// the CARRIER's fact and it is the test-descriptor home's. This value says
/// which of the two answered, and carries that home's own body whole.
///
/// # Authority
///
/// **It is not a third refusal family and it declares no shape of its own.**
/// Each arm holds the family body its own home established, unwrapped and
/// unsummarized, so nothing here is a second answer to either question: a reader
/// holding this value reads exactly what one of the two homes said, and the
/// projection that turns it into a diagnostic is that home's projection.
///
/// A single family covering both would have to give a root disagreement and a
/// token magnitude one issue roster, one shape, and one related-identity tag —
/// and two homes' facts under one tag derive one related identity for two
/// unrelated observations.
///
/// # Bounds
///
/// It says the shell was not rendered and which home refused. It says nothing
/// about which of the two would have refused had the other not: the composition
/// question is answered first because a shell rendered over the wrong
/// declaration is not made lawful by fitting its bound, so a plan that disagrees
/// with its assembly never reaches the rendering at all.
#[must_use = "a shell composition refusal names which home refused and carries that home's body"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShellComposition {
    /// The carrier plan and the assembly are not one declaration's, so there is
    /// no lawful shell for this pair. This home's own body.
    NotOneDeclarations(CarrierAssembly),
    /// The carrier's own rendering refused. The test-descriptor home's own body,
    /// carried exactly as that home's composition road returned it.
    Rendering(ShellRendering),
}

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
    addressing: ContentAddressing,
    expectation: ExpectedGeneratedSupportSchemaId,
    trial: AxisCargo<DeclaredTrialCargo>,
    evaluation: AxisCargo<ProvedCargo>,
    bench: AxisCargo<ProvedCargo>,
    support: Option<SupportMacroName>,
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

/// One door road's COMPLETE account over one captured surface: what it
/// generated, and what happened to every kind it did not.
///
/// # Authority
///
/// **A door answers for the whole sealed roster or it answers for nothing.** The
/// joined value below it carries what was PRODUCED — the kind's own terminal,
/// the carrier that delivers its cargo, and the assembly that joined them — and
/// it is silent about every kind that produced nothing, because there is nothing
/// of them for it to hold. Silence is exactly what a disposition exists to
/// abolish, so the account seats one beside it: [`KindDispositions`] carries a
/// required seat per row of the enumerated kind roster, and a reader asking why
/// a kind is absent reads the answer instead of inferring it from an emptiness.
///
/// The two halves are not two accounts of one thing. The generated kinds' seats
/// name the one output a disposition names, and the terminals that produced them
/// are read off the joined value — so what was planned, what was proved, and
/// what each build receives are answered once, where they already were.
///
/// # Bounds
///
/// The projected half is a TYPE PARAMETER for the reason
/// [`JoinedExpansion`] states, and this value reads it through nothing at all:
/// which kind that terminal stands for, and therefore which seat of the roster
/// says GENERATED about it, is decided by the DOOR that built the record. A
/// value that decided it here would be this home electing what a door meant.
///
/// It claims nothing about a kind's standing anywhere but at THIS door over
/// THIS surface. A kind unavailable under one door's profile is a fact about
/// that profile, and another door over the same declaration answers its own
/// roster.
#[must_use = "an accounted expansion is what one door produced and what happened to every kind it did not"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountedExpansion<Projected> {
    joined: JoinedExpansion<Projected>,
    dispositions: KindDispositions,
}

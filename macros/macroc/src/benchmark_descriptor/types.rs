//! The benchmark-descriptor home's declarations: the bench row vocabulary in the
//! harness's own field shape, the one-file reporter adapter and the single swap
//! point that makes it backend-agnostic, the shell the two ride into a bench
//! target, and the magnitudes and refusal family this home answers with.
//!
//! Declarations only.
//! Every road that reaches a private field — a row's input-size axis, its work
//! formula, its work-observation bindings, a payload's rows, the adapter's
//! backend spelling, and the shell's tree — lives in `type_guard.rs`, this file's
//! own child.
//!
//! # Nothing of the harness is imported, and nothing of the carrier is redeclared
//!
//! The row vocabulary here is conforming DATA in the harness's declared field
//! shape, exactly as the first crossing's is. The CARRIER is not redeclared: the
//! shell's name, the wall's namespaced name, the twin-rooted path, and the two
//! rename twins are the test-descriptor home's declarations, read from there,
//! because the wall declares one physical carrier and a carrier declared twice is
//! two carriers.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    GeneratedUnitSubject, GeneratorVersionSubject, MeasuredSubject, OwnerIdentityRef,
    ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit,
    WorkCurrencySubject,
};
use crate::planning::CauseAnchoring;
use crate::test_descriptor::{BoundPath, ShellName, WallName};
use crate::token::GeneratedTree;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
// ---------------------------------------------------------------------------

/// The magnitude governing how many sizes one row's input-size axis may state.
///
/// # Bounds
///
/// Thirty-two. A growth class is read off a CURVE, so the axis must carry more
/// than one point — and a curve of more than thirty-two points is a measurement
/// campaign rather than a gate, which is a different thing with a different
/// budget.
///
/// The authority and the number are written together in `type_contract.rs`, one
/// row per family, so a family cannot stand on the compile-time ladder while
/// wearing another road's authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputSizeLimit;

/// The magnitude governing how many bytes one declared work formula may carry.
///
/// # Bounds
///
/// Two hundred and fifty-six. The formula is the declaration's own encoded form
/// rather than a name, because two different formulas an owner happened to name
/// alike would otherwise encode identically — and a formula past this width has
/// stopped being a formula the gate counts work against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkFormulaLimit;

/// The magnitude governing how many work-observation bindings one row may name.
///
/// # Bounds
///
/// Eight. Each observation is one counted quantity the gate reads against the
/// declared formula, and a row observing more than eight has stopped measuring
/// one workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkObservationLimit;

/// The magnitude governing how many rows one bench table may declare.
///
/// # Bounds
///
/// One hundred and twenty-eight, and DELIBERATELY narrower than the trial
/// table's. Every bench row is measured across its whole input-size axis under
/// declared sample and warmup counts, so a bench table's cost is its rows times
/// its axis times its samples — where a trial table's is its rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchRowLimit;

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration.
    /// The shared vocabulary's own refusals are not restated here: a wall name and
    /// a twin-rooted path are built before they reach any road in this home, and
    /// they refuse in the carrier's family where they are declared.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum BenchDeclarationRefusal {
        /// A spelling the rendering writes as a Rust identifier is not one.
        SpellingNotAnIdentifier = "spelling-not-an-identifier",
            "a rendered spelling is not one Rust identifier";
        /// The input-size axis states one size or none, so a growth class read
        /// off it would be read off a point rather than a curve.
        AxisNotACurve = "axis-not-a-curve",
            "an input-size axis states fewer than two sizes";
        /// The axis states more sizes than the declared magnitude.
        AxisUnbounded = "axis-unbounded",
            "an input-size axis states more sizes than the declared magnitude";
        /// Two positions of one axis state one size, so the curve doubles back on
        /// a point it already measured.
        AxisSizeDoubled = "axis-size-doubled",
            "two positions of one input-size axis state one size";
        /// The declared work formula carries no bytes at all, which is a formula
        /// nobody wrote rather than an operation that declares none — an
        /// operation declaring none states that by carrying no formula.
        WorkFormulaEmpty = "work-formula-empty",
            "a declared work formula carries no bytes";
        /// The declared work formula carries more bytes than the declared
        /// magnitude.
        WorkFormulaUnbounded = "work-formula-unbounded",
            "a declared work formula carries more bytes than the declared magnitude";
        /// The row names more work observations than the declared magnitude.
        WorkObservationsUnbounded = "work-observations-unbounded",
            "a bench row names more work observations than the declared magnitude";
        /// The table declares no row at all.
        RowsAbsent = "rows-absent",
            "a bench table declares no row";
        /// The table declares more rows than the declared magnitude.
        RowsUnbounded = "rows-unbounded",
            "a bench table declares more rows than the declared magnitude";
        /// Two rows of one table carry one lens spelling, so the rendered adapter
        /// would declare one function twice.
        LensSpellingDoubled = "lens-spelling-doubled",
            "two rows of one bench table carry one lens spelling";
    }
}

// ---------------------------------------------------------------------------
// The bench row vocabulary, in the harness's field shape.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// The declared contention posture one measurement was taken under.
    ///
    /// A closed pair, stated ALWAYS. A measurement taken with the host to itself
    /// and a measurement taken with declared competing work present are different
    /// measurements, and a measurement under an undeclared posture is
    /// inadmissible — which is exactly why there is no third arm standing for
    /// "unstated".
    pub enum ContentionPosture {
        /// The host was measured with no declared competing work present.
        Uncontended = "uncontended",
            "measured with the host to itself";
        /// The host was measured with declared competing work present.
        Contended = "contended",
            "measured with declared competing work present";
    }
}

/// One declared work formula, as the declaration's own encoded bytes.
///
/// # Bounds
///
/// Carried as BYTES rather than as a name, because two different formulas an
/// owner happened to name alike would encode identically — and the gate reads
/// work counts against the formula rather than against what it is called.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkFormula {
    encoded: Bounded<u8, WorkFormulaLimit>,
}

/// The gate's own declared tolerances, stated beside the row they govern.
///
/// # Authority
///
/// **Spec, not vibes.** These are declared constants in the descriptor rather
/// than numbers a runner tuned, so a threshold a measurement is judged against is
/// a value somebody wrote down and can be held to.
///
/// # Bounds
///
/// The ratio threshold is a COUNT in the unit the bench contract declares, never
/// a fraction: a float in a specification is a number nobody can compare exactly,
/// and the schema's own field shape for every budget is a count.
///
/// A record with three named seats rather than a roster of counts, because the
/// schema's roster is positional and a positional roster is a shape that can be
/// declared short — a table whose second budget silently became its third is a
/// gate judging against the wrong tolerance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredBudgets {
    /// How many samples the gate takes at each point of the axis.
    pub samples: u64,
    /// How many warmup iterations run before sampling starts.
    pub warmup: u64,
    /// The ratio the planted-worse gap must clear, as a declared count.
    pub ratio_threshold: u64,
}

/// The four namespaced references one bench row states about itself.
///
/// Every seat is public and required. The preflight and the planted-worse
/// falsifier are REFERENCES — the callables that stand behind them ride the
/// binding, exactly as a descriptor row references its check rather than carrying
/// one — and the complexity claim is a NEUTRAL reference, because a standalone
/// public vocabulary never names a product type and the machine maps its own
/// complexity contract into this seat from the product side.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchReferences {
    /// What is measured.
    pub workload: WallName,
    /// The correctness preflight the host order runs first.
    pub correctness_preflight: WallName,
    /// The planted-worse falsifier the host order runs second.
    pub planted_worse: WallName,
    /// The neutral complexity claim this row's envelope stands under.
    pub complexity_claim: WallName,
}

/// What one row declares about how it is measured, rather than about what it
/// measures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchMeasurement {
    /// The gate's declared tolerances.
    pub budgets: DeclaredBudgets,
    /// The declared contention posture.
    pub contention: ContentionPosture,
    /// The declared work formula, where the operation states one.
    ///
    /// Optional because only some operations declare one, and the absence is a
    /// STATED fact: where a formula is declared the gate reads work counts
    /// against it and wall time is the secondary human observation, and where
    /// none is declared there is no work count to read.
    pub work_formula: Option<WorkFormula>,
}

/// What makes one bench row measurable: the callables the host order invokes, in
/// the order it invokes them, and the work observations it reads.
///
/// # Authority
///
/// **The host order is law and this shape carries it.** The preflight trial
/// passes; the planted-worse gate proves the measurement distinguishes the
/// declared class; only then is the measurement backend invoked. A row that could
/// name a measured callable without naming the two gates is a row that would be
/// benchmarked without either — so all three seats are required, and there is no
/// arm for a row that skips one.
///
/// # Bounds
///
/// The preflight is a BINDING and a binding does not pass by itself, so the
/// invocation it runs under is the shell's own declared-budgets argument — one
/// declared profile for the whole target rather than a second one this row
/// invented.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchAttachment {
    /// The callable under measurement.
    pub measured: BoundPath,
    /// The deliberately worse realization the gate must separate from it.
    pub planted_worse: BoundPath,
    /// The correctness preflight's own binding.
    pub preflight: BoundPath,
    observations: Bounded<BoundPath, WorkObservationLimit>,
}

/// One bench row, in the harness's declared field shape, plus the lens name the
/// rendered adapter declares it under.
///
/// # Bounds
///
/// The lens spelling is not a row field — the harness's roster has no seat for it
/// — and it is carried here because the adapter must name the function it
/// registers with a backend, and a producer that did not name its lens would be
/// handing the adapter an unnamable row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchRow {
    lens: String,
    references: BenchReferences,
    axis: Bounded<u64, InputSizeLimit>,
    measurement: BenchMeasurement,
    attachment: BenchAttachment,
}

/// The complete payload one bench table is declared from.
///
/// # Bounds
///
/// The provenance is fixed to the PRODUCED form for the reason the trial table's
/// is: a table this home rendered was emitted by a producer by construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchTablePayload {
    module: String,
    table: WallName,
    producer: WallName,
    rows: NonEmptyBounded<BenchRow, BenchRowLimit>,
}

// ---------------------------------------------------------------------------
// The reporter adapter and its one swap point.
// ---------------------------------------------------------------------------

/// The measurement backend the rendered adapter binds the neutral table to.
///
/// # Authority
///
/// **This value is the ONE swap point.** Every backend-naming token the adapter
/// renders — the attribute on each registered function and the call the report
/// road makes — is spelled from this one value, so a consumer changing backends
/// changes one declared name and nothing else. Backend-agnostic by construction
/// rather than by a promise: there is no second place a backend name can enter
/// the rendering.
///
/// # Bounds
///
/// The spelling is what the consumer's bench target reaches the backend by, so a
/// renamed dev dependency is named the way the consumer named it. It is refused
/// unless it is one Rust identifier, because it is written in path position.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BenchBackend {
    spelling: String,
}

/// The one-file reporter adapter: the module the neutral bench table is bound to
/// a backend inside.
///
/// # Nonclaims
///
/// The adapter REPORTS and never renders a verdict. The host order's gates decide
/// whether a measurement was admissible; the backend measures and prints, and a
/// backend that returned a verdict would be a second authority over what the
/// numbers mean.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchReporterAdapter {
    module: String,
    backend: BenchBackend,
}

// ---------------------------------------------------------------------------
// The carrier's cargo for this crossing.
// ---------------------------------------------------------------------------

/// The benchmark projection's rendered delivery: the bench table and the reporter
/// adapter, carried into a bench target by the same shell the first crossing
/// rides.
///
/// # Bounds
///
/// The seats are exactly what a rendered unit is rebuilt from — role, semantic
/// key, profile at its version, origin trail, and the tree — plus the exported
/// name the bench target invokes it by.
#[must_use = "a benchmark shell is the carrier the bench target invokes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchmarkShell {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    name: ShellName,
    tree: GeneratedTree,
}

/// What a benchmark-descriptor plan decided, read off the plan's own public
/// surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or the unit it measures would be an account that
/// sometimes says less than it knows.
///
/// # Bounds
///
/// There is no verified-CLAIM seat, and the absence is the honest shape rather
/// than a dropped fact: the plan's kind content declares none
/// ([`BenchmarkDescriptorContent`](crate::planning::BenchmarkDescriptorContent)),
/// because the harness's bench row roster carries a NEUTRAL complexity reference
/// — a standalone public vocabulary never names a product type — so the
/// reference itself is part of the caller-supplied row material and the claim a
/// product's own evidence home declares is mapped onto it at the PRODUCT's
/// integration. A product claim carried here would reach no emitted seat and
/// would put the product's vocabulary inside the statement a neutral crossing is
/// rendered from.
///
/// # Nonclaims
///
/// A benchmark is evidence about one realization, never a specification. Holding
/// this claims the plan carries these facts under its one rendered role, and
/// nothing about what any realization must do.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchmarkPlan {
    /// The rendered role the shell stands for.
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
    /// The rendering engine the shell is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// The unit measured.
    pub measured: OwnerIdentityRef<MeasuredSubject>,
    /// The named work currency the envelope is stated in.
    pub work_currency: OwnerIdentityRef<WorkCurrencySubject>,
}

/// How reading a plan into [`BenchmarkPlan`] disagrees with the plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkPlanIssue {
    /// The plan declares no member under its kind's one rendered role, so there
    /// is no shell to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands somewhere other than the declaration site.
    DestinationNotDeclarationSite {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
}

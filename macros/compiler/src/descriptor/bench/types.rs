//! The bench home's declarations: the kind, its two seats, the question it owes, the bench row vocabulary in the harness's own field shape, and the one-file reporter adapter with the single value a consumer swaps to change measurement backends.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.

use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{BoundPath, FunctionName, ModuleName, Name};

#[path = "type_guard.rs"]
mod guard;

/// Sizes one row's input-size axis may state.
///
/// A growth class is read off a CURVE, so the axis must carry more than one point — and a curve of more than this is a measurement campaign rather than a gate, which is a different thing with a different budget.
pub const INPUT_SIZE_LIMIT: usize = 32;

/// Bytes one declared work formula may carry.
///
/// The formula is the declaration's own encoded form rather than a name, because two different formulas an owner happened to name alike would otherwise encode identically.
pub const WORK_FORMULA_LIMIT: usize = 256;

/// Work-observation bindings one row may name.
///
/// Each observation is one counted quantity the gate reads against the declared formula, and a row observing more than this has stopped measuring one workload.
pub const WORK_OBSERVATION_LIMIT: usize = 8;

/// Rows one bench table may declare.
///
/// Deliberately narrower than a trial table's: every bench row is measured across its whole axis under declared sample and warmup counts, so a bench table's cost is its rows times its axis times its samples, where a trial table's is its rows.
pub const BENCH_ROW_LIMIT: usize = 128;

/// The kind one bench declaration produces: a bench table and the adapter that binds it to a measurement backend, both delivered to the consumer's bench target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchTable;

/// The two seats a bench rendering fills.
///
/// Two rather than one, because they are two independent units: the table is cargo the carrier's gate forwards, and the adapter is an item beside it.
/// A rendering that produced one and not the other is caught by the seat rather than by a count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchRole {
    /// The module carrying every declared row.
    Table,
    /// The one-file reporter adapter that binds the table to a backend.
    Adapter,
}

/// The question a bench table owes beyond the universal ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchQuestion {
    /// Which benchmarks measure the unit this table stands for.
    WhichBenchmarksMeasure,
}

/// The typed answer to [`BenchQuestion`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BenchAnswer {
    /// The table that measures, and how many rows it declares.
    MeasuringBenchmarks {
        /// The table's own namespaced name.
        table: Name,
        /// How many rows stand under it.
        rows: u64,
    },
}

/// The declared contention posture one measurement was taken under.
///
/// A measurement under an undeclared posture is inadmissible, and the FIELD is what enforces that: every row states this seat, so "unstated" is not a value and never becomes one — which holds whatever the roster's width is.
///
/// One arm, because one arm is all the declared facts support.
/// It says exactly what every row this home can render says: the measurement was taken with no competing work DECLARED beside it. Nothing here claims the host was quiet — that is a fact about a machine, and this is a fact about a declaration.
///
/// A contended arm returns when the first contended benchmark arrives carrying the facts its payload is designed from, and the schema identity moves then because the closed choice grew an arm.
/// What stays refused in the meantime is a bare contended arm carrying nothing, which is the undeclared measurement this seat exists to rule out wearing the word meant to rule it out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentionPosture {
    /// The measurement was taken with no competing work declared beside it.
    NoDeclaredContention,
}

/// One road the measurement backend publishes.
///
/// The backend is a dependency the consumer names rather than something the harness publishes, so its roads are their own small table and every one of them is spelled from the adapter's single declared backend value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendRoad {
    /// The attribute one registered function wears.
    Bench,
    /// The clause the axis is handed to.
    Args,
    /// The road that keeps a measured value from being optimized away.
    BlackBox,
    /// The road that runs the registered functions.
    Main,
}

/// One declared work formula, as the declaration's own encoded bytes.
///
/// Carried as BYTES rather than as a name, because two different formulas an owner happened to name alike would encode identically — and the gate reads work counts against the formula rather than against what it is called.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkFormula {
    encoded: Bounded<u8, WORK_FORMULA_LIMIT>,
}

/// The gate's own declared tolerances, stated beside the row they govern.
///
/// Declared constants in the descriptor rather than numbers a runner tuned, so a threshold a measurement is judged against is a value somebody wrote down and can be held to.
///
/// A record of three named seats rather than a roster of counts, because the schema's roster is positional and a positional roster is a shape that can be declared short — a table whose second budget silently became its third is a gate judging against the wrong tolerance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Budgets {
    /// How many samples the gate takes at each point of the axis.
    pub samples: u64,
    /// How many warmup iterations run before sampling starts.
    pub warmup: u64,
    /// The ratio the planted-worse gap must clear, as a declared count.
    pub ratio_threshold: u64,
}

/// The four namespaced references one bench row states about itself.
///
/// The preflight and the planted-worse falsifier are REFERENCES — the callables that stand behind them ride the attachment — and the complexity claim is a NEUTRAL reference, because a standalone public vocabulary never names a consumer's type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct References {
    /// What is measured.
    pub workload: Name,
    /// The correctness preflight the host order runs first.
    pub correctness_preflight: Name,
    /// The planted-worse falsifier the host order runs second.
    pub planted_worse: Name,
    /// The neutral complexity claim this row's envelope stands under.
    pub complexity_claim: Name,
}

/// What one row declares about how it is measured, rather than about what it measures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Measurement {
    /// The gate's declared tolerances.
    pub budgets: Budgets,
    /// The declared contention posture.
    pub contention: ContentionPosture,
    /// The declared work formula, where the operation states one.
    ///
    /// Optional because only some operations declare one, and the absence is a STATED fact: where a formula is declared the gate reads work counts against it, and where none is declared there is no work count to read.
    pub work_formula: Option<WorkFormula>,
}

/// What makes one bench row measurable: the callables the host order invokes, in the order it invokes them, and the work observations it reads.
///
/// The host order is law and this shape carries it: the preflight passes, the planted-worse gate proves the measurement distinguishes the declared class, and only then is the backend invoked.
/// All three seats are required, so a row that would be benchmarked without either gate is unwritable rather than refused.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attachment {
    /// The callable under measurement.
    pub measured: BoundPath,
    /// The deliberately worse realization the gate must separate from it.
    pub planted_worse: BoundPath,
    /// The correctness preflight's own binding.
    pub preflight: BoundPath,
    observations: Bounded<BoundPath, WORK_OBSERVATION_LIMIT>,
}

/// One bench row, in the harness's declared field shape, plus the lens the rendered adapter registers it under.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    lens: FunctionName,
    references: References,
    axis: Bounded<u64, INPUT_SIZE_LIMIT>,
    measurement: Measurement,
    attachment: Attachment,
}

/// The measurement backend the rendered adapter binds the neutral table to.
///
/// This value is the ONE swap point. Every backend-naming token the adapter renders is spelled from it, so a consumer changing backends changes one declared name and nothing else — backend-agnostic by construction rather than by a promise, because there is no second place a backend name can enter the rendering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Backend(String);

/// The one-file reporter adapter: the module the neutral bench table is bound to a backend inside.
///
/// The adapter REPORTS and never renders a verdict. The host order's gates decide whether a measurement was admissible; the backend measures and prints, and a backend that returned a verdict would be a second authority over what the numbers mean.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adapter {
    module: ModuleName,
    backend: Backend,
}

/// The complete payload one bench delivery is declared from: the table's rows, and the adapter that binds them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Benches {
    module: ModuleName,
    table: Name,
    rows: NonEmpty<Row, BENCH_ROW_LIMIT>,
    adapter: Adapter,
}

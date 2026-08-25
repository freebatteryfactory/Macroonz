//! The bench home's declarations: the kind, its two seats, the question it owes, and the neutral benchmark declaration that target-owned executable facts complete.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.

use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{FunctionName, HelperRefusal, ModuleName, Name, SupportName};

#[path = "type_guard.rs"]
mod guard;

/// The transcript position a captured reading of this grammar is separated by.
///
/// The helper readings of one declaration share the captured-helper role and are told apart by position alone, in one closed space across the grammars the descriptor home declares: this one is the third.
pub const BENCH_HELPER_POSITION: u32 = 2;

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

/// The kind one benchmark declaration produces: a bench table and its target-supplied report reader.
///
/// The two units land apart because they are two materials: the table is stamp-grammar material delivered at the declaration site, and the target-supplied report reader is typed Rust delivered as bench-target cargo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchTable;

/// The two seats a bench rendering fills.
///
/// Two rather than one, because they are two independent units: the table is cargo the carrier's gate forwards, and the report reader is an item beside it.
/// A rendering that produced one and not the other is caught by the seat rather than by a count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchRole {
    /// The function carrying the stamped benchmark table.
    Table,
    /// The typed report-reader seat beside the table.
    Reporter,
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
/// Four named seats rather than a positional roster, so no threshold can silently acquire a default denominator or move into a neighboring seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Budgets {
    /// How many samples the gate takes at each point of the axis.
    pub samples: u32,
    /// How many warmup iterations run before sampling starts.
    pub warmups: u32,
    /// The numerator of the exact ratio the planted-worse gap must clear.
    pub ratio_numerator: u64,
    /// The denominator of the exact ratio the planted-worse gap must clear.
    pub ratio_denominator: u64,
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

/// One benchmark row's declaration-owned facts, plus the lens its target-owned expressions arrive under.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    lens: FunctionName,
    references: References,
    axis: Bounded<u64, INPUT_SIZE_LIMIT>,
    measurement: Measurement,
    observations: Bounded<Name, WORK_OBSERVATION_LIMIT>,
}

/// The module that carries one target-supplied `fn(&BenchReport)` value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reporter {
    module: ModuleName,
}

/// How one bench helper body was not read.
///
/// Its own type, because a diagnostic's family tag is a fact about the type: this grammar is a declaration's bench reading, and the trial and mutation grammars each carry their own.
#[must_use = "a bench capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchCaptureError(HelperRefusal);

/// The complete neutral benchmark declaration one delivery is projected from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchmarkDeclaration {
    support: SupportName,
    table_function: FunctionName,
    table: Name,
    rows: NonEmpty<Row, BENCH_ROW_LIMIT>,
    reporter: Reporter,
}

//! The public vocabulary of one benchmark declaration and its derived row identity.

#[path = "type_guard.rs"]
mod guard;

use crate::descriptor::{EncodeRefusal, NamespacedName};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use std::num::NonZeroU32;

/// The derivation domain and starting position of the benchmark-row identity family.
pub const BENCH_ROW_KEY_TAG: DomainTag =
    DomainTag::declared("bench-row-key", IdentityProfileVersion::declared(1));

/// The workload one row measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkloadRef(NamespacedName);

/// The correctness preflight one row runs before it measures anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreflightRef(NamespacedName);

/// The deliberately worse control one row measures against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlantedWorseRef(NamespacedName);

/// The complexity claim a row is judged under, which this home never interprets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComplexityClaimRef(NamespacedName);

/// One kind of work a callable may count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkObservationRef(NamespacedName);

/// One authored benchmark table's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenchTableName(NamespacedName);

/// At least two distinct input sizes, in the order they were authored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSizeAxis(Vec<u64>);

/// Why an input-size axis was not admitted.
#[must_use = "a refusal is the reason an input-size axis was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSizeAxisRefusal {
    /// Fewer than two sizes were declared, so no growth relation can be observed.
    TooShort {
        /// How many sizes were declared.
        found: usize,
    },
    /// One size appeared twice on the axis.
    DuplicateSize {
        /// The repeated size.
        size: u64,
        /// Its first position.
        first: usize,
        /// Its repeated position.
        duplicate: usize,
    },
}

/// One ratio held as two integers, so a comparison never depends on rounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactRatio {
    numerator: u64,
    denominator: u64,
}

/// The sample count, warmup count, and exact gap ratio one row declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredBudgets {
    samples: NonZeroU32,
    warmups: u32,
    ratio: ExactRatio,
}

/// Why declared budgets were not admitted.
#[must_use = "a refusal is the reason benchmark budgets were not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredBudgetsRefusal {
    /// No sample was declared, so no work evidence can be produced.
    NoSamples,
    /// The gap ratio's numerator was zero, so no positive gap was declared.
    ZeroRatioNumerator,
    /// The gap ratio's denominator was zero.
    ZeroRatioDenominator,
}

/// The contention posture a row is declared and invoked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentionPosture {
    /// The caller declares no contended environment.
    NoDeclaredContention,
}

/// Bytes the owner spells to say what work it expects, carried through unread.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkFormula {
    bytes: Vec<u8>,
}

/// Why a work formula was not admitted.
#[must_use = "a refusal is the reason a work formula was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkFormulaRefusal {
    /// A present formula carried no bytes, where absence is spelled `None`.
    Empty,
}

/// The identity derived from one complete row declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenchRowKey(ContentAddress);

/// The four names one row joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchReferences {
    workload: WorkloadRef,
    preflight: PreflightRef,
    planted_worse: PlantedWorseRef,
    complexity: ComplexityClaimRef,
}

/// The four measurement facts one row declares.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchMeasurement {
    input_sizes: InputSizeAxis,
    budgets: DeclaredBudgets,
    contention: ContentionPosture,
    formula: Option<WorkFormula>,
}

/// One immutable row: eight declared facts and the identity derived from all of them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchRow {
    references: BenchReferences,
    measurement: BenchMeasurement,
    key: BenchRowKey,
}

/// Why a benchmark row was not built.
#[must_use = "a refusal is the reason a benchmark row was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchRowRefusal {
    /// The row's canonical preimage outgrew the public width checked before shared identity framing, an unreachable compatibility ceiling on supported targets.
    Encoding(EncodeRefusal),
}

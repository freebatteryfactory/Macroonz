#![doc = include_str!("README.md")]

mod encode;
mod types;

pub use types::{
    BENCH_ROW_KEY_TAG, BenchMeasurement, BenchReferences, BenchRow, BenchRowKey, BenchRowRefusal,
    BenchTableName, ComplexityClaimRef, ContentionPosture, DeclaredBudgets, DeclaredBudgetsRefusal,
    ExactRatio, InputSizeAxis, InputSizeAxisRefusal, PlantedWorseRef, PreflightRef, WorkFormula,
    WorkFormulaRefusal, WorkObservationRef, WorkloadRef,
};

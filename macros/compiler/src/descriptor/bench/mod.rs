#![doc = include_str!("README.md")]

mod capture;
mod render;
mod type_contract;
mod types;

pub use capture::captured;
pub use render::{
    axis_literals, bench_table, budgets, contention, declared_row, matched_clauses, observations,
    reporter, row_expression, work_formula,
};
pub use types::{
    BENCH_HELPER_POSITION, BENCH_ROW_LIMIT, BenchAnswer, BenchCaptureError, BenchQuestion,
    BenchRole, BenchTable, BenchmarkDeclaration, Budgets, ContentionPosture, INPUT_SIZE_LIMIT,
    Measurement, References, Reporter, Row, WORK_FORMULA_LIMIT, WORK_OBSERVATION_LIMIT,
    WorkFormula,
};

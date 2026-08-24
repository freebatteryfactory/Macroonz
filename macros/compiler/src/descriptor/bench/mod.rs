#![doc = include_str!("README.md")]

mod render;
mod type_contract;
mod types;

pub use render::{
    axis_literals, bench_attribute, bench_table, budgets, contention, declared_row, observations,
    path, registered_function, reporter_adapter, row_expression, row_module, work_formula,
};
pub use type_contract::BUDGET_ORDER;
pub use types::{
    Adapter, Attachment, BENCH_ROW_LIMIT, Backend, BackendRoad, BenchAnswer, BenchQuestion,
    BenchRole, BenchTable, Benches, Budgets, ContentionPosture, INPUT_SIZE_LIMIT, Measurement,
    References, Row, WORK_FORMULA_LIMIT, WORK_OBSERVATION_LIMIT, WorkFormula,
};

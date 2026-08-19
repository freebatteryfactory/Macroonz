#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::benchmark_plan;
pub use render::{
    ARGS_CLAUSE, BACKEND_MAIN, BENCH_ATTRIBUTE, BENCH_BINDING, BENCH_BINDING_ROAD, BENCH_ROW,
    BENCH_ROW_ROAD, BENCH_TABLE_STAMP, BLACK_BOX, COMPLEXITY_CLAIM_REF, CONTENTION_POSTURE,
    DECLARED_BUDGETS, DECLARED_BUDGETS_ROAD, MEASURED_FUNCTION, PLANTED_WORSE_FUNCTION,
    PLANTED_WORSE_REF, PREFLIGHT_REF, REPORT_FUNCTION, ROW_CLAUSE, SIZE_PARAMETER, WORKLOAD_REF,
    WORK_FORMULA, WORK_FORMULA_ROAD, axis, bench_attribute, bench_row_expression, bench_table,
    budgets, byte_literal, contention, count_literal, declared_row, observations,
    registered_function, reporter_adapter, row_module, work_formula,
};
pub use type_contract::{BUDGET_ORDER, CROSSING_OWED, CrossingOwed};
pub use types::{
    BenchAttachment, BenchBackend, BenchDeclarationRefusal, BenchMeasurement, BenchReferences,
    BenchReporterAdapter, BenchRow, BenchRowLimit, BenchTablePayload, BenchmarkPlan,
    BenchmarkPlanIssue, BenchmarkShell, ContentionPosture, DeclaredBudgets, InputSizeLimit,
    WorkFormula, WorkFormulaLimit, WorkObservationLimit,
};

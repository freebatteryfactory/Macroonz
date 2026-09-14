//! The benchmark carrier receives the measured work and its independent worse control.

macroonz::support! { job_bench_support {
    declaring: crate,
    reporter: crate::benchmark::read::qualified,
    dispatch_cost_measured: crate::benchmark::work::counted_dispatches,
    dispatch_cost_planted_worse: crate::benchmark::work::quadratic_dispatches,
    dispatch_cost_judge: macroonz::harness::bench::WorkJudgeBinding::bound(
        macroonz::harness::bench::ComplexityClaimRef::named("neutral-job", "linear-dispatches")?,
        crate::benchmark::judge::fixed_axis_judge,
    ),
    dispatch_cost_preflight: crate::benchmark::preflight::completion()?,
} }

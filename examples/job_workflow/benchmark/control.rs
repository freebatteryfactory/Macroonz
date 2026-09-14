//! The same declaration receives an undistinguished control and must refuse.

macroonz::support! { job_bench_support {
    declaring: crate,
    reporter: crate::benchmark::read::undistinguished,
    dispatch_cost_measured: crate::benchmark::work::counted_dispatches,
    dispatch_cost_planted_worse: crate::benchmark::work::counted_dispatches,
    dispatch_cost_judge: macroonz::harness::bench::WorkJudgeBinding::bound(
        macroonz::harness::bench::ComplexityClaimRef::named("neutral-job", "linear-dispatches")?,
        crate::benchmark::judge::fixed_axis_judge,
    ),
    dispatch_cost_preflight: crate::benchmark::preflight::completion()?,
} }
